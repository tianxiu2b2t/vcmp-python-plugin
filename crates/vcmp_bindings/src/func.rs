use crate::raw::PluginFuncs;

pub struct VcmpFunctions {
    inner: PluginFuncs,
}

impl From<PluginFuncs> for VcmpFunctions {
    fn from(value: PluginFuncs) -> Self {
        Self { inner: value }
    }
}
