#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

handle! { HFINDFILE;
	/// Handle to a
	/// [file search](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew).
	/// Originally just a `HANDLE`.
}

impl HFINDFILE {
	/// [`FindFirstFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew)
	/// function.
	///
	/// This method is rather tricky, consider using
	/// [`path::dir_list`](crate::path::dir_list).
	#[must_use]
	pub fn FindFirstFile(
		file_name: &str,
		wfd: &mut WIN32_FIND_DATA,
	) -> SysResult<(FindCloseGuard, bool)> {
		unsafe {
			match ffi::FindFirstFileW(WString::from_str(file_name).as_ptr(), pvoid(wfd)).as_mut() {
				Some(ptr) => {
					let h = HFINDFILE::from_ptr(ptr);
					// When using a filter, if no files are found, the function
					// is successful but it returns an invalid handle.
					let has_something = h != HFINDFILE::INVALID;
					Ok((FindCloseGuard::new(h), has_something))
				},
				None => match GetLastError() {
					co::ERROR::FILE_NOT_FOUND => Ok((
						FindCloseGuard::new(HFINDFILE::NULL), // not an error, first file not found
						false,
					)),
					err => Err(err),
				},
			}
		}
	}

	/// [`FindNextFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findnextfilew)
	/// function.
	///
	/// This method is rather tricky, consider using
	/// [`path::dir_list`](crate::path::dir_list).
	#[must_use]
	pub fn FindNextFile(&self, wfd: &mut WIN32_FIND_DATA) -> SysResult<bool> {
		match unsafe { ffi::FindNextFileW(self.ptr(), pvoid(wfd)) } {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false), // not an error, no further files found
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
