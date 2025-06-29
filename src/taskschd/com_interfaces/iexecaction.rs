#![allow(non_camel_case_types, non_snake_case)]

use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { IExecAction: "4c3d624d-fd6b-49a3-b9b7-09cb3cd3f047";
	/// [`IExecAction`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iexecaction)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let action: w::IAction; // initialized somewhere
	/// # let action = unsafe { w::IAction::null() };
	///
	/// let exec_action = action
	///     .QueryInterface::<w::IExecAction>()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl oleaut_IDispatch for IExecAction {}
impl taskschd_IAction for IExecAction {}
impl taskschd_IExecAction for IExecAction {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IExecAction`](crate::IExecAction).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IExecAction: taskschd_IAction {
	fn_com_bstr_get! { get_Arguments: IExecActionVT;
		/// [`IExecAction::get_Arguments`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-get_arguments)
		/// method.
	}

	fn_com_bstr_get! { get_Path: IExecActionVT;
		/// [`IExecAction::get_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-get_path)
		/// method.
	}

	fn_com_bstr_get! { get_WorkingDirectory: IExecActionVT;
		/// [`IExecAction::get_WorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-get_workingdirectory)
		/// method.
	}

	fn_com_bstr_set! { put_Arguments: IExecActionVT, arguments;
		/// [`IExecAction::get_Arguments`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-put_arguments)
		/// method.
	}

	fn_com_bstr_set! { put_Path: IExecActionVT, path;
		/// [`IExecAction::put_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-put_path)
		/// method.
	}

	fn_com_bstr_set! { put_WorkingDirectory: IExecActionVT, working_directory;
		/// [`IExecAction::put_WorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-put_workingdirectory)
		/// method.
	}
}
