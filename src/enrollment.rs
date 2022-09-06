use crate::config;
use office::{DataType, Excel};

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum EnrollmentError {
    #[snafu(display("Unable to open enrollment workbook: {filename}"))]
    FileOpenError { source: office::Error, filename: String },

    #[snafu(display("Unable to read next line"))]
    FileEmptyError,

    #[snafu(display("Sheet '{sheet}' not found in workbook"))]
    SheetNotFoundError { sheet: String, source: office::Error },

    #[snafu(display(
        "Expected cell type {expected_type} at cell ({col}, {row})"
    ))]
    CellTypeMismatchError { row: usize, col: usize, expected_type: &'static str },

    #[doc(hidden)]
    __Nonexhaustive,
}

pub(crate) fn parse_enrollment(
    filename: &str,
) -> Result<config::EnrollmentData, EnrollmentError> {
    let mut workbook =
        Excel::open(filename).context(FileOpenSnafu { filename })?;

    Ok(workbook
        .worksheet_range(config::ENROLLMENT_SHEET_NAME)
        .context(SheetNotFoundSnafu {
            sheet: config::ENROLLMENT_SHEET_NAME.to_string(),
        })?
        .rows()
        .enumerate()
        .skip(config::ENROLLMENT_HEADER_ROW)
        .map(
            |(idx, row)| -> Result<(String, String, String), EnrollmentError> {
                Ok((
                    extract_cell_value!(
                        &row[config::ENROLLMENT_EMAIL_COL_INDEX],
                        DataType::String
                    )
                    .context(CellTypeMismatchSnafu {
                        col: config::ENROLLMENT_EMAIL_COL_INDEX + 1,
                        row: idx + 1,
                        expected_type: "String",
                    })?.to_string(),
                    extract_cell_value!(
                        &row[config::ENROLLMENT_EMAIL_COL_INDEX],
                        DataType::String
                    )
                    .context(CellTypeMismatchSnafu {
                        col: config::ENROLLMENT_EMAIL_COL_INDEX + 1,
                        row: idx + 1,
                        expected_type: "String",
                    })?.to_string(),
                    extract_cell_value!(
                        &row[config::ENROLLMENT_STUD_NAME_COL_INDEX],
                        DataType::String
                    )
                    .context(CellTypeMismatchSnafu {
                        col: config::ENROLLMENT_STUD_ID_COL_INDEX + 1,
                        row: idx + 1,
                        expected_type: "String",
                    })?.to_string(),
                ))
            },
        )
        .flatten()
        .collect::<config::EnrollmentData>())
}

macro_rules! extract_cell_value {
    ($expression: expr, $target_type: path) => {
        if let $target_type(a) = $expression {
            Some(a)
        } else {
            None
        }
    };
}
use extract_cell_value;
