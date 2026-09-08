pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> &str {
    &student.1
}

pub fn last_name(student: &Student) -> &str {
    &student.2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_works() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let result = id(&student);
        assert_eq!(result, 20);
    }

    #[test]
    fn first_name_works() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let result = first_name(&student);
        assert_eq!(result, "Pedro");
    }

    #[test]
    fn last_name_works() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let result = last_name(&student);
        assert_eq!(result, "Domingos");
    }
}
