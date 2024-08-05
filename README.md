# xp-SQLx


## General
Exploring SQLx library.

Uses dockerized MySQL database exploration.
See Justfile for creation and teardown commands.

For memory transposition (from row to column) to create arrow rep :
async code benching is a bit awkward.  (not vibing with criterion crate, divan doesn't support async at all yet)

So just wrapping run code in cli and using hyperfine.  Appropriate granularity to current purposes.

(Curious about future zero-copy implementation using async stream -- and what optimizations compiler might render without my carefulyl placing in memory.)
comparative times (for memory transpose approaches)



## SQLx syntax notes
- `Query`
  - Yields Row
- `Query_as`
  - Yields Struct
- `Query_File`
  - Reads from .sql file
- `Query_Scalar`
  - yields a value (first value of row; useful for unwrapping a COUNT(*), for example)

- `_` : unchecked
- `!` : compile-time checing
- `FromRow` : for use with ***un**checked* query_as (not query*, not !)

## Docker Notes

`docker run` : image ~~> container
`docker start`: container(off) ~~> container(on)

## Bench
No notable differences across data crunch sizes tested. (1..10_000)

**Takeaway**: no difference in perf between `Vec<Struct>` ~~> `Polars::DataFrame` and `Struct<Vec<Field>>` ~~> `Polars::DataFrame`
(would be nice to check allocations at somepoint, but not worth tooling play to do [re: async bench limitations])

```shell
~/coding_dirs/rust/xp-sqlx on  master [!] via 🦀 v1.81.0-nightly
❯ j hyperf 10
Release:--------------------------------------------------------
hyperfine --warmup 3 'target/release/transpose_implementations v-of-struct 10'
Benchmark 1: target/release/transpose_implementations v-of-struct 10
  Time (mean ± σ):      18.8 ms ±   0.5 ms    [User: 3.8 ms, System: 3.2 ms]
  Range (min … max):    17.6 ms …  20.2 ms    140 runs

hyperfine --warmup 3 'target/release/transpose_implementations struct-of-v 10'
Benchmark 1: target/release/transpose_implementations struct-of-v 10
  Time (mean ± σ):      18.6 ms ±   0.5 ms    [User: 3.7 ms, System: 3.1 ms]
  Range (min … max):    17.4 ms …  19.9 ms    151 runs

hyperfine --warmup 3 'target/release/transpose_implementations series-to-df 10'
Benchmark 1: target/release/transpose_implementations series-to-df 10
  Time (mean ± σ):      18.3 ms ±   0.5 ms    [User: 3.4 ms, System: 3.1 ms]
  Range (min … max):    17.0 ms …  20.8 ms    149 runs

Debug:----------------------------------------------------------
hyperfine --warmup 3 'target/debug/transpose_implementations v-of-struct 10'
Benchmark 1: target/debug/transpose_implementations v-of-struct 10
  Time (mean ± σ):      36.1 ms ±   3.0 ms    [User: 12.8 ms, System: 4.0 ms]
  Range (min … max):    28.3 ms …  42.2 ms    98 runs

hyperfine --warmup 3 'target/debug/transpose_implementations struct-of-v 10'
Benchmark 1: target/debug/transpose_implementations struct-of-v 10
  Time (mean ± σ):      36.2 ms ±   3.2 ms    [User: 12.8 ms, System: 4.0 ms]
  Range (min … max):    28.6 ms …  41.8 ms    68 runs

hyperfine --warmup 3 'target/release/transpose_implementations series-to-df 10'
Benchmark 1: target/release/transpose_implementations series-to-df 10
  Time (mean ± σ):      18.5 ms ±   0.6 ms    [User: 3.4 ms, System: 3.2 ms]
  Range (min … max):    17.5 ms …  21.1 ms    142 runs
```
