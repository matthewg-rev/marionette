mod interface;
mod configuration;

extern crate sensible_env_logger;
#[macro_use] extern crate log;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use marionette_core::plugin::interface::{Function, PluginError, PluginRegistrar};
use marionette_core::plugin::comm::{CompilerInfo};
use marionette_core::byte_stream::ByteStream;
use marionette_core::cfg::block::Block;
use marionette_core::export_plugin;
use marionette_core::textualizer::{Mark, Textualizer};
use crate::configuration::{Configuration, Instruction, Reg};

use crate::interface::{AddressedData, LuaDisassembly, LuaHeader, LuaDisassemblerInstance, LuaChunkHeader, LuaChunk, LuaInstruction, LuaConstant, LuaConstantValue, LuaLocal, LuaUpvalue};

pub struct GetCompilerInfo;
pub struct CanDisassemble;
pub struct NewDisassemblyInstance;
pub struct Disassemble;
pub struct GetFunctions;
pub struct DebugDump;
pub struct GetCfg;

macro_rules! read_into_addressed {
    ($field:expr, $stream:ident, $read:expr) => {
        $field.start_address = $stream.current_address();
        $field.data = $read($stream);
        $field.end_address = $stream.current_address();
    };
}

macro_rules! read_into_addressed_multiple {
    ($($field:expr)+, $stream:ident, $read:expr) => {
        $(
            read_into_addressed!($field, $stream, $read);
        )+
    };
}

macro_rules! read_addressed {
    ($field:expr, $stream:ident, $read:expr) => {
        let (value, mut $stream) = AddressedData::new(&mut $stream, $read);
        $field = value;
    };
}

macro_rules! read_addressed_multiple {
    ($stream:ident, $read:expr, $($field:expr),+) => {
        $(
            read_addressed!($field, $stream, $read);
        )+
    };
}

impl Function for CanDisassemble {
    fn call(&self, args: Vec<u8>) -> Result<Vec<u8>, PluginError> {
        // since this is a single argument function, we can just assume
        // that all the bytes are the argument
        let mut stream = ByteStream::new(args);
        let mut return_stream = ByteStream::new(Vec::new());
        if !stream.is_out_of_bounds(4) {
            let bytes = stream.read_bytes(4);
            if bytes.unwrap() == [0x1b, 0x4c, 0x75, 0x61] {
                debug!("lua magic bytes present");
                return_stream.write_u8(1);
                return Ok(return_stream.get_bytes());
            }
        }

        return_stream.write_u8(0);
        Ok(return_stream.get_bytes())
    }
}

impl Function for GetCompilerInfo {
    fn call(&self, _: Vec<u8>) -> Result<Vec<u8>, PluginError> {
        let mut return_stream = ByteStream::new(Vec::new());
        let info = CompilerInfo {
            name: "Lua".to_string(),
            major: 0,
            minor: 0
        };

        return_stream.write_struct(info);
        Ok(return_stream.get_bytes())
    }
}

impl Function for NewDisassemblyInstance {
    fn call(&self, args: Vec<u8>) -> Result<Vec<u8>, PluginError> {
        let mut return_stream = ByteStream::new(Vec::new());
        let instance = LuaDisassemblerInstance {
            raw: args,
            upvalues: Vec::new(),
            allocated_upvalues: Vec::new(),
            configuration: Configuration::new(0, vec![]),
            disassembly: LuaDisassembly::default()
        };

        return_stream.write_struct(instance);
        Ok(return_stream.get_bytes())
    }
}

