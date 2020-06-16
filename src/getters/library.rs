#[derive(Getters)]
pub struct Deal {
    #[getter(skip)] // 実装しないこともできるし、実装を自前のものに差し替えることもできる
    id: uuid::Uuid,
    #[getter(rename = "amount")] // Getterの名前を変えることもできる
    value: bigdecimal::BigDecimal,
    currency: Currency,
}

impl Deal {
    pub fn new(value: bigdecimal::BigDecimal, currency: Currency) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            value,
            currency,
        }
    }
}

pub enum Currency {
    Eur,
    Gbp,
    Usd,
    Chf,
    Jpy,
}
