use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    model::user::{UpdateUser, User},
    Error,
};

use super::UserRepository;

#[derive(Debug, Clone)]
pub struct PostgresUserRepository {
    pub pool: PgPool,
    argon: Argon2<'static>,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool, argon: Argon2<'static>) -> Self {
        Self { pool, argon }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn get_user(&self, id: &Uuid) -> Result<User, Error> {
        let mut tx = self
            .pool
            .clone()
            .begin()
            .await
            .map_err(Error::TransactionError)?;

        let result = sqlx::query!(r#"SELECT * FROM users WHERE id = $1;"#, id)
            .fetch_one(&mut *tx)
            .await
            .map_err(Error::ReadError)?;

        tx.commit().await.map_err(Error::TransactionError)?;

        Ok(User {
            id: result.id,
            email: result.email,
            hash: result.hash,
            is_admin: result.is_admin,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }

    async fn list_users(&self) -> Result<Vec<User>, Error> {
        let mut tx = self
            .pool
            .clone()
            .begin()
            .await
            .map_err(Error::TransactionError)?;

        let result = sqlx::query!(r#"SELECT * FROM users;"#)
            .fetch_all(&mut *tx)
            .await
            .map_err(Error::ReadError)?;

        tx.commit().await.map_err(Error::TransactionError)?;

        let users: Vec<User> = result
            .into_iter()
            .map(|value| User {
                id: value.id,
                email: value.email,
                hash: value.hash,
                is_admin: value.is_admin,
                created_at: value.created_at,
                updated_at: value.updated_at,
            })
            .collect();

        Ok(users)
    }

    async fn create_user(&self, email: &str, password: &str) -> Result<User, Error> {
        let mut tx = self
            .pool
            .clone()
            .begin()
            .await
            .map_err(Error::TransactionError)?;

        let new_id = Uuid::new_v4();

        let hash = self.hash_password(password).await?;

        let result = sqlx::query!(
            r#"
                INSERT INTO users ( id, email, hash )
                VALUES (
                    $1,
                    $2,
                    $3
                )
                RETURNING *;
            "#,
            new_id,
            email,
            hash
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::ReadError)?;

        tx.commit().await.map_err(Error::TransactionError)?;

        Ok(User {
            id: result.id,
            email: result.email,
            hash: result.hash,
            is_admin: result.is_admin,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }

    async fn update_user(&self, id: &Uuid, update: UpdateUser) -> Result<User, Error> {
        todo!()
    }

    async fn delete_user(&self, id: &Uuid) -> Result<User, Error> {
        let mut tx = self
            .pool
            .clone()
            .begin()
            .await
            .map_err(Error::TransactionError)?;

        let result = sqlx::query!(
            r#"
                DELETE FROM users
                WHERE id = $1
                RETURNING *;
            "#,
            id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::ReadError)?;

        tx.commit().await.map_err(Error::TransactionError)?;

        Ok(User {
            id: result.id,
            email: result.email,
            hash: result.hash,
            is_admin: result.is_admin,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }

    async fn verify_user_password(&self, email: &str, password: &str) -> Result<(), Error> {
        let mut tx = self
            .pool
            .clone()
            .begin()
            .await
            .map_err(Error::TransactionError)?;

        let result = sqlx::query!(r#"SELECT * FROM users WHERE email = $1;"#, email)
            .fetch_one(&mut *tx)
            .await
            .map_err(Error::ReadError)?;

        tx.commit().await.map_err(Error::TransactionError)?;

        self.verify_password(password, &result.hash).await
    }

    async fn hash_password(&self, password: &str) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);

        self.argon
            .hash_password(password.as_bytes(), &salt)
            .map(|value| value.to_string())
            .map_err(|_| Error::Hash)
    }

    async fn verify_password(&self, password: &str, hash: &str) -> Result<(), Error> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| Error::Hash)?;

        self.argon
            .verify_password(password.as_bytes(), &parsed_hash)
            .inspect_err(|err| {
                println!("there is an error: {err:#?}");
            })
            .map_err(|_| Error::Hash)
    }
}
