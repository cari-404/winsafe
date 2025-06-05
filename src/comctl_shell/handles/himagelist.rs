#![allow(non_camel_case_types)]

use crate::co;
use crate::decl::*;

impl HIMAGELIST {
	/// Calls [`SHGetFileInfo`](crate::SHGetFileInfo) to retrieve one or more
	/// shell file icons, then passes them to
	/// [`AddIcon`](crate::HIMAGELIST::AddIcon).
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let mut himgl = w::HIMAGELIST::Create(
	///     w::SIZE::new(16, 16),
	///     co::ILC::COLOR32,
	///     1,
	///     1,
	/// )?;
	///
	/// himgl.add_icons_from_shell(&["mp3", "wav"])?;
	/// # w::HrResult::Ok(())
	/// ```
	pub fn add_icons_from_shell(&self, file_extensions: &[impl AsRef<str>]) -> HrResult<()> {
		let sz = self.GetIconSize()?;
		if sz != SIZE::new(16, 16) && sz != SIZE::new(32, 32) {
			return Err(co::HRESULT::E_NOTIMPL); // only 16x16 or 32x32 icons can be loaded
		}

		for file_extension in file_extensions.iter() {
			let (_, shfi) = SHGetFileInfo(
				&format!("*.{}", file_extension.as_ref()),
				co::FILE_ATTRIBUTE::NORMAL,
				co::SHGFI::USEFILEATTRIBUTES
					| co::SHGFI::ICON
					| if sz == SIZE::new(16, 16) {
						co::SHGFI::SMALLICON
					} else {
						co::SHGFI::LARGEICON
					},
			)?;
			self.AddIcon(&shfi.hIcon)?;
		}
		Ok(())
	}
}
