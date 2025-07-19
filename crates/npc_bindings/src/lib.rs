use std::sync::OnceLock;

/// bindgen 报错了，只能用 github copliot 生成了
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod exports;

use crate::exports::NPCExports;

/// 全局的 NPC Exports
pub static NPC_EXPORTS: OnceLock<NPCExports> = OnceLock::new();

pub fn init_npc_plugin(func: NPCExports) -> &'static NPCExports {
    NPC_EXPORTS.get_or_init(|| func)
}

pub fn is_initialized_func() -> bool {
    NPC_EXPORTS.get().is_some()
}

pub fn npc_plugin_func() -> &'static NPCExports {
    NPC_EXPORTS.get().unwrap()
}
