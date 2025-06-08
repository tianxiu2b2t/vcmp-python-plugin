use crate::func::VcmpFunctions;

pub trait WeaponMethods {
    fn set_weapon_data_value(&self, weapon: i32, field: i32, value: f64);
    fn reset_weapon_data_value(&self, weapon: i32, field: i32);
    fn reset_weapon_data(&self, weapon: i32);
    fn reset_all_weapon_data(&self);
    fn get_weapon_data_value(&self, weapon: i32, field: i32) -> f64;
    fn is_weapon_data_value_modified(&self, weapon: i32, field: i32) -> bool;
}

impl WeaponMethods for VcmpFunctions {
    fn set_weapon_data_value(&self, weapon: i32, field: i32, value: f64) {
        (self.inner.SetWeaponDataValue)(weapon, field, value);
    }
    fn reset_weapon_data_value(&self, weapon: i32, field: i32) {
        (self.inner.ResetWeaponDataValue)(weapon, field);
    }
    fn reset_weapon_data(&self, weapon: i32) {
        (self.inner.ResetWeaponData)(weapon);
    }
    fn reset_all_weapon_data(&self) {
        (self.inner.ResetAllWeaponData)();
    }
    fn get_weapon_data_value(&self, weapon: i32, field: i32) -> f64 {
        (self.inner.GetWeaponDataValue)(weapon, field)
    }
    fn is_weapon_data_value_modified(&self, weapon: i32, field: i32) -> bool {
        (self.inner.IsWeaponDataValueModified)(weapon, field) != 0
    }
}
