// Purpose: Control flow graph block
// Path: marionette_core\src\cfg\block.rs

use bincode::{Decode, Encode};

pub static BLOCK_UID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[derive(Clone)]
pub struct Block<T: Clone + Decode + Encode> {
    pub id: usize,
    pub content: Vec<T>,
    pub left: Option<Box<Block<T>>>,
    pub right: Option<Box<Block<T>>>,
}

impl<T: Clone + Decode + Encode> Block<T> {
    pub fn clear_uid_counter() { BLOCK_UID_COUNTER.store(0, std::sync::atomic::Ordering::SeqCst); }
    
    pub fn new(content: Vec<T>) -> Self {
        let id = BLOCK_UID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Block { id, content, left: None, right: None }
    }
}