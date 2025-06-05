#![allow(dead_code, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;

const_values_num_privs! {
	GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS u32 = 0x0000_0004
	GMEM_INVALID_HANDLE u32 = 0x8000
	INFINITE u32 = 0xffff_ffff
	INVALID_FILE_ATTRIBUTES i32 = -1
	LMEM_INVALID_HANDLE u32 = 0x8000
	MAX_COMPUTERNAME_LENGTH usize = 15
	MAX_MODULE_NAME32 usize = 255
	MAX_PATH usize = 260
	SECURITY_SQOS_PRESENT u32 = 0x0010_0000
}

/// [`IS_INTRESOURCE`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-is_intresource)
/// macro.
#[must_use]
pub(crate) fn IS_INTRESOURCE(val: *const u16) -> bool {
	(unsafe { std::mem::transmute::<_, usize>(val) } >> 16) == 0
}

/// [`MAKEINTRESOURCE`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
/// macro.
#[must_use]
pub(crate) const fn MAKEINTRESOURCE(val: isize) -> *const u16 {
	val as u16 as _
}

/// If value is `FALSE`, yields `Err(GetLastError)`, otherwise `Ok()`.
#[must_use]
pub(crate) fn bool_to_sysresult(expr: BOOL) -> SysResult<()> {
	match expr {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// If value is `FALSE`, yields `ERR(ERROR::INVALID_PARAMETER)`, otherwise
/// `Ok()`.
#[must_use]
pub(crate) const fn bool_to_invalidparm(expr: BOOL) -> SysResult<()> {
	match expr {
		0 => Err(co::ERROR::INVALID_PARAMETER),
		_ => Ok(()),
	}
}

/// If pointer is null, yields `Err(GetLastError)`, otherwise `Ok(ptr)`.
#[must_use]
pub(crate) fn ptr_to_sysresult(ptr: HANDLE) -> SysResult<HANDLE> {
	if ptr.is_null() { Err(GetLastError()) } else { Ok(ptr) }
}

/// If pointer is null, yields `Err(ERROR::INVALID_PARAMETER)`, otherwise
/// `Ok(ptr)`.
#[must_use]
pub(crate) const fn ptr_to_invalidparm(ptr: HANDLE) -> SysResult<HANDLE> {
	if ptr.is_null() { Err(co::ERROR::INVALID_PARAMETER) } else { Ok(ptr) }
}

/// If pointer is null, yields `Err(GetLastError)`, otherwise `Ok(Handle)`.
#[must_use]
pub(crate) fn ptr_to_sysresult_handle<H>(ptr: HANDLE) -> SysResult<H>
where
	H: Handle,
{
	ptr_to_sysresult(ptr).map(|ptr| unsafe { Handle::from_ptr(ptr) })
}

/// If pointer is null, yields `Err(ERROR::INVALID_PARAMETER)`, otherwise
/// `Ok(Handle)`.
#[must_use]
pub(crate) fn ptr_to_invalidparm_handle<H>(ptr: HANDLE) -> SysResult<H>
where
	H: Handle,
{
	ptr_to_invalidparm(ptr).map(|ptr| unsafe { Handle::from_ptr(ptr) })
}

/// If the pointer is null, yields `None`, otherwise `Some(Handle)`.
#[must_use]
pub(crate) fn ptr_to_option_handle<H>(ptr: HANDLE) -> Option<H>
where
	H: Handle,
{
	if ptr.is_null() { None } else { Some(unsafe { H::from_ptr(ptr) }) }
}

/// If value is `ERROR::SUCCESS`, yields `Ok(())`, otherwise `Err(err)`.
#[must_use]
pub(crate) const fn error_to_sysresult(lstatus: i32) -> SysResult<()> {
	match unsafe { co::ERROR::from_raw(lstatus as _) } {
		co::ERROR::SUCCESS => Ok(()),
		err => Err(err),
	}
}

/// If value is -1, yields `Err(GetLastError())`, otherwise `Ok(dword)`.
#[must_use]
pub(crate) fn minus1_as_error(dword: u32) -> SysResult<u32> {
	const MINUS_ONE: u32 = -1i32 as u32;
	match dword {
		MINUS_ONE => Err(GetLastError()),
		dword => Ok(dword),
	}
}

/// Converts a constant reference to FFI's `PCVOID`.
#[must_use]
pub(crate) const fn pcvoid<T>(reference: &T) -> PCVOID {
	reference as *const _ as _
}

/// Converts an optional constant reference to FFI's `PCVOID`.
#[must_use]
pub(crate) const fn pcvoid_or_null<T>(reference: Option<&T>) -> PCVOID {
	match reference {
		Some(p) => pcvoid(p),
		None => std::ptr::null(),
	}
}

/// Converts a mutable reference to FFI's `PVOID`.
#[must_use]
pub(crate) const fn pvoid<T>(reference: &mut T) -> PVOID {
	reference as *mut _ as _
}

/// Converts an optional mutable reference to FFI's `PCVOID`.
#[must_use]
pub(crate) const fn pvoid_or_null<T>(reference: Option<&mut T>) -> PVOID {
	match reference {
		Some(p) => pvoid(p),
		None => std::ptr::null_mut(),
	}
}

/// If the vector is empty, returns null, otherwise calls `as_ptr`.
///
/// This is necessary because an empty vector returns garbage as its underlying
/// pointer, see:
/// * https://github.com/rust-lang/rust/issues/39625
#[must_use]
pub(crate) const fn vec_ptr<T>(v: &[T]) -> *const T {
	if v.is_empty() { std::ptr::null() } else { v.as_ptr() }
}

/// Converts a string to an ISO-8859-1 null-terminated byte array.
#[must_use]
pub(crate) fn str_to_iso88591(s: &str) -> Vec<u8> {
	s.chars()
		.map(|ch| ch as u8)
		.chain(std::iter::once(0)) // append a terminating null
		.collect()
}

/// Parses a null-delimited multi-string, ending with two terminating nulls.
///
/// # Safety
///
/// If `len` is not informed, make sure the string has two terminating nulls.
#[must_use]
pub(crate) unsafe fn parse_multi_z_str(src: *const u16, len: Option<usize>) -> Vec<String> {
	let given_len = len.unwrap_or(usize::MAX);
	let mut src = src;
	let mut strings = Vec::<String>::new();
	let mut ch = 0; // relative index of char in current string
	let mut tot_ch = 0; // absolute index of char in original src

	loop {
		if unsafe { *src.add(ch) } == 0 || tot_ch == given_len {
			let slice = unsafe { std::slice::from_raw_parts(src, ch) };
			if slice.is_empty() {
				break; // empty string means two consecutive nulls
			}
			strings.push(WString::from_wchars_slice(slice).to_string());
			src = unsafe { src.add(ch + 1) };
			ch = 0;
		} else {
			ch += 1;
		}

		if len.is_some() && tot_ch == given_len {
			break;
		}
		tot_ch += 1;
	}
	strings
}

/// Creates two vectors:
/// * the first with each string converted to `WString`;
/// * the second with the pointers to each `WString` in the first vector.
#[must_use]
pub(crate) fn create_wstr_ptr_vecs(
	strings: Option<&[impl AsRef<str>]>,
) -> (Vec<WString>, Vec<*const u16>) {
	match strings {
		Some(ss) => {
			let wstrs = ss.iter().map(|s| WString::from_str(s)).collect::<Vec<_>>();
			let pwstrs = wstrs.iter().map(|w| w.as_ptr()).collect::<Vec<_>>();
			(wstrs, pwstrs)
		},
		None => (Vec::default(), Vec::default()),
	}
}
