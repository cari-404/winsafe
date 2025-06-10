use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};

pub(in crate::kernel) struct DirListIter<'a> {
	dir_path: String,
	filter: Option<&'a str>,
	hfind: Option<FindCloseGuard>,
	wfd: WIN32_FIND_DATA,
	no_more: bool,
}

impl<'a> Iterator for DirListIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.no_more {
			return None;
		}

		let found = match &self.hfind {
			None => {
				// first pass
				let dir_final = match self.filter {
					None => format!("{}\\*", self.dir_path),
					Some(filter) => format!("{}\\{}", self.dir_path, filter),
				};

				let found = match HFINDFILE::FindFirstFile(&dir_final, &mut self.wfd) {
					Err(e) => {
						// An actual error happened.
						self.no_more = true; // prevent further iterations
						return Some(Err(e)); // and return the error
					},
					Ok((hfind, found)) => {
						// Call succeeded, bool returned.
						self.hfind = Some(hfind); // store our find handle
						found
					},
				};
				found
			},
			Some(hfind) => {
				// subsequent passes
				match hfind.FindNextFile(&mut self.wfd) {
					Err(e) => {
						// An actual error happened.
						self.no_more = true; // prevent further iterations
						return Some(Err(e)); // and return the error
					},
					Ok(found) => found, // call succeeded, bool returned
				}
			},
		};

		if found {
			// A file was found.
			let file_name = self.wfd.cFileName();
			if file_name == "." || file_name == ".." {
				// Skip the dot ones.
				self.next()
			} else {
				// Assembly the full path and return it.
				Some(Ok(format!("{}\\{}", self.dir_path, self.wfd.cFileName())))
			}
		} else {
			None // no file found, halt
		}
	}
}

impl<'a> DirListIter<'a> {
	#[must_use]
	pub(in crate::kernel) fn new(dir_path: String, filter: Option<&'a str>) -> Self {
		Self {
			dir_path: path::rtrim_backslash(&dir_path).to_owned(),
			filter,
			hfind: None,
			wfd: WIN32_FIND_DATA::default(),
			no_more: false,
		}
	}
}

pub(in crate::kernel) struct DirWalkIter<'a> {
	runner: DirListIter<'a>,
	subdir_runner: Option<Box<DirWalkIter<'a>>>,
	no_more: bool,
}

impl<'a> Iterator for DirWalkIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.no_more {
			return None;
		}

		match &mut self.subdir_runner {
			None => {
				let cur_file = self.runner.next();
				match cur_file {
					Some(cur_file) => {
						// A file was found.
						match cur_file {
							Err(e) => {
								// Actually an error.
								self.no_more = true; // prevent further iterations
								Some(Err(e)) // return the error
							},
							Ok(cur_file) => {
								if path::is_directory(&cur_file) {
									self.subdir_runner = Some(Box::new(Self::new(cur_file))); // recursively
									self.next()
								} else {
									Some(Ok(cur_file))
								}
							},
						}
					},
					None => None, // no file found, halt
				}
			},
			Some(subdir_runner) => {
				let inner_file = subdir_runner.next();
				match inner_file {
					None => {
						// Subdir_runner finished his work.
						self.subdir_runner = None;
						self.next()
					},
					Some(inner_file) => Some(inner_file),
				}
			},
		}
	}
}

impl<'a> DirWalkIter<'a> {
	#[must_use]
	pub(in crate::kernel) fn new(dir_path: String) -> Self {
		Self {
			runner: DirListIter::new(dir_path, None),
			subdir_runner: None,
			no_more: false,
		}
	}
}

pub(in crate::kernel) struct HheapHeapwalkIter<'a> {
	hheap: &'a HHEAP,
	entry: PROCESS_HEAP_ENTRY,
	has_more: bool,
}

