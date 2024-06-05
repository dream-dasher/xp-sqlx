//! comparing (running) come different ways implementations that
//! get a sqlx stream into a polars::DataFrame

use std::time::Instant;
use xp_sqlx::stream_to_df::{direct_transpose, recopy_transpose, vstruct_transpose};

#[derive(Debug)]
struct TimesTaken {
    pub direct: u128,
    pub vstruct: u128,
    pub recopy: u128,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut times_taken = TimesTaken {
        direct: 0,
        vstruct: 0,
        recopy: 0,
    };

    let now = Instant::now();
    direct_transpose(20_000).await?;
    let elapsed_time = now.elapsed();
    times_taken.direct = elapsed_time.as_millis();

    let now = Instant::now();
    recopy_transpose(20_000).await?;
    let elapsed_time = now.elapsed();
    times_taken.recopy = elapsed_time.as_millis();

    let now = Instant::now();
    vstruct_transpose(20_000).await?;
    let elapsed_time = now.elapsed();
    times_taken.vstruct = elapsed_time.as_millis();

    println!("\n\nTimes Recorded (ms):\n{:#?}", times_taken);
    Ok(())
}
