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

impl SquirrelVM {
    pub fn open(initial_stack_size: i64) -> HSQUIRRELVM {
        (sq_imports().get_api().open)(initial_stack_size)
    }

    pub fn new_thread(friend_vm: HSQUIRRELVM, initial_stack_size: i64) -> HSQUIRRELVM {
        (sq_imports().get_api().newthread)(friend_vm, initial_stack_size)
    }

    pub fn set_error_handler(vm: HSQUIRRELVM) {
        (sq_imports().get_api().seterrorhandler)(vm);
    }

    pub fn close(vm: HSQUIRRELVM) {
        (sq_imports().get_api().close)(vm);
    }

    pub fn set_foreign_ptr(vm: HSQUIRRELVM, p: *mut std::ffi::c_void) {
        (sq_imports().get_api().setforeignptr)(vm, p);
    }

    pub fn get_foreign_ptr(vm: HSQUIRRELVM) -> *mut std::ffi::c_void {
        (sq_imports().get_api().getforeignptr)(vm)
    }

    pub fn set_print_func(vm: HSQUIRRELVM, print_func: raw::SQPRINTFUNCTION, arg1: raw::SQPRINTFUNCTION) {
        (sq_imports().get_api().setprintfunc)(vm, print_func, arg1);
    }

    pub fn get_print_func(vm: HSQUIRRELVM) -> raw::SQPRINTFUNCTION {
        (sq_imports().get_api().getprintfunc)(vm)
    }

    pub fn suspend_vm(vm: HSQUIRRELVM) -> raw::SQRESULT {
        (sq_imports().get_api().suspendvm)(vm)
    }

    pub fn wakeup_vm(vm: HSQUIRRELVM, resumed_ret: raw::SQBool, retval: raw::SQBool, raise_error: raw::SQBool, throw_error: raw::SQBool) -> raw::SQRESULT {
        (sq_imports().get_api().wakeupvm)(vm, resumed_ret, retval, raise_error, throw_error)
    }

    pub fn get_vm_state(vm: HSQUIRRELVM) -> i64 {
        (sq_imports().get_api().getvmstate)(vm)
    }

    pub fn compile(vm: HSQUIRRELVM, read: raw::SQLEXREADFUNC, p: *mut std::ffi::c_void, source_name: *const u16, raise_error: raw::SQBool) -> raw::SQRESULT {
        (sq_imports().get_api().compile)(vm, read, p, source_name, raise_error)
    }

    pub fn compile_buffer(vm: HSQUIRRELVM, s: *const u16, size: i64, source_name: *const u16, raise_error: raw::SQBool) -> raw::SQRESULT {
        (sq_imports().get_api().compilebuffer)(vm, s, size, source_name, raise_error)
    }

    pub fn enable_debug_info(vm: HSQUIRRELVM, enable: raw::SQBool) {
        (sq_imports().get_api().enabledebuginfo)(vm, enable);
    }

    pub fn notify_all_exceptions(vm: HSQUIRRELVM, enable: raw::SQBool) {
        (sq_imports().get_api().notifyallexceptions)(vm, enable);
    }

    pub fn set_compiler_error_handler(vm: HSQUIRRELVM, f: raw::SQCOMPILERERROR) {
        (sq_imports().get_api().setcompilererrorhandler)(vm, f);
    }

    pub fn push(vm: HSQUIRRELVM, idx: i64) {
        (sq_imports().get_api().push)(vm, idx);
    }

    pub fn pop(vm: HSQUIRRELVM, nelemstopop: i64) {
        (sq_imports().get_api().pop)(vm, nelemstopop);
    }

    pub fn poptop(vm: HSQUIRRELVM) {
        (sq_imports().get_api().poptop)(vm);
    }

    pub fn remove(vm: HSQUIRRELVM, idx: i64) {
        (sq_imports().get_api().remove)(vm, idx);
    }

    pub fn gettop(vm: HSQUIRRELVM) -> i64 {
        (sq_imports().get_api().gettop)(vm)
    }

    pub fn settop(vm: HSQUIRRELVM, newtop: i64) {
        (sq_imports().get_api().settop)(vm, newtop);
    }

    pub fn reservestack(vm: HSQUIRRELVM, nsize: i64) -> raw::SQRESULT {
        (sq_imports().get_api().reservestack)(vm, nsize)
    }