pub fn internal_read_lua_constant(size_t: u8, number_size: u8, stream: &mut ByteStream) -> (AddressedData<LuaConstant>, &mut ByteStream) {
    let mut addressed_data = AddressedData {
        data: LuaConstant {
            value: AddressedData {
                data: LuaConstantValue::Nil,
                start_address: stream.current_address(),
                end_address: stream.current_address(),
            },
            type_tag: AddressedData {
                data: 0,
                start_address: stream.current_address(),
                end_address: stream.current_address(),
            },
        },
        start_address: stream.current_address(),
        end_address: stream.current_address(),
    };

    read_into_addressed!(addressed_data.data.type_tag, stream, |stream: &mut ByteStream| {
        stream.read_u8().unwrap()
    });

    if addressed_data.data.type_tag.data == 0 {
        read_into_addressed!(addressed_data.data.value, stream, |stream: &mut ByteStream| {
            LuaConstantValue::Nil
        });
    } else if addressed_data.data.type_tag.data == 1 {
        read_into_addressed!(addressed_data.data.value, stream, |stream: &mut ByteStream| {
            LuaConstantValue::Boolean(stream.read_u8().unwrap() != 0)
        });
    } else if addressed_data.data.type_tag.data == 3 {
        read_into_addressed!(addressed_data.data.value, stream, |stream: &mut ByteStream| {
            LuaConstantValue::Number(if number_size == 4 {
                stream.read_f32().unwrap() as f64
            } else {
                stream.read_f64().unwrap()
            })
        });
    } else if addressed_data.data.type_tag.data == 4 {
        let (mut string_constant, stream) = internal_read_lua_string(size_t, -1, stream);
        string_constant.data = string_constant.data.trim_end_matches('\0').to_string();

        addressed_data.data.value = AddressedData {
            data: LuaConstantValue::String(string_constant.data),
            start_address: string_constant.start_address,
            end_address: string_constant.end_address,
        };
    } else {
        read_into_addressed!(addressed_data.data.value, stream, |stream: &mut ByteStream| {
            LuaConstantValue::Nil
        });
    }

    (addressed_data, stream)
}

pub fn internal_read_lua_instruction(instruction_size: u8, stream: &mut ByteStream) -> (AddressedData<LuaInstruction>, &mut ByteStream) {
    let instruction = LuaInstruction::default();
    let mut addressed_data = AddressedData {
        data: instruction,
        start_address: stream.current_address(),
        end_address: stream.current_address(),
    };

    let instruction = if instruction_size == 4 {
        stream.read_u32().unwrap() as u64
    } else {
        stream.read_u64().unwrap()
    };

    addressed_data.data.opcode = (instruction & 0x3F) as u8;
    addressed_data.data.a = ((instruction >> 6) & 0xFF) as u8;
    addressed_data.data.b = ((instruction >> 23) & 0x1FF) as u8;
    addressed_data.data.c = ((instruction >> 14) & 0x1FF) as u8;
    addressed_data.data.bx = ((instruction >> 14) & 0x3FFFF) as u16;
    addressed_data.data.sbx = ((instruction >> 14) & 0x3FFFF) as u16;
    addressed_data.end_address = stream.current_address();

    (addressed_data, stream)
}

pub fn internal_read_lua_int(instance: LuaDisassemblerInstance, stream: &mut ByteStream) -> (AddressedData<u64>, LuaDisassemblerInstance, &mut ByteStream) {
    let mut addressed_data = AddressedData {
        data: 0,
        start_address: stream.current_address(),
        end_address: stream.current_address(),
    };

    if instance.disassembly.header.data.int_size.data == 4 {
        addressed_data.data = stream.read_u32().unwrap() as u64;
    } else {
        addressed_data.data = stream.read_u64().unwrap();
    }

    addressed_data.end_address = stream.current_address();
    (addressed_data, instance, stream)
}

pub fn internal_read_lua_string(size_t: u8, len: i32, stream: &mut ByteStream) -> (AddressedData<String>, &mut ByteStream) {
    let mut addressed_data = AddressedData {
        data: String::new(),
        start_address: stream.current_address(),
        end_address: stream.current_address(),
    };

    let len = if len == -1 {
        if size_t == 4 {
            stream.read_u32().unwrap() as u64
        } else {
            stream.read_u64().unwrap()
        }
    } else {
        len as u64
    };

    for _ in 0..len {
        addressed_data.data.push(stream.read_u8().unwrap() as char);
    }

    // trim the null terminator
    if addressed_data.data.ends_with('\0') {
        addressed_data.data.pop();
    }

    addressed_data.end_address = stream.current_address();
    (addressed_data, stream)
}

pub fn internal_disassemble_chunk_header(instance: LuaDisassemblerInstance, stream: &mut ByteStream) -> (AddressedData<LuaChunkHeader>, LuaDisassemblerInstance, &mut ByteStream) {
    let header = LuaChunkHeader::default();
    let mut addressed_data = AddressedData {
        data: header,
        start_address: stream.current_address(),
        end_address: stream.current_address(),
    };

    let (mut chunk_name, stream) = internal_read_lua_string(instance.disassembly.header.data.size_t_size.data, -1, stream);
    chunk_name.data = chunk_name.data.trim_end_matches('\0').to_string();
    addressed_data.data.name = chunk_name;

    let (line_defined, instance, stream) = internal_read_lua_int(instance, stream);
    addressed_data.data.line_defined = line_defined;

    let (last_line_defined, instance, stream) = internal_read_lua_int(instance, stream);
    addressed_data.data.last_line_defined = last_line_defined;

    read_into_addressed_multiple!(
        addressed_data.data.num_upvalues
        addressed_data.data.num_parameters
        addressed_data.data.is_vararg
        addressed_data.data.max_stack_size,
        stream, |stream: &mut ByteStream| {
            stream.read_u8().unwrap()
        }
    );

    addressed_data.end_address = stream.current_address();
    (addressed_data, instance, stream)
}

