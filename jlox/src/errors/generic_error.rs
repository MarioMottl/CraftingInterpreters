use crate::errors::generic_report::report;

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}
