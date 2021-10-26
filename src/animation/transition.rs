

#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    // List of properties affected by transition
    pub property: String,
    // Duration of the transition
    pub duration: f32,
    // Delay of the transition
    pub delay: f32,
}

impl Transition {
    pub fn new() -> Self {
        Transition {
            property: String::new(),
            duration: 0.0,
            delay: 0.0,
        }
    }
}