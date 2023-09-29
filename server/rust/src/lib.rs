mod bindings;
use std::sync::*;
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[implement(bindings::Reader)]
struct Reader;

impl bindings::IReader_Impl for Reader {
    fn P0(&self) -> Result<HSTRING> {
        // TODO: since Rust is always pinned this should just use an HSTRING reference
        // and not create a copy.
        Ok(h!("P0").clone())
    }
    fn P1(&self) -> Result<HSTRING> {
        Ok(h!("P1").clone())
    }
}

#[implement(IActivationFactory)]
struct ReaderFactory;

impl IActivationFactory_Impl for ReaderFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Reader.into())
    }
}

#[implement(bindings::Writer)]
struct Writer(RwLock<[HSTRING;2]>);

impl bindings::IWriter_Impl for Writer {
    fn P0(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[0].clone())
    }
    fn SetP0(&self, p1: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[0] = p1.clone();
        Ok(())
    }

    fn P1(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[1].clone())
    }
    fn SetP1(&self, p1: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[1] = p1.clone();
        Ok(())
    }
}

#[implement(IActivationFactory)]
struct WriterFactory;

impl IActivationFactory_Impl for WriterFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Writer(RwLock::new(Default::default())).into())
    }
}

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: std::mem::ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    // TODO: this to_string conversion seems wasteful. Should be able to compare
    // these to h!("...") in the match expression.
    let factory: Option<IActivationFactory> = match (*name).to_string().as_str() {
        "Test.Reader" => Some(ReaderFactory.into()),
        "Test.Writer" => Some(WriterFactory.into()),
        _ => None,
    };

    if let Some(factory) = factory {
        *result = std::mem::transmute(factory);
        S_OK
    } else {
        *result = std::ptr::null_mut();
        CLASS_E_CLASSNOTAVAILABLE
    }
}
