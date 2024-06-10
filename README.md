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


## Bench
No notable differences across data crunch sizes tested. (1..10_000)

**Takeaway**: no difference in perf between `Vec<Struct>` ~~> `Polars::DataFrame` and `Struct<Vec<Field>>` ~~> `Polars::DataFrame`
(would be nice to check allocations at somepoint, but not worth tooling play to do [re: async bench limitations])

```shell
~/coding_dirs/rust/xp-sqlx on  master via 🦀 v1.81.0-nightly took 13s
❮ j hyperf 10
NOTE: we only care about 'v-of-struct' and 'struct-of-v'; 'all' was quick substitution bench framework
Release:
hyperfine --warmup 3 'target/release/transpose_implementations v-of-struct 10'
Benchmark 1: target/release/transpose_implementations v-of-struct 10
  Time (mean ± σ):      18.8 ms ±   0.5 ms    [User: 3.8 ms, System: 3.2 ms]
  Range (min … max):    17.6 ms …  20.5 ms    138 runs

hyperfine --warmup 3 'target/release/transpose_implementations struct-of-v 10'
Benchmark 1: target/release/transpose_implementations struct-of-v 10
  Time (mean ± σ):      18.6 ms ±   0.6 ms    [User: 3.7 ms, System: 3.1 ms]
  Range (min … max):    17.6 ms …  22.6 ms    145 runs

Debug (for compiler insights, mostly):
hyperfine --warmup 3 'target/debug/transpose_implementations v-of-struct 10'
Benchmark 1: target/debug/transpose_implementations v-of-struct 10
  Time (mean ± σ):      36.3 ms ±   2.8 ms    [User: 12.9 ms, System: 3.9 ms]
  Range (min … max):    29.4 ms …  42.3 ms    95 runs

hyperfine --warmup 3 'target/debug/transpose_implementations struct-of-v 10'
Benchmark 1: target/debug/transpose_implementations struct-of-v 10
  Time (mean ± σ):      36.1 ms ±   3.4 ms    [User: 12.8 ms, System: 4.0 ms]
  Range (min … max):    27.8 ms …  41.8 ms    70 runs```
