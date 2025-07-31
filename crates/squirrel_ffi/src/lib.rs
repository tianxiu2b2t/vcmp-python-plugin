pub mod sq_ffi;
pub mod sq;

use vcmp_bindings::{func::PluginMethods, vcmp_func};

use crate::sq::{init_sq_imports, sq_imports, SquirrelImports};

pub fn init_squirrel() {
//    void OnSquirrelScriptLoad() {
// 	// See if we have any imports from Squirrel
// 	size_t size;
// 	int32_t sqId      = VCMP->FindPlugin(const_cast<char*>("SQHost2"));
// 	const void ** sqExports = VCMP->GetPluginExports(sqId, &size);

// 	// We do!
// 	if (sqExports != NULL && size > 0) {
// 		// Cast to a SquirrelImports structure
// 		SquirrelImports ** sqDerefFuncs = (SquirrelImports **)sqExports;
		
// 		// Now let's change that to a SquirrelImports pointer
// 		SquirrelImports * sqFuncs       = (SquirrelImports *)(*sqDerefFuncs);
		
// 		// Now we get the virtual machine
// 		if (sqFuncs) {
// 			// Get a pointer to the VM and API
// 			v = *(sqFuncs->GetSquirrelVM());
// 			sq = *(sqFuncs->GetSquirrelAPI());

// 			// Register functions
// 			RegisterFuncs(v);
			
// 			// Register constants
// 			RegisterConsts(v);
// 		}
// 	}
// 	else
// 		OutputError("Failed to attach to SQHost2.");
// }
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