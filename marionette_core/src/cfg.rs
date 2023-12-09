// Purpose: Control flow graph utilities.
// Path: marionette_core\src\cfg.rs

pub mod block;

use block::Block;

pub struct Cfg {
    pub root: Block<u8>
}