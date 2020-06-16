#[derive(PartialEq, Debug)]
pub struct Amount(i32);

impl std::convert::From<i32> for Amount {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl std::convert::Into<i32> for Amount {
    fn into(self) -> i32 {
        self.0
    }
}

impl std::ops::Add for Amount {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for Amount {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl std::ops::Mul for Amount {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Amount: {}", self.0)
    }
}
