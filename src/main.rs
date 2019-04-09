#[derive(Debug)]
enum InstructionSet{
    Mov = 1,
    Show = 2,
    Add = 3,
    Addl = 4,
    Nop = 5,
    Sto = 6,
    Movl = 7,
    Halt = 0
}

impl InstructionSet{
    pub fn from_i16(n:i16)->InstructionSet{
        match n {
            0 => InstructionSet::Halt,
            1 => InstructionSet::Mov,
            2 => InstructionSet::Show,
            3 => InstructionSet::Add,
            4 => InstructionSet::Addl,
            5 => InstructionSet::Nop,
            6 => InstructionSet::Sto,
            7 => InstructionSet::Movl,
            _ => InstructionSet::Halt,
        }
    }
}

trait Cpu{
    fn load(& mut self, new_mem: Vec<u32>);
    fn fetch(& mut self) -> u32;
    fn decode(& mut self, reg: u32) -> (InstructionSet, i16);
    fn execute(& mut self, instruction: InstructionSet, operand:i16);
    fn disassemble(& mut self);
}
 
 
struct Simpletron{
    prog_mem: Vec<u32>,
    memory : Vec<i16>,
    working_reg : i16,
    program_counter : usize,
    stop : bool,
}

impl Simpletron{
    fn new()->Simpletron{
        Simpletron{ prog_mem: vec![0; 1024], memory:vec![0; 1024], working_reg: 0, program_counter: 0, stop : false }
    }
}

impl Cpu for Simpletron{
    fn load(& mut self, new_mem: Vec<u32>) {
        self.prog_mem[..new_mem.len()].copy_from_slice(&new_mem);
    }

    fn fetch(& mut self) -> u32{
        let rc = self.prog_mem[self.program_counter];
        self.program_counter += 1;
        rc
    }

    fn decode(& mut self, reg: u32) -> (InstructionSet, i16) {
        let upper = reg & 0xffff0000;
        let upper = upper.rotate_right(16) as i16;
        let upper = InstructionSet::from_i16(upper);
        let operand = (reg & 0xffff) as i16;
        (upper, operand)
    }

    fn execute(& mut self, instruction: InstructionSet, operand : i16) {
        match instruction {
            InstructionSet::Mov => self.working_reg = self.memory[operand as usize],
            InstructionSet::Movl=>self.working_reg = operand,
            InstructionSet::Show => println!("memory[{}] = {}", operand, self.memory[operand as usize]),
            InstructionSet::Addl => self.working_reg += operand,
            InstructionSet::Sto => self.memory[operand as usize] = self.working_reg,        
            InstructionSet::Halt => { println!("Halting"); self.stop = true},
            _ => {
                println!("Un processed instruction"); 
                self.stop = true;
            }
        }
    }

    fn disassemble(& mut self) {
        loop{
            let f = self.fetch();
            let f = self.decode(f);
            self.execute(f.0, f.1);
            if self.stop == true {
                break;
            }
        }
    }
}

fn main() {
    // Load w with 1, Add 1 to it, Store it in location 0, then Show location 0
    let test_prog = vec![0x00070001, 0x00040001, 0x00060000, 0x00020000, 0];
    println!("Hello, world!");
    let mut s = Simpletron::new();
    s.load(test_prog);
    s.disassemble();
}
