#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::kernel::{ffi_types::*, privs::*};
use crate::prelude::*;

/// [`BUTTON_IMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-button_imagelist)
/// struct.
#[repr(C)]
pub struct BUTTON_IMAGELIST {
	pub himl: HIMAGELIST,
	pub margin: RECT,
	pub uAlign: co::BIA,
}

/// [`BUTTON_SPLITINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-button_splitinfo)
/// struct.
#[repr(C)]
pub struct BUTTON_SPLITINFO {
	pub mask: co::BCSIF,
	pub himlGlyph: HIMAGELIST,
	pub uSplitStyle: co::BCSS,
	pub size: SIZE,
}

/// [`COLORSCHEME`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-colorscheme)
/// struct.
#[repr(C)]
pub struct COLORSCHEME {
	dwSize: u32,
	pub clrBtnHighlight: COLORREF,
	pub clrBtnShadow: COLORREF,
}

impl_default!(COLORSCHEME, dwSize);

/// [`DATETIMEPICKERINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-datetimepickerinfo)
/// struct.
#[repr(C)]
pub struct DATETIMEPICKERINFO {
	cbSize: u32,
	pub rcCheck: RECT,
	pub stateCheck: co::STATE_SYSTEM,
	pub rcButton: RECT,
	pub stateButton: co::STATE_SYSTEM,
	pub hwndEdit: HWND,
	pub hwndUD: HWND,
	pub hwndDropDown: HWND,
}

impl_default!(DATETIMEPICKERINFO, cbSize);

/// [`EDITBALLOONTIP`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-editballoontip)
/// struct.
#[repr(C)]
pub struct EDITBALLOONTIP<'a, 'b> {
	cbStruct: u32,
	pszTitle: *mut u16,
	pszText: *mut u16,
	pub ttiIcon: co::TTI,

	_pszTitle: PhantomData<&'a mut u16>,
	_pszText: PhantomData<&'b mut u16>,
}

impl_default!(EDITBALLOONTIP, cbStruct, 'a, 'b);

impl<'a, 'b> EDITBALLOONTIP<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, pszTitle, set_pszTitle);
	pub_fn_string_ptr_get_set!('b, pszText, set_pszText);
}

/// [`HDITEM`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-hditemw)
/// struct.
#[repr(C)]
pub struct HDITEM<'a> {
	pub mask: co::HDI,
	pub cxy: i32,
	pszText: *mut u16,
	pub hbm: HBITMAP,
	cchTextMax: i32,
	pub fmt: co::HDF,
	pub lParam: isize,
	pub iImage: i32,
	pub iOrder: i32,
	pub typeFilter: co::HDFT,
	pub pvFilter: *mut std::ffi::c_void,
	pub state: co::HDIS,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(HDITEM, 'a);

impl<'a> HDITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`HDHITTESTINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-hdhittestinfo)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct HDHITTESTINFO {
	pub pt: POINT,
	pub flags: co::HHT,
	pub iItem: i32,
}

/// [`HDLAYOUT`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-hdlayout)
/// struct.
#[repr(C)]
pub struct HDLAYOUT<'a, 'b> {
	prc: *mut RECT,
	pwpos: *mut WINDOWPOS,

	_prc: PhantomData<&'a mut RECT>,
	_pwpos: PhantomData<&'b mut WINDOWPOS>,
}

impl_default!(HDLAYOUT, 'a, 'b);

