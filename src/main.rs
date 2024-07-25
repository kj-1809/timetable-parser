mod json_parser;
mod model;
mod database;
mod helpers;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    // year 1
    // was mutable before
    // let group_sizes: Vec<u32> = vec![8, 8, 5, 5, 5, 5, 5, 5, 5, 5];
    // end
    // let row_start = 3;
    // let row_end = 143;
    // let col_start = 3;
    // let col_end = 114;
    // let worksheet_name = "1ST YEAR A";
    // let output_filename = "classes_data1A.json";
    // let excel_filename = "dataset.xlsx";

    // year 2
    

    let pool = database::init_db().await.unwrap();
    
    let mut classes_data = json_parser::parse_timetable(
        &group_sizes,
        row_start,
        row_end,
        col_start,
        col_end,
        worksheet_name,
        output_filename,
        excel_filename,
    );

    helpers::cleanup(&mut classes_data);

    database::create_groups(&classes_data, &pool).await?;
    database::create_classes(&classes_data, &pool).await?;

    // json_parser::parse_timetable(
    //     &group_sizes,
    //     3,
    //     143,
    //     3,
    //     114,
    //     "1ST YEAR B ",
    //     "classes_data1B.json",
    //     excel_filename,
    // )
    Ok(()) 
}


