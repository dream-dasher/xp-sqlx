use polars::prelude::*;

#[derive(Debug)]
struct Record {
        id:     i32,
        name:   String,
        active: bool,
}

fn main() {
        // Create instances of the struct
        let records = vec![Record { id: 1, name: "Alice".into(), active: true }, Record {
                id:     2,
                name:   "Bob".into(),
                active: false,
        }];

        // Convert struct instances to Series
        let id_series = Series::new("a", records.iter().map(|r| r.id).collect::<Vec<_>>());
        let name_series = Series::new("b", records.iter().map(|r| &r.name).cloned().collect::<Vec<_>>());
        let active_series = Series::new("c", records.iter().map(|r| r.active).collect::<Vec<_>>());

        // Create a DataFrame
        let df = DataFrame::new(vec![id_series, name_series, active_series]).unwrap();
        println!("{:?}", df);
}
