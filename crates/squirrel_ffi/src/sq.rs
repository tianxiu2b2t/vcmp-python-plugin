/*
    Squirrel FFI for Rust + Python
    
    FFI: https://bitbucket.org/stormeus/0.4-squirrel/src/master/
    
*/

use std::sync::OnceLock;
use vcmp_bindings::func::plugin::PluginExports;

use crate::sq_ffi::raw::{self, HSQUIRRELVM, sq_api};

#[derive(Debug, Clone)]
pub struct SquirrelImports {
    pub inner: raw::SquirrelImports,
}

impl SquirrelImports {
    pub fn get_vm(&self) -> Option<HSQUIRRELVM> {
        unsafe {
            // 调用GetSquirrelVM函数并解引用获取VM指针
            let ptr = (self.inner.GetSquirrelVM)();
            if ptr.is_null() {
                return None;
            }
            Some(*ptr)
        }
    }

    pub fn get_api(&self) -> &sq_api {
        unsafe {
            // 调用GetSquirrelAPI函数并解引用获取API指针
            let api_ptr = (self.inner.GetSquirrelAPI)();
            &*(api_ptr as *const sq_api)
        }
    }
}


pub const SQUIRREL_LOAD_IDENTIFIER: u32 = 0x7D6E22D8;
pub static SQIURREL_IMPORTS: OnceLock<SquirrelImports> = OnceLock::new();

pub fn init_sq_imports(exports: PluginExports) -> Result<(), &'static str> {
    unsafe {
        if exports.exports_ptr.is_null() {
            return Err("Exports pointer is null");
        }

        // 转换指针类型: void** -> SquirrelImports**
        let sq_deref_funcs = exports.exports_ptr as *mut *mut raw::SquirrelImports;

        // 解引用获取SquirrelImports*
        let sq_funcs_ptr = *sq_deref_funcs;
        if sq_funcs_ptr.is_null() {
            return Err("Dereferenced exports pointer is null");
        }
        // 解引用获取实际的SquirrelImports结构体
        let raw_imports = &*sq_funcs_ptr;

        SQIURREL_IMPORTS
            .set(SquirrelImports {
                inner: *raw_imports,
            })
            .unwrap();
        Ok(())
    }
}

pub fn is_initialized_sq_imports() -> bool {
    SQIURREL_IMPORTS.get().is_some()
}

pub fn sq_imports() -> &'static SquirrelImports {
    SQIURREL_IMPORTS.get().unwrap()
}
