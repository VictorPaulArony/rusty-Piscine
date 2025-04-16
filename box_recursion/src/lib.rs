#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    // Initializes WorkEnvironment with grade set to None
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    // Adds a worker at the start of the list
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker {
            role,
            name,
            next: self.grade.take(),
        });
        self.grade = Some(new_worker);
    }

    // Removes the last added worker and returns their name
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|worker| {
            self.grade = worker.next;
            worker.name
        })
    }

    // Returns the name and role of the last added worker
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| (worker.name.clone(), worker.role.clone()))
    }
}