extern crate bimap;

//use std::io;
use bimap::BiMap;

struct Compiler {
    opcode_list: BiMap<&'static str, i32>,
    opcode_alias_list: BiMap<&'static str, &'static str>,
}

impl Compiler {
    fn new() -> Compiler {
        let mut opcode_list: BiMap<&'static str, i32> = BiMap::new();
        opcode_list.insert("OP_0",     0x00);
        opcode_list.insert("OP_1",     0x51);
        opcode_list.insert("OP_2",     0x52);
        opcode_list.insert("OP_NOP",   0x61);
        opcode_list.insert("OP_DUP",   0x76);
        opcode_list.insert("OP_IF",    0x63);
        opcode_list.insert("OP_NOTIF", 0x64);
        opcode_list.insert("OP_ELSE",  0x67);
        opcode_list.insert("OP_ENDIF", 0x68);

        let mut opcode_alias_list: BiMap<&'static str, &'static str> = BiMap::new();
        opcode_alias_list.insert("OP_FALSE", "OP_0");
        opcode_alias_list.insert("OP_TRUE",  "OP_1");

        return Compiler {
            opcode_list: opcode_list,
            opcode_alias_list: opcode_alias_list,
        };
    }
    fn compile_single(&self, code: &str) -> i32 {
        let mut codes_unalias: &str = "";
        let alias_opcode: &str = match self.opcode_alias_list.get_by_left(&code) {
            Some(&value) => value,
            None => code,
        };
        let hex: i32 = match self.opcode_list.get_by_left(&alias_opcode) {
            Some(&value) => value,
            None => panic!("[Compiler] opcode not found."),
        };
        return hex;
    }
    fn compile(&self, codes: Vec<&str>) -> Vec<i32> {
        let mut bytecode: Vec<i32> = vec![];

        for code in codes {
            let hex = self.compile_single(code);
            bytecode.push(hex);
        }

        return bytecode;
    }

}
struct VM {
    stack: Vec<i32>,
    code: Vec<i32>,
    pc: usize,
}

impl VM {
    fn new(code: Vec<i32>) -> VM {
        VM {
            code: code,
            pc: 0,
            stack: vec![]
        }
    }
    fn dump(self) {
        println!("pc:{}", self.pc);
        print!("stack: [");
        for value in self.stack {
            print!("{:#x}, ",value);
        }
        println!("]");

        print!("code: [");
        for code in self.code {
            print!("{:#x}, ",code);
        }
        println!("]");
    }
    fn run(&mut self) {
        while self.code.len() > self.pc {
            self.step();
        }
    }
    fn step(&mut self) {
        let compiler = Compiler::new();
        if      self.code[self.pc] == compiler.compile_single("OP_0")   { self.op_pushnumber(0); }
        else if self.code[self.pc] == compiler.compile_single("OP_1")   { self.op_pushnumber(1); }
        else if self.code[self.pc] == compiler.compile_single("OP_2")   { self.op_pushnumber(2); }
        else if self.code[self.pc] == compiler.compile_single("OP_NOP") { self.op_nop(); }
        else if self.code[self.pc] == compiler.compile_single("OP_DUP") { self.op_dup(); }
        else if self.code[self.pc] == compiler.compile_single("OP_IF")  { self.op_if(); }
        else { panic!("The opcode is not implemented yet,"); }
    }
    fn op_pushnumber(&mut self, num: i32){
        self.stack.push(num);
        self.pc += 1;
    }
    fn op_dup(&mut self){

        let num: i32 = match self.stack.pop() {
            Some(num) => num,
            None => panic!("stack is empty."),
        };

        self.stack.push(num);
        self.stack.push(num);
        self.pc += 1;
    }
    fn op_nop(&mut self){
        self.pc += 1;
    }
    fn op_if(&mut self){
        let expression = self.code[self.pc + 1];
    }
}

fn main() {

    let compiler = Compiler::new();
    let bytecode = compiler.compile(vec!["OP_1", "OP_2", "OP_DUP"]);

    let mut vm = VM::new(bytecode);
    vm.run();

    vm.dump();
}
