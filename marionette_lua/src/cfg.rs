use marionette_core::assembly::Data;
use std::{collections::HashMap, fmt::Debug};
use petgraph::graph::{Graph, NodeIndex};
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
pub struct LuaBlock {
    pub instructions: Vec<LuaInstruction>,
    pub start: usize,
    pub outgoing: Vec<NodeIndex>,
    pub incoming: Vec<NodeIndex>,
}

impl Debug for LuaBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pc: {:?}\n", self.start);

        for instr in &self.instructions {
            write!(f, "{:?}\n", instr);
        }

        Ok(())
    }
}

impl LuaBlock {
    pub fn new(start: usize) -> LuaBlock {
        LuaBlock {
            instructions: Vec::new(),
            start: start,
            outgoing: Vec::new(),
            incoming: Vec::new(),
        }
    }
}

pub fn build_skeleton(function: &LuaFunction) -> Result<Graph<LuaBlock, ()>, String> {
    pub fn try_add_block(graph: &mut Graph<LuaBlock, ()>, start: usize) -> NodeIndex {
        for node in graph.node_indices() {
            if graph[node].start == start {
                return node;
            }
        }

        let node = graph.add_node(LuaBlock::new(start));
        node
    }

    let mut skeleton: Graph<LuaBlock, ()> = Graph::new();
    skeleton.add_node(LuaBlock::new(0));

    for (i, instruction) in function.code.iter().enumerate() {
        let opcode = instruction.opcode();

        let is_branching = BRANCHING_OPCODES.contains(opcode);
        let is_possible_skip = POSSIBLE_SKIP_OPCODES.contains(opcode);
        let is_returning = RETURNING_OPCODES.contains(opcode);

        if !is_branching && !is_returning { continue; }
        
        let is_conditional = CONDITIONAL_OPCODES.contains(opcode);
        let is_non_conditional_branching = NONCONDITIONAL_BRANCHING_OPCODES.contains(opcode);
        let is_backwards_conditional = BACKWARDS_CONDITIONAL_OPCODES.contains(opcode);

        if is_conditional || is_non_conditional_branching || is_backwards_conditional {
            if instruction.jump_target.is_none() {
                continue;
            }
            if instruction.jump_target.unwrap() >= function.code.len() {
                continue;
            }
            try_add_block(&mut skeleton, instruction.jump_target.unwrap());

            if i + 1 >= function.code.len() {
                continue;
            }
            try_add_block(&mut skeleton, i + 1);
        } else if is_possible_skip {
            let c = match instruction.instruction {
                LuaOpcode::ABC(_, _, _, c) => c,
                _ => 0,
            };

            if c != 0 {
                if i + 2 >= function.code.len() {
                    continue;
                }
                try_add_block(&mut skeleton, i + 2);

                if i + 1 >= function.code.len() {
                    continue;
                }
                try_add_block(&mut skeleton, i + 1);
            }
        } else if is_returning {
            if i + 1 >= function.code.len() {
                continue;
            }
            try_add_block(&mut skeleton, i + 1);
        }
    }
    
    Ok(skeleton)
}

pub fn fill_skeleton(function: &LuaFunction, mut graph: Graph<LuaBlock, ()>) -> Result<Graph<LuaBlock, ()>, String> {
    pub fn find_block_by_start(graph: &Graph<LuaBlock, ()>, start: usize) -> Option<NodeIndex> {
        for node in graph.node_indices() {
            if graph[node].start == start {
                return Some(node);
            }
        }

        None
    }

    let mut current = find_block_by_start(&graph, 0);
    if current.is_none() {
        return Err("Could not find the root block".to_string());
    }
    let mut current = current.unwrap();

    for (i, instruction) in function.code.iter().enumerate() {
        let possible_block = find_block_by_start(&graph, i);
        if possible_block.is_some() {
            current = possible_block.unwrap();
        }
        graph[current].instructions.push(instruction.clone());
    }

    Ok(graph)
}

pub fn add_edges(function: &LuaFunction, mut graph: Graph<LuaBlock, ()>) -> Result<Graph<LuaBlock, ()>, String> {
    pub fn find_block_by_start(graph: &Graph<LuaBlock, ()>, start: usize) -> Option<NodeIndex> {
        graph.node_indices().find(|&node| graph[node].start == start)
    }

    pub fn try_add_edge(mut edges: &mut Vec<(NodeIndex, NodeIndex)>, from: NodeIndex, to: NodeIndex) {
        if !edges.contains(&(from, to)) {
            edges.push((from, to));
        }
    }

    let mut edges: Vec<(NodeIndex, NodeIndex)> = Vec::new();

    for current in graph.node_indices() {
        let last_instruction = match graph[current].instructions.last() {
            Some(instr) => instr,
            None => continue,
        };

        if last_instruction.pc + 1 >= function.code.len() as u64 {
            continue;
        } else {
            if let Some(next) = find_block_by_start(&graph, last_instruction.pc as usize + 1) {
                try_add_edge(&mut edges, current, next);
            }
        }

        let opcode = last_instruction.opcode();
        let is_branching = BRANCHING_OPCODES.contains(opcode);
        let is_possible_skip = POSSIBLE_SKIP_OPCODES.contains(opcode);
        let is_returning = RETURNING_OPCODES.contains(opcode);

        if !is_branching && !is_returning {
            continue;
        }

        let is_conditional = CONDITIONAL_OPCODES.contains(opcode);
        let is_non_conditional_branching = NONCONDITIONAL_BRANCHING_OPCODES.contains(opcode);
        let is_backwards_conditional = BACKWARDS_CONDITIONAL_OPCODES.contains(opcode);

        if is_conditional || is_non_conditional_branching || is_backwards_conditional {
            if let Some(jump_target) = last_instruction.jump_target {
                if jump_target < function.code.len() {
                    if let Some(target) = find_block_by_start(&graph, jump_target) {
                        try_add_edge(&mut edges, current, target);
                    }
                }
            }

            if last_instruction.pc + 1 < function.code.len() as u64 {
                if let Some(next) = find_block_by_start(&graph, last_instruction.pc as usize + 1) {
                    try_add_edge(&mut edges, current, next);
                }
            }
        } else if is_possible_skip {
            if let LuaOpcode::ABC(_, _, _, c) = last_instruction.instruction {
                if c != 0 {
                    if last_instruction.pc + 2 < function.code.len() as u64 {
                        if let Some(target) = find_block_by_start(&graph, last_instruction.pc as usize + 2) {
                            try_add_edge(&mut edges, current, target);
                        }
                    }

                    if last_instruction.pc + 1 < function.code.len() as u64 {
                        if let Some(next) = find_block_by_start(&graph, last_instruction.pc as usize + 1) {
                            try_add_edge(&mut edges, current, next);
                        }
                    }
                }
            }
        } else if is_returning {
            if last_instruction.pc + 1 < function.code.len() as u64 {
                if let Some(next) = find_block_by_start(&graph, last_instruction.pc as usize + 1) {
                    try_add_edge(&mut edges, current, next);
                }
            }
        }
    }

    graph.extend_with_edges(edges);
    Ok(graph)
}

pub fn get_graph(mut function: LuaFunction) -> Result<Graph<LuaBlock, ()>, String> {
    function.update_targets();

    let mut graph = build_skeleton(&function)?;
    graph = fill_skeleton(&function, graph)?;
    graph = add_edges(&function, graph)?;

    let dot = petgraph::dot::Dot::new(&graph);
    println!("{:?}", dot);

    Ok(graph)
}