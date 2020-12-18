// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute 'rustlings hint generics3' for hints!

// I AM DONE

use std::fmt;
use fmt::Display;
use fmt::Formatter;

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

//// Answer 1
//impl Display for ReportCard<f32> {
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//        write!(f, "{} ({}) - achieved a grade of {}",
//            &self.student_name, &self.student_age, &self.grade)
//    }
//}
//
//impl ReportCard<f32> {
//    pub fn print(&self) -> String {
//        format!("{}", &self)
//    }
//}
//
//impl Display for ReportCard<String> {
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//        write!(f, "{} ({}) - achieved a grade of {}",
//            &self.student_name, &self.student_age, &self.grade)
//    }
//}
//
//impl ReportCard<String> {
//    pub fn print(&self) -> String {
//        format!("{}", &self)
//    }
//}

// Answer 2
impl<T: Display> Display for ReportCard<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

impl<T: Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{}", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard::<f32> {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard::<String> {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
