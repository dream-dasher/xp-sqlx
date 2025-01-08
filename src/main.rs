use std::fmt::Debug;

use clap::{Parser, Subcommand};
use futures::TryStreamExt;
use sqlx::{Row, mysql::MySqlPool};

/// Simple query on subdomains
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        // Regex string to compare to `account.subdomain`
        #[command(subcommand)]
        command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
        /// Search subdomain names using regex
        SubdomainSearch {
                ///Regex pattern to match against subdomain names
                regex: String,
        },
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
        let args = Args::parse();

        // SDM gated US-prod_webDB
        let pool_us = MySqlPool::connect("mysql://127.0.0.1:13309/pagerduty_production").await?;
        // SDM gated EU-prod_webDB
        let pool_eu = MySqlPool::connect("mysql://127.0.0.1:13310/pagerduty_production").await?;

        match &args.command {
                Commands::SubdomainSearch { regex } => {
                        for (pool, region_str) in [(pool_us, "US"), (pool_eu, "EU")] {
                                println!("\nPool corresponding to region: {:?}", region_str);
                                let mut rows_dyn_fetch =
                                        sqlx::query("SELECT subdomain FROM accounts WHERE subdomain REGEXP ? LIMIT 3")
                                                .bind(regex)
                                                .fetch(&pool);

                                while let Some(row) = rows_dyn_fetch.try_next().await? {
                                        // dbg!(row);
                                        let subdomain: &str = row.try_get("subdomain")?;
                                        println!("Dyn fetch: Subdomain, first three in __ order: {}", subdomain);
                                }

                                // NOTE: verified (macro) queries take param values as *arguments* rather than bind methods.
                                let mut rows_stat_fetch = sqlx::query!(
                                        "SELECT subdomain FROM accounts WHERE subdomain REGEXP ? LIMIT 3",
                                        format!("{}", regex)
                                )
                                .fetch(&pool);

                                while let Some(row) = rows_stat_fetch.try_next().await? {
                                        // dbg!(row);
                                        let subdomain = row.subdomain;
                                        println!(
                                                "Static check fetch: Subdomain, first three in __ order: {}",
                                                subdomain
                                        );
                                }

                                let mut rows = sqlx::query("SELECT subdomain FROM accounts WHERE id = 10 LIMIT 1")
                                        .fetch(&pool);

                                while let Some(row) = rows.try_next().await? {
                                        // dbg!(row);
                                        let subdomain: &str = row.try_get("subdomain")?;
                                        println!("Subdomain for id '10': {}", subdomain);
                                }
                        }
                }
        }

        Ok(())
}
