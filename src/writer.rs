use dirs::home_dir;
use std::{io::Error, path::Path};

use crate::{config, utils::with_temp_dir};
use simple_excel_writer::{blank, row, CellValue, Column, Row, Workbook};

fn create_file(
    filename: &str, grades: &config::GradeMap,
    enrollment: &config::EnrollmentData,
) -> Result<(), Error> {
    let mut workbook = Workbook::create(&format!("{}.xlsx", filename));

    let mut sheet = workbook.create_sheet(config::WRITER_SHEET_NAME);

    // set column width
    sheet.add_column(Column { width: 15.0 });
    sheet.add_column(Column { width: 15.0 });
    sheet.add_column(Column { width: 15.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 15.0 });
    sheet.add_column(Column { width: 15.0 });

    // Write header rows to file
    workbook.write_sheet(&mut sheet, |sheet_writer| {
        sheet_writer.append_row(row![
            "StuRollNo",
            "Mark",
            "IsAbs",
            "StuNm",
            "InEligible",
            "rsSts"
        ])?;
        sheet_writer.append_row(row![
            "Roll No",
            "Marks",
            "Is Absent",
            "Student Name",
            "InEligible",
            "Result Status"
        ])?;

        for (email, name, student_id) in enrollment.iter() {
            let current_grade = grades[email].as_str();
            let (current_grade, current_status) = match grades[email].as_str() {
                "EX" => ("0.00", "Y"),
                "N/A" | "" => ("0.00", "N"),
                _ => (current_grade, "N"),
            };

            sheet_writer.append_row(row![
                student_id.as_str(),
                current_grade,
                current_status,
                name.as_str(),
                blank!(2)
            ])?;
        }

        sheet_writer.append_row(row![blank!(6)])?;

        Ok(())
    })?;

    workbook.close().map(|_result| ())
}

/// Creates CAMU-compatible excel files from the class gradebook and enrollment data
///
/// # Arguments
///
/// * `output_dir` - A string slice that holds the output directory
/// * `gradebook` - A reference to a Gradebook struct
/// * `enrollment` - A reference to an EnrollmentData struct
///
pub(crate) fn create_files(
    output_dir: &str, gradebook: &config::Gradebook,
    enrollment: &config::EnrollmentData,
) -> Result<(), Error> {
    with_temp_dir!(output_dir, {
        for grade in gradebook.keys() {
            create_file(grade, &gradebook[grade].1, enrollment)?;
        }
    });

    Ok(())
}
