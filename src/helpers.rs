use crate::model::Class;
use std::collections::HashMap;

pub fn cleanup(classes_data: &mut HashMap<String, HashMap<String, Vec<Class>>>) {
    
    let mut new_classes_data : HashMap<String, HashMap<String, Vec<Class>>> = HashMap::new();

    for (group_name, timetable) in classes_data.iter_mut() {
        let new_group_name = group_name.replace(char::is_whitespace, "");
        for (_day, classes) in timetable.iter_mut() {
            for class in classes {
                class.name = class.name.replace(char::is_whitespace, "");
            }
        }
        new_classes_data.insert(new_group_name, timetable.clone());
    }

    *classes_data = new_classes_data;
}

