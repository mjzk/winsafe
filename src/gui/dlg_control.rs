use std::sync::Arc;

use crate::aliases::ErrResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, paint_control_borders};
use crate::structs::{POINT, SIZE};

#[derive(Clone)]
pub(in crate::gui) struct DlgControl(Arc<Obj>);

struct Obj { // actual fields of DlgControl
	base: DlgBase,
	position: POINT,
	ctrl_id: Option<u16>,
}

impl DlgControl {
	pub(in crate::gui) fn new(
		parent_base_ref: &Base,
		dialog_id: u16,
		position: POINT,
		ctrl_id: Option<u16>) -> DlgControl
	{
		let dlg = Self(
			Arc::new(
				Obj {
					base: DlgBase::new(Some(parent_base_ref), dialog_id),
					position,
					ctrl_id,
				},
			),
		);
		dlg.0.base.ui_thread_message_handler();
		dlg.default_message_handlers(parent_base_ref);
		dlg
	}

	pub(in crate::gui) fn base_ref(&self) -> &Base {
		self.0.base.base_ref()
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()>,
	{
		self.0.base.run_ui_thread(func);
	}

	fn default_message_handlers(&self, parent_base_ref: &Base) {
		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let self2 = self.clone();
			move |_| {
				// Create the control.
				self2.0.base.create_dialog_param()?; // may panic

				// Set control position within parent.
				let mut dlg_pos = self2.0.position;
				multiply_dpi(Some(&mut dlg_pos), None)?;
				self2.base_ref().hwnd_ref().SetWindowPos(
					HwndPlace::None,
					dlg_pos, SIZE::default(),
					co::SWP::NOZORDER | co::SWP::NOSIZE,
				)?;

				// Give the control an ID.
				self2.base_ref().hwnd_ref().SetWindowLongPtr(
					co::GWLP::ID,
					self2.0.ctrl_id.unwrap_or_else(|| auto_ctrl_id()) as _,
				);
				Ok(0)
			}
		});

		self.base_ref().user_events_ref().wm_nc_paint({
			let self2 = self.clone();
			move |p| {
				paint_control_borders(*self2.base_ref().hwnd_ref(), p)?;
				Ok(())
			}
		});
	}
}
