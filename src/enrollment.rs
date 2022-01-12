use crate::config;
use crate::utils;
use office::{DataType, Excel};

pub(crate) fn parse_enrollment(filename: &str) -> config::EnrollmentData {
    let mut workbook = Excel::open(filename)
        .expect(&format!("Unable to open enrollment workbook: {}", filename));

    workbook
        .worksheet_range(config::ENROLLMENT_SHEET_NAME).unwrap()
        .rows()
        .skip(config::ENROLLMENT_HEADER_ROW)
        .map(|row| {
            (
                utils::cast!(
                    row[config::ENROLLMENT_EMAIL_COL_INDEX].to_owned(),
                    DataType::String
                ),
                utils::cast!(
                    row[config::ENROLLMENT_STUD_NAME_COL_INDEX].to_owned(),
                    DataType::String
                ),
                utils::cast!(
                    row[config::ENROLLMENT_STUD_ID_COL_INDEX].to_owned(),
                    DataType::String
                ),
            )
        }).collect::<config::EnrollmentData>()

}
