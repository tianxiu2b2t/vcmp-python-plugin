#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod raw;

pub mod sq;

use std::sync::OnceLock;

use tracing::{Level, event};
use vcmp_bindings::{func::PluginMethods, vcmp_func};

use crate::raw::HSQUIRRELVM;

pub const SQUIRREL_LOAD_IDENTIFIER: u32 = 0x7D6E22D8;

pub static SQUIRREL_IMPORTS: OnceLock<sq::SquirrelImports> = OnceLock::new();

pub fn init_squirrel() {
    for ele in vcmp_func().get_plugins() {
        println!("Plugin: {}", ele);
    }
    let id = match vcmp_func().find_plugin("SQHost2") {
        Some(id) => id,
        None => {
            event!(Level::ERROR, "Failed to find SQHost plugin");
            return;
        }
    };

    let exports = vcmp_func().get_plugin_exports(id);
    let sq = sq::SquirrelImports::from(exports);
    SQUIRREL_IMPORTS.set(sq).unwrap();

    event!(Level::INFO, "Squirrel initialized");
    
    // register func
    register_func("helloworld", helloworld, "")

}

pub fn get_sq() -> &'static sq::SquirrelImports {
    SQUIRREL_IMPORTS.get().unwrap()
}

pub extern "C" fn helloworld(_vm: HSQUIRRELVM) -> i64 {
    println!("This is calling from rust!");
    1
}

pub fn register_func(
    name: &str,
    // callable
    func: extern "C" fn(vm: HSQUIRRELVM) -> i64,
    // params
    params: &str
) {
    let sq = get_sq();
    let vm = sq.get_vm();
    let sq_api = sq.get_api();
    println!("Registering function: {}", name);
    sq_api.push_root_table(vm);
    sq_api.push_string(vm, name);
    sq_api.new_closure(vm, func, 0);

    sq_api.set_params_check(vm, 0, format!("t{params}").as_str());
    sq_api.set_native_closure_name(vm, -1, name);
    sq_api.new_slot(vm, -3, false);
    sq_api.pop(vm, 1);
}