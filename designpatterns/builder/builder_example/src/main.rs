use builder_pattern::UserProfileBuilder;

fn main() {
    let user_profile = UserProfileBuilder::name("John Doe")
    .email("john.doe@example.com")
    .phone("+1 123-456-7890")
    .address("123 Main St, Anytown, USA")
    .build();

    println!("User Profile: ");
    println!("Email: {:?}", user_profile.email);

    if let Some(email) = &user_profile.email {
        println!("Email: {:?}", email);
    }

    if let Some(phone) = &user_profile.phone {
        println!("Phone number: {:?}", phone);
    }

    if let Some(address) = &user_profile.address {
        println!("Address: {:?}", address);
    }
}