impl<'a> Iterator for HheapHeapwalkIter<'a> {
	type Item = SysResult<&'a PROCESS_HEAP_ENTRY>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		match unsafe { ffi::HeapWalk(self.hheap.ptr(), pvoid(&mut self.entry)) } {
			0 => {
				self.has_more = false; // no further iterations
				match GetLastError() {
					co::ERROR::NO_MORE_ITEMS => None, // search completed successfully
					err => Some(Err(err)),            // actual error
				}
			},
			_ => {
				// Returning a reference cannot be done until GATs
				// stabilization, so we simply cheat the borrow checker.
				let ptr = &self.entry as *const PROCESS_HEAP_ENTRY;
				Some(Ok(unsafe { &*ptr }))
			},
		}
	}
}

impl<'a> HheapHeapwalkIter<'a> {
	#[must_use]
	pub(in crate::kernel) fn new(hheap: &'a HHEAP) -> Self {
		Self {
			hheap,
			entry: PROCESS_HEAP_ENTRY::default(),
			has_more: true,
		}
	}
}

pub(in crate::kernel) struct HprocesslistHeapIter<'a> {
	hpl: &'a mut HPROCESSLIST,
	hl32: HEAPLIST32,
	first_pass: bool,
	has_more: bool,
}

impl<'a> Iterator for HprocesslistHeapIter<'a> {
	type Item = SysResult<&'a HEAPLIST32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Heap32ListFirst(&mut self.hl32)
		} else {
			self.hpl.Heap32ListNext(&mut self.hl32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.hl32 as *const HEAPLIST32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no heap found
				}
			},
		}
	}
}

impl<'a> HprocesslistHeapIter<'a> {
	#[must_use]
	pub(in crate::kernel) fn new(hpl: &'a mut HPROCESSLIST) -> Self {
		Self {
			hpl,
			hl32: HEAPLIST32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

pub(in crate::kernel) struct HprocesslistModuleIter<'a> {
	hpl: &'a mut HPROCESSLIST,
	me32: MODULEENTRY32,
	first_pass: bool,
	has_more: bool,
}

impl<'a> Iterator for HprocesslistModuleIter<'a> {
	type Item = SysResult<&'a MODULEENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Module32First(&mut self.me32)
		} else {
			self.hpl.Module32Next(&mut self.me32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.me32 as *const MODULEENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no module found
				}
			},
		}
	}
}

impl<'a> HprocesslistModuleIter<'a> {
	#[must_use]
	pub(in crate::kernel) fn new(hpl: &'a mut HPROCESSLIST) -> Self {
		Self {
			hpl,
			me32: MODULEENTRY32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

pub(in crate::kernel) struct HprocesslistProcessIter<'a> {
	hpl: &'a mut HPROCESSLIST,
	pe32: PROCESSENTRY32,
	first_pass: bool,
	has_more: bool,
}

impl<'a> Iterator for HprocesslistProcessIter<'a> {
	type Item = SysResult<&'a PROCESSENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Process32First(&mut self.pe32)
		} else {
			self.hpl.Process32Next(&mut self.pe32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.pe32 as *const PROCESSENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no process found
				}
			},
		}
	}
}

impl<'a> HprocesslistProcessIter<'a> {
	#[must_use]
	pub(in crate::kernel) fn new(hpl: &'a mut HPROCESSLIST) -> Self {
		Self {
			hpl,
			pe32: PROCESSENTRY32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

pub(in crate::kernel) struct HprocesslistThreadIter<'a> {
	hpl: &'a mut HPROCESSLIST,
	te32: THREADENTRY32,
	first_pass: bool,
	has_more: bool,
}

impl<'a> Iterator for HprocesslistThreadIter<'a> {
	type Item = SysResult<&'a THREADENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Thread32First(&mut self.te32)
		} else {
			self.hpl.Thread32Next(&mut self.te32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.te32 as *const THREADENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no thread found
				}
			},
		}
	}
}

impl<'a> HprocesslistThreadIter<'a> {
	#[must_use]
	pub(in crate::kernel) fn new(hpl: &'a mut HPROCESSLIST) -> Self {
		Self {
			hpl,
			te32: THREADENTRY32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}
