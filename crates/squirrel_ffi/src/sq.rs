
use std::sync::OnceLock;
use vcmp_bindings::func::plugin::PluginExports;

use crate::sq_ffi::raw::{self, sq_api, HSQUIRRELVM, SQVM};

#[derive(Debug, Clone)]
pub struct SquirrelImports {
    pub inner: raw::SquirrelImports
}

/*

let exports = vcmp_func().get_plugin_exports(id);

    let ptr = exports.exports_ptr;
    println!("ptr: {:?}", ptr);

    // SquirrelImports ** sqDerefFuncs = (SquirrelImports **)sqExports;
    // SquirrelImports * sqFuncs       = (SquirrelImports *)(*sqDerefFuncs);
    // SquirrelImports sq_funcs
    let sq_imports = unsafe { &mut *(ptr as *mut SquirrelImports) };
    println!("sq_imports: {:?}", sq_imports);
    let sq_api_ptr = sq_imports.GetSquirrelAPI;
    let sq_vm_ptr = sq_imports.GetSquirrelVM;
    println!("sq_api_ptr: {:?}", sq_api_ptr);
    println!("sq_vm_ptr: {:?}", sq_vm_ptr);

    let sq_api = unsafe { &mut *(sq_api_ptr as *mut sq_ffi::raw::HSQAPI) };
    let sq_vm = unsafe { &mut *(sq_vm_ptr as *mut sq_ffi::raw::SQVM) };

    println!("sq_api: {:?}", sq_api);
    println!("sq_vm: {:?}", sq_vm);
*/

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

pub struct SquirrelVM {
    pub inner: HSQUIRRELVM,
    pub api: &'static sq_api
}
impl SquirrelVM {
    pub fn new() -> Self {
        Self {
            inner: sq_imports().get_vm().unwrap(),
            api: sq_imports().get_api()
        }
    }

    /* feat squirrel vm stack operator */
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

        SQIURREL_IMPORTS.set(SquirrelImports { inner: *raw_imports }).unwrap();
        Ok(())
    }
}

pub fn is_initialized_sq_imports() -> bool {
    SQIURREL_IMPORTS.get().is_some()
}

pub fn sq_imports() -> &'static SquirrelImports {
    SQIURREL_IMPORTS.get().unwrap()
}