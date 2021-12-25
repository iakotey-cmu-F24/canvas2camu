use crate::config;
use crate::utils;
use office::{DataType, Excel};
use std::collections::HashMap;

pub(crate) fn parse_enrollment(filename: &str) -> config::EnrollmentData {
    let mut workbook = Excel::open(filename).expect(&format!("Unable to open enrollment workbook: {}", filename));
    let mut data = HashMap::new();

    if let Ok(range) = workbook.worksheet_range("Detailed Enrollment List") {

        for row in range.rows().skip(config::ENROLLMENT_HEADER_ROW) {
            data.insert(
                utils::cast!(
                    row[config::ENROLLMENT_EMAIL_COL_INDEX].to_owned(),
                    DataType::String
                ),
                utils::cast!(
                    row[config::ENROLLMENT_STUD_ID_COL_INDEX].to_owned(),
                    DataType::String
                ),
            );
        }
    }

    data
}
