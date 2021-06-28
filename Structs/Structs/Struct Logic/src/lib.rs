#[derive(Debug)]
pub struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    pub fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            /* Something goes here */
        } else {
            return Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            };
        }
    }

    pub fn is_international(&self) -> /* Add return type */{
        /* Something goes here */
    }

    pub fn get_fees(&self, cents_per_gram: i32) -> /* Add return type */ {
        /* Something goes here */
    }
}