use crate::{
    domain::Greeting,
    ports::GreetingRepository,
};

pub struct GreetingService<R> {
    repository: R,
}

impl<R: GreetingRepository> GreetingService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn greet(&self, name: &str) -> Greeting {
        self.repository.save(Greeting::for_name(name))
    }
}

