#![allow(unused_variables, non_snake_case)]

#[derive(Debug)]
pub struct StatusRegister {
    CarryFlag: bool,
    ZeroFlag: bool,
    OverflowFlag: bool,
}

impl StatusRegister {
    fn new() -> Self {
        StatusRegister {
            CarryFlag: true,
            ZeroFlag: false,
            OverflowFlag: true,
        }
    }

    fn iter_mut(&mut self) -> std::vec::IntoIter<&mut bool> {
        vec![
            &mut self.CarryFlag,
            &mut self.ZeroFlag,
            &mut self.OverflowFlag,
        ]
        .into_iter()
    }
}

pub struct CPU {
    pub memory: [u8; 0xffff],
    pub status: StatusRegister,
}

impl CPU {
    pub fn new() -> CPU {
        let memory = [0; 0xFFFF];
        CPU {
            memory,
            status: StatusRegister::new(),
        }
    }

    fn execute(&mut self) {
        let mut shifter = 0b1000_0000;
        println!("Before: {:?}", self.status);
        for status in self.status.iter_mut() {
            // mut status here!
            *status = !*status;
            println!("{}", status);
            shifter <<= 1;
        }
        println!("After: {:?}", self.status);
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.execute();
}
