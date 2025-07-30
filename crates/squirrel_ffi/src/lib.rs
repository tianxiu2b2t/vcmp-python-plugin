pub mod sq_ffi;

use vcmp_bindings::{func::PluginMethods, vcmp_func};

use crate::sq_ffi::raw::SquirrelImports;

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
        println!("Plugin: {:?}", vcmp_func().get_plugin_info(i as i32));
    }
    let id = match vcmp_func().find_plugin("SQHost2") {
        Some(id) => id,
        None => return,
    };
    let exports = vcmp_func().get_plugin_exports(id);

    let ptr = exports.exports_ptr;

    let sq_funcs = unsafe { &mut *(ptr as *mut SquirrelImports) };
    let sq_funcs =  &*sq_funcs;
    let v = unsafe { &mut *(sq_funcs.GetSquirrelVM)() };
    let sq = unsafe { &mut *(sq_funcs.GetSquirrelAPI)() };

    println!("v: {:?}", v);
    println!("sq: {:?}", sq);


    vcmp_func();
}