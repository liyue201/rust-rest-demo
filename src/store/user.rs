use rbatis::crud::CRUD;
use rbatis::Error;

use super::Store;

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Store {
    pub async fn create_user(&self, user: User) -> Result<User, Error> {
        let r = self.rb.save(&user, &[]).await?;
        let mut u = user.clone();
        u.id = r.last_insert_id.unwrap();
        Ok(u)
    }

    pub async fn fetch_user_by_name(&self, username: &str) -> Result<Option<User>, Error> {
        return self.rb.fetch_by_column::<Option<User>, _>("username", username).await;
    }

    pub async fn fetch_user_by_id(&self, id: i32) -> Result<Option<User>, Error> {
        return self.rb.fetch_by_column::<Option<User>, _>("id", id).await;
    }
}
