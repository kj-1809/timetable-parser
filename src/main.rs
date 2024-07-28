mod database;
mod helpers;
mod json_parser;
mod model;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // year 1

    let group_sizes: Vec<u32> = vec![8, 8, 8, 5, 5, 5, 5, 5, 4];
    let row_start = 5;
    let row_end = 145;
    let col_start = 4;
    let col_end = 109;
    let worksheet_name = "1ST YEAR A";
    let excel_filename = "dataset.xlsx";

    // year 2

    // let row_start = 2;
    // let row_end = 142;
    // let col_start = 3;
    // let col_end = 50;
    // let worksheet_name = " 2ND YEAR A";
    // let excel_filename = "dataset.xlsx";
    // let group_sizes: Vec<u32> = vec![2, 2, 3, 3, 2, 1, 4, 1, 3, 3];

    let mut classes_data = json_parser::parse_timetable(
        &group_sizes,
        row_start,
        row_end,
        col_start,
        col_end,
        worksheet_name,
        excel_filename,
    );

    let pool = database::init_db().await.unwrap();

    helpers::cleanup(&mut classes_data);

    database::create_groups(&classes_data, &pool).await?;
    database::create_classes(&classes_data, &pool).await?;

    Ok(())
}
