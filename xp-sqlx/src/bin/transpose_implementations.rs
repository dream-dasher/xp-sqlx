//! comparing (running) come different ways implementations that
//! get a sqlx stream into a polars::DataFrame

use std::time::Instant;

use clap::{Parser, ValueEnum};
use derive_more::{Constructor, Display};
use xp_sqlx::stream_to_df::{struct_of_v_macro, v_of_struct_macro};

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
    StructOfV,
    // set to 'v-struct', but not worth hunting for more syntax to get rename = "lower" to work
    VOfStruct,
    All,
}

#[derive(Debug, Constructor, Display)]
#[display(fmt = "Vec<Struct>_macro! impl time: {}\nStruct<Vec<field>...>_macro! impl time: {}",
          v_of_struct_macro,
          struct_of_v_macro)]
struct TimesTaken {
    pub struct_of_v_macro: u128,
    pub v_of_struct_macro: u128,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = Args::parse();
    let reps = args.repetition;

    let now = Instant::now();
    match args.implementation {
        TransImpl::VOfStruct => v_of_struct_macro(reps).await?,
        TransImpl::StructOfV => struct_of_v_macro(reps).await?,
        TransImpl::All => {
            let mut elapsed_times_struct = TimesTaken::new(0, 0);

            let now = Instant::now();
            v_of_struct_macro(reps).await?;
            let elapsed_time = now.elapsed();
            elapsed_times_struct.v_of_struct_macro = elapsed_time.as_millis();

            let now = Instant::now();
            struct_of_v_macro(reps).await?;
            let elapsed_time = now.elapsed();
            elapsed_times_struct.struct_of_v_macro = elapsed_time.as_millis();

            println!("\n\nTimes Elapsed:\n{}", elapsed_times_struct);
        }
    };

    let elapsed_time = now.elapsed();
    println!("\n\nTotal Time Recorded (ms):\n{:#?}",
             elapsed_time.as_millis());
    Ok(())
}
