// src/domain/user.rs
use uuid::Uuid;
use crate::domain::*;

pub struct User {
    id: Uuid,
    email: String,
    password: String,
    is_registered: bool,
}

impl User {
    pub fn new(id: Uuid, email: String, password: String) -> Self {
        User {
            id,
            email,
            password,
            is_registered: false,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email_as_ref(&self) -> &str {
        &self.email
    }

    pub fn password_as_ref(&self) -> &str {
        &self.password
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }

    pub fn register(&mut self) -> Result<(), UserDomainError> {
        if self.is_registered {
            return Err(UserDomainError::UserAlreadyRegistered(self.email.clone()));
        }

        self.is_registered = true;
        Ok(())
    }
}

impl Default for User {
    fn default() -> Self {
        User::new(
            Uuid::new_v4(),
            "boyg87059@gmail.com".to_string(),
            "password".to_string(),
        )
    }
}
