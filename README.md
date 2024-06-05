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
Times Recorded (ms):
```
TimesTaken {
    direct: 46_692,  <-- not even converted to a dataframe
    vstruct: 28_609,
    recopy: 58_234,
}
```
