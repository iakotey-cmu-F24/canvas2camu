use std::collections::HashMap;


pub(crate) const ENROLLMENT_SHEET_NAME : &str = "Detailed Enrollment List";

/// The number of unnecessary rows in the enrollment file

pub(crate) const ENROLLMENT_USELESS_ROW_COUNT : usize = 8;

/// The number of rows which do not contain student data

pub(crate) const ENROLLMENT_NON_DATA_ROW_COUNT : usize = ENROLLMENT_USELESS_ROW_COUNT + 1;


/// Name of column containing Student Id data. i.e. Column B

pub(crate) const ENROLLMENT_STUD_ID_COL_NAME : &'static str = "Roll No.";

/// Name of column containing student name

pub(crate) const ENROLLMENT_STUD_NAME_COL_NAME : &'static str = "Student Name";

/// Name of column containing student email data

pub(crate) const ENROLLMENT_EMAIL_COL_NAME : &'static str = "Email";

pub(crate) const ENROLLMENT_REQUIRED_COLS : &'static [&'static str] =
    &[ENROLLMENT_STUD_ID_COL_NAME, ENROLLMENT_STUD_NAME_COL_NAME, ENROLLMENT_EMAIL_COL_NAME];


pub(crate) type EnrollmentItem = (String, String, String);

pub(crate) type EnrollmentData = Vec<EnrollmentItem>;

pub(crate) type GradeMap = HashMap<String, String>;

pub(crate) type GradebookEntry = (usize, GradeMap);

pub(crate) type Gradebook = HashMap<String, GradebookEntry>;

pub(crate) const WRITER_SHEET_NAME : &str = "Mark Upload";
