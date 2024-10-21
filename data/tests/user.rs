use data::repository::user::{postgres::PostgresUserRepository, UserRepository};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

mod utils;

use utils::connect;

async fn build_repo(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<PostgresUserRepository> {
    let conn = connect(pool_options, connect_options).await?;

    Ok(PostgresUserRepository::from(conn.clone()))
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
async fn create_user(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let user = user_repo
        .create_user("test@myemail.com", "my_test_password")
        .await
        .unwrap();

    assert_eq!(&user.email, "test@myemail.com");

    Ok(())
}

#[sqlx::test(fixtures("user"))]
async fn verify_user_hash(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let user_repo = build_repo(pool_options, connect_options).await?;

    let user = user_repo.create_user().await.unwrap();

    assert_eq!(&user.email, "test@myemail.com");

    Ok(())
}
