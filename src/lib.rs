use std::io;

/// An interpreter for Brainfuck
///
/// ## Example
/// ```
/// use brainfuck_interpreter::BrainfuckInterpreter;
///
/// let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
/// let cell_array_size = 30_000;
/// let bfi = BrainfuckInterpreter::new(program, cell_array_size);
/// bfi.run();
/// ```
#[derive(Debug, Clone)]
pub struct BrainfuckInterpreter {
    cells: Vec<Cell>,
    instructions: Vec<Instruction>,
    cell_ptr: CellPos,
    instruction_ptr: CellPos,
}

impl BrainfuckInterpreter {
    pub fn new(program: &str, cell_array_size: usize) -> Self {
        Self {
            cells: vec![0; cell_array_size],
            instructions: Self::parse_program(program),
            cell_ptr: 0,
            instruction_ptr: 0,
        }
    }

    fn parse_program(program: &str) -> Vec<Instruction> {
        let mut bracket_stack = Vec::new();
        let mut instructions = Vec::new();
        // i tracked manually instead of by .enumerate to ignore non-relevant character
        let mut i = 0;
        for ch in program.chars() {
            let instruction = match ch {
                '>' => Instruction::MoveRight,
                '<' => Instruction::MoveLeft,
                '+' => Instruction::Increment,
                '-' => Instruction::Decrement,
                '.' => Instruction::OutputChar,
                ',' => Instruction::InputChar,
                '[' => {
                    bracket_stack.push(i);
                    Instruction::JumpForward(usize::MAX)
                }
                ']' => {
                    let opening = bracket_stack
                        .pop()
                        .expect("Closing bracket at position {i} should have an opening tag");
                    let Instruction::JumpForward(j) = &mut instructions[opening] else {
                        panic!("Expected instruction::JumpForward while parsing");
                    };
                    *j = i;
                    Instruction::JumpBack(opening)
                }
                _ => continue,
            };
            i += 1;
            instructions.push(instruction);
        }

        if let Some(unclosed) = bracket_stack.pop() {
            panic!(
                "Opening bracket at position {} should have a closing bracket",
                unclosed
            );
        }

        instructions
    }

    pub fn run(mut self) {
        while self.instruction_ptr < self.instructions.len() {
            let instruction = self.instructions[self.instruction_ptr];
            match instruction {
                Instruction::MoveRight => self.cell_ptr += 1,
                Instruction::MoveLeft => self.cell_ptr -= 1,
                Instruction::Increment => {
                    self.cells[self.cell_ptr] = self.cells[self.cell_ptr].wrapping_add(1)
                }
                Instruction::Decrement => {
                    self.cells[self.cell_ptr] = self.cells[self.cell_ptr].wrapping_sub(1)
                }
                Instruction::OutputChar => eprint!("{}", self.cells[self.cell_ptr] as char),
                Instruction::InputChar => match take_input() {
                    Ok(Some(num)) => self.cells[self.cell_ptr] = num,
                    Ok(None) => eprintln!("Please enter atleast one valid ascii character"),
                    Err(err) => eprintln!("Error occured while reading input: {}", err),
                },
                Instruction::JumpForward(i) => {
                    if self.cells[self.cell_ptr] == 0 {
                        self.instruction_ptr = i;
                    }
                }
                Instruction::JumpBack(i) => {
                    if self.cells[self.cell_ptr] != 0 {
                        self.instruction_ptr = i;
                    }
                }
            }
            self.instruction_ptr += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    OutputChar,
    InputChar,
    JumpForward(CellPos),
    JumpBack(CellPos),
}

type Cell = u8;
type CellPos = usize;

fn take_input() -> io::Result<Option<u8>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.chars().next().and_then(|ch| ch.try_into().ok()))
}
