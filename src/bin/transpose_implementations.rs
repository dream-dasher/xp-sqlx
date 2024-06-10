//! comparing (running) come different ways implementations that
//! get a sqlx stream into a polars::DataFrame

use std::time::Instant;

use clap::{Parser, ValueEnum};
use derive_more::{Constructor, Display};
use xp_sqlx::stream_to_df::{direct_transpose, recopy_transpose, vstruct_transpose};

/// Arguments to select MemoryTranspose Implementations and Repetition of DB draws (increasing data transposed)
/// Principally for use with Hyperfine to do benchmarking.
/// (Preferred benchmarking framework (Divan) does not currently support async operations.)
///
/// Note: This requires an active DataBase on a specific port.
/// Justfile in the associatted repo has docker create and destroy code to set one up.
/// `template.env` has default address for the dockerized DB
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Select the implementation to test
    implementation: TransImpl,
    /// Number of times to repeat the test
    repetition:     u32,
}

#[derive(ValueEnum, Clone, Debug)]
enum TransImpl {
    Direct,
    Recopy,
    // set to 'v-struct', but not worth hunting for more syntax to get rename = "lower" to work
    VStruct,
    All,
}

#[derive(Debug, Constructor, Display)]
#[display(fmt = "Direct impl time: {}\nRecopy impl time: {}\nVStruct impl time: {}",
          direct,
          recopy,
          vstruct)]
struct TimesTaken {
    pub direct:  u128,
    pub vstruct: u128,
    pub recopy:  u128,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = Args::parse();
    let reps = args.repetition;

    let now = Instant::now();
    match args.implementation {
        TransImpl::Direct => direct_transpose(reps).await?,
        TransImpl::VStruct => recopy_transpose(reps).await?,
        TransImpl::Recopy => vstruct_transpose(reps).await?,
        TransImpl::All => {
            let mut elapsed_times_struct = TimesTaken::new(0, 0, 0);

            let now = Instant::now();
            direct_transpose(reps).await?;
            let elapsed_time = now.elapsed();
            elapsed_times_struct.direct = elapsed_time.as_millis();

            let now = Instant::now();
            recopy_transpose(reps).await?;
            let elapsed_time = now.elapsed();
            elapsed_times_struct.recopy = elapsed_time.as_millis();

            let now = Instant::now();
            vstruct_transpose(reps).await?;
            let elapsed_time = now.elapsed();
            elapsed_times_struct.vstruct = elapsed_time.as_millis();

            println!("\n\nTimes Elapsed: {}", elapsed_times_struct);
        }
    };

    let elapsed_time = now.elapsed();
    println!("\n\nTotal Time Recorded (ms):\n{:#?}",
             elapsed_time.as_millis());
    Ok(())
}
