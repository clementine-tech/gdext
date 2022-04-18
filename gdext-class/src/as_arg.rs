use gdext_sys as sys;
use sys::GodotFfi;

use crate::{EngineClass, GodotClass, Obj};

mod private {
    pub trait Sealed {}
}
use private::Sealed;

pub trait AsArg: private::Sealed {
    #[doc(hidden)]
    fn as_arg_ptr(&self) -> sys::GDNativeTypePtr;
}

impl<T: GodotClass> Sealed for Obj<T> {}
impl<T: GodotClass> AsArg for Obj<T> {
    fn as_arg_ptr(&self) -> sys::GDNativeTypePtr {
        self.sys() as _ // TODO:check
    }
}

impl<T: EngineClass> Sealed for &T {}
impl<T: EngineClass> AsArg for &T {
    fn as_arg_ptr(&self) -> sys::GDNativeTypePtr {
        self.as_object_ptr() as _ // TODO:check
    }
}