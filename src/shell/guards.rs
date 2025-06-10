use std::ops::{Deref, DerefMut};

use crate::decl::*;
use crate::guard::*;
use crate::prelude::*;
use crate::shell::ffi;

/// RAII implementation for [`SHFILEINFO`](crate::SHFILEINFO) which
/// automatically calls
/// [`DestroyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
/// on `hIcon` field when the object goes out of scope.
pub struct DestroyIconShfiGuard {
	shfi: SHFILEINFO,
}

impl Drop for DestroyIconShfiGuard {
	fn drop(&mut self) {
		if let Some(h) = self.shfi.hIcon.as_opt() {
			let _ = unsafe { DestroyIconGuard::new(h.raw_copy()) };
		}
	}
}

impl Deref for DestroyIconShfiGuard {
	type Target = SHFILEINFO;

	fn deref(&self) -> &Self::Target {
		&self.shfi
	}
}

impl DerefMut for DestroyIconShfiGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.shfi
	}
}

impl DestroyIconShfiGuard {
	/// Constructs the guard by taking ownership of the struct.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`DestroyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(shfi: SHFILEINFO) -> Self {
		Self { shfi }
	}

	/// Ejects the underlying struct, leaving
	/// [`SHFILEINFO::default`](crate::SHFILEINFO::default) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> SHFILEINFO {
		std::mem::take(&mut self.shfi)
	}
}

/// RAII implementation for [`SHSTOCKICONINFO`](crate::SHSTOCKICONINFO) which
/// automatically calls
/// [`DestroyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
/// on `hIcon` field when the object goes out of scope.
pub struct DestroyIconSiiGuard {
	sii: SHSTOCKICONINFO,
}

impl Drop for DestroyIconSiiGuard {
	fn drop(&mut self) {
		if let Some(h) = self.sii.hIcon.as_opt() {
			let _ = unsafe { DestroyIconGuard::new(h.raw_copy()) };
		}
	}
}

impl Deref for DestroyIconSiiGuard {
	type Target = SHSTOCKICONINFO;

	fn deref(&self) -> &Self::Target {
		&self.sii
	}
}

impl DerefMut for DestroyIconSiiGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.sii
	}
}

impl DestroyIconSiiGuard {
	/// Constructs the guard by taking ownership of the struct.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`DestroyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(sii: SHSTOCKICONINFO) -> Self {
		Self { sii }
	}

	/// Ejects the underlying struct, leaving
	/// [`SHSTOCKICONINFO::default`](crate::SHSTOCKICONINFO::default) in its
	/// place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> SHSTOCKICONINFO {
		std::mem::take(&mut self.sii)
	}
}

handle_guard! { DragFinishGuard: HDROP;
	ffi::DragFinish;
	/// RAII implementation for [`HDROP`](crate::HDROP) which automatically
	/// calls
	/// [`DragFinish`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragfinish)
	/// when the object goes out of scope.
}

/// RAII implementation for [`PIDL`](crate::PIDL) which automatically calls
/// [`CoTaskMemFree`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
/// when the object goes out of scope.
pub struct CoTaskMemFreePidlGuard {
	pidl: PIDL,
}

impl Drop for CoTaskMemFreePidlGuard {
	fn drop(&mut self) {
		let ptr = self.pidl.ptr();
		if !ptr.is_null() {
			let _ = unsafe { CoTaskMemFreeGuard::new(ptr as _, 0) };
		}
	}
}

impl Deref for CoTaskMemFreePidlGuard {
	type Target = PIDL;

	fn deref(&self) -> &Self::Target {
		&self.pidl
	}
}

impl CoTaskMemFreePidlGuard {
	/// Constructs the guard.
	///
	/// # Safety
	///
	/// Be sure the `PIDL` must be freed with
	/// [`CoTaskMemFree`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
	/// at the end of the scope.
	#[must_use]
	pub const unsafe fn new(pidl: PIDL) -> Self {
		Self { pidl }
	}
}
