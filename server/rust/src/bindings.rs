// Bindings generated by `windows-bindgen` 0.52.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReader {
    type Vtable = IReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReader {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8297f123_7b25_5e8b_9311_c14c9da93f1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub P0: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub P1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWriter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWriter {
    type Vtable = IWriter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWriter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfb2d5024_d511_5915_a4fe_14cdaab8726d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub P0: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetP0: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub P1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetP1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Reader(::windows_core::IUnknown);
impl Reader {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            Reader,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn P0(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).P0)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn P1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).P1)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for Reader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"rc(Test.Reader;{8297f123-7b25-5e8b-9311-c14c9da93f1d})",
        );
}
unsafe impl ::windows_core::Interface for Reader {
    type Vtable = IReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Reader {
    const IID: ::windows_core::GUID = <IReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Reader {
    const NAME: &'static str = "Test.Reader";
}
::windows_core::imp::interface_hierarchy!(
    Reader,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for Reader {}
unsafe impl ::core::marker::Sync for Reader {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Writer(::windows_core::IUnknown);
impl Writer {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            Writer,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn P0(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).P0)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetP0(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetP0)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn P1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).P1)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetP1(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetP1)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for Writer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"rc(Test.Writer;{fb2d5024-d511-5915-a4fe-14cdaab8726d})",
        );
}
unsafe impl ::windows_core::Interface for Writer {
    type Vtable = IWriter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Writer {
    const IID: ::windows_core::GUID = <IWriter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Writer {
    const NAME: &'static str = "Test.Writer";
}
::windows_core::imp::interface_hierarchy!(
    Writer,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for Writer {}
unsafe impl ::core::marker::Sync for Writer {}
pub trait IReader_Impl: Sized {
    fn P0(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn P1(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::RuntimeName for IReader {
    const NAME: &'static str = "Test.IReader";
}
impl IReader_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IReader_Impl,
        const OFFSET: isize,
    >() -> IReader_Vtbl {
        unsafe extern "system" fn P0<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.P0() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn P1<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.P1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IReader, OFFSET>(),
            P0: P0::<Identity, Impl, OFFSET>,
            P1: P1::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IReader as ::windows_core::ComInterface>::IID
    }
}
pub trait IWriter_Impl: Sized {
    fn P0(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetP0(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn P1(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetP1(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWriter {
    const NAME: &'static str = "Test.IWriter";
}
impl IWriter_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IWriter_Impl,
        const OFFSET: isize,
    >() -> IWriter_Vtbl {
        unsafe extern "system" fn P0<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWriter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.P0() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetP0<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWriter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetP0(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn P1<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWriter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.P1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetP1<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWriter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetP1(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IWriter, OFFSET>(),
            P0: P0::<Identity, Impl, OFFSET>,
            SetP0: SetP0::<Identity, Impl, OFFSET>,
            P1: P1::<Identity, Impl, OFFSET>,
            SetP1: SetP1::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWriter as ::windows_core::ComInterface>::IID
    }
}
