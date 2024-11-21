//! `_.sql` files to {file_name, param_list, maybe_output_fields, polars::DataFrame}
#![allow(unused)]
//!
//! There are two ways to do this.  Use of `query*!` macros is direct and gives compile time checking of the query-to-output type validity.
//! *However*, the `query*!` macros consume file name *before* constant definition and ... may pose issues with macro acquisition of file names.
//! This means that scaling such a system to automatically work with any files added, without boilerplate, may be onerous.  
//! TODO: Investigate macro ordering ... maybe there's a nice way of getting file names and generating code that way.
//!
//! An alternate approach is to use the `query*` *functions*, which give more flexibility with variable consumption.
//! However, they do not offer compile time checking.  **But**, because of the automated consumption they should also be ammenable to automated testing.
//!
//! Note: both macro and function queries will yield results -- though with the functions those results are possibly expected to check for a bit more.

use std::path::Path;

use chrono::NaiveDate;
use clap::{Parser, ValueEnum};
use derive_more::{Constructor, Display};
use dialoguer::{Input, Select};
use futures::TryStreamExt;
use include_dir::{Dir, include_dir};
use sqlx::{Arguments,
           Either::*,
           Execute, Executor, FromRow, MySql, Row, Statement,
           mysql::{MySqlArguments, MySqlPoolOptions},
           query::Query};

/// Student to use with `query!`
///
/// More work, and more potential for mistakes
/// but more control than with `query_as!`
#[derive(Debug, Display, FromRow)]
#[display("StudentQ:{} Name: {} {} Born: {}",
          "id.unwrap_or_default()",
          "first_name.clone().unwrap_or_default()",
          "last_name.clone().unwrap_or_default()",
          "dob.map_or(\"N/A\".to_string(), |dob| dob.to_string())")]
struct StudentQA {
    // this is part of FromRow, which query_as! does not use
    // #[sqlx(rename = "StudentID")]
    #[sqlx(rename = "StudentID")]
    id:         Option<i32>,
    #[sqlx(rename = "FirstName")]
    first_name: Option<String>,
    #[sqlx(rename = "LastName")]
    last_name:  Option<String>,
    #[sqlx(rename = "DateOfBirth")]
    dob:        Option<NaiveDate>,
    #[sqlx(rename = "School")]
    school:     Option<String>,
    #[sqlx(rename = "Email")]
    email:      Option<String>,
}

/// Arguments for clap
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// whether to dialogue for query
    #[arg(short, long)]
    interactive_query: bool,
    /// whether to run the static queries
    #[arg(short, long)]
    static_queries:    bool,
    /// whether to display query file inof
    #[arg(short, long)]
    file_info:         bool,
}

// include directory
static SQL_QUERIES: Dir = include_dir!("data/sql_queries");

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // CLAP
    let args = Args::parse();
    // SQLX
    let pool = MySqlPoolOptions::new().max_connections(2)
                                      .connect("mysql://root:root@127.0.0.1/university")
                                      .await?;
    // INCLUDE_DIR
    let file_list = SQL_QUERIES.files();

    if args.static_queries {
        let student_qa_macro: StudentQA =
            sqlx::query_as!(StudentQA,
                            r#"
                                                    SELECT StudentID as id, 
                                                           FirstName as first_name, 
                                                           LastName as last_name, 
                                                           DateOfBirth as dob, 
                                                           School as school, 
                                                           Email as email
                                                    FROM students 
                                                    WHERE StudentID =?
                                                    "#,
                            5).fetch_one(&pool)
                              .await?;
        println!("macro: {}", student_qa_macro);

        // Function
        let student_qa_func: StudentQA =
            sqlx::query_as("SELECT * FROM students WHERE StudentID = ?").bind(5)
                                                                        .fetch_one(&pool)
                                                                        .await?;
        println!("func : {}", student_qa_func);
    }

    if args.file_info {
        println!("file list:");
        let file_vec: Vec<_> =
            file_list.enumerate()
                     .inspect(|(i, f)| println!("     file_{}: '{}'", i, f.path().display()))
                     .map(|(_, f)| f)
                     .collect();
        println!("{:#?}", file_vec);
        println!();

        // get file -> File
        let file1 = SQL_QUERIES.get_file("students_w_id.sql")
                               .expect("file manaually verified to be present");
        // get file path
        let file1_path = file1.path();
        // get file contents
        let file1_str = file1.contents_utf8()
                             .expect("file manaually verified to be utf8");
        println!();
        println!("file1 path    : {}", file1_path.display());
        println!("file1 contents:\n{}", file1_str);
        println!();
    }

    if args.interactive_query {
        let files: Vec<_> = SQL_QUERIES.files().collect();
        let file_paths: Vec<_> = files.iter().map(|f| f.path().display()).collect();
        let selection = Select::new().with_prompt("What do you choose?")
                                     .items(&file_paths)
                                     .interact()
                                     .expect("dialogue to work");
        dbg!(&selection);
        let str_query = files[selection].contents_utf8().expect("utf8 file");
        println!("You chose: {}", file_paths[selection]);
        println!("Contents :\n{}", str_query);

        // get prepared statement
        // get parameters for statement
        let statement = pool.prepare(str_query).await?;
        let params = statement.parameters();
        let cols = statement.columns();
        dbg!(&statement);
        println!("Parameters, if sany: {:?}", params);
        println!("Columns: {:?}", cols);

        // extract num params
        let param_number = match params {
            Some(either) => match either {
                Left(slice) => slice.len(),
                Right(size) => size,
            },
            None => 0,
        };

        let mut arguments = MySqlArguments::default();
        for p in 0..param_number {
            let param: String = Input::new().with_prompt("Enter Parameter:")
                                            .interact_text()
                                            .unwrap();
            arguments.add(param);
        }

        let resp = statement.query_with(arguments).fetch_all(&pool).await?;
        println!("---------------------------");
        println!("Response:\n{:?}", resp);

        // rows.iter()
        //     .enumerate()
        //     .for_each(|(i, r)| println!("row {}: {:?}", i, r));
    }

    if !args.static_queries && !args.file_info && !args.interactive_query {
        println!("No actions selected");
    }
    Ok(())
}
