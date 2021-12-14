use crate::config;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub(crate) fn parse_gradebook_file(
    filename: &str,
) -> config::Gradebook {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't open file {}: {}", filename, err),
    };

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut course_grades: config::Gradebook =
        HashMap::new();

    let first_line = lines
        .nth(0)
        .ok_or(())
        .expect("")
        .expect("Could not read from Gradebook");
    let third_line = lines
        .nth(1)
        .ok_or(())
        .expect("")
        .expect("Could not read from Gradebook");

    let tokens = first_line
        .trim()
        .split(",")
        .zip(third_line.trim().split(",")) // add the line that contains marks
        .enumerate() // tag on a counter
        .map(|(index, (title, grade_possible))| {
            (index + 1, title, grade_possible) // Add 1 to index to account for enumerate starting from 0
        })
        .skip(config::GRADEBOOK_NON_GRADE_COL_COUNT) // Skip the nun-grade cols
        .filter(|(_, _, grade_possible)| {
            // use the marks line to separate grades from grade statistics
            !grade_possible.is_empty()
                && !grade_possible.eq_ignore_ascii_case("(read only)")
        });

    tokens.for_each(|(index, title, _)| {
        course_grades.insert(title.trim().to_string(), (index, HashMap::new()));
    });

    lines.for_each(|record| match record {
        Err(why) => panic!("{}", why),
        Ok(line) => {
            let chunks: Vec<&str> =
                line.split(config::GRADEBOOK_CSV_DELIMITER).collect();

            course_grades.iter_mut().for_each(|(_, (index, map))| {
                map.insert(
                    chunks[config::GRADEBOOK_EMAIL_COL_INDEX].to_string(),
                    chunks[*index].to_string(),
                );
            });
        }
    });

    course_grades
}
