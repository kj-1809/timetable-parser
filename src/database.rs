use crate::model::Class;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::collections::{HashMap, HashSet};

#[derive(sqlx::FromRow, Debug)]
pub struct GroupToCourses {
    pub group_name: String,
    pub course_name: String,
}

pub async fn init_db() -> Result<Pool<MySql>, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("Database url must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    return Ok(pool);
}

pub async fn create_groups(
    classes_data: &HashMap<String, HashMap<String, Vec<Class>>>,
    pool: &Pool<MySql>,
) -> Result<(), sqlx::Error> {
    let mut courses: HashSet<String> = HashSet::new();
    let mut group_names: HashSet<String> = HashSet::new();
    let mut group_to_courses: Vec<GroupToCourses> = Vec::new();

    for (group_name, timetable_for_the_group) in classes_data.iter() {
        group_names.insert(group_name.to_string());
        let mut set: HashSet<String> = HashSet::new();
        for (_, classes) in timetable_for_the_group.iter() {
            for class in classes {
                set.insert(class.name[0..6].to_string());
                courses.insert(class.name[0..6].to_string());
            }
        }

        for course in set.iter() {
            group_to_courses.push(GroupToCourses {
                group_name: group_name.to_string(),
                course_name: course.to_string(),
            });
        }
    }

    for course in courses.iter() {
        let query_string = format!("INSERT INTO course(Name) values (\"{}\")", course);
        println!("{}", query_string);
        sqlx::query(&query_string).execute(pool).await?;
    }

    for group in group_names.iter() {
        let query_string = format!("INSERT INTO class_group(Name) values (\"{}\")", group);
        println!("{}", query_string);
        sqlx::query(&query_string).execute(pool).await?;
    }

    for x in group_to_courses {
        let query_string = format!(
            "INSERT INTO group_to_courses values (\"{}\", \"{}\")",
            x.group_name, x.course_name
        );
        println!("{}", query_string);
        sqlx::query(&query_string).execute(pool).await?;
    }

    return Ok(());
}

pub async fn create_classes(
    classes_data: &HashMap<String, HashMap<String, Vec<Class>>>,
    pool: &Pool<MySql>,
) -> Result<(), sqlx::Error> {
    for (group_name, timetable) in classes_data.iter() {
        for (day, classes) in timetable.iter() {
            for class in classes.iter() {
                let query_string = format!(
                    r#"INSERT INTO detail_class(course_name, location, professor,type,  group_name, slot, day) values ("{}", "{}", "{}", "{}", "{}", "{}", "{}")"#,
                    class.name[0..6].to_string(),
                    class.location,
                    class.professor,
                    class.name.chars().nth(6).unwrap(),
                    group_name,
                    class.slot,
                    day
                );
                println!("{}", query_string);
                sqlx::query(&query_string).execute(pool).await?;
            }
        }
    }

    return Ok(());
}
