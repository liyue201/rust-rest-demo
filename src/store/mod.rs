pub(crate) mod user;

use std::sync::Arc;
use rbatis::rbatis::Rbatis;

pub struct Store {
    rb: Arc<Rbatis>,
}

impl Store {
    pub fn new(rb: Arc<Rbatis>) -> Self {
        Store {
            rb
        }
    }
}

