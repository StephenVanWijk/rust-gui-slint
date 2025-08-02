#[allow(unused)]

#[derive(Debug, Clone, PartialEq)]
pub struct Address{
    pub recipient: String,
    pub street_name: String,
    pub building_number_or_ode: String,
    pub dependent_locality_code: String,
    pub locality: String,
    pub post_code: String,
    pub region: String,
    pub country: String,
    pub language_code: String,
}

impl Address{
    pub fn new() -> Self {
        Address {
            recipient: String::new(),
            street_name: String::new(),
            building_number_or_ode: String::new(),
            dependent_locality_code: String::new(),
            locality: String::new(),
            post_code: String::new(),
            region: String::new(),
            country: String::new(),
            language_code: String::new(),
        }
    }

    pub fn address_init() -> Address {
        let mut address_interface = Address::new();
        address_interface.recipient = "De heer Stephen van Wijk".to_string();
        address_interface.street_name = "Jachtlaan".to_string();
        address_interface.building_number_or_ode = "".to_string();
        address_interface.dependent_locality_code = "Brouwers Molen Noord".to_string();
        address_interface.locality = "Apeldoorn".to_string();
        address_interface.post_code = "7336 AB".to_string();
        address_interface.region = "Gelderland".to_string();
        address_interface.country = "Nederland".to_string();
        address_interface.language_code = "nl".to_string();
        address_interface
}
}
