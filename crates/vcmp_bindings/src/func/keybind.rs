use crate::{func::VcmpFunctions, utils::Keybind, VcmpError, VcmpResult};

pub trait KeybindMethods {
    fn register_key_bind(
        &self,
        release: bool,
        key1: i32,
        key2: Option<i32>,
        key3: Option<i32>,
    ) -> Keybind;

    fn remove_key_bind(&self, slot: i32);

    fn remove_all_key_binds(&self);

    fn get_key_bind_unused_slot(&self) -> i32;

    fn get_key_bind_data(&self, slot: i32) -> VcmpResult<Keybind>;
}

impl KeybindMethods for VcmpFunctions {
    fn register_key_bind(
        &self,
        release: bool,
        key1: i32,
        key2: Option<i32>,
        key3: Option<i32>,
    ) -> Keybind {
        let slot = self.get_key_bind_unused_slot();
        let k1 = key1;
        let k2 = key2.unwrap_or(0);
        let k3 = key3.unwrap_or(0);
        (self.inner.RegisterKeyBind)(slot, release as u8, k1, k2, k3);

        Keybind {
            slot,
            can_release: release,
            key: k1,
            key2: k2,
            key3: k3,
        }
    }
    fn remove_key_bind(&self, slot: i32) {
        (self.inner.RemoveKeyBind)(slot);
    }
    fn remove_all_key_binds(&self) {
        (self.inner.RemoveAllKeyBinds)();
    }

    fn get_key_bind_unused_slot(&self) -> i32 {
        (self.inner.GetKeyBindUnusedSlot)()
    }
    fn get_key_bind_data(&self, slot: i32) -> VcmpResult<Keybind> {
        let (mut release, mut key, mut key2, mut key3) = (0_u8, 0, 0, 0);
        let code = (self.inner.GetKeyBindData)(slot, &mut release, &mut key, &mut key2, &mut key3);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Keybind {
            slot,
            can_release: release != 0,
            key,
            key2,
            key3,
        })
        }
    }
}
