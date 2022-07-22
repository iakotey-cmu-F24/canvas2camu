use crate::config;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum GradebookError {
    #[snafu(display("Unable to open file: {path}"))]
    FileOpenError { source: io::Error, path: String },

    #[snafu(display("Unable to read next line"))]
    FileEmptyError,

    #[snafu(display("Unable to parse header row"))]
    HeaderParseError { source: csv::Error },

    #[snafu(display("Unable to parse entry #{index}"))]
    EntryParseError { index: usize, source: csv::Error },

    #[snafu(display("Required header 'SIS Login ID' in gradebook"))]
    EmailHeaderNotFoundError,

    #[snafu(display("No Points entry found"))]
    NoPointsError,

    #[doc(hidden)]
    __Nonexhaustive,
}

pub(crate) fn parse_gradebook_file(
    filename: &str,
) -> Result<config::Gradebook, GradebookError> {

    let mut rdr =
        csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut records = rdr.records();

    let headers = records.next().unwrap().unwrap();
    let points_row = records.nth(1).unwrap().unwrap();

    let tokens = headers
        .iter()
        .zip(points_row.iter())
        .enumerate()
        .skip(config::GRADEBOOK_NON_GRADE_COL_COUNT)
        .map(|(idx, (course, points))| (idx, course.trim(), points))
        .filter(|(idx, course, points)| {
            !points.eq_ignore_ascii_case("(read only)")
        });

    let mut course_grades: config::Gradebook = HashMap::new();

    tokens.for_each(|(index, title, _)| {
        course_grades.insert(title.trim().to_string(), (index, HashMap::new()));
    });

    records.for_each(|record| match record {
        Err(why) => panic!("{}", why),
        Ok(line) => {
            course_grades.iter_mut().for_each(|(_, (index, map))| {
                map.insert(
                    line.get(config::GRADEBOOK_EMAIL_COL_INDEX)
                        .unwrap()
                        .trim()
                        .to_string(),
                    line.get(*index).unwrap().trim().to_string(),
                );
            });
        },
    });

    course_grades
}
