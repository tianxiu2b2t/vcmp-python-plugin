use vcmp_bindings::func::plugin::PluginExports;

use crate::raw::{
    self, SQBool, SQChar, SQFloat, SQInteger, SQObjectType, SQStackInfos, SQUnsignedInteger, SQUserPointer, HSQOBJECT, HSQUIRRELVM, SQCOMPILERERROR, SQFUNCTION, SQLEXREADFUNC, SQPRINTFUNCTION, SQREADFUNC, SQRELEASEHOOK, SQRESULT, SQWRITEFUNC
};

use std::ffi::{CStr, CString};
use std::os::raw::c_void;

#[derive(Debug, Clone)]
pub struct SquirrelImports {
    pub inner: raw::SquirrelImports
}

impl SquirrelImports {
    pub fn get_vm(&self) -> HSQUIRRELVM {
        unsafe { *(self.inner.GetSquirrelVM)() }
    }
    pub fn get_api(&self) -> SQAPI {
        let inner_api = unsafe { *(self.inner.GetSquirrelAPI)() };
        SQAPI::new(inner_api)
    }
}

impl From<PluginExports> for SquirrelImports {
    fn from(exports: PluginExports) -> Self {
        let ptr = exports.exports_ptr as *mut *const raw::SquirrelImports;
        let imports = unsafe { **ptr };
        Self {
            inner: imports
        }
    }
}

#[derive(Debug, Clone)]
pub struct SQAPI {
    inner: raw::sq_api,
}

impl SQAPI {
    /// 从原始HSQAPI指针创建SQAPI实例
    pub fn new(inner: raw::HSQAPI) -> Self {
        Self {
            inner: unsafe { *inner },
        }
    }

    // =========================================================================
    // 1. VM管理
    // =========================================================================

    /// 创建新的Squirrel虚拟机
    pub fn open(&self, initial_stack_size: SQInteger) -> HSQUIRRELVM {
        (self.inner.open)(initial_stack_size)
    }

    /// 从现有VM创建新线程
    pub fn new_thread(&self, friend_vm: HSQUIRRELVM, initial_stack_size: SQInteger) -> HSQUIRRELVM {
        (self.inner.newthread)(friend_vm, initial_stack_size)
    }

    /// 设置VM的错误处理器
    pub fn set_error_handler(&self, vm: HSQUIRRELVM) {
        (self.inner.seterrorhandler)(vm)
    }

    /// 关闭虚拟机并释放资源
    pub fn close(&self, vm: HSQUIRRELVM) {
        (self.inner.close)(vm)
    }

    /// 为VM设置外部指针（用户数据）
    pub fn set_foreign_ptr(&self, vm: HSQUIRRELVM, ptr: SQUserPointer) {
        (self.inner.setforeignptr)(vm, ptr)
    }

    /// 获取VM的外部指针（用户数据）
    pub fn get_foreign_ptr(&self, vm: HSQUIRRELVM) -> SQUserPointer {
        (self.inner.getforeignptr)(vm)
    }

    /// 设置VM的打印函数（标准输出和错误输出）
    pub fn set_print_func(
        &self,
        vm: HSQUIRRELVM,
        print_func: SQPRINTFUNCTION,
        error_func: SQPRINTFUNCTION,
    ) {
        (self.inner.setprintfunc)(vm, print_func, error_func)
    }

    /// 获取VM的标准打印函数
    pub fn get_print_func(&self, vm: HSQUIRRELVM) -> SQPRINTFUNCTION {
        (self.inner.getprintfunc)(vm)
    }

    /// 暂停虚拟机执行
    pub fn suspend_vm(&self, vm: HSQUIRRELVM) -> SQRESULT {
        (self.inner.suspendvm)(vm)
    }

    /// 唤醒暂停的虚拟机
    pub fn wakeup_vm(
        &self,
        vm: HSQUIRRELVM,
        resume_ret: SQBool,
        ret_val: SQBool,
        raise_error: SQBool,
        throw_error: SQBool,
    ) -> SQRESULT {
        (self.inner.wakeupvm)(vm, resume_ret, ret_val, raise_error, throw_error)
    }

