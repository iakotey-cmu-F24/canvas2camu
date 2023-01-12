use std::collections::HashMap;

use office::{DataType, Excel};
use snafu::prelude::*;

use crate::config::{self, EnrollmentItem};


#[derive(Debug, Snafu)]


pub enum EnrollmentError {
    #[snafu(display("Unable to open enrollment workbook: {filename}"))]
    FileOpenError {
        source :   office::Error,
        filename : String
    },

    #[snafu(display("Unable to read next line"))]
    FileEmptyError,

    #[snafu(display("Sheet '{expected_sheet}' not found in workbook"))]
    SheetNotFoundError {
        expected_sheet : String,
        source :         office::Error
    },

    #[snafu(display("Expected cell type {expected_type} at cell ({col}, {row})"))]
    CellTypeMismatchError {
        row :           usize,
        col :           usize,
        expected_type : &'static str
    },

    NotEnoughRowsError {
        rows : usize
    },

    MissingColumnError {
        col_name : &'static str
    },

    #[doc(hidden)]
    __Nonexhaustive
}

pub(crate) fn parse_enrollment(filename : &str) -> Result<config::EnrollmentData, EnrollmentError> {

    let mut workbook = Excel::open(filename).context(FileOpenSnafu { filename })?;

    let work_sheet =
        workbook.worksheet_range(config::ENROLLMENT_SHEET_NAME).context(SheetNotFoundSnafu {
            expected_sheet : config::ENROLLMENT_SHEET_NAME.to_string()
        })?;


    let mut row_iter = work_sheet.rows();


    row_iter
        .advance_by(config::ENROLLMENT_USELESS_ROW_COUNT)
        .map_err(|r| NotEnoughRowsSnafu { rows : r }.build())?;


    let header_row = row_iter.next();


    let header_map = header_row
        .context(NotEnoughRowsSnafu { rows : config::ENROLLMENT_USELESS_ROW_COUNT })?
        .iter()
        .enumerate()
        .map(|(idx, cell)| {

            match cell {
                office::DataType::String(inner) => Ok((inner.as_str(), idx)),
                _ => {
                    Err(EnrollmentError::CellTypeMismatchError {
                        col :           idx + 1,
                        row :           config::ENROLLMENT_USELESS_ROW_COUNT + 1,
                        expected_type : "String"
                    })
                },
            }
        })
        .collect::<Result<HashMap<_, _>, _>>()?;


    for name in config::ENROLLMENT_REQUIRED_COLS {

        header_map.get(name).ok_or(EnrollmentError::MissingColumnError { col_name : name })?;
    }


    row_iter
        .enumerate()
        .map(|(idx, row)| -> Result<EnrollmentItem, EnrollmentError> {

            let row_no = config::ENROLLMENT_NON_DATA_ROW_COUNT + idx + 1;

            Ok((
                extract_cell_value!(
                    &row[header_map[config::ENROLLMENT_EMAIL_COL_NAME]],
                    DataType::String
                )
                .context(CellTypeMismatchSnafu {
                    col :           header_map[config::ENROLLMENT_EMAIL_COL_NAME] + 1,
                    row :           row_no,
                    expected_type : "String"
                })?
                .to_string(),
                extract_cell_value!(
                    &row[header_map[config::ENROLLMENT_STUD_NAME_COL_NAME]],
                    DataType::String
                )
                .context(CellTypeMismatchSnafu {
                    col :           header_map[config::ENROLLMENT_STUD_NAME_COL_NAME] + 1,
                    row :           row_no,
                    expected_type : "String"
                })?
                .to_string(),
                extract_cell_value!(
                    &row[header_map[config::ENROLLMENT_STUD_ID_COL_NAME]],
                    DataType::String
                )
                .context(CellTypeMismatchSnafu {
                    col :           header_map[config::ENROLLMENT_STUD_ID_COL_NAME] + 1,
                    row :           row_no,
                    expected_type : "String"
                })?
                .to_string()
            ))
        })
        .collect::<Result<config::EnrollmentData, _>>()
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
