use std::sync::{Arc, Mutex};

use crate::{domain::Greeting, ports::GreetingRepository};

#[derive(Clone, Default)]
pub struct InMemoryGreetingRepository {
    greetings: Arc<Mutex<Vec<Greeting>>>,
}

impl GreetingRepository for InMemoryGreetingRepository {
    fn save(&self, greeting: Greeting) -> Greeting {
        self.greetings
            .lock()
            .expect("greeting repository mutex poisoned")
            .push(greeting.clone());
        greeting
    }
}

