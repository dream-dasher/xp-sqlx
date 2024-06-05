//! comparing (running) come different ways implementations that
//! get a sqlx stream into a polars::DataFrame

use xp_sqlx::stream_to_df::{direct_transpose, recopy_transpose, vstruct_transpose};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    direct_transpose(1).await?;
    vstruct_transpose(1).await?;
    recopy_transpose(1).await?;

    Ok(())
}
