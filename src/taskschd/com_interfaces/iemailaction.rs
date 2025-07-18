#![allow(non_camel_case_types, non_snake_case)]

use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { IEmailAction: "10f62c64-7e16-4314-a0c2-0c3683f99d40";
	/// [`IEmailAction`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iemailaction)
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
	/// let email_action = action
	///     .QueryInterface::<w::IEmailAction>()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl oleaut_IDispatch for IEmailAction {}
impl taskschd_IAction for IEmailAction {}
impl taskschd_IEmailAction for IEmailAction {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IEmailAction`](crate::IEmailAction).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IEmailAction: taskschd_IAction {
	fn_com_bstr_get! { get_Bcc: IEmailActionVT;
		/// [`IEmailAction::get_Bcc`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_bcc)
		/// method.
	}

	fn_com_bstr_get! { get_Body: IEmailActionVT;
		/// [`IEmailAction::get_Body`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_body)
		/// method.
	}

	fn_com_bstr_get! { get_Cc: IEmailActionVT;
		/// [`IEmailAction::get_Cc`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_cc)
		/// method.
	}

	fn_com_bstr_get! { get_From: IEmailActionVT;
		/// [`IEmailAction::get_From`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_from)
		/// method.
	}

	fn_com_bstr_get! { get_ReplyTo: IEmailActionVT;
		/// [`IEmailAction::get_ReplyTo`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_replyto)
		/// method.
	}

	fn_com_bstr_get! { get_Server: IEmailActionVT;
		/// [`IEmailAction::get_Server`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_server)
		/// method.
	}

	fn_com_bstr_get! { get_Subject: IEmailActionVT;
		/// [`IEmailAction::get_Subject`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_subject)
		/// method.
	}

	fn_com_bstr_get! { get_To: IEmailActionVT;
		/// [`IEmailAction::get_To`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_to)
		/// method.
	}

	fn_com_bstr_set! { put_Bcc: IEmailActionVT, bcc;
		/// [`IEmailAction::put_Bcc`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_bcc)
		/// method.
	}

	fn_com_bstr_set! { put_Body: IEmailActionVT, body;
		/// [`IEmailAction::put_Body`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_body)
		/// method.
	}

	fn_com_bstr_set! { put_Cc: IEmailActionVT, cc;
		/// [`IEmailAction::put_Cc`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_cc)
		/// method.
	}

	fn_com_bstr_set! { put_From: IEmailActionVT, from;
		/// [`IEmailAction::put_From`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_from)
		/// method.
	}

	fn_com_bstr_set! { put_ReplyTo: IEmailActionVT, reply_to;
		/// [`IEmailAction::put_ReplyTo`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_replyto)
		/// method.
	}

	fn_com_bstr_set! { put_Server: IEmailActionVT, server;
		/// [`IEmailAction::put_Server`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_server)
		/// method.
	}

	fn_com_bstr_set! { put_Subject: IEmailActionVT, subject;
		/// [`IEmailAction::put_Subject`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_subject)
		/// method.
	}

	fn_com_bstr_set! { put_To: IEmailActionVT, to;
		/// [`IEmailAction::put_To`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_to)
		/// method.
	}
}
