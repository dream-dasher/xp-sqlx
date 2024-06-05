# xp-SQLx


comparative times (for memory transpose approaches)
Times Recorded (ms):
```
TimesTaken {
    direct: 46692,  <-- not even converted to a dataframe
    vstruct: 28609,
    recopy: 58234,
}
```


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
