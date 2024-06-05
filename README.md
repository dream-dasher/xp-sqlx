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
- Query
 - Yields Row
- Query_as
 - Yields Struct
- Query_File
 - Reads from .sql file
- Query_Scalar
 - yields a value (first value of row; useful for unwrapping a COUNT(*), for example)

_ : unchecked
! : compile-time checing
FromRow : for use with ***un**checked* query_as (not query*, not !)


## quick times record

**Takeaway**: no difference in perf between Vec<Struct> ~~> Polars::DataFrame and Struct<Vec<Field>> ~~> Polars::DataFrame
(would be nice to check allocations at somepoint, but not worth tooling play to do [re: async bench limitations])

```shell
~/coding_dirs/rust/xp-sqlx on  master [!] is 📦 v0.1.0 via 🦀 v1.80.0-nightly
❮ j hyperf 100
NOTE: we only care about 'recopy' and 'vstruct', 'direct' does not do DF creation, 'all' was quick substitution bench framework
Release:
hyperfine --warmup 3 'target/release/transpose_implementations recopy 100'
Benchmark 1: target/release/transpose_implementations recopy 100
  Time (mean ± σ):      72.0 ms ±   6.1 ms    [User: 13.6 ms, System: 8.4 ms]
  Range (min … max):    59.8 ms …  95.5 ms    34 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

hyperfine --warmup 3 'target/release/transpose_implementations v-struct 100'
Benchmark 1: target/release/transpose_implementations v-struct 100
  Time (mean ± σ):      71.3 ms ±   3.1 ms    [User: 14.8 ms, System: 8.7 ms]
  Range (min … max):    65.8 ms …  76.9 ms    34 runs

Debug (for compiler insights, mostly):
hyperfine --warmup 3 'target/debug/transpose_implementations recopy 100'
Benchmark 1: target/debug/transpose_implementations recopy 100
  Time (mean ± σ):     255.1 ms ±  16.8 ms    [User: 99.3 ms, System: 17.5 ms]
  Range (min … max):   219.7 ms … 290.1 ms    11 runs

hyperfine --warmup 3 'target/debug/transpose_implementations v-struct 100'
Benchmark 1: target/debug/transpose_implementations v-struct 100
  Time (mean ± σ):     218.8 ms ±  29.9 ms    [User: 84.3 ms, System: 13.9 ms]
  Range (min … max):   141.3 ms … 255.3 ms    15 runs
```

Times Recorded (ms):
```
TimesTaken {
    direct: 46_692,  <-- not even converted to a dataframe
    vstruct: 28_609,
    recopy: 58_234,
}
```
