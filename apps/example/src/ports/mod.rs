use crate::domain::Greeting;

pub trait GreetingRepository: Send + Sync + 'static {
    fn save(&self, greeting: Greeting) -> Greeting;
}

