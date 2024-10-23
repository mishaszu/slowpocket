use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, PasswordHash, Version,
};
use data::{
    model::user::{PasswordUpdate, UpdateUser},
    repository::user::{postgres::PostgresUserRepository, UserRepository},
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

mod utils;

use utils::connect;
use uuid::Uuid;

async fn build_repo(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<PostgresUserRepository> {
    let conn = connect(pool_options, connect_options).await?;

    let config = Argon2::new_with_secret(
        b"mysecret",
        Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    )
    .unwrap();

    Ok(PostgresUserRepository::new(conn, config))
}

#[sqlx::test(fixtures("user"))]
async fn list_users(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let users = user_repo.list_users().await.unwrap();

    assert_eq!(users.len(), 1);
    let first_user = users.first().unwrap();
    assert_eq!(&first_user.email, "test@myemail.com");

    Ok(())
}

#[sqlx::test(fixtures("user"))]
async fn get_user(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let user_id = Uuid::parse_str("a74f9b43-8a49-4d97-8270-9879d37c600d").unwrap();

    let user = user_repo.get_user(&user_id).await.unwrap();

    assert_eq!(&user.email, "test@myemail.com");

    Ok(())
}

#[sqlx::test(fixtures("user"))]
async fn create_user(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let user = user_repo
        .create_user("test2@myemail.com", "my_test_password")
        .await
        .unwrap();

    assert_eq!(&user.email, "test2@myemail.com");

    let users = user_repo.list_users().await.unwrap();

    assert_eq!(users.len(), 2);

    Ok(())
}

#[sqlx::test(fixtures("user"))]
async fn update_user_email(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let update_email = "another@myemail.com";
    let user_id = Uuid::parse_str("a74f9b43-8a49-4d97-8270-9879d37c600d").unwrap();

    let update = UpdateUser {
        email: Some(update_email.to_string()),
        password: None,
    };

    let user = user_repo.update_user(&user_id, update).await.unwrap();

    assert_eq!(&user.email, update_email);

    let user = user_repo.get_user(&user_id).await.unwrap();

    assert_eq!(&user.email, update_email);

    Ok(())
}

#[sqlx::test(fixtures("user"))]
async fn update_user_password(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let new_password = "brand_new_pass";
    let old_password = "dev_only_pass";

    let user_id = Uuid::parse_str("a74f9b43-8a49-4d97-8270-9879d37c600d").unwrap();

    let update = UpdateUser {
        email: None,
        password: Some(PasswordUpdate {
            new_password: new_password.to_string(),
            old_password: old_password.to_string(),
        }),
    };

    let user = user_repo.update_user(&user_id, update).await.unwrap();

    let validate = user_repo.verify_password(new_password, &user.hash).await;

    assert!(validate.is_ok());

    Ok(())
}

#[sqlx::test(fixtures("user"))]
async fn update_user_email_and_password(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let update_email = "another@myemail.com";
    let new_password = "brand_new_pass";
    let old_password = "dev_only_pass";

    let user_id = Uuid::parse_str("a74f9b43-8a49-4d97-8270-9879d37c600d").unwrap();

    let update = UpdateUser {
        email: Some(update_email.to_string()),
        password: Some(PasswordUpdate {
            new_password: new_password.to_string(),
            old_password: old_password.to_string(),
        }),
    };

    let user = user_repo.update_user(&user_id, update).await.unwrap();

    let validate = user_repo.verify_password(new_password, &user.hash).await;

    assert!(validate.is_ok());

    Ok(())
}

#[sqlx::test(fixtures("user"))]
async fn delete_user(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let user = user_repo
        .create_user("test2@myemail.com", "my_test_password")
        .await
        .unwrap();

    assert_eq!(&user.email, "test2@myemail.com");

    // let user_id = Uuid::parse_str("a74f9b43-8a49-4d97-8270-9879d37c600d").unwrap();

    let user2 = user_repo.delete_user(&user.id).await.unwrap();

    assert_eq!(user2.id, user.id);

    match user_repo.get_user(&user.id).await {
        Ok(_) => panic!("Can get user after deletion"),
        Err(data::Error::NotFound(_)) => Ok(()),
        Err(data::Error::ReadError(sqlx::Error::RowNotFound)) => Ok(()),
        Err(err) => panic!("Get wrong error after getting deleted user: {err}"),
    }
}

#[sqlx::test(fixtures("user"))]
async fn verify_and_hash(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let user_pass = "some_test_pass";
    let hash = user_repo.hash_password(user_pass).await.unwrap();

    let verify = user_repo.verify_password(user_pass, &hash).await;

    assert!(verify.is_ok());

    Ok(())
}

#[sqlx::test(fixtures("user"))]
async fn hash_password(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    user_repo.hash_password("dev_only_pass").await.unwrap();

    Ok(())
}

#[sqlx::test(fixtures("user"))]
async fn verify_user_password(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let user_pass = "dev_only_pass";
    let user_email = "test@myemail.com";
    let verify = user_repo.verify_user_password(user_email, user_pass).await;

    assert!(verify.is_ok());

    Ok(())
}

#[test]
fn test_password_hashing() {
    let pass = "dev_only_pass";
    let salt = SaltString::generate(&mut OsRng);

    let config1 = Argon2::new_with_secret(
        b"mysecret",
        Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    )
    .unwrap();
    let hash1 = config1
        .hash_password(pass.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // println!(">{hash1}<");

    let config2 = Argon2::new_with_secret(
        b"mysecret",
        Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    )
    .unwrap();

    let hash2 = PasswordHash::new(&hash1).unwrap();

    let verification = config2.verify_password(pass.as_bytes(), &hash2);

    assert!(verification.is_ok());

    let saved_hash = "$argon2id$v=19$m=19456,t=2,p=1$l9VfAtWMe+bWqP81cgsDuQ$Z+ExthpqUCPuHSwxtHI1RP17OyVGo1/bapupD+cJYzw";

    let hash3 = PasswordHash::new(saved_hash).unwrap();

    let verification = config2.verify_password(pass.as_bytes(), &hash3);

    assert!(verification.is_ok());
}
