use chrono::NaiveDate;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::TryStreamExt;
use polars::prelude::*;
use sqlx::mysql::MySqlPoolOptions;
use xp_sqlx::stream_to_df::{direct_transpose, recopy_transpose, vstruct_transpose};

fn main() {
    todo!();
}