    /// 获取虚拟机当前状态（运行/暂停/空闲）
    pub fn get_vm_state(&self, vm: HSQUIRRELVM) -> SQInteger {
        (self.inner.getvmstate)(vm)
    }

    // =========================================================================
    // 2. 编译相关
    // =========================================================================

    /// 从自定义读取函数编译Squirrel代码
    pub fn compile(
        &self,
        vm: HSQUIRRELVM,
        read_func: SQLEXREADFUNC,
        user_ptr: SQUserPointer,
        source_name: &str,
        raise_error: SQBool,
    ) -> SQRESULT {
        let c_source_name = CString::new(source_name).expect("Invalid source name");
        (self.inner.compile)(vm, read_func, user_ptr, c_source_name.as_ptr(), raise_error)
    }

    /// 从内存缓冲区编译Squirrel代码
    pub fn compile_buffer(
        &self,
        vm: HSQUIRRELVM,
        buffer: &str,
        source_name: &str,
        raise_error: SQBool,
    ) -> SQRESULT {
        let c_buffer = CString::new(buffer).expect("Invalid buffer");
        let c_source_name = CString::new(source_name).expect("Invalid source name");
            (self.inner.compilebuffer)(
                vm,
                c_buffer.as_ptr(),
                buffer.len() as SQInteger,
                c_source_name.as_ptr(),
                raise_error,
            )
    }

    /// 启用/禁用调试信息生成
    pub fn enable_debug_info(&self, vm: HSQUIRRELVM, enable: SQBool) {
        (self.inner.enabledebuginfo)(vm, enable)
    }

    /// 启用/禁用所有异常通知
    pub fn notify_all_exceptions(&self, vm: HSQUIRRELVM, enable: SQBool) {
        (self.inner.notifyallexceptions)(vm, enable)
    }

    /// 设置编译器错误处理器
    pub fn set_compiler_error_handler(&self, vm: HSQUIRRELVM, handler: SQCOMPILERERROR) {
        (self.inner.setcompilererrorhandler)(vm, handler)
    }

    // =========================================================================
    // 3. 栈操作
    // =========================================================================

    /// 将指定索引的栈元素复制到栈顶
    pub fn push(&self, vm: HSQUIRRELVM, index: SQInteger) {
        (self.inner.push)(vm, index)
    }

    /// 从栈顶弹出指定数量的元素
    pub fn pop(&self, vm: HSQUIRRELVM, num_elements: SQInteger) {
        (self.inner.pop)(vm, num_elements)
    }

    /// 弹出栈顶元素
    pub fn pop_top(&self, vm: HSQUIRRELVM) {
        (self.inner.poptop)(vm)
    }

    /// 移除指定索引的栈元素
    pub fn remove(&self, vm: HSQUIRRELVM, index: SQInteger) {
        (self.inner.remove)(vm, index)
    }

    /// 获取当前栈顶索引
    pub fn get_top(&self, vm: HSQUIRRELVM) -> SQInteger {
        (self.inner.gettop)(vm)
    }

    /// 设置栈顶位置（截断或扩展栈）
    pub fn set_top(&self, vm: HSQUIRRELVM, new_top: SQInteger) {
        (self.inner.settop)(vm, new_top)
    }

    /// 预分配栈空间
    pub fn reserve_stack(&self, vm: HSQUIRRELVM, size: SQInteger) -> SQRESULT {
        (self.inner.reservestack)(vm, size)
    }

    /// 比较栈顶两个元素（结果：-1/0/1）
    pub fn cmp(&self, vm: HSQUIRRELVM) -> SQInteger {
        (self.inner.cmp)(vm)
    }

    /// 将源VM的栈元素移动到目标VM
    pub fn move_vm(&self, dest: HSQUIRRELVM, src: HSQUIRRELVM, index: SQInteger) {
        (self.inner.move_)(dest, src, index)
    }

    // =========================================================================
    // 4. 数据对象创建
    // =========================================================================

