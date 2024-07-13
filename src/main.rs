mod json_parser;
mod model;

fn main() {
    let mut group_sizes: Vec<u32> = vec![8, 8, 5, 5, 5, 5, 5, 5, 5, 5];
    let row_start = 3;
    let row_end = 143;
    let col_start = 3;
    let col_end = 114;
    let worksheet_name = "1ST YEAR A";
    let output_filename = "classes_data1A.json";
    let excel_filename = "dataset.xlsx";

    // json_parser::parse_timetable(
    //     &group_sizes,
    //     row_start,
    //     row_end,
    //     col_start,
    //     col_end,
    //     worksheet_name,
    //     output_filename,
    //     excel_filename,
    // );

    json_parser::parse_timetable(
        &group_sizes,
        3,
        143,
        3,
        114,
        "1ST YEAR B ",
        "classes_data1B.json",
        excel_filename,
    )
}


