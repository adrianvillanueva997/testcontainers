use sqlx::postgres::PgPoolOptions;
use testcontainers::{clients, images};

#[tokio::test]
async fn deploy_postgres() -> Result<(), sqlx::Error> {
    let docker_client = clients::Cli::docker();
    let postgres = docker_client.run(images::postgres::Postgres::default());
    postgres.start();
    let port = postgres.get_host_port_ipv4(5432);
    let connection_string = format!("postgres://postgres:postgres@localhost:{}/postgres", port);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.0, 150);
    Ok(())
}
