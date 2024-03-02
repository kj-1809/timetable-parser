use calamine::{open_workbook, DataType, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Write, vec};

// let mut cur = 0;
// let mut batches: Vec<Vec<u32>> = Vec::new();
// for i in &group_sizes {
//     batches.push(vec![cur, cur + (i * 2) - 1]);
//     cur += i * 2;
// }
// for i in batches {
//     println!("{:?}", i);
// }

#[derive(Serialize, Deserialize, Debug)]
struct Class {
    name: String,
    location: String,
    professor: String,
    slot: u32,
}

impl Clone for Class {
    fn clone(&self) -> Self {
        Class {
            name: self.name.clone(),
            location: self.location.clone(),
            professor: self.professor.clone(),
            slot: self.slot.clone(),
        }
    }
}

fn main() {
    let mut matrix: Vec<Vec<String>> = Vec::new();
    let mut workbook: Xlsx<_> = open_workbook("dataset.xlsx").unwrap();
    let mut classes_data: HashMap<String, HashMap<String, Vec<Class>>> = HashMap::new();
    let group_sizes: Vec<u32> = vec![8, 8, 5, 5, 5, 5, 5, 5, 5, 5];
    let mut groups: Vec<Vec<String>> = Vec::new();
    let get_day_string: HashMap<u32, String> = HashMap::from([
        (0, "Monday".to_string()),
        (1, "Tuesday".to_string()),
        (2, "Wednesday".to_string()),
        (3, "Thursday".to_string()),
        (4, "Friday".to_string()),
    ]);

    if let Ok(range) = workbook.worksheet_range("1ST YEAR A") {
        for (i, row) in range.rows().enumerate() {
            let mut temp: Vec<String> = Vec::new();
            if i <= 2 || i >= 144 {
                continue;
            }
            for (j, ele) in row.iter().enumerate() {
                if j <= 2 || j >= 115 {
                    continue;
                }
                if let Some(value) = ele.as_string() {
                    temp.push(value.trim().to_string());
                } else {
                    temp.push("0".to_string())
                }
            }
            matrix.push(temp);
        }
    }
    let n = matrix.len();
    let m = matrix[0].len();
    // N x M = 141 x 112

    // get the groups
    let mut pos = 0;
    for i in &group_sizes {
        let mut group: Vec<String> = Vec::new();
        for _ in 0..i.clone() {
            group.push(matrix[0][pos].clone());
            pos += 2;
        }
        groups.push(group);
    }

    // initialize empty hashmaps for all groups
    for i in &groups {
        for ele in i {
            let mut hashmap_with_data: HashMap<String, Vec<Class>> = HashMap::new();
            for day in 0..5 {
                hashmap_with_data.insert(get_day_string.get(&day).unwrap().to_string(), Vec::new());
            }
            classes_data.insert(ele.clone(), hashmap_with_data);
        }
    }

    // real deal
    let mut current_group = 0;
    let mut group_size_left = group_sizes[current_group];

    for j in (0..m).step_by(2) {
        let mut row = 1;
        let mut classes_for_the_subgroup: HashMap<String, Vec<Class>> = HashMap::new();

        for day in 0..5 {
            classes_for_the_subgroup
                .insert(get_day_string.get(&day).unwrap().to_string(), Vec::new());
        }

        while row < n {
            let day: u32 = (row as u32 - 1) / 28;
            let day_string = get_day_string.get(&day).unwrap().to_string();
            let adjusted_row = (row as u32 - 1) % 28;

            if matrix[row][j] != "0" {
                let class_type = matrix[row][j].chars().rev().next().unwrap();
                let classes_for_the_day = classes_for_the_subgroup.get_mut(&day_string).unwrap();

                println!("class_type  : {}", class_type);

                if class_type == 'T' {
                    if matrix[row + 1][j + 1] != "0" {
                        classes_for_the_day.push(Class {
                            name: matrix[row][j].clone(),
                            location: matrix[row + 1][j].clone(),
                            professor: matrix[row + 1][j + 1].clone(),
                            slot: (adjusted_row) / 2,
                        });
                        row += 2;
                    } else {
                        classes_for_the_day.push(Class {
                            name: matrix[row][j].clone(),
                            location: matrix[row + 1][j].clone(),
                            professor: matrix[row + 2][j].clone(),
                            slot: (adjusted_row) / 2,
                        });
                        classes_for_the_day.push(Class {
                            name: matrix[row][j].clone(),
                            location: matrix[row + 1][j].clone(),
                            professor: matrix[row + 2][j].clone(),
                            slot: ((adjusted_row) / 2) + 1,
                        });
                        row += 4;
                    }
                } else if class_type == 'P' {
                    if matrix[row + 3][j] == "0" {
                        classes_for_the_day.push(Class {
                            name: matrix[row][j].clone(),
                            location: matrix[row + 1][j].clone(),
                            professor: matrix[row + 2][j].clone(),
                            slot: (adjusted_row) / 2,
                        });
                        classes_for_the_day.push(Class {
                            name: matrix[row][j].clone(),
                            location: matrix[row + 1][j].clone(),
                            professor: matrix[row + 2][j].clone(),
                            slot: ((adjusted_row) / 2) + 1,
                        })
                    } else {
                        classes_for_the_day.push(Class {
                            name: matrix[row][j].clone(),
                            location: format!(
                                "{} {}",
                                matrix[row + 1][j].clone(),
                                matrix[row + 2][j].clone()
                            ),
                            professor: matrix[row + 3][j].clone(),
                            slot: (adjusted_row) / 2,
                        });
                        classes_for_the_day.push(Class {
                            name: matrix[row][j].clone(),
                            location: format!(
                                "{} {}",
                                matrix[row + 1][j].clone(),
                                matrix[row + 2][j].clone()
                            ),
                            professor: matrix[row + 3][j].clone(),
                            slot: ((adjusted_row) / 2) + 1,
                        })
                    }
                    row += 4;
                } else if class_type == 'L' {
                    for sub_group in &groups[current_group] {
                        let classes_for_subgroup =
                            classes_data.get_mut(&sub_group.clone()).unwrap();
                        let classes_for_subgroup_day =
                            classes_for_subgroup.get_mut(&day_string).unwrap();
                        classes_for_subgroup_day.push(Class {
                            name: matrix[row][j].clone(),
                            location: matrix[row + 1][j].clone(),
                            professor: matrix[row + 1]
                                [j + (2 * group_sizes[current_group] as usize) - 1]
                                .clone(),
                            slot: ((adjusted_row) / 2),
                        });
                    }
                    row += 2;
                } else if class_type == 'F' {
                    for sub_group in &groups[current_group] {
                        let classes_for_subgroup =
                            classes_data.get_mut(&sub_group.clone()).unwrap();
                        let classes_for_subgroup_day =
                            classes_for_subgroup.get_mut(&day_string).unwrap();

                        let mut new_name = matrix[row][j].clone();
                        new_name.pop();
                        new_name.push('P');

                        classes_for_subgroup_day.push(Class {
                            name: new_name.clone(),
                            location: matrix[row + 1][j].clone(),
                            professor: matrix[row + 2][j].clone(),
                            slot: ((adjusted_row) / 2),
                        });
                        classes_for_subgroup_day.push(Class {
                            name: new_name.clone(),
                            location: matrix[row + 1][j].clone(),
                            professor: matrix[row + 2][j].clone(),
                            slot: ((adjusted_row) / 2) + 1,
                        });
                    }
                    row += 4;
                } else {
                    println!("row {} matrix : {}", row, matrix[row][j]);
                    std::panic!("Shit data !")
                }
            } else {
                row += 2;
            }
        }
        // row iteration ends

        group_size_left -= 1;
        if group_size_left <= 0 {
            current_group += 1;
            if current_group < group_sizes.len() {
                group_size_left = group_sizes[current_group];
            }
        }

        //merge the old data with the newly found data

        let classes_for_subgroup = classes_data.get_mut(&matrix[0][j]).unwrap();

        for day_num in 0..5 {
            let day_num_str = get_day_string.get(&day_num).unwrap().to_string();

            let classes_for_subgroup_day = classes_for_subgroup.get_mut(&day_num_str).unwrap();
            let classes_for_the_subgroup_day = classes_for_the_subgroup.get(&day_num_str).unwrap();

            classes_for_subgroup_day.extend(classes_for_the_subgroup_day.clone());
        }
    }

    let parsed_classes_data = serde_json::to_string(&classes_data).unwrap();

    let mut file = File::create("classes_data.json").expect("Failed to open file");
    file.write(parsed_classes_data.as_bytes())
        .expect("error writing to the file");
}