    /// 创建新用户数据（返回数据指针）
    pub fn new_user_data(&self, vm: HSQUIRRELVM, size: SQUnsignedInteger) -> SQUserPointer {
        (self.inner.newuserdata)(vm, size)
    }

    /// 创建新表并推入栈顶
    pub fn new_table(&self, vm: HSQUIRRELVM) {
        (self.inner.newtable)(vm)
    }

    /// 创建新数组并推入栈顶
    pub fn new_array(&self, vm: HSQUIRRELVM, size: SQInteger) {
        (self.inner.newarray)(vm, size)
    }

    /// 创建新闭包并推入栈顶
    pub fn new_closure(&self, vm: HSQUIRRELVM, func: SQFUNCTION, free_vars: SQUnsignedInteger) {
        (self.inner.newclosure)(vm, func, free_vars)
    }

    /// 检查函数参数类型匹配
    pub fn set_params_check(
        &self,
        vm: HSQUIRRELVM,
        check_count: SQInteger,
        type_mask: &str,
    ) -> SQRESULT {
        let c_mask = CString::new(type_mask).expect("Invalid type mask");
        (self.inner.setparamscheck)(vm, check_count, c_mask.as_ptr())
    }

    /// 将栈元素绑定为闭包的环境
    pub fn bind_env(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.bindenv)(vm, index)
    }

    // =========================================================================
    // 5. 基本类型推入栈
    // =========================================================================

    /// 推入字符串到栈顶（自动处理长度）
    pub fn push_string(&self, vm: HSQUIRRELVM, s: &str) {
        let string = format!("{s}\0");
        let ptr = string.as_ptr();
        (self.inner.pushstring)(vm, ptr as *const i8, -1)
    }

    /// 推入浮点数到栈顶
    pub fn push_float(&self, vm: HSQUIRRELVM, f: SQFloat) {
        (self.inner.pushfloat)(vm, f)
    }

    /// 推入整数到栈顶
    pub fn push_integer(&self, vm: HSQUIRRELVM, n: SQInteger) {
        (self.inner.pushinteger)(vm, n)
    }

    /// 推入布尔值到栈顶
    pub fn push_bool(&self, vm: HSQUIRRELVM, b: SQBool) {
        (self.inner.pushbool)(vm, b)
    }

    /// 推入用户指针到栈顶
    pub fn push_user_pointer(&self, vm: HSQUIRRELVM, ptr: SQUserPointer) {
        (self.inner.pushuserpointer)(vm, ptr)
    }

    /// 推入null到栈顶
    pub fn push_null(&self, vm: HSQUIRRELVM) {
        (self.inner.pushnull)(vm)
    }

    // =========================================================================
    // 6. 类型检查与转换
    // =========================================================================

    /// 获取指定索引元素的类型
    pub fn get_type(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQObjectType {
        (self.inner.gettype)(vm, index)
    }

    /// 获取指定索引元素的大小（数组/表长度等）
    pub fn get_size(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQInteger {
        (self.inner.getsize)(vm, index)
    }

    /// 获取对象的基类并推入栈顶
    pub fn get_base(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.getbase)(vm, index)
    }

    /// 检查栈顶两个元素的实例关系（a instanceof b）
    pub fn instance_of(&self, vm: HSQUIRRELVM) -> SQBool {
        (self.inner.instanceof)(vm)
    }

    /// 将指定元素转换为字符串并替换它
    pub fn to_string(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.tostring)(vm, index)
    }

    /// 将指定元素转换为布尔值（输出参数）
    pub fn to_bool(&self, vm: HSQUIRRELVM, index: SQInteger, out: &mut SQBool) {
        (self.inner.tobool)(vm, index, out as *mut _)
    }

    /// 获取指定元素的字符串值（输出参数：字符串指针）
    pub fn get_string(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        out: &mut *const SQChar,
    ) -> SQRESULT {
        (self.inner.getstring)(vm, index, out as *mut _)
    }

    /// 获取指定元素的整数值（输出参数）
    pub fn get_integer(&self, vm: HSQUIRRELVM, index: SQInteger, out: &mut SQInteger) -> SQRESULT {
        (self.inner.getinteger)(vm, index, out as *mut _)
    }

    /// 获取指定元素的浮点值（输出参数）
    pub fn get_float(&self, vm: HSQUIRRELVM, index: SQInteger, out: &mut SQFloat) -> SQRESULT {
        (self.inner.getfloat)(vm, index, out as *mut _)
    }

    /// 获取指定元素的布尔值（输出参数）
    pub fn get_bool(&self, vm: HSQUIRRELVM, index: SQInteger, out: &mut SQBool) -> SQRESULT {
        (self.inner.getbool)(vm, index, out as *mut _)
    }

    /// 获取指定元素的线程句柄（输出参数）
    pub fn get_thread(&self, vm: HSQUIRRELVM, index: SQInteger, out: &mut HSQUIRRELVM) -> SQRESULT {
        (self.inner.getthread)(vm, index, out as *mut _)
    }

    /// 获取指定元素的用户指针（输出参数）
    pub fn get_user_pointer(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        out: &mut SQUserPointer,
    ) -> SQRESULT {
        (self.inner.getuserpointer)(vm, index, out as *mut _)
    }

    /// 获取指定用户数据的指针和类型标签（输出参数）
    pub fn get_user_data(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        data_ptr: &mut SQUserPointer,
        type_tag: &mut SQUserPointer,
    ) -> SQRESULT {
        (self.inner.getuserdata)(vm, index, data_ptr as *mut _, type_tag as *mut _)
    }

    /// 为指定元素设置类型标签
    pub fn set_type_tag(&self, vm: HSQUIRRELVM, index: SQInteger, tag: SQUserPointer) -> SQRESULT {
        (self.inner.settypetag)(vm, index, tag)
    }

    /// 获取指定元素的类型标签（输出参数）
    pub fn get_type_tag(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        out: &mut SQUserPointer,
    ) -> SQRESULT {
        (self.inner.gettypetag)(vm, index, out as *mut _)
    }

    /// 为指定元素设置释放钩子（销毁时调用）
    pub fn set_release_hook(&self, vm: HSQUIRRELVM, index: SQInteger, hook: SQRELEASEHOOK) {
        (self.inner.setreleasehook)(vm, index, hook)
    }

    /// 获取临时缓冲区（用于字符串操作等）
    pub fn get_scratchpad(&self, vm: HSQUIRRELVM, min_size: SQInteger) -> *mut SQChar {
        (self.inner.getscratchpad)(vm, min_size)
    }

    // =========================================================================
    // 7. 闭包与函数信息
    // =========================================================================

    /// 获取闭包的参数数量和自由变量数量（输出参数）
    pub fn get_closure_info(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        out_params: &mut SQUnsignedInteger,
        out_free_vars: &mut SQUnsignedInteger,
    ) -> SQRESULT {
        (self.inner.getclosureinfo)(vm, index, out_params as *mut _, out_free_vars as *mut _)
    }

    /// 为原生闭包设置名称
    pub fn set_native_closure_name(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        name: &str,
    ) -> SQRESULT {
        let c_name = CString::new(name).expect("Invalid name");
        (self.inner.setnativeclosurename)(vm, index, c_name.as_ptr())
    }

    // =========================================================================
    // 8. 类与实例
    // =========================================================================

    /// 为实例设置用户指针
    pub fn set_instance_up(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        ptr: SQUserPointer,
    ) -> SQRESULT {
        (self.inner.setinstanceup)(vm, index, ptr)
    }

    /// 获取实例的用户指针（输出参数，带类型检查）
    pub fn get_instance_up(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        out: &mut SQUserPointer,
        type_tag: SQUserPointer,
    ) -> SQRESULT {
        (self.inner.getinstanceup)(vm, index, out as *mut _, type_tag)
    }

    /// 设置类的用户数据大小
    pub fn set_class_ud_size(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        size: SQInteger,
    ) -> SQRESULT {
        (self.inner.setclassudsize)(vm, index, size)
    }

    /// 创建新类并推入栈顶（可指定基类）
    pub fn new_class(&self, vm: HSQUIRRELVM, has_base: SQBool) -> SQRESULT {
        (self.inner.newclass)(vm, has_base)
    }

    /// 从类创建实例并推入栈顶
    pub fn create_instance(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.createinstance)(vm, index)
    }

    /// 设置指定元素的属性（从栈顶获取）
    pub fn set_attributes(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.setattributes)(vm, index)
    }

    /// 获取指定元素的属性并推入栈顶
    pub fn get_attributes(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.getattributes)(vm, index)
    }

    /// 获取实例的类并推入栈顶
    pub fn get_class(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.getclass)(vm, index)
    }

    /// 创建指定元素的弱引用并推入栈顶
    pub fn weak_ref(&self, vm: HSQUIRRELVM, index: SQInteger) {
        (self.inner.weakref)(vm, index)
    }

    /// 获取指定类型的默认委托并推入栈顶
    pub fn get_default_delegate(&self, vm: HSQUIRRELVM, type_: SQObjectType) -> SQRESULT {
        (self.inner.getdefaultdelegate)(vm, type_)
    }

    // =========================================================================
    // 9. 表操作
    // =========================================================================

    /// 推入根表到栈顶
    pub fn push_root_table(&self, vm: HSQUIRRELVM) {
        (self.inner.pushroottable)(vm)
    }

    /// 推入注册表到栈顶
    pub fn push_registry_table(&self, vm: HSQUIRRELVM) {
        (self.inner.pushregistrytable)(vm)
    }

    /// 推入常量表到栈顶
    pub fn push_const_table(&self, vm: HSQUIRRELVM) {
        (self.inner.pushconsttable)(vm)
    }

    /// 将栈顶元素设为根表
    pub fn set_root_table(&self, vm: HSQUIRRELVM) -> SQRESULT {
        (self.inner.setroottable)(vm)
    }

    /// 将栈顶元素设为常量表
    pub fn set_const_table(&self, vm: HSQUIRRELVM) -> SQRESULT {
        (self.inner.setconsttable)(vm)
    }

    /// 为表创建新槽位（从栈顶获取键值）
    pub fn new_slot(&self, vm: HSQUIRRELVM, index: SQInteger, is_static: bool) -> SQRESULT {
        (self.inner.newslot)(vm, index, is_static as u64)
    }

    /// 删除表的槽位
    pub fn delete_slot(&self, vm: HSQUIRRELVM, index: SQInteger, push_val: SQBool) -> SQRESULT {
        (self.inner.deleteslot)(vm, index, push_val)
    }

    /// 设置表的元素（从栈顶获取值）
    pub fn set(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.set)(vm, index)
    }

    /// 获取表的元素并推入栈顶
    pub fn get(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.get)(vm, index)
    }

    /// 直接获取表的元素（不触发委托）
    pub fn raw_get(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.rawget)(vm, index)
    }

    /// 直接设置表的元素（不触发委托）
    pub fn raw_set(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.rawset)(vm, index)
    }

    /// 直接删除表的槽位（不触发委托）
    pub fn raw_delete_slot(&self, vm: HSQUIRRELVM, index: SQInteger, push_val: SQBool) -> SQRESULT {
        (self.inner.rawdeleteslot)(vm, index, push_val)
    }

    // =========================================================================
    // 10. 数组操作
    // =========================================================================

    /// 向数组追加元素（从栈顶获取）
    pub fn array_append(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.arrayappend)(vm, index)
    }

    /// 从数组弹出最后一个元素
    pub fn array_pop(&self, vm: HSQUIRRELVM, index: SQInteger, push_val: SQBool) -> SQRESULT {
        (self.inner.arraypop)(vm, index, push_val)
    }

    /// 调整数组大小
    pub fn array_resize(&self, vm: HSQUIRRELVM, index: SQInteger, new_size: SQInteger) -> SQRESULT {
        (self.inner.arrayresize)(vm, index, new_size)
    }

    /// 反转数组
    pub fn array_reverse(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.arrayreverse)(vm, index)
    }

    /// 从数组移除指定索引的元素
    pub fn array_remove(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        item_index: SQInteger,
    ) -> SQRESULT {
        (self.inner.arrayremove)(vm, index, item_index)
    }

    /// 向数组插入元素（从栈顶获取）
    pub fn array_insert(&self, vm: HSQUIRRELVM, index: SQInteger, dest_pos: SQInteger) -> SQRESULT {
        (self.inner.arrayinsert)(vm, index, dest_pos)
    }

    // =========================================================================
    // 11. 委托与克隆
    // =========================================================================

    /// 为对象设置委托（从栈顶获取）
    pub fn set_delegate(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.setdelegate)(vm, index)
    }

    /// 获取对象的委托并推入栈顶
    pub fn get_delegate(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.getdelegate)(vm, index)
    }

    /// 克隆对象并推入栈顶
    pub fn clone(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.clone)(vm, index)
    }

    // =========================================================================
    // 12. 变量与迭代
    // =========================================================================

    /// 设置闭包的自由变量
    pub fn set_free_variable(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        var_index: SQUnsignedInteger,
    ) -> SQRESULT {
        (self.inner.setfreevariable)(vm, index, var_index)
    }

    /// 迭代表的下一个键值对（用于for循环）
    pub fn next(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.next)(vm, index)
    }

    /// 获取弱引用指向的值并推入栈顶
    pub fn get_weak_ref_val(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.getweakrefval)(vm, index)
    }

    /// 清空表/数组等容器
    pub fn clear(&self, vm: HSQUIRRELVM, index: SQInteger) -> SQRESULT {
        (self.inner.clear)(vm, index)
    }

    // =========================================================================
    // 13. 函数调用与恢复
    // =========================================================================

    /// 调用栈顶的函数
    pub fn call(
        &self,
        vm: HSQUIRRELVM,
        params: SQInteger,
        has_ret_val: SQBool,
        raise_error: SQBool,
    ) -> SQRESULT {
        (self.inner.call)(vm, params, has_ret_val, raise_error)
    }

    /// 恢复暂停的协程
    pub fn resume(&self, vm: HSQUIRRELVM, has_ret_val: SQBool, raise_error: SQBool) -> SQRESULT {
        (self.inner.resume)(vm, has_ret_val, raise_error)
    }

    // =========================================================================
    // 14. 本地变量与自由变量
    // =========================================================================

    /// 获取指定层级的本地变量名称
    pub fn get_local(
        &self,
        vm: HSQUIRRELVM,
        level: SQUnsignedInteger,
        index: SQUnsignedInteger,
    ) -> &CStr {
        unsafe {
            CStr::from_ptr((self.inner.getlocal)(vm, level, index))
        }
    }

    /// 获取闭包的自由变量名称
    pub fn get_free_variable(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        var_index: SQUnsignedInteger,
    ) -> &CStr {
        unsafe {
            CStr::from_ptr((self.inner.getfreevariable)(vm, index, var_index))
        }
    }

    // =========================================================================
    // 15. 错误处理
    // =========================================================================

    /// 抛出错误
    pub fn throw_error(&self, vm: HSQUIRRELVM, err: &str) -> SQRESULT {
        let c_err = CString::new(err).expect("Invalid error string");
        (self.inner.throwerror)(vm, c_err.as_ptr())
    }

    /// 重置VM的错误状态
    pub fn reset_error(&self, vm: HSQUIRRELVM) {
        (self.inner.reseterror)(vm)
    }

    /// 获取最后一个错误并推入栈顶
    pub fn get_last_error(&self, vm: HSQUIRRELVM) {
        (self.inner.getlasterror)(vm)
    }

    // =========================================================================
    // 16. 对象操作
    // =========================================================================

    /// 获取栈元素的原始对象表示（输出参数）
    pub fn get_stack_obj(
        &self,
        vm: HSQUIRRELVM,
        index: SQInteger,
        out: &mut HSQOBJECT,
    ) -> SQRESULT {
        (self.inner.getstackobj)(vm, index, out as *mut _)
    }

    /// 将原始对象推入栈顶
    pub fn push_object(&self, vm: HSQUIRRELVM, obj: HSQOBJECT) {
        (self.inner.pushobject)(vm, obj)
    }

    /// 增加对象的引用计数
    pub fn add_ref(&self, vm: HSQUIRRELVM, obj: &mut HSQOBJECT) {
        (self.inner.addref)(vm, obj as *mut _)
    }

    /// 减少对象的引用计数（返回是否已释放）
    pub fn release(&self, vm: HSQUIRRELVM, obj: &mut HSQOBJECT) -> SQBool {
        (self.inner.release)(vm, obj as *mut _)
    }

    /// 重置对象为null
    pub fn reset_object(&self, obj: &mut HSQOBJECT) {
        (self.inner.resetobject)(obj as *mut _)
    }

    /// 将对象转换为字符串
    pub fn obj_to_string(&self, obj: &HSQOBJECT) -> &CStr {
        unsafe {
            CStr::from_ptr((self.inner.objtostring)(obj))
        }
    }

    /// 将对象转换为布尔值
    pub fn obj_to_bool(&self, obj: &HSQOBJECT) -> SQBool {
        (self.inner.objtobool)(obj)
    }

    /// 将对象转换为整数
    pub fn obj_to_integer(&self, obj: &HSQOBJECT) -> SQInteger {
        (self.inner.objtointeger)(obj)
    }

    /// 将对象转换为浮点数
    pub fn obj_to_float(&self, obj: &HSQOBJECT) -> SQFloat {
        (self.inner.objtofloat)(obj)
    }

    /// 获取对象的类型标签（输出参数）
    pub fn get_obj_type_tag(&self, obj: &HSQOBJECT, out: &mut SQUserPointer) -> SQRESULT {
        (self.inner.getobjtypetag)(obj, out as *mut _)
    }

    // =========================================================================
    // 17. 垃圾回收
    // =========================================================================

    /// 执行垃圾回收（返回回收的对象数量）
    pub fn collect_garbage(&self, vm: HSQUIRRELVM) -> SQInteger {
        (self.inner.collectgarbage)(vm)
    }

    // =========================================================================
    // 18. 闭包序列化
    // =========================================================================

    /// 将闭包写入流
    pub fn write_closure(
        &self,
        vm: HSQUIRRELVM,
        write_func: SQWRITEFUNC,
        user_ptr: SQUserPointer,
    ) -> SQRESULT {
        (self.inner.writeclosure)(vm, write_func, user_ptr)
    }

    /// 从流读取闭包
    pub fn read_closure(
        &self,
        vm: HSQUIRRELVM,
        read_func: SQREADFUNC,
        user_ptr: SQUserPointer,
    ) -> SQRESULT {
        (self.inner.readclosure)(vm, read_func, user_ptr)
    }

    // =========================================================================
    // 19. 内存管理
    // =========================================================================

    /// 分配内存（Squirrel内部使用的malloc）
    pub fn malloc(&self, size: SQUnsignedInteger) -> *mut c_void {
        (self.inner.malloc)(size)
    }

    /// 重分配内存
    pub fn realloc(
        &self,
        ptr: *mut c_void,
        old_size: SQUnsignedInteger,
        new_size: SQUnsignedInteger,
    ) -> *mut c_void {
        (self.inner.realloc)(ptr, old_size, new_size)
    }

    /// 释放内存
    pub fn free(&self, ptr: *mut c_void, size: SQUnsignedInteger) {
        (self.inner.free)(ptr, size)
    }

    // =========================================================================
    // 20. 调试信息
    // =========================================================================

    /// 获取栈跟踪信息（输出参数）
    pub fn stack_infos(
        &self,
        vm: HSQUIRRELVM,
        level: SQInteger,
        out: &mut SQStackInfos,
    ) -> SQRESULT {
        (self.inner.stackinfos)(vm, level, out as *mut _)
    }

    /// 设置调试钩子
    pub fn set_debug_hook(&self, vm: HSQUIRRELVM) {
        (self.inner.setdebughook)(vm)
    }
}
