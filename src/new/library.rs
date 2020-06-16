#[derive(new)]
pub struct User {
    name: String,
    gender: Gender,
    #[new(value = "None")] // new時に適当に初期化してほしいときとか
    age: Option<u16>,
}

#[derive(new)] // new_xxxみたいな名前で作ってくれる、xxxはsnake_caseになる
pub enum Gender {
    Male,
    Female,
    Bigender,
    Genderfluid,
    Agender,
    Demigender,
}

#[derive(new)]
pub struct TypedUser<T>
where
    T: UserType,
{
    name: String,
    gender: Gender,
    _type: std::marker::PhantomData<T>, // phantom dataは適当に初期化してくれる
}

pub trait UserType {}

pub struct Owner {}

impl UserType for Owner {}

pub struct Admin {}

impl UserType for Admin {}

pub struct Guest {}

impl UserType for Guest {}
