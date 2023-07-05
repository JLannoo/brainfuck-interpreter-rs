use std::{io, collections::HashMap};

#[derive (Clone, Copy, Debug, PartialEq)]
enum Instruction {
    PointerInc,
    PointerDec,
    ByteInc,
    ByteDec,
    Output,
    Input,
    OpenLoop,
    CloseLoop,
}

#[derive (Debug)]
struct StackItem {
    index: usize,
}

struct BFInterpreterConfig {
    tape_size: Option<usize>,
    custom_instructions: Option<HashMap<char, Instruction>>,
}

#[derive (Debug)]
struct BFInterpreter {
    instruction_pointer: usize,
    instructions_map: HashMap<char, Instruction>,
    instructions: Vec<Instruction>,
    current_instruction: Instruction,

    data_pointer: usize,
    data: Vec<u8>,

    loop_stack: Vec<StackItem>,

    output: Vec<char>,
}

impl BFInterpreter {
    /// Creates a new BFInterpreter with the default config
    /// 
    /// You can pass a custom config to change the tape size and add custom instructions
    /// 
    /// # Examples
    /// ```
    /// // Interpreter with default config
    /// let mut interpreter = BFInterpreter::new(None);
    /// 
    /// // Interpreter with custom config
    /// let mut custom_instructions = HashMap::new();
    /// custom_instructions.insert('D', Instruction::PointerInc);
    /// custom_instructions.insert('A', Instruction::PointerDec);
    /// custom_instructions.insert('W', Instruction::ByteInc);
    /// custom_instructions.insert('S', Instruction::ByteDec);
    /// ...
    /// 
    /// let mut interpreter = BFInterpreter::new(Some(BFInterpreterConfig {
    ///    tape_size: Some(1024),
    ///   custom_instructions: Some(custom_instructions),
    /// }));
    /// 
    /// interpreter.run(...);
    /// ```
    pub fn new(config: Option<BFInterpreterConfig>) -> Self {
        let (tape_size, custom_instructions) = match config {
            None => (None, None),
            Some(v) => (v.tape_size, v.custom_instructions),
        };

        Self {
            instruction_pointer: 0,
            instructions_map: match custom_instructions {
                None => HashMap::from([
                    ('>', Instruction::PointerInc),
                    ('<', Instruction::PointerDec),
                    ('+', Instruction::ByteInc),
                    ('-', Instruction::ByteDec),
                    ('.', Instruction::Output),
                    (',', Instruction::Input),
                    ('[', Instruction::OpenLoop),
                    (']', Instruction::CloseLoop),
                ]),
                Some(v) => v,
            },
            instructions: Vec::new(),
            current_instruction: Instruction::Output,
            data_pointer: 0,
            data: vec![0; match tape_size {
                None => 1024,
                Some(v) => v,
            }],
            loop_stack: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn run(&mut self, instructions: &str) -> String {
        self.init(instructions);
        
        let closing_brackets = self.instructions.iter().filter(|&i| *i == Instruction::CloseLoop).count();
        let opening_brackets = self.instructions.iter().filter(|&i| *i == Instruction::OpenLoop).count();

        if closing_brackets != opening_brackets {
            panic!("Unbalanced brackets");
        };

        while self.instruction_pointer < self.instructions.len() {
            self.current_instruction = match self.instructions.get(self.instruction_pointer) {
                Some(v) => *v,
                None => panic!("Error gettin instruction"),
            };

            // println!("Instruction: {:#?}", self);

            match self.instructions[self.instruction_pointer] {
                Instruction::PointerInc => self.pointer_inc(),
                Instruction::PointerDec => self.pointer_dec(),
                Instruction::ByteInc => self.byte_inc(),
                Instruction::ByteDec => self.byte_dec(),
                Instruction::Output => self.output(),
                Instruction::Input => self.input(),
                Instruction::OpenLoop => self.jump(),
                Instruction::CloseLoop => self.jump(),
            }
            self.instruction_pointer += 1;
        }

        self.output.iter().collect()
    }

    fn pointer_inc(&mut self) {
        self.data_pointer += 1;
    }

    fn pointer_dec(&mut self) {
        self.data_pointer -= 1;
    }

    fn byte_inc(&mut self) {
        match self.data.get(self.data_pointer) {
            None => self.data[self.data_pointer] = 1,
            Some(v) => self.data[self.data_pointer] = v+1,
        }
    }

    fn byte_dec(&mut self) {
        match self.data.get(self.data_pointer) {
            None => self.data[self.data_pointer] = 255,
            Some(v) => self.data[self.data_pointer] = v-1,
        }
    }

    fn output(&mut self) {
        self.output.push(self.data[self.data_pointer] as char);
    }

    fn input(&mut self) {
        println!("Enter a char: ");

        let mut line = String::new();
        let input = io::stdin().read_line(&mut line);

        match input {
            Ok(_) => {
                let c = line.chars().next().unwrap();
                self.data[self.data_pointer] = c as u8;
            },
            Err(_) => self.input(),
        }
    }

    fn jump(&mut self) {
        match self.current_instruction {
            Instruction::CloseLoop => {
                match self.data[self.data_pointer] {
                    // If not 0 jump to the start of the loop, else continue
                    0 => (),
                    _ => self.instruction_pointer = self.loop_stack.last().unwrap().index,
                }
            },
            Instruction::OpenLoop => {
                match self.data[self.data_pointer] {
                    // If 0 jump to the end of the loop, else continue
                    0 => self.instruction_pointer = self.get_loop_end(),
                    _ => self.loop_stack.push(StackItem { index: self.instruction_pointer }),
                }
            },
            _ => panic!("SHOULD NOT HAVE JUMPED")
        }
    }

    fn get_loop_end(&self) -> usize {
        let mut loopdepth = self.loop_stack.len();
        let mut pointer = self.instruction_pointer;

        while loopdepth > 0 {
            pointer += 1;

            match self.instructions[pointer] {
                Instruction::OpenLoop => loopdepth += 1,
                Instruction::CloseLoop => loopdepth -= 1,
                _ => (),
            }
        };

        pointer
    }

    fn init(&mut self, instructions: &str) {
        self.instruction_pointer = 0;
        self.instructions = instructions
            .chars()
            .map(|c| match self.instructions_map.get(&c) {
                Some(v) => *v,
                None => panic!("Invalid instruction"),
            })
            .collect();
            
        self.data_pointer = 0;
        self.data = vec![0; self.data.len()];

        self.loop_stack = Vec::new();

        self.output = Vec::new();
    }
}

fn main() {
    // Print 3 hearts with default instructions
    let mut interpreter = BFInterpreter::new(None);
    let output = interpreter.run("+++>+++<[>.<-]");
    println!("{}", output);

    let custom_map = HashMap::from([
        ('D', Instruction::PointerInc),
        ('A', Instruction::PointerDec),
        ('W', Instruction::ByteInc),
        ('S', Instruction::ByteDec),
        ('O', Instruction::Output),
        ('I', Instruction::Input),
        ('(', Instruction::OpenLoop),
        (')', Instruction::CloseLoop),
    ]);


    // Print 3 hearts with custom instructions
    let config = BFInterpreterConfig {
        tape_size: Some(100),
        custom_instructions: Some(custom_map),
    };
    let mut custom_interpreter = BFInterpreter::new(Some(config));
    let output = custom_interpreter.run("WWWDWWWA(DOAS)");
    println!("{}", output);
    
    // Print Hello World
    let output = interpreter.run("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
    println!("{}", output);
    
}
