use futures::TryStreamExt;
use sqlx::mysql::MySqlPool;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // SDM gated US-prod_webDB
    let pool = MySqlPool::connect("mysql://127.0.0.1:13309/pagerduty_production").await?;
    // let pool = MySqlPool::connect("mysql://root:root@127.0.0.1/student").await?;

    let mut rows = sqlx::query("SELECT subdomain FROM accounts LIMIT 3").fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        // dbg!(row);
        let subdomain: &str = row.try_get("subdomain")?;
        println!("Subdomain, first three in __ order: {}", subdomain);
    }

    let mut rows = sqlx::query("SELECT subdomain FROM accounts WHERE id = 10 LIMIT 1").fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        // dbg!(row);
        let subdomain: &str = row.try_get("subdomain")?;
        println!("Subdomain for id '10': {}", subdomain);
    }

    Ok(())
}
