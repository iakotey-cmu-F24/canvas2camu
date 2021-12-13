use crate::config;
use crate::utils;
use office::{DataType, Excel};
use std::collections::HashMap;

pub(crate) fn parse_enrollment(filename: &str) -> HashMap<String, String> {
    let mut workbook = Excel::open(filename).unwrap();
    let mut data = HashMap::new();

    if let Ok(range) = workbook.worksheet_range("Detailed Enrollment List") {
        let (start, end) = {
            let total_cells = range.get_size().0 * range.get_size().1;
            let last_row = total_cells / config::ENROLLMENT_LAST_COLUMN;

            (
                (config::ENROLLMENT_HEADER_ROW, 0 as usize),
                (last_row, config::ENROLLMENT_LAST_COLUMN),
            )
        };

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
