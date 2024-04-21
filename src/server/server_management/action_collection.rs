use crate::server::remote_actions::remote_action::{Action, ActionEnum};

pub trait ActionCollectionTrait {
    fn new() -> ActionCollection;
}

pub struct ActionCollection {
    pub actions: Vec<ActionEnum>,
}

impl ActionCollectionTrait for ActionCollection {
    fn new() -> ActionCollection{
        return ActionCollection { actions: Vec::new() };
    }
}
