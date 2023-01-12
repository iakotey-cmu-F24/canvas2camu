use std::collections::HashMap;
use std::fs::File;
use std::io;

use snafu::prelude::*;

use crate::config;

#[derive(Debug, Snafu)]

pub enum GradebookError {
    #[snafu(display("Unable to open file: {path}"))]
    FileOpenError { source : io::Error, path : String },

    #[snafu(display("Unable to read next line"))]
    FileEmptyError,

    #[snafu(display("Unable to parse header row"))]
    HeaderParseError { source : csv::Error },

    #[snafu(display("Unable to parse entry #{index}"))]
    EntryParseError { index : usize, source : csv::Error },

    #[snafu(display("Unable to retrieve field {field_index} of entry #{entry_index}"))]
    EntryFieldNotFoundError { entry_index : usize, field_index : usize },

    #[snafu(display("Required header 'SIS Login ID' in gradebook"))]
    EmailHeaderNotFoundError,

    #[snafu(display("No Points entry found"))]
    NoPointsError,

    #[doc(hidden)]
    __Nonexhaustive
}

pub(crate) fn parse_gradebook_file(filename : &str) -> Result<config::Gradebook, GradebookError> {

    let file = File::open(filename).context(FileOpenSnafu { path : filename.to_string() })?;

    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut records = rdr.records();

    let mut headers = records.next().context(FileEmptySnafu)?.context(HeaderParseSnafu)?;

    let header_email_index = headers
        .iter()
        .enumerate()
        .filter(|x| x.1.eq_ignore_ascii_case("SIS Login ID"))
        .next()
        .context(EmailHeaderNotFoundSnafu)?
        .0;

    let mut points_row = loop {

        let record = records.next().context(NoPointsSnafu)?.context(HeaderParseSnafu)?;

        if let Some(pat) = record.get(0) {

            if pat.trim().eq_ignore_ascii_case("Points Possible") {

                break record;
            }
        }
    };

    headers.trim();

    points_row.trim();

    let tokens = headers
        .iter()
        .zip(points_row.iter())
        .enumerate()
        .map(|(idx, (course, points))| (idx, course.trim(), points))
        .filter(|(_, _, points)| !points.eq_ignore_ascii_case("Points Possible"))
        .filter(|(_, _, points)| !points.trim().is_empty())
        .filter(|(_, _, points)| !points.eq_ignore_ascii_case("(read only)"));

    let mut course_grades : config::Gradebook = HashMap::new();

    tokens.for_each(|(index, title, _)| {

        course_grades.insert(title.trim().to_string(), (index, HashMap::new()));
    });

    for (idx, record) in records.enumerate() {

        let mut record = record.context(EntryParseSnafu { index : idx + 1 })?;

        record.trim();

        for (_, (index, map)) in course_grades.iter_mut() {

            map.insert(
                record
                    .get(header_email_index)
                    .context(EntryFieldNotFoundSnafu {
                        entry_index : idx + 1,
                        field_index : header_email_index
                    })?
                    .to_string(),
                record
                    .get(*index)
                    .context(EntryFieldNotFoundSnafu {
                        entry_index : idx + 1,
                        field_index : *index
                    })?
                    .to_string()
            );
        }
    }

    Ok(course_grades)
}
