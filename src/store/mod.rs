pub(crate) mod user;

use rbatis::rbatis::Rbatis;

pub struct Store {
    rb: Rbatis,
}

impl Store {
    pub fn new(rb: Rbatis) -> Self {
        Store {
            rb
        }
    }
}