pub fn internal_disassemble_chunk(instance: LuaDisassemblerInstance, stream: &mut ByteStream) -> (AddressedData<LuaChunk>, LuaDisassemblerInstance, &mut ByteStream) {
    let mut addressed_data = AddressedData {
        data: LuaChunk::default(),
        start_address: stream.current_address(),
        end_address: stream.current_address(),
    };

    let (chunk_header, instance, stream) = internal_disassemble_chunk_header(instance, stream);
    addressed_data.data.header = chunk_header;

    let (num_instructions, instance, stream) = internal_read_lua_int(instance, stream);
    for _ in 0..num_instructions.data {
        let (instruction, stream) = internal_read_lua_instruction(instance.disassembly.header.data.instruction_size.data, stream);
        addressed_data.data.instructions.push(instruction);
    }

    let (num_constants, mut instance, stream) = internal_read_lua_int(instance, stream);
    for _ in 0..num_constants.data {
        let (constant, stream) = internal_read_lua_constant(
            instance.disassembly.header.data.size_t_size.data,
            instance.disassembly.header.data.lua_number_size.data,
            stream
        );
        addressed_data.data.constants.push(constant);
    }

    let (num_protos, mut instance, stream) = internal_read_lua_int(instance, stream);
    for _ in 0..num_protos.data {
        let (proto, new_instance, stream) = internal_disassemble_chunk(instance.clone(), stream);
        addressed_data.data.functions.push(proto);
        instance = new_instance;
    }

    let (num_lines, instance, stream) = internal_read_lua_int(instance, stream);
    for _ in 0..num_lines.data {
        let mut line: AddressedData<u32> = AddressedData {
            data: 0,
            start_address: stream.current_address(),
            end_address: stream.current_address(),
        };
        line.data = stream.read_u32().unwrap();
        line.end_address = stream.current_address();
        addressed_data.data.line_info.push(line);
    }

    let (num_locals, instance, stream) = internal_read_lua_int(instance, stream);
    for _ in 0..num_locals.data {
        let start_address = stream.current_address();
        let (name, stream) = internal_read_lua_string(instance.disassembly.header.data.size_t_size.data, -1, stream);
        let mut start_pc: AddressedData<u32> = AddressedData {
            data: 0,
            start_address: stream.current_address(),
            end_address: stream.current_address(),
        };
        start_pc.data = stream.read_u32().unwrap();
        start_pc.end_address = stream.current_address();

        let mut end_pc: AddressedData<u32> = AddressedData {
            data: 0,
            start_address: stream.current_address(),
            end_address: stream.current_address(),
        };
        end_pc.data = stream.read_u32().unwrap();
        end_pc.end_address = stream.current_address();

        let local = LuaLocal {
            name,
            start_pc,
            end_pc,
        };
        let local = AddressedData {
            data: local,
            start_address,
            end_address: stream.current_address(),
        };
        addressed_data.data.locals.push(local);
    }

    let mut new_instance = instance.clone();
    let instance = new_instance;

    let (num_upvalues, mut instance, stream) = internal_read_lua_int(instance, stream);
    for _ in 0..num_upvalues.data {
        let start_address = stream.current_address();
        let (upvalue, stream) = internal_read_lua_string(instance.disassembly.header.data.size_t_size.data, -1, stream);
        addressed_data.data.upvalues.push(AddressedData {
            data: LuaUpvalue {
                name: upvalue,
            },
            start_address,
            end_address: stream.current_address(),
        });
    }

    addressed_data.end_address = stream.current_address();
    instance.disassembly.functions.push(addressed_data.clone());

    debug!("\tdisassembled chunk @0x{:X} -> 0x{:X}", addressed_data.start_address, addressed_data.end_address);
    debug!("\t\t|c| = {}", num_instructions.data);
    debug!("\t\t|k| = {}", num_constants.data);
    debug!("\t\t|p| = {}", num_protos.data);
    (addressed_data, instance, stream)
}

