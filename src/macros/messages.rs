#![allow(unused_macros)]

/// Struct for a message that has no parameters and no meaningful return value.
macro_rules! pub_struct_msg_empty {
	(
		$name:ident : $wmconst:expr;
		$( #[$msdn:meta] )*
	) => {
		$( #[$msdn] )*
		/// message, which has no parameters.
		///
		/// Return type: `()`.
		#[derive(Clone, Copy)]
		pub struct $name {}

		impl crate::prelude::MsgSend for $name {
			type RetType = ();

			unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&mut self) -> crate::msg::WndMsg {
				crate::msg::WndMsg {
					msg_id: $wmconst,
					wparam: 0,
					lparam: 0,
				}
			}
		}
	};
}

/// Struct for a handleable message that has no parameters and no meaningful
/// return value.
macro_rules! pub_struct_msg_empty_handleable {
	(
		$name:ident : $wmconst:expr;
		$( #[$msdn:meta] )*
	) => {
		pub_struct_msg_empty! {
			$name : $wmconst;
			$( #[$msdn] )*
		}

		impl crate::prelude::MsgSendRecv for $name {
			unsafe fn from_generic_wm(_: crate::msg::WndMsg) -> Self {
				Self {}
			}
		}
	};
}

/// Struct for WM_CHAR-based handleable messages.
macro_rules! pub_struct_msg_char_code {
	(
		$name:ident : $wmconst:expr;
		$( #[$msdn:meta] )*
	) => {
		$( #[$msdn] )*
		/// message parameters.
		///
		/// Return type: `()`.
		pub struct $name {
			pub char_code: u16,
			pub repeat_count: u16,
			pub scan_code: u8,
			pub is_extended_key: bool,
			pub has_alt_key: bool,
			pub key_was_previously_down: bool,
			pub key_is_being_released: bool,
		}

		impl crate::prelude::MsgSend for $name {
			type RetType = ();

			unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&mut self) -> crate::msg::WndMsg {
				crate::msg::WndMsg {
					msg_id: $wmconst,
					wparam: self.char_code as _,
					lparam: crate::MAKEDWORD(
						self.repeat_count,
						crate::MAKEWORD(
							self.scan_code,
							if self.is_extended_key { 0b0000_0001 } else { 0 } |
							if self.has_alt_key { 0b0010_0000 } else { 0 } |
							if self.key_was_previously_down { 0b0100_0000 } else { 0 } |
							if self.key_is_being_released { 0b1000_0000 } else { 0 },
						),
					) as _,
				}
			}
		}

		impl crate::prelude::MsgSendRecv for $name {
			unsafe fn from_generic_wm(p: crate::msg::WndMsg) -> Self {
				use crate::{HIBYTE, HIWORD, LOBYTE, LOWORD};
				Self {
					char_code: p.wparam as _,
					repeat_count: LOWORD(p.lparam as _),
					scan_code: LOBYTE(HIWORD(p.lparam as _)),
					is_extended_key: (HIBYTE(HIWORD(p.lparam as _)) & 0b0000_0001) != 0,
					has_alt_key: (HIBYTE(HIWORD(p.lparam as _)) & 0b0010_0000) != 0,
					key_was_previously_down: (HIBYTE(HIWORD(p.lparam as _)) & 0b0100_0000) != 0,
					key_is_being_released: (HIBYTE(HIWORD(p.lparam as _)) & 0b1000_0000) != 0,
				}
			}
		}
	};
}

/// Struct for WM_KEY-based handleable messages.
macro_rules! pub_struct_msg_char_key {
	(
		$name:ident : $wmconst:expr;
		$( #[$msdn:meta] )*
	) => {
		$( #[$msdn] )*
		/// message parameters.
		///
		/// Return type: `()`.
		pub struct $name {
			pub vkey_code: co::VK,
			pub repeat_count: u16,
			pub scan_code: u8,
			pub is_extended_key: bool,
			pub has_alt_key: bool,
			pub key_was_previously_down: bool,
			pub key_is_being_released: bool,
		}

		impl crate::prelude::MsgSend for $name {
			type RetType = ();

			unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&mut self) -> crate::msg::WndMsg {
				crate::msg::WndMsg {
					msg_id: $wmconst,
					wparam: self.vkey_code.raw() as _,
					lparam: crate::MAKEDWORD(
						self.repeat_count,
						crate::MAKEWORD(
							self.scan_code,
							if self.is_extended_key { 0b0000_0001 } else { 0 } |
							if self.has_alt_key { 0b0010_0000 } else { 0 } |
							if self.key_was_previously_down { 0b0100_0000 } else { 0 } |
							if self.key_is_being_released { 0b1000_0000 } else { 0 },
						),
					) as _,
				}
			}
		}

		impl crate::prelude::MsgSendRecv for $name {
			unsafe fn from_generic_wm(p: crate::msg::WndMsg) -> Self {
				use crate::{HIBYTE, HIWORD, LOBYTE, LOWORD};
				Self {
					vkey_code: unsafe{co::VK::from_raw(p.wparam as _)},
					repeat_count: LOWORD(p.lparam as _),
					scan_code: LOBYTE(HIWORD(p.lparam as _)),
					is_extended_key: (HIBYTE(HIWORD(p.lparam as _)) & 0b0000_0001) != 0,
					has_alt_key: (HIBYTE(HIWORD(p.lparam as _)) & 0b0010_0000) != 0,
					key_was_previously_down: (HIBYTE(HIWORD(p.lparam as _)) & 0b0100_0000) != 0,
					key_is_being_released: (HIBYTE(HIWORD(p.lparam as _)) & 0b1000_0000) != 0,
				}
			}
		}
	};
}

/// Struct for WM_CTLCOLOR* handleable messages.
macro_rules! pub_struct_msg_ctlcolor {
	(
		$name:ident : $wmconst:expr;
		$( #[$msdn:meta] )*
	) => {
		$( #[$msdn] )*
		/// message parameters.
		///
		/// Return type: `HBRUSH`.
		pub struct $name {
			pub hdc: crate::HDC,
			pub hwnd: crate::HWND,
		}

		impl crate::prelude::MsgSend for $name {
			type RetType = crate::HBRUSH;

			unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
				unsafe {
					<crate::HBRUSH as crate::prelude::Handle>::from_ptr(v as _)
				}
			}

			fn as_generic_wm(&mut self) -> crate::msg::WndMsg {
				crate::msg::WndMsg {
					msg_id: $wmconst,
					wparam: <crate::HDC as crate::prelude::Handle>::ptr(&self.hdc) as _,
					lparam: <crate::HWND as crate::prelude::Handle>::ptr(&self.hwnd) as _,
				}
			}
		}

		impl crate::prelude::MsgSendRecv for $name {
			unsafe fn from_generic_wm(p: crate::msg::WndMsg) -> Self {
				unsafe {
					Self {
						hdc: <crate::HDC as crate::prelude::Handle>::from_ptr(p.wparam as _),
						hwnd: <crate::HWND as crate::prelude::Handle>::from_ptr(p.lparam as _),
					}
				}
			}
		}
	};
}

/// Struct for WM_*BUTTON* handleable messages and others.
macro_rules! pub_struct_msg_button {
	(
		$name:ident : $wmconst:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		/// message parameters.
		///
		/// Return type: `()`.
		pub struct $name {
			pub vkey_code: crate::co::VK,
			pub coords: crate::POINT,
		}

		impl crate::prelude::MsgSend for $name {
			type RetType = ();

			unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&mut self) -> WndMsg {
				crate::msg::WndMsg {
					msg_id: $wmconst,
					wparam: self.vkey_code.raw() as _,
					lparam: u32::from(self.coords) as _,
				}
			}
		}

		impl crate::prelude::MsgSendRecv for $name {
			unsafe fn from_generic_wm(p: crate::msg::WndMsg) -> Self {
				Self {
					vkey_code: unsafe { crate::co::VK::from_raw(p.wparam as _) },
					coords: crate::POINT {
						x: crate::LOWORD(p.lparam as _) as _,
						y: crate::HIWORD(p.lparam as _) as _,
					},
				}
			}
		}
	};
}
