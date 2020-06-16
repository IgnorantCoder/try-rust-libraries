#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_more;

mod builder;
mod getters;
mod more;
mod new;

fn main() {
    {
        // try derive new
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
        // try derive getters
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

    {
        // try derive builder
        let user0 = builder::naive::UserBuilder::default()
            .id(uuid::Uuid::new_v4())
            .first_name("John".to_owned())
            .last_name("Smith".to_owned())
            .build();
        assert!(user0.is_ok());
        let user1 = builder::naive::UserBuilder::default()
            .id(uuid::Uuid::new_v4())
            .first_name("John".to_owned())
            .build();
        assert!(user1.is_err());

        let user2 = builder::library::UserBuilder::default()
            .id(uuid::Uuid::new_v4())
            .first_name("John")
            .last_name("Smith")
            .age(30)
            .build();
        assert!(user2.is_ok());
        let user3 = builder::library::UserBuilder::default()
            .id(uuid::Uuid::new_v4())
            .first_name("John")
            .last_name("Smith")
            .build();
        assert!(user3.is_ok());
        let user5 = builder::library::UserBuilder::default()
            .id(uuid::Uuid::new_v4())
            .first_name("John")
            .build();
        assert!(user5.is_err());
    }

    {
        // try derive more
        assert_eq!(
            "Amount: 20",
            format!("{}", more::naive::Amount::from(20)).as_str()
        );
        assert_eq!(
            more::naive::Amount::from(20) + more::naive::Amount::from(30),
            more::naive::Amount::from(50)
        );
        assert_eq!(
            more::naive::Amount::from(20) - more::naive::Amount::from(30),
            more::naive::Amount::from(-10)
        );

        assert_eq!(
            "Amount: 20",
            format!("{}", more::naive::Amount::from(20)).as_str()
        );
        assert_eq!(
            more::library::Amount::from(20) + more::library::Amount::from(30),
            more::library::Amount::from(50)
        );
        assert_eq!(
            more::library::Amount::from(20) - more::library::Amount::from(30),
            more::library::Amount::from(-10)
        );
    }

    {
        // apply
        use std::sync::{Arc, Mutex};
        let _counter0 = Arc::new(Mutex::new(0)); // Resultとかもラップするの面倒

        use apply::Apply;
        let _counter1 = 0.apply(Mutex::new).apply(Arc::new); // 適用したい順番でかけるし嬉しい
    }

    {
        // boolinator
        use boolinator::Boolinator;

        #[derive(PartialEq, Debug)]
        pub enum ErrorType {
            OhMyGod,
        }

        assert_eq!(Some(5), true.as_some_from(|| 5));
        assert_eq!(None, false.as_some_from(|| 5));
        assert_eq!(Ok(()), true.ok_or_else(|| ErrorType::OhMyGod));
        assert_eq!(
            Err(ErrorType::OhMyGod),
            false.ok_or_else(|| ErrorType::OhMyGod)
        );
    }
}
