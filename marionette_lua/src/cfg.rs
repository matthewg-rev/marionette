use marionette_core::assembly::Data;
use std::{collections::{HashMap, HashSet}, fmt::{self, Debug}};
use petgraph::{dot::{Config, Dot}, graph::{Graph, NodeIndex}, prelude::StableDiGraph, visit::EdgeRef};
use lazy_static::*;
use crate::lua_binary::*;

lazy_static! {
    static ref BRANCHING_OPCODES: Vec<u8> = vec![
        22, 31, 32, // JMP, FORLOOP, FORPREP
        33, 26, 27, // TFORLOOP, TEST, TESTSET
        23, 24, 25, 2 // EQ, LT, LE, LOADBOOL
    ];

    static ref CONDITIONAL_OPCODES: Vec<u8> = vec![
        23, 24, 25, // EQ, LT, LE
        26, 27, 33 // TEST, TESTSET, TFORLOOP
    ];

    static ref RETURNING_OPCODES: Vec<u8> = vec![
        30, 29 // RETURN, TAILCALL
    ];

    static ref NONCONDITIONAL_BRANCHING_OPCODES: Vec<u8> = vec![
        22, 32 // JMP, FORPREP
    ];

    static ref BACKWARDS_CONDITIONAL_OPCODES: Vec<u8> = vec![
        31, // FORLOOP
    ];

    static ref POSSIBLE_SKIP_OPCODES: Vec<u8> = vec![
        2, // LOADBOOL
    ];
}

#[derive(Default)]
pub struct Block<T: Debug> {
    pub id: usize,
    pub instructions: Vec<T>,
    pub trivia: Option<Vec<String>>,
}

impl<T: Debug> Block<T> {
    pub fn new(id: usize) -> Self {
        Block {
            id,
            instructions: Vec::new(),
            trivia: None,
        }
    }

    pub fn add_instruction(&mut self, instr: T) {
        self.instructions.push(instr);
    }
}

impl<T: Debug> Debug for Block<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {:?}\n", self.id);

        for instr in &self.instructions {
            write!(f, "{:?}\n", instr);
        }

        Ok(())
    }
}

pub fn build_control_flow_graph<T, F1, F2, F3>(
    instructions: &Vec<T>,
    is_branching: F1,
    branch_targets: F2,
    is_exiting: F3,
) -> (StableDiGraph<Block<T>, ()>, Option<NodeIndex>)
where
    T: Clone + std::fmt::Debug,
    F1: Fn(&T) -> bool,
    F2: Fn(&T) -> Vec<isize>,
    F3: Fn(&T) -> bool,
{

    if instructions.is_empty() {
        return (StableDiGraph::new(), None);
    }

    // Step 1: Identify the block boundaries
    let mut leaders = HashSet::new();
    leaders.insert(0); // The first instruction always starts a block

    for (index, instr) in instructions.iter().enumerate() {
        if is_branching(instr) {
            let targets = branch_targets(instr);
            for &target in &targets {
                let target_index = (index as isize + target) as usize;
                if target_index < instructions.len() {
                    leaders.insert(target_index);
                }
            }
        }
        if is_exiting(instr) || is_branching(instr) {
            if index + 1 < instructions.len() {
                leaders.insert(index + 1);
            }
        }
    }

    // Convert block_starts to a sorted vector
    let mut block_boundaries: Vec<usize> = leaders.into_iter().collect();
    block_boundaries.sort_unstable();

    // Step 2: Create blocks and fill them with instructions
    let mut graph = StableDiGraph::<Block<T>, ()>::new();
    let mut instr_index_to_node: HashMap<usize, NodeIndex> = HashMap::new();
    let mut current_boundary_idx = 0;

    while current_boundary_idx < block_boundaries.len() {
        let start = block_boundaries[current_boundary_idx];
        let end = if current_boundary_idx + 1 < block_boundaries.len() {
            block_boundaries[current_boundary_idx + 1]
        } else {
            instructions.len()
        };

        let mut block = Block::new(start);
        for index in start..end {
            block.add_instruction(instructions[index].clone());
        }

        let node = graph.add_node(block);
        for index in start..end {
            instr_index_to_node.insert(index, node);
        }

        current_boundary_idx += 1;
    }

    // Step 3: Create edges between blocks based on control flow
    for (index, instr) in instructions.iter().enumerate() {
        if is_branching(instr) {
            let targets = branch_targets(instr);
            for &target in &targets {
                let target_index = (index as isize + target) as usize;
                if let Some(&target_node) = instr_index_to_node.get(&target_index) {
                    graph.add_edge(instr_index_to_node[&index], target_node, ());
                }
            }
        } else {
            if !is_exiting(instr) {
                if let Some(&next_node) = instr_index_to_node.get(&(index + 1)) {
                    if next_node != instr_index_to_node[&index] {
                        graph.add_edge(instr_index_to_node[&index], next_node, ());
                    }
                }
            }
        }
    }
    
    (graph, instr_index_to_node.get(&0).cloned())
}


pub fn get_graph(mut function: LuaFunction) -> Result<(StableDiGraph<Block<LuaInstruction>, ()>, Option<NodeIndex>), String> {
    let (mut graph, root) = build_control_flow_graph(&function.code, |insn| {
        // is_branching
        match insn.opcode {
            LuaOpcode::JMP
            | LuaOpcode::FORPREP
            | LuaOpcode::FORLOOP
            | LuaOpcode::TEST
            | LuaOpcode::TESTSET
            | LuaOpcode::EQ
            | LuaOpcode::LT
            | LuaOpcode::LE
            | LuaOpcode::TFORLOOP
             => true,
            | LuaOpcode::LOADBOOL =>
                match insn.components {
                    LuaLayout::ABC(_, _, _, c) => c != 0,
                    _ => false,
                },
            _ => false,
        }
    }, |insn| {
        // branch_targets
        match insn.opcode {
            LuaOpcode::JMP
            | LuaOpcode::FORPREP => {
                match insn.components {
                    LuaLayout::AsBx(_, _, s_bx) => vec![(s_bx + 1).try_into().unwrap_or(0)],
                    LuaLayout::SBx(_, s_bx) => vec![(s_bx + 1).try_into().unwrap_or(0)],
                    _ => vec![],
                }
            },
            LuaOpcode::FORLOOP => {
                match insn.components {
                    LuaLayout::AsBx(_, _, s_bx) => vec![1, (s_bx + 1).try_into().unwrap_or(0)],
                    _ => vec![],
                }
            },
            LuaOpcode::TEST
            | LuaOpcode::TESTSET
            | LuaOpcode::EQ
            | LuaOpcode::LT
            | LuaOpcode::LE
            | LuaOpcode::TFORLOOP => vec![1, 2],
            LuaOpcode::LOADBOOL => {
                match insn.components {
                    LuaLayout::ABC(_, _, _, c) => {
                        if c != 0 {
                            vec![2]
                        } else {
                            vec![]
                        }
                    },
                    _ => vec![],
                }
            },
            _ => vec![],
        }
    }, |insn| {
        // is_exiting
        match insn.opcode {
            LuaOpcode::RETURN
            | LuaOpcode::TAILCALL => true,
            _ => false,
        }
    });


    let dot = petgraph::dot::Dot::new(&graph);
    println!("{:?}", dot);

    // let formatted_cfg = format_cfg(&graph);

    Ok((graph, root))
}