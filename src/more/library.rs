use derive_more::{Add, Display, From, Into, Mul, Sub};

#[derive(PartialEq, Debug, Add, Sub, Display, From, Into)]
#[display(fmt = "Amount: {}", _0)]
pub struct Amount(i32);