    pub fn cmp(vm: HSQUIRRELVM) -> i64 {
        (sq_imports().get_api().cmp)(vm)
    }

    pub fn move_(dest: HSQUIRRELVM, src: HSQUIRRELVM, idx: i64) {
        (sq_imports().get_api().move_)(dest, src, idx);
    }

    pub fn newuserdata(vm: HSQUIRRELVM, size: u64) -> raw::SQUserPointer {
        (sq_imports().get_api().newuserdata)(vm, size)
    }

    pub fn newtable(vm: HSQUIRRELVM) {
        (sq_imports().get_api().newtable)(vm);
    }

    pub fn newarray(vm: HSQUIRRELVM, size: i64) {
        (sq_imports().get_api().newarray)(vm, size);
    }

    pub fn newclosure(vm: HSQUIRRELVM, func: raw::SQFUNCTION, nfreevars: u64) {
        (sq_imports().get_api().newclosure)(vm, func, nfreevars);
    }

    pub fn setparamscheck(vm: HSQUIRRELVM, nparamscheck: i64, typemask: *const u8) -> raw::SQRESULT {
        (sq_imports().get_api().setparamscheck)(vm, nparamscheck, typemask);
    }

    pub fn bindenv(vm: HSQUIRRELVM, idx: i64) -> raw::SQRESULT {
        (sq_imports().get_api().bindenv)(vm, idx);
    }

    pub fn pushstring(vm: HSQUIRRELVM, s: *const u8, len: i64) {
        (sq_imports().get_api().pushstring)(vm, s, len);
    }

    pub fn pushfloat(vm: HSQUIRRELVM, f: f64) {
        (sq_imports().get_api().pushfloat)(vm, f);
    }

    pub fn pushinteger(vm: HSQUIRRELVM, n: i64) {
        (sq_imports().get_api().pushinteger)(vm, n);
    }

    pub fn pushbool(vm: HSQUIRRELVM, b: raw::SQBool) {
        (sq_imports().get_api().pushbool)(vm, b);
    }

    pub fn pushuserpointer(vm: HSQUIRRELVM, p: raw::SQUserPointer) {
        (sq_imports().get_api().pushuserpointer)(vm, p);
    }

    pub fn pushnull(vm: HSQUIRRELVM) {
        (sq_imports().get_api().pushnull)(vm);
    }

    pub fn gettype(vm: HSQUIRRELVM, idx: i64) -> raw::SQObjectType {
        (sq_imports().get_api().gettype)(vm, idx)
    }

    pub fn getsize(vm: HSQUIRRELVM, idx: i64) -> i64 {
        (sq_imports().get_api().getsize)(vm, idx)
    }

    pub fn getbase(vm: HSQUIRRELVM, idx: i64) -> raw::HSQUIRRELVM {
        (sq_imports().get_api().getbase)(vm, idx)
    }

    pub fn instanceof(vm: HSQUIRRELVM) -> raw::SQBool {
        (sq_imports().get_api().instanceof)(vm)
    }

    pub fn tostring(vm: HSQUIRRELVM, idx: i64) -> raw::SQRESULT {
        (sq_imports().get_api().tostring)(vm, idx)
    }

    pub fn tobool(vm: HSQUIRRELVM, idx: i64, b: *mut raw::SQBool) {
        (sq_imports().get_api().tobool)(vm, idx, b);
    }

    pub fn getstring(vm: HSQUIRRELVM, idx: i64, c: *mut *const u8) -> raw::SQRESULT {
        (sq_imports().get_api().getstring)(vm, idx, c)
    }

    pub fn getinteger(vm: HSQUIRRELVM, idx: i64, i: *mut i64) -> raw::SQRESULT {
        (sq_imports().get_api().getinteger)(vm, idx, i)
    }

    pub fn getfloat(vm: HSQUIRRELVM, idx: i64, f: *mut f64) -> raw::SQRESULT {
        (sq_imports().get_api().getfloat)(vm, idx, f)
    }

    pub fn getbool(vm: HSQUIRRELVM, idx: i64, b: *mut raw::SQBool) -> raw::SQRESULT {
        (sq_imports().get_api().getbool)(vm, idx, b)
    }

    pub fn getthread(vm: HSQUIRRELVM, idx: i64, thread: *mut HSQUIRRELVM) -> raw::SQRESULT {
        (sq_imports().get_api().getthread)(vm, idx, thread)
    }

