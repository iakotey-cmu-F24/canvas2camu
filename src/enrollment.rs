use crate::config;
use crate::utils;
use office::{DataType, Excel};
use std::io;

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum EnrollmentError {
    #[snafu(display("Unable to open enrollment workbook: {filename}"))]
    FileOpenError { source: office::Error, filename: String },

    #[snafu(display("Unable to read next line"))]
    FileEmptyError,

    #[snafu(display("Sheet '{sheet}' not found in workbook"))]
    SheetNotFoundError { sheet: String, source: office::Error },

    #[doc(hidden)]
    __Nonexhaustive,
}


    workbook
        .worksheet_range(config::ENROLLMENT_SHEET_NAME)
        .expect(
            format!(
                "Ensure the workbook has the appropriate sheet named '{}'",
                config::ENROLLMENT_SHEET_NAME
            )
            .as_str(),
        )
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
        })
        .collect::<config::EnrollmentData>()
}
