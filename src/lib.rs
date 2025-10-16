use std::ffi::{CStr, c_void};

use libc::{RTLD_LAZY, RTLD_NOW};
use thiserror::Error;

pub struct Library(*mut c_void);

#[derive(Debug, Clone, Default)]
pub struct Options {
    lazy: bool,
}

#[derive(Debug, Clone, Error)]
#[error("failed to load library {0}")]
pub struct LoadError(String);

#[derive(Debug, Clone, Error)]
#[error("failed to find symbol {name} {error}")]
pub struct SymbolNotFound {
    name: String,
    error: String,
}

impl Library {
    pub unsafe fn load(name: &CStr) -> Result<Self, LoadError> {
        unsafe { Self::load_with_options(name, Options::default()) }
    }

    pub unsafe fn load_with_options(name: &CStr, options: Options) -> Result<Self, LoadError> {
        let flags = if options.lazy { RTLD_LAZY } else { RTLD_NOW };

        // Clear any potential errors before calling `dlopen`
        let _ = unsafe { libc::dlerror() };

        // SAFETY: TODO
        let ptr = unsafe { libc::dlopen(name.as_ptr(), flags) };

        if ptr.is_null() {
            let error = unsafe { libc::dlerror() };
            assert!(!error.is_null(), "error pointer should not be null");

            let error = unsafe { CStr::from_ptr(error) };
            return Err(LoadError(error.to_string_lossy().into_owned()));
        }

        Ok(Self(ptr))
    }

    pub unsafe fn get_symbol<T>(&self, name: &CStr) -> Result<Symbol<T>, SymbolNotFound> {
        // Clear any potential errors before calling `dlsym`
        let _ = unsafe { libc::dlerror() };

        let ptr = unsafe { libc::dlsym(self.0, name.as_ptr()) };

        if ptr.is_null() {
            let error = unsafe { libc::dlerror() };

            // If the error is not null then the nullptr was actually a valid return!
            // (Unless, ofcourse, we ended up racing a different call to `dlerror`)
            if !error.is_null() {
                let error = unsafe { CStr::from_ptr(error) };
                return Err(SymbolNotFound {
                    name: name.to_string_lossy().into_owned(),
                    error: error.to_string_lossy().into_owned(),
                });
            }
        }

        Ok(Symbol(&raw const ptr as *const T))
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        let _ = unsafe { libc::dlclose(self.0) };
    }
}

pub struct Symbol<T>(*const T);

impl<T> std::ops::Deref for Symbol<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
