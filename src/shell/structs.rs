#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;
use crate::kernel::{ffi_types::*, privs::*};

/// [`COMDLG_FILTERSPEC`](https://learn.microsoft.com/en-us/windows/win32/api/shtypes/ns-shtypes-comdlg_filterspec)
/// struct.
#[repr(C)]
pub struct COMDLG_FILTERSPEC<'a, 'b> {
	pszName: *mut u16,
	pszSpec: *mut u16,

	_pszName: PhantomData<&'a mut u16>,
	_pszSpec: PhantomData<&'b mut u16>,
}

impl_default!(COMDLG_FILTERSPEC, 'a, 'b);

impl<'a, 'b> COMDLG_FILTERSPEC<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, pszName, set_pszName);
	pub_fn_string_ptr_get_set!('b, pszSpec, set_pszSpec);
}

/// [`ITEMIDLIST`](https://learn.microsoft.com/en-us/windows/win32/api/shtypes/ns-shtypes-itemidlist)
/// struct.
#[repr(C)]
pub struct ITEMIDLIST {
	pub mkid: SHITEMID,
}

/// [`NOTIFYICONDATA`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-notifyicondataw)
/// struct.
#[repr(C)]
pub struct NOTIFYICONDATA {
	cbSize: u32,
	pub hWnd: HWND,
	pub uID: u32,
	pub uFlags: co::NIF,
	pub uCallbackMessage: co::WM,
	pub hIcon: HICON,
	szTip: [u16; 128],
	pub dwState: co::NIS,
	pub dwStateMask: co::NIS,
	szInfo: [u16; 256],
	pub uVersion: u32, // union with uTimeout, which is deprecated
	szInfoTitle: [u16; 64],
	pub dwInfoFlags: co::NIIF,
	pub guidItem: GUID,
	pub hBalloonIcon: HICON,
}

impl_default!(NOTIFYICONDATA, cbSize);

impl NOTIFYICONDATA {
	pub_fn_string_arr_get_set!(szTip, set_szTip);
	pub_fn_string_arr_get_set!(szInfo, set_szInfo);
	pub_fn_string_arr_get_set!(szInfoTitle, set_szInfoTitle);
}

/// [`PIDL`](https://learn.microsoft.com/en-us/windows/win32/api/shtypes/ns-shtypes-itemidlist)
/// struct.
///
/// # Examples
///
/// Retrieving the `PIDL` from an [`IShellItem`](crate::IShellItem) with
/// [`SHGetIDListFromObject`](crate::SHGetIDListFromObject):
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let _com_guard = w::CoInitializeEx(
///     co::COINIT::APARTMENTTHREADED | co::COINIT::DISABLE_OLE1DDE)?;
///
/// let f = w::SHCreateItemFromParsingName::<w::IShellItem>(
///     "C:\\Temp",
///     None::<&w::IBindCtx>,
/// )?;
///
/// let pidl = w::SHGetIDListFromObject(&f)?;
/// # w::HrResult::Ok(())
/// ```
#[repr(transparent)]
pub struct PIDL(*mut ITEMIDLIST);

impl PIDL {
	/// Constructs a new `PIDL` object by wrapping a pointer.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	///
	/// # Safety
	///
	/// Be sure the pointer has the correct type and isn’t owned by anyone else,
	/// otherwise you may cause memory access violations.
	#[must_use]
	pub const unsafe fn from_ptr(p: *mut ITEMIDLIST) -> Self {
		Self(p)
	}

	/// Returns the underlying [`ITEMIDLIST`](crate::ITEMIDLIST) pointer value.
	#[must_use]
	pub const fn ptr(&self) -> *mut ITEMIDLIST {
		self.0
	}
}

/// [`SHFILEINFO`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shfileinfow)
/// struct.
#[repr(C)]
pub struct SHFILEINFO {
	pub hIcon: HICON,
	pub iIcon: i32,
	dwAttributes: u32,
	szDisplayName: [u16; MAX_PATH],
	szTypeName: [u16; 80],
}

impl_default!(SHFILEINFO);

impl SHFILEINFO {
	pub_fn_string_arr_get_set!(szDisplayName, set_szDisplayName);
	pub_fn_string_arr_get_set!(szTypeName, set_szTypeName);
}

/// [`SHFILEOPSTRUCT`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shfileopstructw)
/// struct.
#[repr(C)]
pub struct SHFILEOPSTRUCT<'a, 'b, 'c> {
	pub hwnd: HWND,
	pub wFunc: co::FO,
	pFrom: *mut u16, // double-null terminated
	pTo: *mut u16,   // double-null terminated
	pub fFlags: co::FOF,
	fAnyOperationsAborted: BOOL,
	hNameMappings: *mut std::ffi::c_void, // lots of stuff going here...
	lpszProgressTitle: *mut u16,

	_pFrom: PhantomData<&'a mut u16>,
	_pTo: PhantomData<&'b mut u16>,
	_lpszProgressTitle: PhantomData<&'c mut u16>,
}

impl_default!(SHFILEOPSTRUCT, 'a, 'b, 'c);

impl<'a, 'b, 'c> SHFILEOPSTRUCT<'a, 'b, 'c> {
	pub_fn_bool_get_set!(fAnyOperationsAborted, set_fAnyOperationsAborted);

	/// Retrieves the `pFrom` field.
	#[must_use]
	pub fn pFrom(&self) -> Option<Vec<String>> {
		unsafe { self.pFrom.as_mut().map(|p| parse_multi_z_str(p, None)) }
	}

	/// Sets the `pFrom` field.
	///
	/// **Note:** You must create the string with
	/// [`WString::from_str_vec`](crate::WString::from_str_vec).
	pub fn set_pFrom(&mut self, val: Option<&'a mut WString>) {
		self.pFrom = val.map_or(std::ptr::null_mut(), |v| unsafe { v.as_mut_ptr() });
	}

	/// Retrieves the `pTo` field.
	#[must_use]
	pub fn pTo(&self) -> Option<Vec<String>> {
		unsafe { self.pTo.as_mut().map(|p| parse_multi_z_str(p, None)) }
	}

	/// Sets the `pTo` field.
	///
	/// **Note:** You must create the string with
	/// [`WString::from_str_vec`](crate::WString::from_str_vec).
	pub fn set_pTo(&mut self, val: Option<&'b mut WString>) {
		self.pTo = val.map_or(std::ptr::null_mut(), |v| unsafe { v.as_mut_ptr() });
	}

	pub_fn_string_ptr_get_set!('c, lpszProgressTitle, set_lpszProgressTitle);
}

/// [`SHITEMID`](https://learn.microsoft.com/en-us/windows/win32/api/shtypes/ns-shtypes-shitemid)
/// struct.
#[repr(C, packed)]
pub struct SHITEMID {
	cb: u16,
	abID: [u8; 1],
}

/// [`SHSTOCKICONINFO`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shstockiconinfo)
/// struct.
#[repr(C)]
pub struct SHSTOCKICONINFO {
	cbSize: u32,
	pub hIcon: HICON,
	pub iSysImageIndex: i32,
	pub iIcon: i32,
	szPath: [u16; MAX_PATH],
}

impl_default!(SHSTOCKICONINFO, cbSize);

impl SHSTOCKICONINFO {
	pub_fn_string_arr_get_set!(szPath, get_szPath);
}
