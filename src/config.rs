/// First row containin header data from CAMU
/// Header row is currently row 9
pub(crate) const ENROLLMENT_HEADER_ROW: usize = 9;

/// Last column in CAMU detailed enrollment report
/// Currently Column T which contains email data
pub(crate) const ENROLLMENT_LAST_COLUMN: usize = 20;

pub(crate) const ENROLLMENT_EMAIL_COL_INDEX: usize = 19;

/// Index of column containing Student Id data. i.e. Column B
pub(crate) const ENROLLMENT_STUD_ID_COL_INDEX: usize = 1;

pub(crate) const GRADEBOOK_USELESS_COL_COUNT: usize = 2;
pub(crate) const GRADEBOOK_USELESS_LINE_COUNT: usize = 2;

pub(crate) const GRADEBOOK_NON_GRADE_COL_COUNT: usize = 4;

pub(crate) const GRADEBOOK_CSV_DELIMITER: char = ',';
pub(crate) const GRADEBOOK_EMAIL_COL_INDEX: usize = 3;