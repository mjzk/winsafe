#![allow(non_camel_case_types, non_snake_case)]

use crate::dshow::decl::{IBaseFilter, IEnumFilters};
use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{HRES, PCSTR, PCVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IFilterGraph`](crate::IFilterGraph) virtual table.
#[repr(C)]
pub struct IFilterGraphVT {
	pub IUnknownVT: IUnknownVT,
	pub AddFilter: fn(ComPtr, ComPtr, PCSTR) -> HRES,
	pub RemoveFilter: fn(ComPtr, ComPtr) -> HRES,
	pub EnumFilters: fn(ComPtr, *mut ComPtr) -> HRES,
	pub FindFilterByName: fn(ComPtr, PCSTR, *mut ComPtr) -> HRES,
	pub ConnectDirect: fn(ComPtr, ComPtr, ComPtr, PCVOID) -> HRES,
	pub Reconnect: fn(ComPtr, ComPtr) -> HRES,
	pub Disconnect: fn(ComPtr, ComPtr) -> HRES,
	pub SetDefaultSyncSource: fn(ComPtr) -> HRES,
}

com_interface! { IFilterGraph: "56a8689f-0ad4-11ce-b03a-0020af0ba770";
	/// [`IFilterGraph`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifiltergraph)
	/// COM interface over [`IFilterGraphVT`](crate::vt::IFilterGraphVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IFilterGraph for IFilterGraph {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IFilterGraph`](crate::IFilterGraph).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IFilterGraph: ole_IUnknown {
	/// [`IFilterGraph::AddFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-addfilter)
	/// method.
	fn AddFilter(&self, filter: &IBaseFilter, name: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IFilterGraphVT>();
			ok_to_hrresult(
				(vt.AddFilter)(
					self.ptr(),
					filter.ptr(),
					WString::from_str(name).as_ptr(),
				),
			)
		}
	}

	/// [`IFilterGraph::EnumFilters`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-enumfilters)
	/// method.
	#[must_use]
	fn EnumFilters(&self) -> HrResult<IEnumFilters> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IFilterGraphVT>();
			ok_to_hrresult(
				(vt.EnumFilters)(self.ptr(), &mut ppv_queried),
			).map(|_| IEnumFilters::from(ppv_queried))
		}
	}

	/// [`IFilterGraph::FindFilterByName`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-findfilterbyname)
	/// method.
	#[must_use]
	fn FindFilterByName(&self, name: &str) -> HrResult<IBaseFilter> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IFilterGraphVT>();
			ok_to_hrresult(
				(vt.FindFilterByName)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					&mut ppv_queried,
				),
			).map(|_| IBaseFilter::from(ppv_queried))
		}
	}

	/// [`IFilterGraph::RemoveFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-removefilter)
	/// method.
	fn RemoveFilter(&self, filter: &IBaseFilter) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IFilterGraphVT>();
			ok_to_hrresult((vt.RemoveFilter)(self.ptr(), filter.ptr()))
		}
	}

	/// [`IFilterGraph::SetDefaultSyncSource`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-setdefaultsyncsource)
	/// method.
	fn SetDefaultSyncSource(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IFilterGraphVT>();
			ok_to_hrresult((vt.SetDefaultSyncSource)(self.ptr()))
		}
	}
}
