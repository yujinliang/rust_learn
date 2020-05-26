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

    fn iter_mut(&mut self) -> StatusRegisterIterMut {
        StatusRegisterIterMut {
            status: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a mut StatusRegister {
    type IntoIter = StatusRegisterIterMut<'a>;
    type Item = &'a mut bool;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct StatusRegisterIterMut<'a> {
    status: &'a mut StatusRegister,
    index: usize,
}

impl<'a> Iterator for StatusRegisterIterMut<'a> {
    type Item = &'a mut bool;

    // Invariant to keep: index is 0, 1, 2 or 3
    // Every call, this increments by one, capped at 3
    // index should never be 0 on two different calls
    // and similarly for 1 and 2.
    fn next(&mut self) -> Option<Self::Item> {
        let result = unsafe {
            match self.index {
                // Safety: Since each of these three branches are
                // executed exactly once, we hand out no more than one mutable reference
                // to each part of self.status
                // Since self.status is valid for 'a
                // Each partial borrow is also valid for 'a
                0 => &mut *(&mut self.status.CarryFlag as *mut _),
                1 => &mut *(&mut self.status.ZeroFlag as *mut _),
                2 => &mut *(&mut self.status.OverflowFlag as *mut _),
                _ => return None,
            }
        };
        // If self.index isn't 0, 1 or 2, we'll have already returned
        // So this bumps us up to 1, 2 or 3.
        self.index += 1;
        Some(result)
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
