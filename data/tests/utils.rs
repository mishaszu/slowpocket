use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

pub async fn connect(
    pool_options: PgPoolOptions,
    connect_options: PgConnectOptions,
) -> sqlx::Result<PgPool> {
    let username = "postgres".to_string();
    // dotenvy::var("TEST_DB_USERNAME").map_err(|e| sqlx::Error::Configuration(e.into()))?;
    let password = "dev_only_pwd".to_string();
    // dotenvy::var("TEST_DB_PASSWORD").map_err(|e| sqlx::Error::Configuration(e.into()))?;
    pool_options
        .connect_with(connect_options.username(&username).password(&password))
        .await
}
