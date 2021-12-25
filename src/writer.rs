use dirs::home_dir;
use std::path::Path;

use crate::config;
use simple_excel_writer::{blank, row, CellValue, Column, Row, Workbook};

pub(crate) fn create_file(
    filename: &str, grades: &config::GradeMap,
    enrollment: config::EnrollmentData,
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

            grades.iter().for_each(|(email, grade)| {
                sheet_writer
                    .append_row(row![
                        &*enrollment[email],
                        match &**grade {
                            "EX" | "N/A" | "" => "0.00",
                            _ => &**grade,
                        },
                        match &**grade {
                            "EX" | "N/A" | "" => "N",
                            _ => "Y",
                        },
                        blank!(2)
                    ])
                    .expect("Unable to write to file");
            });

            sheet_writer.append_row(row![blank!(6)])
        })
        .expect("Unable to write to file");

    workbook.close().expect("close excel error!");
}
