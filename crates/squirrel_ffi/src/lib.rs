#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod raw;

pub mod sq;
pub mod error;
use std::sync::OnceLock;

use tracing::{Level, event};
use vcmp_bindings::{func::PluginMethods, vcmp_func};

use crate::{error::{SQFFIResult, SQError}, raw::HSQUIRRELVM};


pub const SQUIRREL_LOAD_IDENTIFIER: u32 = 0x7D6E22D8;
pub static SQUIRREL_IMPORTS: OnceLock<sq::SquirrelImports> = OnceLock::new();

pub fn init_squirrel() -> SQFFIResult<()> {
    let id = match vcmp_func().find_plugin("SQHost2") {
        Some(id) => id,
        None => {
            return Err(SQError::FailedToAttachPlugin);
        }
    };

    let exports = vcmp_func().get_plugin_exports(id);
    let sq = sq::SquirrelImports::from(exports);
    SQUIRREL_IMPORTS.set(sq).unwrap();

    event!(Level::INFO, "Squirrel initialized");
    Ok(())
}

pub fn get_sq() -> &'static sq::SquirrelImports {
    SQUIRREL_IMPORTS.get().unwrap()
}

pub fn register_func(
    name: &str,
    func: extern "C" fn(vm: HSQUIRRELVM) -> i64,
    params: &str
) {
    let sq = get_sq();
    let vm = sq.get_vm();
    let sq_api = sq.get_api();

    sq_api.push_root_table(vm);
    sq_api.push_string(vm, name);
    sq_api.new_closure(vm, func, 0);

    sq_api.set_params_check(vm, 0, format!("t{params}").as_str());
    sq_api.set_native_closure_name(vm, -1, name);
    sq_api.new_slot(vm, -3, false);
    sq_api.pop(vm, 1);
}