
pub struct VM {
    pc: isize,
    acc: isize,
    /// Vector of decoded instructions, and a couple tags needed for part 2
    /// This should really be a struct but oh well
    ///
    /// Insn: Opcode
    /// isize: operand
    /// first bool: Whether this instruction has been swapped in this loop of the code
    /// second bool: Whether this instruction has been swapped ever
    code: Vec<(Insn, isize, bool, bool)>,
}

impl VM {
    pub fn new(asm: &Vec<String>) -> VM {
        let mut code = vec![];
        for line in asm {
            let opcode = &line[..3];
            let operand = &line[4..];
            code.push((Insn::from_str(opcode), operand.trim().parse().unwrap(), false, false))
        }

        VM {
            pc: 0,
            acc: 0,
            code
        }
    }

    pub fn exec(&mut self, insn: (Insn, isize, bool, bool)) -> bool{
        match insn.0 {
            Insn::Acc => {
                self.acc += insn.1;
                true
            }
            Insn::Jmp => {
                self.pc += insn.1;
                false
            }
            Insn::Nop => true
        }
    }

    pub fn rst(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Insn {
    Acc,
    Jmp,
    Nop
}

impl Insn {
    fn from_str(s: &str) -> Insn {
        match s {
            "nop" => Insn::Nop,
            "acc" => Insn::Acc,
            "jmp" => Insn::Jmp,
            x => unreachable!("{}", x)
        }
    }

    fn can_be_swapped(&self) -> bool {
        match self {
            Insn::Acc => false,
            _ => true
        }
    }
}

pub fn main() {
    let contents = std::fs::read_to_string("inputs/day8").unwrap().lines().map(ToString::to_string).collect::<Vec<_>>();

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &Vec<String>) {
    let mut vm = VM::new(contents);
    let mut seen = vec![];

    loop {
        if seen.contains(&vm.pc) {
            println!("{}", vm.acc);
            break;
        }

        let code = vm.code[vm.pc as usize];
        seen.push(vm.pc);
        if vm.exec(code) {
            vm.pc += 1;
        }

    }
}

fn part2(contents: &Vec<String>) {
    let mut vm = VM::new(contents);
    let mut seen = vec![];

    loop {
        if seen.contains(&vm.pc) {
            // Find the first instruction that isn't currently swapped, and doesn't have the sticky flag set
            // Sticky flag exists to prevent loops where the first nop/jmp in the input is swapped continuously
            let (i, (swap_insn, _, swapped, _)) = vm.code.iter_mut().enumerate().find(|(_, (insn, _, sw, sticky))| insn.can_be_swapped() && !*sw && !*sticky).unwrap();
            // Swap the instruction
            if *swap_insn == Insn::Nop {
                *swap_insn = Insn::Jmp;
            } else {
                *swap_insn = Insn::Nop;
            }
            *swapped = true;
            // Find the first instruction with the swap flag set. Since this is starting from 0 it'll either be before or equal to the instruction just swapped
            if let Some((j, (insn, _, sw, sticky))) = vm.code.iter_mut().enumerate().find(|(_, (_, _, sw, _))| *sw) {
                // Swap if the indices dont match
                if i != j {
                    if *insn == Insn::Nop {
                        *insn = Insn::Jmp;
                    } else {
                        *insn = Insn::Nop;
                    }
                    *sw = false;
                    // Set sticky so that this instruction isn't swapped again.
                    *sticky = true;
                }
            }
            vm.rst();
            seen.clear();
        }

        // None here means that we've reached the end of the vector, and so the halt condition.
        if let Some(code) = vm.code.get(vm.pc as usize) {
            seen.push(vm.pc);
            if vm.exec(*code) {
                vm.pc += 1;
            }
        } else {
            println!("{}", vm.acc);
            break;
        }

    }
}
