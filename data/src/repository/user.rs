use std::future::Future;

use uuid::Uuid;

use crate::{
    model::user::{UpdateUser, User},
    Error,
};

pub mod postgres;

pub enum UserRepositorySettings {
    Mock,
    Postgres,
}

pub trait UserRepository {
    fn get_user(&self, id: &Uuid) -> impl Future<Output = Result<User, Error>> + Send;

    fn create_user(
        &self,
        email: &str,
        password: &str,
    ) -> impl Future<Output = Result<User, Error>> + Send;

    fn update_user(
        &self,
        id: &Uuid,
        update: UpdateUser,
    ) -> impl Future<Output = Result<User, Error>> + Send;

    fn delete_user(&self, id: &Uuid) -> impl Future<Output = Result<User, Error>> + Send;

    fn list_users(&self) -> impl Future<Output = Result<Vec<User>, Error>> + Send;

    fn verify_user_password(
        &self,
        email: &str,
        password: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn hash_password(&self, password: &str) -> impl Future<Output = Result<String, Error>> + Send;
}
