use crate::server::remote_actions::remote_action::{Action, ActionEnum};

pub trait ActionCollectionTrait {
    fn new(new_name: String) -> ActionCollection;
}

pub struct ActionCollection {
    pub actions: Vec<ActionEnum>,
    pub collection_name: String
}

impl ActionCollectionTrait for ActionCollection {
    fn new(new_name: String) -> ActionCollection{
        return ActionCollection { actions: Vec::new(), collection_name: new_name };
    }
}
