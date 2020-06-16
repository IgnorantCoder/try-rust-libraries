pub struct Deal {
    id: uuid::Uuid,
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

    pub fn amount(&self) -> &bigdecimal::BigDecimal {
        &self.value
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }
}

pub enum Currency {
    Eur,
    Gbp,
    Usd,
    Chf,
    Jpy,
}
