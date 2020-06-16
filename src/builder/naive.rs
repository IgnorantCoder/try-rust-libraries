pub struct User {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    age: Option<u16>,
}

impl User {
    pub fn new(id: uuid::Uuid, first_name: String, last_name: String, age: Option<u16>) -> Self {
        Self {
            id,
            first_name,
            last_name,
            age,
        }
    }
}

#[derive(Default)]
pub struct UserBuilder {
    id: Option<uuid::Uuid>,
    first_name: Option<String>,
    last_name: Option<String>,
    age: Option<u16>,
}

impl UserBuilder {
    pub fn id(self, value: uuid::Uuid) -> Self {
        Self {
            id: Some(value),
            ..self
        }
    }

    pub fn first_name(self, value: String) -> Self {
        Self {
            first_name: Some(value),
            ..self
        }
    }

    pub fn last_name(self, value: String) -> Self {
        Self {
            last_name: Some(value),
            ..self
        }
    }

    pub fn age(self, value: u16) -> Self {
        Self {
            age: Some(value),
            ..self
        }
    }

    pub fn build(self) -> Result<User, String> {
        match (self.id, self.first_name, self.last_name, self.age) {
            (Some(id), Some(first_name), Some(last_name), age) => Ok(User::new(
                id.clone(),
                first_name.clone(),
                last_name.clone(),
                age,
            )),
            _ => Err("insufficient".to_owned()),
        }
    }
}
