use brainfuck_interpreter::BrainfuckInterpreter;

const CELL_ARRAY_SIZE: usize = 30_000;

fn main() {
    let program = std::env::args()
        .nth(1)
        .expect("Please provide a program string");
    let interpreter = BrainfuckInterpreter::new(&program, CELL_ARRAY_SIZE);
    interpreter.run();
}
