use crate::func::VcmpFunctions;

pub trait QueryTemplate {}
pub trait SetTemplate {}

impl SetTemplate for VcmpFunctions {}

impl QueryTemplate for VcmpFunctions {}
