#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Student(pub u32, pub String, pub String);


pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> &str {
    student.1.as_str()
}

pub fn last_name(student: &Student) -> &str {
    student.2.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_id() {
        let student = Student(1, "victor".to_string(), "Arony".to_string());
        assert_eq!(id(&student), 1);
        assert_eq!(first_name(&student), "victor");
        assert_eq!(last_name(&student), "Arony");
    }

   
}
