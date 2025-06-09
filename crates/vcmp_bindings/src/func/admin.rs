use crate::{func::VcmpFunctions};

pub trait Administration {
    fn ban_ip(&self, ip: &str);
    fn unban_ip(&self, ip: &str) -> bool;
    fn is_ip_banned(&self, ip: &str) -> bool;
}

impl Administration for VcmpFunctions {
    fn ban_ip(&self, ip: &str) {
        (self.inner.BanIP)(ip.as_ptr() as *mut i8);
    }
    fn unban_ip(&self, ip: &str) -> bool {
        (self.inner.UnbanIP)(ip.as_ptr() as *mut i8) != 0
    }
    fn is_ip_banned(&self, ip: &str) -> bool {
        (self.inner.IsIPBanned)(ip.as_ptr() as *mut i8) != 0
    }
}
