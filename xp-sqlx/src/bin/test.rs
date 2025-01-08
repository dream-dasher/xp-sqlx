use polars::prelude::*;
use proc_macro::ToDataFrame;

// #[derive(ToDataFrame)]
struct MyStruct {
        field1: i32,
        field2: String,
        field3: f64,
}

fn main() {
        // Now you can directly use
        let my_data = vec![MyStruct { field1: 1, field2: "test".to_string(), field3: 3.12534 }];
        // let df = MyStruct::to_dataframe(my_data);
}
