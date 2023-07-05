# brainfuck-interpreter-rs
An interpreter for the [Brainfuck](https://es.wikipedia.org/wiki/Brainfuck) language written in Rust.

## Usage
You can instantiate the interpreter with the `new` method, and then run it with the `run` method.
```rust
let mut interpreter = BFInterpreter::new();

interpreter.run(...);
```

## Example
```rust
// Interpreter with default config
let mut interpreter = BFInterpreter::new(None);

// Interpreter with custom config
let mut custom_instructions = HashMap::new();
// Use WASD instead of +-<> for instructions
custom_instructions.insert('D', Instruction::PointerInc);
custom_instructions.insert('A', Instruction::PointerDec);
custom_instructions.insert('W', Instruction::ByteInc);
custom_instructions.insert('S', Instruction::ByteDec);
...

let mut interpreter = BFInterpreter::new(Some(BFInterpreterConfig {
   tape_size: Some(1024),
   custom_instructions: Some(custom_instructions),
}));

interpreter.run(...);
```