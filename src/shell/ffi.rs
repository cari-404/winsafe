use crate::kernel::ffi_types::*;

extern_sys! { "shell32";
	CommandLineToArgvW(PCSTR, *mut i32) -> *mut PSTR
	DragAcceptFiles(HANDLE, BOOL)
	DragFinish(HANDLE)
	DragQueryFileW(HANDLE, u32, PSTR, u32) -> u32
	DragQueryPoint(HANDLE, PVOID) -> BOOL
	SHAddToRecentDocs(u32, PCVOID)
	SHBindToParent(PCVOID, PCVOID, *mut COMPTR, PVOID) -> HRES
	SHCreateItemFromIDList(PCVOID, PCVOID, *mut COMPTR) -> HRES
	SHCreateItemFromParsingName(PCSTR, PVOID, PCVOID, *mut COMPTR) -> HRES
	SHCreateItemFromRelativeName(COMPTR, PCSTR, COMPTR, PCVOID, *mut COMPTR) -> HRES
	SHCreateItemInKnownFolder(PCVOID, u32, PCSTR, PCVOID, *mut COMPTR) -> HRES
	SHCreateShellItemArrayFromShellItem(COMPTR, PCVOID, *mut COMPTR) -> HRES
	Shell_NotifyIconW(u32, PCVOID) -> BOOL
	ShellAboutW(HANDLE, PCSTR, PCSTR, HANDLE) -> i32
	ShellExecuteW(HANDLE, PCSTR, PCSTR, PCSTR, PCSTR, i32) -> HANDLE
	SHFileOperationW(PVOID) -> i32
	SHGetFileInfoW(PCSTR, u32, PVOID, u32, u32) -> usize
	SHGetIDListFromObject(COMPTR, PVOID) -> HRES
	SHGetStockIconInfo(u32, u32, PVOID) -> HRES
}

extern_sys! { "shlwapi";
	PathCombineW(PSTR, PCSTR, PCSTR) -> PSTR
	PathCommonPrefixW(PCSTR, PCSTR, PSTR) -> i32
	PathSkipRootW(PCSTR) -> PCSTR
	PathStripPathW(PSTR)
	PathUndecorateW(PSTR)
	PathUnquoteSpacesW(PSTR) -> BOOL
	SHCreateMemStream(*const u8, u32) -> COMPTR
}

extern_sys! { "userenv";
	GetAllUsersProfileDirectoryW(PSTR, *mut u32) -> BOOL
	GetDefaultUserProfileDirectoryW(PSTR, *mut u32) -> BOOL
	GetProfilesDirectoryW(PSTR, *mut u32) -> BOOL
}
