/// Declares the type of a handle.
macro_rules! handle_type {
	(
		$(#[$attr:meta])*
		$name:ident
	) => {
		$(#[$attr])*
		#[repr(C)]
		#[derive(Copy, Clone, Eq, PartialEq)]
		pub struct $name(HANDLE);

		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}

		impl Default for $name {
			fn default() -> Self {
				Self(std::ptr::null_mut())
			}
		}

		impl $name {
			/// Creates a new handle instance by wrapping a pointer.
			pub unsafe fn from_ptr<T>(p: *mut T) -> $name {
				Self(p as HANDLE)
			}

			/// Returns the raw underlying pointer for this handle.
			pub unsafe fn as_ptr(self) -> HANDLE {
				self.0
			}

			/// Tells if the handle is invalid (null).
			pub fn is_null(&self) -> bool {
				self.0.is_null()
			}
		}
	};
}

/// Implements conversions from and to `HGDIOBJ`.
macro_rules! convert_hgdiobj {
	($name:ident) => {
		impl From<$name> for HGDIOBJ {
			fn from(h: $name) -> Self {
				unsafe { Self::from_ptr(h.0) }
			}
		}

		impl From<HGDIOBJ> for $name {
			fn from(hgdiobj: HGDIOBJ) -> Self {
				unsafe { Self::from_ptr(hgdiobj.as_ptr()) }
			}
		}
	};
}