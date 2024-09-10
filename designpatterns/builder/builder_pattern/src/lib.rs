pub struct UserProfile {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

pub struct UserProfileBuilder {
    name: String,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
}

impl UserProfileBuilder {
    pub fn name(name: &str) -> Self {
        UserProfileBuilder {
            name: name.to_string(),
            email: None,
            phone: None,
            address: None,
        }
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn phone(mut self, phone: &str) -> Self {
        self.phone = Some(phone.to_string());
        self
    }

    pub fn address(mut self, address: &str) -> Self {
        self.address = Some(address.to_string());
        self
    }

    pub fn build(self) -> UserProfile {
        UserProfile {
            name: self.name,
            email: self.email,
            phone: self.phone,
            address: self.address,
        }
    }
}