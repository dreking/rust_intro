// student* - *course

struct Student {
    name: String,
}

impl Student {
    fn courses(&self, platofmr: Platform) {
        for enrollment in platofmr.enrollments {
            if enrollment.student.name == self.name {
                println!("{} is enrolled in {}", self.name, enrollment.course.name);
            }
        }
    }
}

struct Course {
    name: String,
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment {
            student: student,
            course: course,
        }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new(),
        }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course));
    }
}

pub fn circular_reference() {
    let mut platform = Platform::new();

    let student = Student {
        name: "John".to_string(),
    };
    let course = Course {
        name: "Rust".to_string(),
    };

    platform.enroll(&student, &course);

    student.courses(platform);
}