impl<'a, 'b> HDLAYOUT<'a, 'b> {
	pub_fn_ptr_get_set!('a, prc, set_prc, RECT);
	pub_fn_ptr_get_set!('b, pwpos, set_pwpos, WINDOWPOS);
}

/// [`INITCOMMONCONTROLSEX`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-initcommoncontrolsex)
/// struct
#[repr(C)]
pub struct INITCOMMONCONTROLSEX {
	dwSize: u32,
	pub icc: co::ICC,
}

impl_default!(INITCOMMONCONTROLSEX, dwSize);

/// [`LITEM`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-litem)
/// struct.
#[repr(C)]
pub struct LITEM {
	pub mask: co::LIF,
	pub iLink: i32,
	pub state: co::LIS,
	pub stateMask: co::LIS,
	szID: [u16; MAX_LINKID_TEXT],
	szUrl: [u16; L_MAX_URL_LENGTH],
}

impl_default!(LITEM);

impl LITEM {
	pub_fn_string_arr_get_set!(szID, set_szID);
	pub_fn_string_arr_get_set!(szUrl, set_szUrl);
}

/// [`LVBKIMAGE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvbkimagew)
/// struct.
#[repr(C)]
pub struct LVBKIMAGE<'a> {
	pub uFlags: co::LVBKIF,
	pub hbm: HBITMAP,
	pszImage: *mut u16,
	cchImageMax: u32,
	pub xOffsetPercent: i32,
	pub yOffsetPercent: i32,

	_pszImage: PhantomData<&'a mut u16>,
}

impl_default!(LVBKIMAGE, 'a);

impl<'a> LVBKIMAGE<'a> {
	pub_fn_string_buf_get_set!('a, pszImage, set_pszImage, raw_pszImage, cchImageMax);
}

/// [`LVCOLUMN`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvcolumnw)
/// struct.
#[repr(C)]
pub struct LVCOLUMN<'a> {
	pub mask: co::LVCF,
	pub fmt: co::LVCFMT_C,
	pub cx: i32,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iSubItem: i32,
	pub iImage: i32,
	pub iOrder: i32,
	pub cxMin: i32,
	pub cxDefault: i32,
	pub cxIdeal: i32,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(LVCOLUMN, 'a);

impl<'a> LVCOLUMN<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`LVFINDINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfindinfow)
/// struct.
#[repr(C)]
pub struct LVFINDINFO<'a> {
	pub flags: co::LVFI,
	psz: *mut u16,
	pub lParam: isize,
	pub pt: POINT,
	pub vkDirection: co::VK_DIR,

	_psz: PhantomData<&'a mut u16>,
}

impl_default!(LVFINDINFO, 'a);

impl<'a> LVFINDINFO<'a> {
	pub_fn_string_ptr_get_set!('a, psz, set_psz);
}

/// [`LVFOOTERINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfooterinfo)
/// struct.
#[repr(C)]
pub struct LVFOOTERINFO<'a> {
	pub mask: co::LVFF,
	pszText: *mut u16,
	cchTextMax: i32,
	pub cItems: u32,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(LVFOOTERINFO, 'a);

impl<'a> LVFOOTERINFO<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`LVFOOTERITEM`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfooteritem)
/// struct.
#[repr(C)]
pub struct LVFOOTERITEM<'a> {
	pub mask: co::LVFIF,
	pub iItem: i32,
	pszText: *mut u16,
	cchTextMax: i32,
	pub state: co::LVFIS,
	pub stateMask: co::LVFIS,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(LVFOOTERITEM, 'a);

impl<'a> LVFOOTERITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`LVGROUP`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvgroup)
/// struct.
#[repr(C)]
pub struct LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	cbSize: u32,
	pub mask: co::LVGF,
	pszHeader: *mut u16,
	cchHeader: i32,
	pszFooter: *mut u16,
	cchFooter: i32,
	pub iGroupId: i32,
	pub stateMask: co::LVGS,
	pub state: co::LVGS,
	pub uAlign: co::LVGA_FH,
	pszSubtitle: *mut u16,
	cchSubtitle: i32,
	pszTask: *mut u16,
	cchTask: i32,
	pszDescriptionTop: *mut u16,
	cchDescriptionTop: i32,
	pszDescriptionBottom: *mut u16,
	cchDescriptionBottom: i32,
	pub iTitleImage: i32,
	pub iExtendedImage: i32,
	pub iFirstItem: i32,
	pub cItems: u32,
	pszSubsetTitle: *mut u16,
	cchSubsetTitle: i32,

	_pszHeader: PhantomData<&'a mut u16>,
	_pszFooter: PhantomData<&'b mut u16>,
	_pszSubtitle: PhantomData<&'c mut u16>,
	_pszTask: PhantomData<&'d mut u16>,
	_pszDescriptionTop: PhantomData<&'e mut u16>,
	_pszDescriptionBottom: PhantomData<&'f mut u16>,
	_pszSubsetTitle: PhantomData<&'g mut u16>,
}

impl_default!(LVGROUP, cbSize, 'a, 'b, 'c, 'd, 'e, 'f, 'g);

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	pub_fn_string_buf_get_set!('a, pszHeader, set_pszHeader, raw_pszHeader, cchHeader);
	pub_fn_string_buf_get_set!('b, pszFooter, set_pszFooter, raw_pszFooter, cchFooter);
	pub_fn_string_buf_get_set!('c, pszSubtitle, set_pszSubtitle, raw_pszSubtitle, cchSubtitle);
	pub_fn_string_buf_get_set!('d, pszTask, set_pszTask, raw_pszTask, cchTask);
	pub_fn_string_buf_get_set!('e, pszDescriptionTop, set_pszDescriptionTop, raw_pszDescriptionTop, cchDescriptionTop);
	pub_fn_string_buf_get_set!('f, pszDescriptionBottom, set_pszDescriptionBottom, raw_pszDescriptionBottom, cchDescriptionBottom);
	pub_fn_string_buf_get_set!('g, pszSubsetTitle, set_pszSubsetTitle, raw_pszSubsetTitle, cchSubsetTitle);
}

/// [`LVGROUPMETRICS`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvgroupmetrics)
/// struct.
#[repr(C)]
pub struct LVGROUPMETRICS {
	cbSize: u32,
	pub mask: co::LVGMF,
	pub Left: u32,
	pub Top: u32,
	pub Right: u32,
	pub Bottom: u32,
	pub crLeft: COLORREF,
	pub crTop: COLORREF,
	pub crRight: COLORREF,
	pub crBottom: COLORREF,
	pub crHeader: COLORREF,
	pub crFooter: COLORREF,
}

impl_default!(LVGROUPMETRICS, cbSize);

/// [`LVHITTESTINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvhittestinfo)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct LVHITTESTINFO {
	pub pt: POINT,
	pub flags: co::LVHT,
	pub iItem: i32,
	pub iSubItem: i32,
	pub iGroup: i32,
}

/// [`LVINSERTGROUPSORTED`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvinsertgroupsorted)
/// struct.
#[repr(C)]
pub struct LVINSERTGROUPSORTED<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	pub pfnGroupCompare: Option<PFNLVGROUPCOMPARE>,
	pub pvData: usize,
	pub lvGroup: LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> Default for LVINSERTGROUPSORTED<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	fn default() -> Self {
		Self {
			pfnGroupCompare: None,
			pvData: 0,
			lvGroup: LVGROUP::default(), // has cbSize, so we can't use impl_default_size macro
		}
	}
}

/// [`LVINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvinsertmark)
/// struct.
#[repr(C)]
pub struct LVINSERTMARK {
	cbSize: u32,
	pub dwFlags: co::LVIM,
	pub iItem: i32,
	dwReserved: u32,
}

impl_default!(LVINSERTMARK);

/// [`LVITEM`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvitemw)
/// struct.
#[repr(C)]
pub struct LVITEM<'a> {
	pub mask: co::LVIF,
	pub iItem: i32,
	pub iSubItem: i32,
	pub state: co::LVIS,
	pub stateMask: co::LVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub lParam: isize,
	pub iIndent: i32,
	pub iGroupId: co::LVI_GROUPID,
	pub cColumns: u32,
	pub puColumns: *mut i32,
	pub piColFmt: *mut co::LVCFMT_I,
	pub iGroup: i32,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(LVITEM, 'a);

impl<'a> LVITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`LVITEMINDEX`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvitemindex)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct LVITEMINDEX {
	pub iItem: i32,
	pub iGroup: i32,
}

/// [`LVSETINFOTIP`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvsetinfotip)
/// struct.
#[repr(C)]
pub struct LVSETINFOTIP<'a> {
	cbSize: u32,
	pub dwFlags: u32, // unspecified
	pszText: *mut u16,
	pub iItem: i32,
	pub iSubItem: i32,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(LVSETINFOTIP, cbSize, 'a);

impl<'a> LVSETINFOTIP<'a> {
	pub_fn_string_ptr_get_set!('a, pszText, set_pszText);
}

/// [`LVTILEINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvtileinfo)
/// struct.
#[repr(C)]
pub struct LVTILEINFO<'a> {
	cbSize: u32,
	pub iItem: i32,
	cColumns: u32,
	puColumns: *mut u32,
	piColFmt: *mut co::LVCFMT_C,

	_puColumns: PhantomData<&'a mut u32>,
}

impl_default!(LVTILEINFO, cbSize, 'a);

impl<'a> LVTILEINFO<'a> {
	/// Returns the `puColumns` field.
	#[must_use]
	pub const fn puColumns(&self) -> Option<&'a mut [u32]> {
		unsafe {
			match self.puColumns.as_mut() {
				Some(_) => Some(std::slice::from_raw_parts_mut(self.puColumns, self.cColumns as _)),
				None => None,
			}
		}
	}

	/// Returns the `piColFmt` field.
	#[must_use]
	pub const fn piColFmt(&self) -> Option<&'a mut [co::LVCFMT_C]> {
		unsafe {
			match self.puColumns.as_mut() {
				Some(_) => Some(std::slice::from_raw_parts_mut(self.piColFmt, self.cColumns as _)),
				None => None,
			}
		}
	}

	/// Sets the `puColumns` and `piColFmt` fields.
	///
	/// # Panics
	///
	/// Panics if `puColumns` and `piColFmt` slices have different lengths.
	pub fn set_puColumns_piColFmt(&mut self, val: Option<(&'a mut [u32], &'a mut [co::LVCFMT_C])>) {
		if let Some(val) = val {
			if val.0.len() != val.1.len() {
				panic!("Different slice lengths: {} and {}.", val.0.len(), val.1.len());
			}
			self.cColumns = val.0.len() as _;
			self.puColumns = val.0.as_mut_ptr();
			self.piColFmt = val.1.as_mut_ptr();
		} else {
			self.cColumns = 0;
			self.puColumns = std::ptr::null_mut();
			self.piColFmt = std::ptr::null_mut();
		}
	}
}

/// [`LVTILEVIEWINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvtileviewinfo)
/// struct.
#[repr(C)]
pub struct LVTILEVIEWINFO {
	cbSize: u32,
	pub dwMask: co::LVTVIM,
	pub dwFlags: co::LVTVIF,
	pub sizeTile: SIZE,
	pub cLines: i32,
	pub rcLabelMargin: RECT,
}

impl_default!(LVTILEVIEWINFO, cbSize);

/// [`MCGRIDINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-mcgridinfo)
/// struct.
#[repr(C)]
pub struct MCGRIDINFO<'a> {
	cbSize: u32,
	pub dwPart: co::MCGIP,
	pub dwFlags: co::MCGIF,
	pub iCalendar: i32,
	pub iRow: i32,
	pub iCol: i32,
	bSelected: BOOL,
	pub stStart: SYSTEMTIME,
	pub stEnd: SYSTEMTIME,
	pub rc: RECT,
	pszName: *mut u16,
	cchName: usize,

	_pszName: PhantomData<&'a mut u16>,
}

impl_default!(MCGRIDINFO, cbSize, 'a);

impl<'a> MCGRIDINFO<'a> {
	pub_fn_bool_get_set!(bSelected, set_bSelected);
	pub_fn_string_buf_get_set!('a, pszName, set_pszName, raw_pszName, cchName);
}

/// [`MCHITTESTINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-mchittestinfo)
/// struct.
#[repr(C)]
pub struct MCHITTESTINFO {
	cbSize: u32,
	pub pt: POINT,
	pub uHit: co::MCHT,
	pub st: SYSTEMTIME,
	pub rc: RECT,
	pub iOffset: i32,
	pub iRow: i32,
	pub iCol: i32,
}

impl_default!(MCHITTESTINFO, cbSize);

/// [`MONTHDAYSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/monthdaystate)
/// struct.
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MONTHDAYSTATE(u32);

impl MONTHDAYSTATE {
	/// Returns the state of the bit corresponding to the given day index.
	///
	/// # Panics
	///
	/// Panics if `index` is greater than 31.
	#[must_use]
	pub fn get_day(&self, index: u8) -> bool {
		if index > 31 {
			panic!("MONTHDAYSTATE max index is 31, tried to get {}.", index)
		} else {
			((self.0 >> index) & 1) != 0
		}
	}

	/// Sets the state of the bit corresponding to the given day index.
	///
	/// # Panics
	///
	/// Panics if `index` is greater than 31.
	pub fn set_day(&mut self, index: u8, state: bool) {
		if index > 31 {
			panic!("MONTHDAYSTATE max index is 31, tried to set {}.", index)
		} else if state {
			self.0 |= 1 << index;
		} else {
			self.0 &= !(1 << index);
		}
	}
}

/// [`NMBCDROPDOWN`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbcdropdown)
/// struct.
#[repr(C)]
pub struct NMBCDROPDOWN {
	pub hdr: NMHDR,
	pub rcButton: RECT,
}

/// [`NMBCHOTITEM`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbchotitem)
/// struct.
#[repr(C)]
pub struct NMBCHOTITEM {
	pub hdr: NMHDR,
	pub dwFlags: co::HICF,
}

/// [`NMCHAR`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmchar)
/// struct.
#[repr(C)]
pub struct NMCHAR {
	pub hdr: NMHDR,
	pub ch: u32,
	pub dwItemPrev: u32,
	pub dwItemNext: u32,
}

/// [`NMCUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmcustomdraw)
/// struct.
#[repr(C)]
pub struct NMCUSTOMDRAW {
	pub hdr: NMHDR,
	pub dwDrawStage: co::CDDS,
	pub hdc: HDC,
	pub rc: RECT,
	pub dwItemSpec: usize,
	pub uItemState: co::CDIS,
	pub lItemlParam: isize,
}

/// [`NMDATETIMECHANGE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimechange)
/// struct.
#[repr(C)]
pub struct NMDATETIMECHANGE {
	pub nmhdr: NMHDR,
	pub dwFlags: co::GDT,
	pub st: SYSTEMTIME,
}

/// [`NMDATETIMEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimeformatw)
/// struct.
#[repr(C)]
pub struct NMDATETIMEFORMAT<'a> {
	pub nmhdr: NMHDR,
	pszFormat: *mut u16,
	pub st: SYSTEMTIME,
	pszDisplay: *mut u16,
	szDisplay: [u16; 64], // used as a buffer to pszDisplay

	_pszFormat: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMEFORMAT, 'a);

impl<'a> NMDATETIMEFORMAT<'a> {
	pub_fn_string_ptr_get_set!('a, pszFormat, set_pszFormat);

	/// Returns the `pszDisplay` field.
	#[must_use]
	pub fn pszDisplay(&self) -> String {
		unsafe { WString::from_wchars_nullt(self.pszDisplay) }.to_string()
	}

	/// Sets the `pszDisplay` field.
	pub fn set_pszDisplay(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szDisplay);
	}
}

/// [`NMDATETIMEFORMATQUERY`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimeformatqueryw)
/// struct.
#[repr(C)]
pub struct NMDATETIMEFORMATQUERY<'a> {
	pub nmhdr: NMHDR,
	pszFormat: *mut u16,
	pub szMax: SIZE,

	_pszFormat: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMEFORMATQUERY, 'a);

impl<'a> NMDATETIMEFORMATQUERY<'a> {
	pub_fn_string_ptr_get_set!('a, pszFormat, set_pszFormat);
}

/// [`NMDATETIMESTRING`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimestringw)
/// struct.
#[repr(C)]
pub struct NMDATETIMESTRING<'a> {
	pub nmhdr: NMHDR,
	pszUserString: *mut u16,
	pub st: SYSTEMTIME,
	pub dwFlags: co::GDT,

	_pszUserString: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMESTRING, 'a);

impl<'a> NMDATETIMESTRING<'a> {
	pub_fn_string_ptr_get_set!('a, pszUserString, set_pszUserString);
}

/// [`NMDATETIMEWMKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimewmkeydownw)
/// struct.
#[repr(C)]
pub struct NMDATETIMEWMKEYDOWN<'a> {
	pub nmhdr: NMHDR,
	pub nVirtKey: i32,
	pszFormat: *mut u16,
	pub st: SYSTEMTIME,

	_pszFormat: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMEWMKEYDOWN, 'a);

impl<'a> NMDATETIMEWMKEYDOWN<'a> {
	pub_fn_string_ptr_get_set!('a, pszFormat, set_pszFormat);
}

/// [`NMDAYSTATE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdaystate)
/// struct.
#[repr(C)]
pub struct NMDAYSTATE<'a> {
	pub nmhdr: NMHDR,
	pub stStart: SYSTEMTIME,
	cDayState: i32,
	prgDayState: *mut MONTHDAYSTATE,

	_prgDayState: PhantomData<&'a mut MONTHDAYSTATE>,
}

impl_default!(NMDAYSTATE, 'a);

impl<'a> NMDAYSTATE<'a> {
	pub_fn_array_buf_get_set!('a, prgDayState, set_prgDayState, cDayState, MONTHDAYSTATE);
}

/// [`NMHDDISPINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmhddispinfow)
/// struct.
#[repr(C)]
pub struct NMHDDISPINFO<'a> {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub mask: co::HDI,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub lParam: isize,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(NMHDDISPINFO, 'a);

impl<'a> NMHDDISPINFO<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`NMHDFILTERBTNCLICK`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmhdfilterbtnclick)
/// struct.
#[repr(C)]
pub struct NMHDFILTERBTNCLICK {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub rc: RECT,
}

impl_default!(NMHDFILTERBTNCLICK);

/// [`NMHDR`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nmhdr)
/// struct.
#[repr(C)]
#[derive(PartialEq, Eq)]
pub struct NMHDR {
	/// A window handle to the control sending the message.
	pub hwndFrom: HWND,
	idFrom: usize,
	/// Notification code sent in
	/// [`WM_NOTIFY`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-notify).
	pub code: NmhdrCode,
}

impl_default!(NMHDR);

impl NMHDR {
	/// Returns the `idFrom` field, the ID of the control sending the message.
	#[must_use]
	pub const fn idFrom(&self) -> u16 {
		self.idFrom as _
	}

	/// Sets the `idFrom` field, the ID of the control sending the message.
	pub const fn set_idFrom(&mut self, val: u16) {
		self.idFrom = val as _
	}
}

/// Notification code returned in [`NMHDR`](crate::NMHDR) struct. This code is
/// convertible to/from the specific common control notification codes –
/// [`LVN`](crate::co::LVN), [`TVN`](crate::co::TVN), etc.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// // Convert LVN to NmhrCode:
/// let code = w::NmhdrCode::from(co::LVN::ITEMCHANGED);
///
/// // Convert NmhrCode to LVN – fails it code is not a valid LVN:
/// let lvn = co::LVN::try_from(code)?;
/// # w::SysResult::Ok(())
/// ```
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct NmhdrCode(i32);

impl From<i32> for NmhdrCode {
	fn from(v: i32) -> Self {
		Self(v)
	}
}

impl PartialOrd for NmhdrCode {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(&other.0)
	}
}
impl Ord for NmhdrCode {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.cmp(&other.0)
	}
}

impl std::fmt::Display for NmhdrCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl NmhdrCode {
	#[must_use]
	pub(crate) const fn from_code(v: i32) -> Self {
		Self(v)
	}

	/// Returns the primitive integer underlying value.
	///
	/// This method is similar to [`Into`](std::convert::Into), but it is
	/// `const`, therefore it can be used in
	/// [const contexts](https://doc.rust-lang.org/reference/const_eval.html).
	#[must_use]
	pub const fn raw(&self) -> i32 {
		self.0
	}
}

/// [`NMHEADER`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmheaderw)
/// struct.
#[repr(C)]
pub struct NMHEADER<'a> {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub iButton: i32,
	pitem: *mut HDITEM<'a>,

	_pitem: PhantomData<&'a mut HDITEM<'a>>,
}

impl_default!(NMHEADER, 'a);

impl<'a> NMHEADER<'a> {
	pub_fn_ptr_get_set!('a, pitem, set_pitem, HDITEM<'a>);
}

/// [`NMITEMACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmitemactivate)
/// struct.
#[repr(C)]
pub struct NMITEMACTIVATE {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub iSubItem: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
	pub uChanged: co::LVIF,
	pub ptAction: POINT,
	pub lParam: isize,
	pub uKeyFlags: co::LVKF,
}

/// [`NMOBJECTNOTIFY`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmobjectnotify)
/// struct.
#[repr(C)]
pub struct NMOBJECTNOTIFY<'a> {
	pub hdr: NMHDR,
	pub iItem: i32,
	piid: *mut co::IID,
	Object: COMPTR,
	pub hrResult: co::HRESULT,
	pub dwFlags: u32,

	_piid: PhantomData<&'a mut co::IID>,
}

impl_default!(NMOBJECTNOTIFY, 'a);
impl_drop_comptr!(Object, NMOBJECTNOTIFY, 'a);

impl<'a> NMOBJECTNOTIFY<'a> {
	pub_fn_ptr_get_set!('a, piid, set_piid, co::IID);
	pub_fn_comptr_get_set!(Object, set_Object, ole_IUnknown);
}

/// [`NMIPADDRESS`](https://learn.microsoft.com/en-us/windows/win32/api/Commctrl/ns-commctrl-nmipaddress)
/// struct.
#[repr(C)]
pub struct NMIPADDRESS {
	pub hdr: NMHDR,
	pub iField: i32,
	pub iValue: i32,
}

/// [`NMLINK`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlink)
/// struct.
#[repr(C)]
pub struct NMLINK {
	pub hdr: NMHDR,
	pub item: LITEM,
}

/// [`NMLISTVIEW`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlistview)
/// struct.
#[repr(C)]
pub struct NMLISTVIEW {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub iSubItem: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
	pub uChanged: co::LVIF,
	pub ptAction: POINT,
	pub lParam: isize,
}

/// [`NMLVCACHEHINT`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvcachehint)
/// struct.
#[repr(C)]
pub struct NMLVCACHEHINT {
	pub hdr: NMHDR,
	pub iFrom: i32,
	pub iTo: i32,
}

/// [`NMLVCUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvcustomdraw)
/// struct.
#[repr(C)]
pub struct NMLVCUSTOMDRAW {
	pub mcd: NMCUSTOMDRAW,
	pub clrText: COLORREF,
	pub clrTextBk: COLORREF,
	pub iSubItem: i32,
	pub dwItemType: co::LVCDI,
	pub clrFace: COLORREF,
	pub iIconEffect: i32,
	pub iIconPhase: i32,
	pub iPartId: i32,
	pub iStateId: i32,
	pub rcText: RECT,
	pub uAlign: co::LVGA_HEADER,
}

/// [`NMLVDISPINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvdispinfow)
/// struct.
#[repr(C)]
pub struct NMLVDISPINFO<'a> {
	pub hdr: NMHDR,
	pub item: LVITEM<'a>,
}

/// [`NMLVEMPTYMARKUP`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvemptymarkup)
/// struct.
#[repr(C)]
pub struct NMLVEMPTYMARKUP {
	pub hdr: NMHDR,
	pub dwFlags: co::EMF,
	szMarkup: [u16; L_MAX_URL_LENGTH],
}

impl_default!(NMLVEMPTYMARKUP);

impl NMLVEMPTYMARKUP {
	pub_fn_string_arr_get_set!(szMarkup, set_szMarkup);
}

/// [`NMLVFINDITEM`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvfinditemw)
/// struct.
#[repr(C)]
pub struct NMLVFINDITEM<'a> {
	pub hdr: NMHDR,
	pub iStart: i32,
	pub lvfi: LVFINDINFO<'a>,
}

/// [`NMLVGETINFOTIP`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvgetinfotipw)
/// struct.
#[repr(C)]
pub struct NMLVGETINFOTIP<'a> {
	pub hdr: NMHDR,
	pub dwFlags: co::LVGIT,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iItem: i32,
	pub iSubItem: i32,
	pub lParam: isize,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(NMLVGETINFOTIP, 'a);

impl<'a> NMLVGETINFOTIP<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`NMLVKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvkeydown)
/// struct.
#[repr(C)]
pub struct NMLVKEYDOWN {
	pub hdr: NMHDR,
	pub wVKey: co::VK,
	flags: u32,
}

impl_default!(NMLVKEYDOWN);

/// [`NMLVLINK`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvlink)
/// struct.
#[repr(C)]
pub struct NMLVLINK {
	pub hdr: NMHDR,
	pub link: LITEM,
	pub iItem: i32,
	pub iSubItem: i32,
}

/// [`NMLVODSTATECHANGE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvodstatechange)
/// struct.
#[repr(C)]
pub struct NMLVODSTATECHANGE {
	pub hdr: NMHDR,
	pub iFrom: i32,
	pub iTo: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
}

/// [`NMLVSCROLL`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvscroll)
/// struct.
#[repr(C)]
pub struct NMLVSCROLL {
	pub hdr: NMHDR,
	pub dx: i32,
	pub dy: i32,
}

/// [`NMMOUSE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmmouse)
/// struct.
#[repr(C)]
pub struct NMMOUSE {
	pub hdr: NMHDR,
	pub dwItemSpec: usize,
	pub dwItemData: usize,
	pub pt: POINT,
	pub dwHitInfo: isize,
}

/// [`NMTRBTHUMBPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtrbthumbposchanging)
/// struct.
#[repr(C)]
pub struct NMTRBTHUMBPOSCHANGING {
	pub hdr: NMHDR,
	pub dwPos: u32,
	pub nReason: co::TB,
}

/// [`NMSELCHANGE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmselchange)
/// struct.
#[repr(C)]
pub struct NMSELCHANGE {
	pub nmhdr: NMHDR,
	pub stSelStart: SYSTEMTIME,
	pub stSelEnd: SYSTEMTIME,
}

/// [`NMTCKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtckeydown)
/// struct.
#[repr(C)]
pub struct NMTCKEYDOWN {
	pub hdr: NMHDR,
	pub wVKey: co::VK,
	pub flags: u32,
}

impl_default!(NMTCKEYDOWN);

/// [`NMTREEVIEW`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtreevieww)
/// struct.
#[repr(C)]
pub struct NMTREEVIEW<'a, 'b> {
	pub hdr: NMHDR,
	pub action: u32, // actual type varies
	pub itemOld: TVITEM<'a>,
	pub itemNew: TVITEM<'b>,
	pub ptDrag: POINT,
}

/// [`NMTVCUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvcustomdraw)
/// stuct.
#[repr(C)]
pub struct NMTVCUSTOMDRAW {
	pub nmcd: NMCUSTOMDRAW,
	pub clrText: COLORREF,
	pub clrTextBk: COLORREF,
	pub iLevel: i32,
}

/// [`NMTVITEMCHANGE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvitemchange)
/// struct.
#[repr(C)]
pub struct NMTVITEMCHANGE {
	pub hdr: NMHDR,
	pub uChanged: co::TVIF,
	pub hItem: HTREEITEM,
	pub uStateNew: co::TVIS,
	pub uStateOld: co::TVIS,
	pub lParam: isize,
}

/// [`NMUPDOWN`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmupdown)
/// struct.
#[repr(C)]
pub struct NMUPDOWN {
	pub hdr: NMHDR,
	pub iPos: i32,
	pub iDelta: i32,
}

/// [`NMVIEWCHANGE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmviewchange)
/// struct.
#[repr(C)]
pub struct NMVIEWCHANGE {
	pub nmhdr: NMHDR,
	pub dwOldView: co::MCMV,
	pub dwNewView: co::MCMV,
}

/// [`PBRANGE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-pbrange)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PBRANGE {
	pub iLow: i32,
	pub iHigh: i32,
}

/// [`PROPSHEETHEADER`](https://learn.microsoft.com/en-us/windows/win32/controls/pss-propsheetheader)
/// struct.
#[repr(C)]
pub struct PROPSHEETHEADER<'a, 'b, 'c, 'd, 'e, 'f> {
	dwSize: u32,
	pub dwFlags: co::PSH,
	pub hwndParent: HWND,
	pub hInstance: HINSTANCE,
	hIcon_pszIcon: *mut std::ffi::c_void, // union
	pszCaption: *mut u16,
	pub nPages: u32,
	union0: PROPSHEETHEADER_union0,
	ppsp_phpage: *mut std::ffi::c_void, // union,
	pub pfnCallback: Option<PFNPROPSHEETCALLBACK>,
	hbmWatermark_pszbmWatermark: *mut std::ffi::c_void, // union
	pub hplWatermark: HPALETTE,
	hbmHeader_pszbmHeader: *mut std::ffi::c_void, // union

	_pszIcon: PhantomData<&'a u16>,
	_pszTitle: PhantomData<&'b mut u16>,
	_pStartPage: PhantomData<&'c mut u16>,
	_ppsp_phpage: PhantomData<&'d [PROPSHEETPAGE<'d, 'd, 'd, 'd, 'd, 'd, 'd>]>,
	_pszbmWatermark: PhantomData<&'e mut u16>,
	_pszbmHeader: PhantomData<&'f mut u16>,
}

#[repr(C)]
union PROPSHEETHEADER_union0 {
	nStartPage: u32,
	pStartPage: *const u16,
}

impl_default!(PROPSHEETHEADER, dwSize, 'a, 'b, 'c, 'd, 'e, 'f);

impl<'a, 'b, 'c, 'd, 'e, 'f> PROPSHEETHEADER<'a, 'b, 'c, 'd, 'e, 'f> {
	/// Sets the `hIcon` field, which is part of an union.
	pub const fn set_hIcon(&mut self, hicon: HICON) {
		self.hIcon_pszIcon = hicon.ptr();
	}

	/// Sets the `pszIcon` field, which is part of an union.
	pub fn set_pszIcon(&mut self, buf: &'a mut IdStr) {
		self.hIcon_pszIcon = match buf {
			IdStr::Id(id) => *id as _,
			IdStr::Str(wstr) => unsafe { wstr.as_mut_ptr() as _ },
		};
	}

	pub_fn_string_ptr_get_set!('b, pszCaption, set_pszCaption);

	/// Sets the `nStartPage` field, which is part of an union.
	pub const fn set_nStartPage(&mut self, n: u32) {
		self.union0.nStartPage = n;
	}

	/// Sets the `pStartPage` field, which is part of an union.
	pub fn set_pStartPage(&mut self, buf: &'c mut WString) {
		self.union0.pStartPage = unsafe { buf.as_mut_ptr() } as _;
	}

	/// Sets the `ppsp` field, which is part of an union.
	pub const fn set_ppsp(&mut self, pages: &'d [PROPSHEETPAGE]) {
		self.ppsp_phpage = pages as *const _ as _;
	}

	/// Sets the `phpage` field, which is part of an union.
	pub const fn set_phpage(&mut self, pages: &'d [HPROPSHEETPAGE]) {
		self.ppsp_phpage = pages as *const _ as _;
	}

	/// Sets the `hbmWatermark` field, which is part of an union.
	pub const fn set_hbmWatermark(&mut self, hbm: HBITMAP) {
		self.hbmWatermark_pszbmWatermark = hbm.ptr();
	}

	/// Sets the `pszbmWatermark` field, which is part of an union.
	pub fn set_pszbmWatermark(&mut self, buf: &'e mut IdStr) {
		self.hbmWatermark_pszbmWatermark = match buf {
			IdStr::Id(id) => *id as _,
			IdStr::Str(wstr) => unsafe { wstr.as_mut_ptr() as _ },
		};
	}

	/// Sets the `hbmHeader` field, which is part of an union.
	pub const fn set_hbmHeader(&mut self, hbm: HBITMAP) {
		self.hbmHeader_pszbmHeader = hbm.ptr();
	}

	/// Sets the `pszbmHeader` field, which is part of an union.
	pub fn set_pszbmHeader(&mut self, buf: &'f mut IdStr) {
		self.hbmHeader_pszbmHeader = match buf {
			IdStr::Id(id) => *id as _,
			IdStr::Str(wstr) => unsafe { wstr.as_mut_ptr() as _ },
		};
	}
}

/// [`PROPSHEETPAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/pss-propsheetpage)
/// struct.
#[repr(C)]
pub struct PROPSHEETPAGE<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	dwSize: u32,
	pub dwFlags: co::PSP,
	pub hInstance: HINSTANCE,
	pszTemplate_pResource: *mut std::ffi::c_void, // union
	hIcon_pszIcon: *mut std::ffi::c_void,         // union
	pszTitle: *mut u16,
	pub pfnDlgProc: Option<DLGPROC>,
	pub lParam: isize,
	pub pfnCallback: Option<LPFNPSPCALLBACK>,
	pcRefParent: *mut u32,
	pszHeaderTitle: *mut u16,
	pszHeaderSubTitle: *mut u16,
	pub hActCtx: *mut std::ffi::c_void,
	hbmHeader_pszbmHeader: *mut std::ffi::c_void, // union

	_pszTemplate: PhantomData<&'a mut u16>,
	_pszIcon: PhantomData<&'b mut u16>,
	_pszTitle: PhantomData<&'c mut u16>,
	_pcRefParent: PhantomData<&'d mut u32>,
	_pszHeaderTitle: PhantomData<&'e mut u16>,
	_pszHeaderSubTitle: PhantomData<&'f mut u16>,
	_pszbmHeader: PhantomData<&'g u16>,
}

impl_default!(PROPSHEETPAGE, dwSize, 'a, 'b, 'c, 'd, 'e, 'f, 'g);

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> PROPSHEETPAGE<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	/// Sets the `pszTemplate` field, which is part of an union.
	pub fn set_pszTemplate(&mut self, buf: &'a mut IdStr) {
		self.pszTemplate_pResource = match buf {
			IdStr::Id(id) => *id as _,
			IdStr::Str(wstr) => unsafe { wstr.as_mut_ptr() as _ },
		};
	}

	/// Sets the `pResource` field, which is part of an union.
	pub const fn set_pResource(&mut self, r: &DLGTEMPLATE) {
		self.pszTemplate_pResource = r as *const _ as _;
	}

	/// Sets the `hIcon` field, which is part of an union.
	pub const fn set_hIcon(&mut self, hicon: HICON) {
		self.hIcon_pszIcon = hicon.ptr();
	}

	/// Sets the `pszIcon` field, which is part of an union.
	pub fn set_pszIcon(&mut self, buf: &'b mut IdStr) {
		self.hIcon_pszIcon = match buf {
			IdStr::Id(id) => *id as _,
			IdStr::Str(wstr) => unsafe { wstr.as_mut_ptr() as _ },
		};
	}

	pub_fn_string_ptr_get_set!('c, pszTitle, set_pszTitle);

	/// Sets the `pcRefParent` field.
	pub const fn set_pcRefParent(&mut self, ref_count: &'d mut u32) {
		self.pcRefParent = ref_count;
	}

	pub_fn_string_ptr_get_set!('e, pszHeaderTitle, set_pszHeaderTitle);
	pub_fn_string_ptr_get_set!('f, pszHeaderSubTitle, set_pszHeaderSubTitle);

	/// Sets the `hbmHeader` field, which is part of an union.
	pub const fn set_hbmHeader(&mut self, hbm: HBITMAP) {
		self.hbmHeader_pszbmHeader = hbm.ptr();
	}

	/// Sets the `pszbmHeader` field, which is part of an union.
	pub fn set_pszbmHeader(&mut self, buf: &'g mut IdStr) {
		self.hbmHeader_pszbmHeader = match buf {
			IdStr::Id(id) => *id as _,
			IdStr::Str(wstr) => unsafe { wstr.as_mut_ptr() as _ },
		};
	}
}

/// [`TBADDBITMAP`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbaddbitmap)
/// struct.
#[repr(C)]
pub struct TBADDBITMAP {
	hInst: HINSTANCE,
	nID: usize,
}

impl_default!(TBADDBITMAP);

impl TBADDBITMAP {
	/// Returns the `hInst` and `nID` fields.
	#[must_use]
	pub fn nID(&self) -> BmpIdbRes {
		if self.hInst.ptr() as isize == HINST_COMMCTRL {
			BmpIdbRes::Idb(unsafe { co::IDB::from_raw(self.nID) })
		} else if self.hInst == HINSTANCE::NULL {
			BmpIdbRes::Bmp(unsafe { HBITMAP::from_ptr(self.nID as _) })
		} else {
			unsafe { BmpIdbRes::Res(IdStr::from_ptr(self.nID as _), self.hInst.raw_copy()) }
		}
	}

	/// Sets the `hInst` and `nID` fields.
	pub fn set_nID(&mut self, val: &BmpIdbRes) {
		*self = match val {
			BmpIdbRes::Idb(idb) => Self {
				hInst: unsafe { HINSTANCE::from_ptr(HINST_COMMCTRL as _) },
				nID: idb.raw(),
			},
			BmpIdbRes::Bmp(bmp) => Self {
				hInst: HINSTANCE::NULL,
				nID: bmp.ptr() as _,
			},
			BmpIdbRes::Res(res, hInst) => Self {
				hInst: unsafe { hInst.raw_copy() },
				nID: res.as_ptr() as _,
			},
		}
	}
}

/// [`TBBUTTON`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbbutton)
/// struct.
#[repr(C)]
pub struct TBBUTTON<'a> {
	pub iBitmap: i32,
	pub idCommand: i32,
	pub fsState: co::TBSTATE,
	pub fsStyle: co::BTNS,
	bReserved: [u8; 6], // assumes 64-bit architecture
	pub dwData: usize,
	iString: isize,

	_iString: PhantomData<&'a mut u16>,
}

impl_default!(TBBUTTON, 'a);

impl<'a> TBBUTTON<'a> {
	/// Returns the `iString` field.
	#[must_use]
	pub fn iString(&self) -> IdxStr {
		if IS_INTRESOURCE(self.iString as _) {
			IdxStr::Idx(self.iString as _)
		} else {
			IdxStr::Str(unsafe { WString::from_wchars_nullt(self.iString as _) })
		}
	}

	/// Sets the `iString` field.
	pub fn set_iString(&mut self, val: &'a mut IdxStr) {
		self.iString = match val {
			IdxStr::Idx(i) => *i as _,
			IdxStr::Str(s) => unsafe { s.as_mut_ptr() as _ },
		};
	}
}

/// [`TBBUTTONINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbbuttoninfow)
/// struct.
#[repr(C)]
pub struct TBBUTTONINFO<'a> {
	cbSize: u32,
	pub dwMask: co::TBIF,
	pub idCommand: i32,
	pub iImage: i32,
	pub fsState: co::TBSTATE,
	pub fsStyle: co::BTNS,
	pub cx: u16,
	pub lParam: usize,
	pszText: *mut u16,
	cchText: i32,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(TBBUTTONINFO, cbSize, 'a);

impl<'a> TBBUTTONINFO<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchText);
}

/// [`TBINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbinsertmark)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct TBINSERTMARK {
	pub iButton: i32,
	pub dwFlags: co::TBIMHT,
}

/// [`TBMETRICS`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbmetrics)
/// struct.
#[repr(C)]
pub struct TBMETRICS {
	cbSize: u32,
	pub dwMask: co::TBMF,
	pub cxPad: i32,
	pub cyPad: i32,
	pub cxBarPad: i32,
	pub cyBarPad: i32,
	pub cxButtonSpacing: i32,
	pub cyButtonSpacing: i32,
}

impl_default!(TBMETRICS, cbSize);

/// [`TBREPLACEBITMAP`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbreplacebitmap)
/// struct.
#[repr(C)]
pub struct TBREPLACEBITMAP {
	hInstOld: HINSTANCE,
	nIDOld: usize,
	hInstNew: HINSTANCE,
	nIDNew: usize,
	pub nButtons: i32,
}

impl_default!(TBREPLACEBITMAP);

impl TBREPLACEBITMAP {
	/// Returns the `hInstOld` and `nIDOld` fields.
	#[must_use]
	pub fn olds(&self) -> BmpInstId {
		if self.hInstOld == HINSTANCE::NULL {
			BmpInstId::Bmp(unsafe { HBITMAP::from_ptr(self.nIDOld as _) })
		} else {
			BmpInstId::InstId(unsafe { self.hInstOld.raw_copy() }, self.nIDOld as _)
		}
	}

	/// Sets the `hInstOld` and `nIDOld` fields.
	pub fn set_olds(&mut self, val: BmpInstId) {
		match val {
			BmpInstId::Bmp(hbmp) => {
				self.hInstOld = HINSTANCE::NULL;
				self.nIDOld = hbmp.ptr() as _;
			},
			BmpInstId::InstId(hinst, id) => {
				self.hInstOld = hinst;
				self.nIDOld = id as _;
			},
		}
	}

	/// Returns the `hInstNew` and `nIDNew` fields.
	#[must_use]
	pub fn news(&self) -> BmpInstId {
		if self.hInstNew == HINSTANCE::NULL {
			BmpInstId::Bmp(unsafe { HBITMAP::from_ptr(self.nIDNew as _) })
		} else {
			BmpInstId::InstId(unsafe { self.hInstNew.raw_copy() }, self.nIDNew as _)
		}
	}

	/// Sets the `hInstNew` and `nIDNew` fields.
	pub fn set_news(&mut self, val: BmpInstId) {
		match val {
			BmpInstId::Bmp(hbmp) => {
				self.hInstNew = HINSTANCE::NULL;
				self.nIDNew = hbmp.ptr() as _;
			},
			BmpInstId::InstId(hinst, id) => {
				self.hInstNew = hinst;
				self.nIDNew = id as _;
			},
		}
	}
}

/// [`TCHITTESTINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tchittestinfo)
/// struct.
#[repr(C)]
pub struct TCHITTESTINFO {
	pub pt: POINT,
	pub flags: co::TCHT,
}

impl_default!(TCHITTESTINFO);

/// [`TCITEM`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tcitemw)
/// struct.
#[repr(C)]
pub struct TCITEM<'a> {
	pub mask: co::TCIF,
	pub dwState: co::TCIS,
	pub dwStateMask: co::TCIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub lParam: isize,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(TCITEM, 'a);

impl<'a> TCITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`TVHITTESTINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvhittestinfo)
/// struct.
#[repr(C)]
pub struct TVHITTESTINFO {
	pub pt: POINT,
	pub flags: co::TVHT,
	pub hitem: HTREEITEM,
}

/// [`TVINSERTSTRUCT`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvinsertstructw)
/// struct.
#[repr(C)]
pub struct TVINSERTSTRUCT<'a> {
	pub hParent: HTREEITEM,
	hInsertAfter: isize,
	pub itemex: TVITEMEX<'a>,
}

impl_default!(TVINSERTSTRUCT, 'a);

impl<'a> TVINSERTSTRUCT<'a> {
	/// Returns the `hInsertAfter` field.
	#[must_use]
	pub fn hInsertAfter(&self) -> TreeitemTvi {
		TreeitemTvi::from_isize(self.hInsertAfter)
	}

	/// Sets the `hInsertAfter` field.
	pub fn set_hInsertAfter(&mut self, val: TreeitemTvi) {
		self.hInsertAfter = val.into();
	}
}

/// [`TVITEMEX`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvitemexw)
/// struct.
#[repr(C)]
pub struct TVITEMEX<'a> {
	pub mask: co::TVIF,
	pub hItem: HTREEITEM,
	pub state: co::TVIS,
	pub stateMask: co::TVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub iSelectedImage: i32,
	pub cChildren: i32,
	pub lParam: isize,
	pub iIntegral: i32,
	pub uStateEx: co::TVIS_EX,
	hwnd: HWND,
	pub iExpandedImage: i32,
	iReserved: i32,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(TVITEMEX, 'a);

impl<'a> TVITEMEX<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`TVITEM`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvitemw)
/// struct.
#[repr(C)]
pub struct TVITEM<'a> {
	pub mask: co::TVIF,
	pub hItem: HTREEITEM,
	pub state: co::TVIS,
	pub stateMask: co::TVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub iSelectedImage: i32,
	pub cChildren: i32,
	pub lParam: isize,

	_pszText: PhantomData<&'a mut u16>,
}

impl_default!(TVITEM, 'a);

impl<'a> TVITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, raw_pszText, cchTextMax);
}

/// [`TVSORTCB`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvsortcb)
/// struct.
#[repr(C)]
pub struct TVSORTCB {
	pub hParent: HTREEITEM,
	pub lpfnCompare: Option<PFNTVCOMPARE>,
	pub lParam: isize,
}

impl_default!(TVSORTCB);

/// [`UDACCEL`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-udaccel)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct UDACCEL {
	pub nSec: u32,
	pub nInc: u32,
}