    pub fn getuserpointer(vm: HSQUIRRELVM, idx: i64, p: *mut raw::SQUserPointer) -> raw::SQRESULT {
        (sq_imports().get_api().getuserpointer)(vm, idx, p)
    }

    pub fn getuserdata(vm: HSQUIRRELVM, idx: i64, p: *mut raw::SQUserPointer, typetag: *mut raw::SQUserPointer) -> raw::SQRESULT {
        (sq_imports().get_api().getuserdata)(vm, idx, p, typetag)
    }

    pub fn settypetag(vm: HSQUIRRELVM, idx: i64, typetag: raw::SQUserPointer) -> raw::SQRESULT {
        (sq_imports().get_api().settypetag)(vm, idx, typetag)
    }

    pub fn gettypetag(vm: HSQUIRRELVM, idx: i64, typetag: *mut raw::SQUserPointer) -> raw::SQRESULT {
        (sq_imports().get_api().gettypetag)(vm, idx, typetag)
    }

    pub fn setreleasehook(vm: HSQUIRRELVM, idx: i64, hook: raw::SQRELEASEHOOK) -> raw::SQRESULT {
        (sq_imports().get_api().setreleasehook)(vm, idx, hook)
    }

    pub fn getscratchpad(vm: HSQUIRRELVM, minsize: i64) -> *mut u8 {
        (sq_imports().get_api().getscratchpad)(vm, minsize)
    }

    pub fn getclosureinfo(vm: HSQUIRRELVM, idx: i64, nparams: *mut u64, nfreevars: *mut u64) -> raw::SQRESULT {
        (sq_imports().get_api().getclosureinfo)(vm, idx, nparams, nfreevars)
    }

    pub fn setnativeclosurename(vm: HSQUIRRELVM, idx: i64, name: *const u8) -> raw::SQRESULT {
        (sq_imports().get_api().setnativeclosurename)(vm, idx, name)
    }

    pub fn setinstanceup(vm: HSQUIRRELVM, idx: i64, p: raw::SQUserPointer) -> raw::SQRESULT {
        (sq_imports().get_api().setinstanceup)(vm, idx, p)
    }
    /* feat squirrel vm stack operator */

