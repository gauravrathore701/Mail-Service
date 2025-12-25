#[derive(Clone)]
pub struct GreetingService;

impl GreetingService {
    pub fn new() -> Self {
        Self
    }

    pub fn say_hello(&self) -> String {
        "Mailing Service Activated...".to_string()
    }
}
