use dirs::home_dir;
use std::path::Path;

use crate::{config, utils::with_temp_dir};
use simple_excel_writer::{blank, row, CellValue, Column, Row, Workbook};

pub(crate) fn create_file(
    filename: &str, grades: &config::GradeMap,
    enrollment: &config::EnrollmentData,
) {
    let mut workbook = Workbook::create(&format!("{}.xlsx", filename));

    let mut sheet = workbook.create_sheet(config::WRITER_SHEET_NAME);

    // set column width
    sheet.add_column(Column { width: 15.0 });
    sheet.add_column(Column { width: 15.0 });
    sheet.add_column(Column { width: 15.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 15.0 });
    sheet.add_column(Column { width: 15.0 });

    workbook
        .write_sheet(&mut sheet, |sheet_writer| {
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

            enrollment.iter().for_each(|(email, name, student_id)| {

                let current_grade = grades[email].as_str();
                let (current_grade, current_status) =
                    match grades[email].as_str() {
                        "EX" | "N/A" | "" => ("0.00", "Y"),
                        _ => (current_grade, "N"),
                    };
                
                sheet_writer
                    .append_row(row![
                        student_id.as_str(),
                        current_grade,
                        current_status,
                        name.as_str(),
                        blank!(2)
                    ])
                    .expect("Unable to write to file");
            });

            sheet_writer.append_row(row![blank!(6)])
        })
        .expect("Unable to write to file");

    workbook.close().expect("close excel error!");
}

pub(crate) fn create_files(
    output_dir: &str, gradebook: &config::Gradebook,
    enrollment: &config::EnrollmentData,
) {
    with_temp_dir!(output_dir, {
        gradebook.keys().for_each(|grade| {
            create_file(grade, &gradebook[grade].1, enrollment)
        });
    })
}
