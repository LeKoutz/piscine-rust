use chrono::Utc;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-m-d %H:%M:%S").to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            Err(FormError { 
                form_values: ("name", self.name.clone()), 
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), 
                err: "Username is empty" })
        } else if self.password.len() < 8 {
            Err(FormError { 
                form_values: ("password", self.password.clone()), 
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), 
                err: "Password should be at least 8 characters long" })
        } else if !self.password.contains(|x: char| x.is_ascii_digit()) ||
            !self.password.contains(|x: char| x.is_ascii_alphabetic()) ||
            !self.password.contains(|x: char| x.is_ascii_punctuation()) {
            Err(FormError {
                form_values: ("password", self.password.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be a combination of ASCII numbers, letters and symbols",
            })
        } else {
            Ok(())
        }
    }
}
