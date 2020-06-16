pub struct User {
    name: String,
    gender: Gender,
    age: Option<u16>,
}

impl User {
    pub fn new(name: String, gender: Gender, age: Option<u16>) -> Self {
        Self { name, gender, age }
    }
}

pub enum Gender {
    Male,
    Female,
    Bigender,
    Genderfluid,
    Agender,
    Demigender,
}

impl Gender {
    pub fn new_male() -> Self {
        Gender::Male
    }
    pub fn new_female() -> Self {
        Gender::Female
    }
    pub fn new_bigender() -> Self {
        Gender::Bigender
    }
    pub fn new_genderfluid() -> Self {
        Gender::Genderfluid
    }
    pub fn new_agender() -> Self {
        Gender::Agender
    }
    pub fn new_demigender() -> Self {
        Gender::Demigender
    }
}
