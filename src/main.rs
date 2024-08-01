mod database;
mod helpers;
mod json_parser;
mod model;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // year 1 - A

    // let group_sizes: Vec<u32> = vec![8, 8, 8, 5, 5, 5, 5, 5, 4];
    // let row_start = 5;
    // let row_end = 145;
    // let col_start = 4;
    // let col_end = 109;
    // let worksheet_name = "1ST YEAR A";
    // let excel_filename = "dataset.xlsx";

    // year 1 - B

    // let row_start = 7;
    // let row_end = 147;
    // let col_start = 4;
    // gotta make 3 sections for it to work
    // let col_end = ;

    // let worksheet_name = "1ST YEAR B";
    // let excel_filename = "dataset.xlsx";
    // let group_sizes: Vec<u32> = vec![2, 2, 3, 3, 2, 1, 4, 1, 3, 3];

    // YEAR 2 - B - COE

    // let group_sizes: Vec<u32> = vec![8, 5, 5, 5, 5, 5, 5];
    // let row_start = 6;
    // let row_end = 145;
    // let col_start = 4;
    // let col_end = 79;
    // let worksheet_name = "2ND YEAR B";
    // let excel_filename = "dataset.xlsx";

    // YEAR 2 - B - CSE

    // let group_sizes: Vec<u32> = vec![6, 6];
    // let row_start = 6;
    // let row_end = 145;
    // let col_start = 88;
    // let col_end = 111;
    // let worksheet_name = "2ND YEAR B";
    // let excel_filename = "dataset.xlsx";

    // YEAR - 2 - A COBS

    // let group_sizes: Vec<u32> = vec![3];
    // let row_start = 6;
    // let row_end = 145;
    // let col_start = 74;
    // let col_end = 79;
    // let worksheet_name = "2ND YEAR A";
    // let excel_filename = "dataset.xlsx";

    // YEAR - 2 - B ENC

    let group_sizes: Vec<u32> = vec![4, 4, 4];
    let row_start = 6;
    let row_end = 145;
    let col_start = 142;
    let col_end = 165;
    let worksheet_name = "2ND YEAR B";
    let excel_filename = "dataset.xlsx";

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
