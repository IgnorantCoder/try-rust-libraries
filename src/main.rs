#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate derive_getters;

mod getters;
mod new;

fn main() {
    {
        //try derive new
        let _user0 = new::naive::User::new(
            "someone".to_owned(),
            new::naive::Gender::new_male(),
            Some(22),
        );

        let _user1 =
            new::library::User::new("someone".to_owned(), new::library::Gender::new_female());
        let _user2 = new::library::TypedUser::<new::library::Owner>::new(
            "someone".to_owned(),
            new::library::Gender::new_female(),
        );
    }

    {
        let deal0 = getters::naive::Deal::new(
            bigdecimal::BigDecimal::from(100_000),
            getters::naive::Currency::Jpy,
        );

        let _amount0 = deal0.amount();
        let _currency0 = deal0.currency();

        let deal1 = getters::naive::Deal::new(
            bigdecimal::BigDecimal::from(1_000),
            getters::naive::Currency::Usd,
        );

        let _amount1 = deal1.amount();
        let _currency1 = deal1.currency();
    }
}
