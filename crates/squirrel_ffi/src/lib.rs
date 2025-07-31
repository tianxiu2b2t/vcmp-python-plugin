pub mod sq;
pub mod sq_ffi;

use vcmp_bindings::{func::PluginMethods, vcmp_func};

use crate::sq::{init_sq_imports, sq_imports};

pub fn init_squirrel() {
    for i in 0..vcmp_func().get_plugin_count() {
        println!("Plugin: {}", vcmp_func().get_plugin_info(i as i32).unwrap());
    }
    let id = match vcmp_func().find_plugin("SQHost2\0") {
        Some(id) => id,
        None => return,
    };
    let res = init_sq_imports(vcmp_func().get_plugin_exports(id));
    if res.is_err() {
        println!("Failed to init squirrel imports: {:?}", res);
        return;
    }

    let api = sq_imports().get_api();
    let vm = sq_imports().get_vm();
    println!("VM: {:?}", vm);
    println!("API: {:?}", api);
}
