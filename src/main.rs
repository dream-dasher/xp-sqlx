use clap::Parser;
use futures::TryStreamExt;
use sqlx::mysql::MySqlPool;
use sqlx::Row;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    // like element to search for in subdomain names
    likeness: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = Args::parse();

    // SDM gated US-prod_webDB
    let pool = MySqlPool::connect("mysql://127.0.0.1:13309/pagerduty_production").await?;
    // let pool = MySqlPool::connect("mysql://root:root@127.0.0.1/student").await?;

    let mut rows = sqlx::query("SELECT subdomain FROM accounts WHERE subdomain LIKE ? LIMIT 3")
        .bind(format!("%{}%", args.likeness))
        .fetch(&pool);

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
