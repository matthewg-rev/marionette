use crate::states::explorer::ExplorerState;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct SelectorState {
    pub selected_path: String,
    pub explorer_state: ExplorerState,
}

impl SelectorState {
    pub fn new() -> Self {
        Self {
            selected_path: String::new(),
            explorer_state: ExplorerState::new()
        }
    }
}

impl Debug for SelectorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectorState")
            .field("selected_path", &self.selected_path)
            .field("explorer_state", &self.explorer_state)
            .finish()
    }
}