#[derive(Debug)]
struct Computer {
    registers: [i64; 3],
    program: Vec<i64>,
    pointer: usize,
    output: Vec<i64>,
}

impl Computer {
    fn new(registers: [i64; 3], program: Vec<i64>) -> Self {
        Self {
            registers,
            program,
            pointer: 0,
            output: vec![],
        }
    }

    fn literal_operand(&self) -> i64 {
        self.program[self.pointer + 1]
    }

    fn combo_operand(&self) -> i64 {
        let operand = self.program[self.pointer + 1];
        match operand {
            0..=3 => operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            7 => panic!("Invalid combo operand"),
            _ => unreachable!(),
        }
    }

    fn adv(&mut self) {
        let lhs = self.registers[0];
        let rhs = 2_i64.pow(self.combo_operand() as u32);
        self.registers[0] = lhs / rhs;
        self.pointer += 2;
    }

    fn bxl(&mut self) {
        let lhs = self.registers[1];
        let rhs = self.literal_operand();
        self.registers[1] = lhs ^ rhs;
        self.pointer += 2;
    }

    fn bst(&mut self) {
        let lhs = self.combo_operand();
        self.registers[1] = lhs % 8;
        self.pointer += 2;
    }

    fn jnz(&mut self) {
        if self.registers[0] == 0 {
            self.pointer += 2;
            return;
        }

        self.pointer = self.literal_operand() as usize;
    }

    fn bxc(&mut self) {
        self.registers[1] = self.registers[1] ^ self.registers[2];
        self.pointer += 2;
    }

    fn out(&mut self) {
        let lhs = self.combo_operand();
        self.output.push(lhs % 8);
        self.pointer += 2;
    }

    fn bdv(&mut self) {
        let lhs = self.registers[0];
        let rhs = 2_i64.pow(self.combo_operand() as u32);
        self.registers[1] = lhs / rhs;
        self.pointer += 2;
    }

    fn cdv(&mut self) {
        let lhs = self.registers[0];
        let rhs = 2_i64.pow(self.combo_operand() as u32);
        self.registers[2] = lhs / rhs;
        self.pointer += 2;
    }

    fn run(&mut self) {
        self.pointer = 0;
        self.output = vec![];
        while self.pointer < self.program.len() {
            let opcode = self.program[self.pointer];
            match opcode {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => unreachable!(),
            };
        }
    }

    fn output(&self) -> String {
        self.output
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn parse_input(s: &str) -> Result<([i64; 3], Vec<i64>), String> {
    let (registers_str, program_str) = s
        .split_once("\n\n")
        .ok_or("Missing registers/program delimiter")?;

    let registers = registers_str
        .lines()
        .filter_map(|line| {
            let (_, val) = line.split_once(": ")?;
            val.parse::<i64>().ok()
        })
        .collect::<Vec<_>>();

    let registers: [i64; 3] = registers
        .try_into()
        .map_err(|_| "Invalid registers length".to_string())?;

    let program = program_str
        .split_once(": ")
        .ok_or("Missing program delimiter")?
        .1
        .split(",")
        .filter_map(|v| v.parse::<i64>().ok())
        .collect::<Vec<_>>();

    Ok((registers, program))
}

fn part1(input: &str) {
    let (registers, program) = parse_input(&input).expect("Failed to parse input");
    let mut computer = Computer::new(registers.clone(), program.clone());
    computer.run();
    let output = computer.output();
    println!("part 1: {}", output);
}

fn part2(input: &str) {
    let (registers, program) = parse_input(&input).expect("Failed to parse input");

    let mut valid = vec![0];

    for &out in program.iter().rev() {
        let mut next = Vec::new();

        for v in valid {
            for n in 0..8 {
                let a = (v << 3) | n;
                let mut computer = Computer::new([a, registers[1], registers[2]], program.clone());
                computer.run();
                let result = computer.output[0];
                if result == out {
                    next.push(a);
                }
            }
        }

        valid = next;
    }

    println!("part 2: {}", valid.iter().min().unwrap());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    part1(&input);
    part2(&input);
}