impl Function for Disassemble {
    fn call(&self, args: Vec<u8>) -> Result<Vec<u8>, PluginError> {
        let mut return_stream = ByteStream::new(Vec::new());
        let mut instance = ByteStream::new(args).read_struct::<LuaDisassemblerInstance>().unwrap();
        let mut raw_stream = ByteStream::new(instance.raw.clone());
        debug!("beginning disassembly of lua binary");

        instance.disassembly.header.start_address = raw_stream.current_address();
        read_addressed!(instance.disassembly.header.data.magic, raw_stream, |stream: &mut ByteStream| {
            stream.read_u32().unwrap()
        });

        read_addressed_multiple!(
            raw_stream, |stream: &mut ByteStream| {
                stream.read_u8().unwrap()
            },
            instance.disassembly.header.data.version,
            instance.disassembly.header.data.format,
            instance.disassembly.header.data.endianness,
            instance.disassembly.header.data.int_size,
            instance.disassembly.header.data.size_t_size,
            instance.disassembly.header.data.instruction_size,
            instance.disassembly.header.data.lua_number_size,
            instance.disassembly.header.data.integral_flag
        );

        instance.configuration = match instance.disassembly.header.data.version.data {
            0x51 => Configuration::new(0x51, vec![
                Instruction::new(0, "MOVE", vec![Reg::new("A"), Reg::new("B")]),
                Instruction::new(1, "LOADK", vec![Reg::new("A"), Reg::new("Bx")]),
                Instruction::new(2, "LOADBOOL", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(3, "LOADNIL", vec![Reg::new("A"), Reg::new("B")]),
                Instruction::new(4, "GETUPVAL", vec![Reg::new("A"), Reg::new("B")]),
                Instruction::new(5, "GETGLOBAL", vec![Reg::new("A"), Reg::new("Bx")]),
                Instruction::new(6, "GETTABLE", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(7, "SETGLOBAL", vec![Reg::new("A"), Reg::new("Bx")]),
                Instruction::new(8, "SETUPVAL", vec![Reg::new("A"), Reg::new("B")]),
                Instruction::new(9, "SETTABLE", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(10, "NEWTABLE", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(11, "SELF", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(12, "ADD", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(13, "SUB", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(14, "MUL", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(15, "DIV", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(16, "MOD", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(17, "POW", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(18, "UNM", vec![Reg::new("A"), Reg::new("B")]),
                Instruction::new(19, "NOT", vec![Reg::new("A"), Reg::new("B")]),
                Instruction::new(20, "LEN", vec![Reg::new("A"), Reg::new("B")]),
                Instruction::new(21, "CONCAT", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(22, "JMP", vec![Reg::new("sBx")]),
                Instruction::new(23, "EQ", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(24, "LT", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(25, "LE", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(26, "TEST", vec![Reg::new("A"), Reg::new("C")]),
                Instruction::new(27, "TESTSET", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(28, "CALL", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(29, "TAILCALL", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(30, "RETURN", vec![Reg::new("A"), Reg::new("B")]),
                Instruction::new(31, "FORLOOP", vec![Reg::new("A"), Reg::new("sBx")]),
                Instruction::new(32, "FORPREP", vec![Reg::new("A"), Reg::new("sBx")]),
                Instruction::new(33, "TFORLOOP", vec![Reg::new("A"), Reg::new("C")]),
                Instruction::new(34, "SETLIST", vec![Reg::new("A"), Reg::new("B"), Reg::new("C")]),
                Instruction::new(35, "CLOSE", vec![Reg::new("A")]),
                Instruction::new(36, "CLOSURE", vec![Reg::new("A"), Reg::new("Bx")]),
                Instruction::new(37, "VARARG", vec![Reg::new("A"), Reg::new("B")]),
            ]),
            _ => panic!("unsupported version"),
        };

        instance.disassembly.header.end_address = raw_stream.current_address();
        let (entry_point, mut instance, raw_stream) = internal_disassemble_chunk(instance, raw_stream);
        instance.disassembly.entry_point = entry_point;

        // export instance
        instance.raw = raw_stream.get_bytes();
        return_stream.write_struct(instance);
        debug!("\twrote {} bytes to return stream", return_stream.get_bytes().len());
        Ok(return_stream.get_bytes())
    }
}

impl Function for GetFunctions {
    fn call(&self, args: Vec<u8>) -> Result<Vec<u8>, PluginError> {
        let mut return_stream = ByteStream::new(Vec::new());
        let mut arg_stream = ByteStream::new(args);
        let instance = arg_stream.read_struct::<LuaDisassemblerInstance>().unwrap();

        for function in instance.disassembly.functions.iter() {
            return_stream.write_u64(function.start_address);
        }

        Ok(return_stream.get_bytes())
    }
}

pub fn calculate_tabbing(tabbing: u64) -> String {
    let mut tabbing_string = String::new();
    let tabbing = tabbing - 1;
    for _ in 0..tabbing { tabbing_string.push('\t'); }

    tabbing_string
}

impl Function for DebugDump {
    fn call(&self, args: Vec<u8>) -> Result<Vec<u8>, PluginError> {
        let mut return_stream = ByteStream::new(Vec::new());
        let mut arg_stream = ByteStream::new(args);
        let instance = arg_stream.read_struct::<LuaDisassemblerInstance>().unwrap();
        let mut version_hex = format!("{:x}", instance.disassembly.header.data.version.data);
        version_hex.insert(1, '.');

        let str_len = arg_stream.read_u64().unwrap();
        let file_to_write = arg_stream.read_as_string(str_len as usize).unwrap();

        let mut file = File::create(file_to_write).unwrap();
        file.write_all("DEBUG PROVIDER INFORMATION:\n".as_bytes()).unwrap();
        file.write_all(format!("\tversion: {}\n", env!("CARGO_PKG_VERSION")).as_bytes()).unwrap();
        file.write_all("ASSEMBLY INFORMATION:\n".as_bytes()).unwrap();
        file.write_all(format!("\tLUA VERSION: {}\n", version_hex).as_bytes()).unwrap();
        file.write_all(format!("\tENDIANNESS: {}\n", instance.disassembly.header.data.endianness.data).as_bytes()).unwrap();
        file.write_all(format!("\tINT SIZE: {}\n", instance.disassembly.header.data.int_size.data).as_bytes()).unwrap();
        file.write_all(format!("\tSIZE_T SIZE: {}\n", instance.disassembly.header.data.size_t_size.data).as_bytes()).unwrap();
        file.write_all(format!("\tINSTRUCTION SIZE: {}\n", instance.disassembly.header.data.instruction_size.data).as_bytes()).unwrap();
        file.write_all(format!("\tLUA NUMBER SIZE: {}\n", instance.disassembly.header.data.lua_number_size.data).as_bytes()).unwrap();
        file.write_all(format!("\tINTEGRAL FLAG: {}\n", instance.disassembly.header.data.integral_flag.data).as_bytes()).unwrap();
        file.write_all(format!("\tFORMAT: {}\n", instance.disassembly.header.data.format.data).as_bytes()).unwrap();

        file.write_all("DISASSEMBLY HEX DUMP:\n".as_bytes()).unwrap();
        file.write_all(ByteStream::hex_dump(instance.raw.clone()).as_bytes()).unwrap();

        return_stream.write_struct(instance);
        Ok(return_stream.get_bytes())
    }
}

impl Function for GetCfg {
    fn call(&self, args: Vec<u8>) -> Result<Vec<u8>, PluginError> {
        let mut return_stream = ByteStream::new(Vec::new());
        let mut arg_stream = ByteStream::new(args);
        let instance = arg_stream.read_struct::<LuaDisassemblerInstance>().unwrap();



        Ok(return_stream.get_bytes())
    }
}

export_plugin!(register_plugin);
extern "C" fn register_plugin(registrar: &mut dyn PluginRegistrar) {
    sensible_env_logger::init!();

    // interop functions
    registrar.register_function("can_disassemble", Box::new(CanDisassemble));
    registrar.register_function("get_compiler_info", Box::new(GetCompilerInfo));

    // disassembler functions
    registrar.register_function("new_disassembly_instance", Box::new(NewDisassemblyInstance));
    registrar.register_function("disassemble", Box::new(Disassemble));
    registrar.register_function("get_functions", Box::new(GetFunctions));

    // control flow graph functions
    registrar.register_function("get_cfg", Box::new(GetCfg));

    // debug functions
    registrar.register_function("debug_dump", Box::new(DebugDump));
    info!("loaded lua disassembler plugin {}", env!("CARGO_PKG_VERSION"));
}