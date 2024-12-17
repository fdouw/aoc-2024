use itertools::Itertools;
use regex::Regex;

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut machine = Machine::new(input);
    let part1 = machine.run().iter().join(",");

    let mut part2 = 0;
    let target = machine.tape.iter().join(",");
    for a in 0..u32::MAX {
        machine.reset(a as u64);
        let out = machine.run().iter().join(",");
        if out == target {
            part2 = a;
            break;
        }
    }
    (part1, part2.to_string())
}

struct Machine {
    tape: Vec<u8>,
    pointer: usize,
    a: u64,
    b: u64,
    c: u64,
}

impl Machine {
    fn new(description: String) -> Self {
        let re = Regex::new(r"\d+").unwrap();
        let (reg_data, program_data) = description.split_once("\n\n").unwrap();
        let mut reg_data = re.find_iter(reg_data).map(|m| m.as_str().parse().unwrap());
        let tape = program_data
            .trim_end()
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        Self {
            tape,
            pointer: 0,
            a: reg_data.next().unwrap(),
            b: reg_data.next().unwrap(),
            c: reg_data.next().unwrap(),
        }
    }
    fn reset(&mut self, a: u64) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.pointer = 0;
    }
    fn run(&mut self) -> Vec<u64> {
        let mut output = Vec::new();
        'tape: while let Some(opcode) = self.tape.get(self.pointer) {
            if let Some(operand) = self.tape.get(self.pointer + 1) {
                match *opcode {
                    ADV => self.a >>= self.combo(*operand),
                    BXL => self.b ^= *operand as u64,
                    BST => self.b = self.combo(*operand) & 0b0111,
                    JNZ => {
                        if self.a > 0 {
                            self.pointer = *operand as usize;
                            continue 'tape;
                        }
                    }
                    BXC => self.b = self.b ^ self.c,
                    OUT => output.push(self.combo(*operand) & 0b0111),
                    BDV => self.b = self.a >> self.combo(*operand),
                    CDV => self.c = self.a >> self.combo(*operand),
                    x => panic!("Invalid opcode: {x}"),
                }
            }
            self.pointer += 2;
        }
        return output;
    }
    // fn find_quine(&)
    fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            x => panic!("Invalid combo operand: {x}"),
        }
    }
}

#[test]
fn test_part1() {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    assert_eq!(solve(input.to_string(), false).0, "4,6,3,5,6,3,5,2,1,0")
}

#[test]
fn test_registers() {
    // If register C contains 9, the program 2,6 would set register B to 1.
    let mut m =
        Machine::new("Register A: 0\nRegister B: 0\nRegister C: 9\n\nProgram: 2,6".to_string());
    m.run();
    assert_eq!(m.b, 1);

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    let mut m = Machine::new(
        "Register A: 10\nRegister B: 0\nRegister C: 0\n\nProgram: 5,0,5,1,5,4".to_string(),
    );
    assert_eq!(m.run().iter().join(","), "0,1,2");

    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    let mut m = Machine::new(
        "Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0".to_string(),
    );
    assert_eq!(m.run().iter().join(","), "4,2,5,6,7,7,7,7,3,1,0");
    assert_eq!(m.a, 0);

    // If register B contains 29, the program 1,7 would set register B to 26.
    let mut m =
        Machine::new("Register A: 0\nRegister B: 29\nRegister C: 0\n\nProgram: 1,7".to_string());
    m.run();
    assert_eq!(m.b, 26);

    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    let mut m = Machine::new(
        "Register A: 0\nRegister B: 2024\nRegister C: 43690\n\nProgram: 4,0".to_string(),
    );
    m.run();
    assert_eq!(m.b, 44354);
}