    /*
    pub instanceof: extern "C" fn(v: HSQUIRRELVM) -> SQBool,
    pub tostring: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub tobool: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool),
    pub getstring: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, c: *mut *const SQChar) -> SQRESULT,
    pub getinteger: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, i: *mut SQInteger) -> SQRESULT,
    pub getfloat: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, f: *mut SQFloat) -> SQRESULT,
    pub getbool: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool) -> SQRESULT,
    pub getthread:
        extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, thread: *mut HSQUIRRELVM) -> SQRESULT,
    pub getuserpointer:
        extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer) -> SQRESULT,
    pub getuserdata: extern "C" fn(
        v: HSQUIRRELVM,
        idx: SQInteger,
        p: *mut SQUserPointer,
        typetag: *mut SQUserPointer,
    ) -> SQRESULT,
    pub settypetag:
        extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, typetag: SQUserPointer) -> SQRESULT,
    pub gettypetag:
        extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, typetag: *mut SQUserPointer) -> SQRESULT,
    pub setreleasehook: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, hook: SQRELEASEHOOK),
    pub getscratchpad: extern "C" fn(v: HSQUIRRELVM, minsize: SQInteger) -> *mut SQChar,
    pub getclosureinfo: extern "C" fn(
        v: HSQUIRRELVM,
        idx: SQInteger,
        nparams: *mut SQUnsignedInteger,
        nfreevars: *mut SQUnsignedInteger,
    ) -> SQRESULT,
    pub setnativeclosurename:
        extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, name: *const SQChar) -> SQRESULT,
    pub setinstanceup: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, p: SQUserPointer) -> SQRESULT,
    pub getinstanceup: extern "C" fn(
        v: HSQUIRRELVM,
        idx: SQInteger,
        p: *mut SQUserPointer,
        typetag: SQUserPointer,
    ) -> SQRESULT,
    pub setclassudsize:
        extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, udsize: SQInteger) -> SQRESULT,
    pub newclass: extern "C" fn(v: HSQUIRRELVM, hasbase: SQBool) -> SQRESULT,
    pub createinstance: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub setattributes: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub getattributes: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub getclass: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub weakref: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger),
    pub getdefaultdelegate: extern "C" fn(v: HSQUIRRELVM, t: SQObjectType) -> SQRESULT,
    pub pushroottable: extern "C" fn(v: HSQUIRRELVM),
    pub pushregistrytable: extern "C" fn(v: HSQUIRRELVM),
    pub pushconsttable: extern "C" fn(v: HSQUIRRELVM),
    pub setroottable: extern "C" fn(v: HSQUIRRELVM) -> SQRESULT,
    pub setconsttable: extern "C" fn(v: HSQUIRRELVM) -> SQRESULT,
    pub newslot: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT,
    pub deleteslot: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT,
    pub set: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub get: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub rawget: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub rawset: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub rawdeleteslot: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT,
    pub arrayappend: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub arraypop: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT,
    pub arrayresize: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, newsize: SQInteger) -> SQRESULT,
    pub arrayreverse: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub arrayremove: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, itemidx: SQInteger) -> SQRESULT,
    pub arrayinsert: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, destpos: SQInteger) -> SQRESULT,
    pub setdelegate: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub getdelegate: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub clone: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub setfreevariable:
        extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, nval: SQUnsignedInteger) -> SQRESULT,
    pub next: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub getweakrefval: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub clear: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT,
    pub call: extern "C" fn(
        v: HSQUIRRELVM,
        params: SQInteger,
        retval: SQBool,
        raiseerror: SQBool,
    ) -> SQRESULT,
    pub resume: extern "C" fn(v: HSQUIRRELVM, retval: SQBool, raiseerror: SQBool) -> SQRESULT,
    pub getlocal: extern "C" fn(
        v: HSQUIRRELVM,
        level: SQUnsignedInteger,
        idx: SQUnsignedInteger,
    ) -> *const SQChar,
    pub getfreevariable:
        extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, nval: SQUnsignedInteger) -> *const SQChar,
    pub throwerror: extern "C" fn(v: HSQUIRRELVM, err: *const SQChar) -> SQRESULT,
    pub reseterror: extern "C" fn(v: HSQUIRRELVM),
    pub getlasterror: extern "C" fn(v: HSQUIRRELVM),
    pub getstackobj: extern "C" fn(v: HSQUIRRELVM, idx: SQInteger, po: *mut HSQOBJECT) -> SQRESULT,
    pub pushobject: extern "C" fn(v: HSQUIRRELVM, obj: HSQOBJECT),
    pub addref: extern "C" fn(v: HSQUIRRELVM, po: *mut HSQOBJECT),
    pub release: extern "C" fn(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> SQBool,
    pub resetobject: extern "C" fn(po: *mut HSQOBJECT),
    pub objtostring: extern "C" fn(o: *const HSQOBJECT) -> *const SQChar,
    pub objtobool: extern "C" fn(o: *const HSQOBJECT) -> SQBool,
    pub objtointeger: extern "C" fn(o: *const HSQOBJECT) -> SQInteger,
    pub objtofloat: extern "C" fn(o: *const HSQOBJECT) -> SQFloat,
    pub getobjtypetag: extern "C" fn(o: *const HSQOBJECT, typetag: *mut SQUserPointer) -> SQRESULT,
    pub collectgarbage: extern "C" fn(v: HSQUIRRELVM) -> SQInteger,
    pub writeclosure:
        extern "C" fn(vm: HSQUIRRELVM, writef: SQWRITEFUNC, up: SQUserPointer) -> SQRESULT,
    pub readclosure:
        extern "C" fn(vm: HSQUIRRELVM, readf: SQREADFUNC, up: SQUserPointer) -> SQRESULT,
    pub malloc: extern "C" fn(size: SQUnsignedInteger) -> *mut ::std::os::raw::c_void,
    pub realloc: extern "C" fn(
        p: *mut ::std::os::raw::c_void,
        oldsize: SQUnsignedInteger,
        newsize: SQUnsignedInteger,
    ) -> *mut ::std::os::raw::c_void,
    pub free: extern "C" fn(p: *mut ::std::os::raw::c_void, size: SQUnsignedInteger),
    pub stackinfos:
        extern "C" fn(v: HSQUIRRELVM, level: SQInteger, si: *mut SQStackInfos) -> SQRESULT,
    pub setdebughook: extern "C" fn(v: HSQUIRRELVM),
}
    
     */
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
