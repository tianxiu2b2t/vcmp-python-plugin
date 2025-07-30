#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod raw;

use std::sync::OnceLock;
use crate::sq_ffi::raw::SquirrelImports;

pub const SQUIRREL_LOAD_IDENTIFIER: u32 = 0x7D6E22D8;
pub static SQIURREL_VM: OnceLock<SquirrelImports> = OnceLock::new();

pub fn init_sq_vm(func: SquirrelImports) -> &'static SquirrelImports {
    SQIURREL_VM.get_or_init(|| func)
}

pub fn is_initialized_vm() -> bool {
    SQIURREL_VM.get().is_some()
}

pub fn sq_vm() -> &'static SquirrelImports {
    SQIURREL_VM.get().unwrap()
}
