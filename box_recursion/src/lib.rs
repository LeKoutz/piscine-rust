#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self {
            grade: None
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let worker = Worker{
            role: role.into(),
            name: name.to_string(),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(worker))
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let last_worker = self.grade.take()?;
        self.grade = last_worker.next;
        Some(last_worker.name)
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        let last_worker = self.grade.as_ref()?;
        let role = match last_worker.role {
            Role::CEO => Role::CEO,
            Role::Manager => Role::Manager,
            _ => Role::Worker,
        };
        Some((last_worker.name.clone(), role))
    }
}
