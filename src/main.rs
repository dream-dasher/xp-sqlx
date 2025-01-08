use std::fmt::Debug;

use clap::{Parser, Subcommand};
use futures::TryStreamExt;
use sqlx::{Row, mysql::MySqlPool};

const PD_PROD_SDM_US: &str = "mysql://127.0.0.1:13309/pagerduty_production";
const PD_PROD_SDM_EU: &str = "mysql://127.0.0.1:13310/pagerduty_production";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
        let args = Args::parse();

        // SDM gated US-prod_webDB
        let pool_us = MySqlPool::connect(PD_PROD_SDM_US).await?;
        // SDM gated EU-prod_webDB
        let pool_eu = MySqlPool::connect(PD_PROD_SDM_EU).await?;

        match &args.command {
                Commands::FeaturesDistinct => {
                        let pools = [(pool_us, "US"), (pool_eu, "EU")];
                        for (pool, region_str) in &pools {
                                println!("\nFetching distinct features for region: {:?}", region_str);

                                let mut rows = sqlx::query_file!("data/sql_queries/features_distinct.sql").fetch(pool);

                                while let Some(row) = rows.try_next().await? {
                                        println!(
                                                "Feature: {:?}, Count: {:?}, First: {:?}, Last: {:?}, Region: {:?}",
                                                row.feature_name,
                                                row.feature_count,
                                                row.first_occurrence,
                                                row.last_occurrence,
                                                row.log_of_inferred_region,
                                        );
                                }
                        }
                        for (pool, _region_str) in &pools {
                                let mut rows =
                                        sqlx::query_file_as!(FeatureRecord, "data/sql_queries/features_distinct.sql")
                                                .fetch(pool);

                                while let Some(record) = rows.try_next().await? {
                                        println!("Feature: {:?}", record.feature_name);
                                        println!("  Count: {:?}", record.feature_count);
                                        println!("  First seen: {:?}", record.first_occurrence);
                                        println!("  Last seen: {:?}", record.last_occurrence);
                                        println!("  Server info:");
                                        println!("    Region: {:?}", record.log_of_inferred_region);
                                        println!("    Host: {:?}", record.log_of_hostname);
                                        println!("    Database: {:?}", record.log_of_database);
                                        println!("    Server UUID: {:?}", record.log_of_server_uuid);
                                        println!("    Time of query: {:?}", record.log_of_query_start_time_utc);
                                        println!();
                                }
                        }
                }
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
#[derive(Debug)]
struct FeatureRecord {
        feature_name:                Option<String>,
        feature_count:               i64,
        first_occurrence:            Option<chrono::NaiveDateTime>,
        last_occurrence:             Option<chrono::NaiveDateTime>,
        log_of_inferred_region:      Option<String>,
        log_of_hostname:             Option<String>,
        log_of_server_uuid:          Option<String>,
        log_of_database:             Option<String>,
        log_of_query_start_time_utc: Option<chrono::NaiveDateTime>,
}

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
        /// List distinct features and their counts
        FeaturesDistinct,
        /// Search subdomain names using regex
        SubdomainSearch {
                ///Regex pattern to match against subdomain names
                regex: String,
        },
}
