use std::io::{Cursor, Read};
use std::str::FromStr;

use byteorder::{ReadBytesExt, BE};

use crate::errors::{TAMError, TAMResult};
use crate::instruction::Instruction;

const MEM_SIZE: usize = 65535;

const CT: usize = 1;
const PB: usize = 2;
const PT: usize = 3;
const SB: usize = 4;
const ST: usize = 5;
const HB: usize = 6;
const HT: usize = 7;
const LB: usize = 8;
const CP: usize = 15;

/// TAM emulator
pub struct TAM {
    code: [u32; MEM_SIZE],
    data: [i16; MEM_SIZE],
    registers: [usize; 16],
    trace: bool,
}

impl TAM {
    /// Construct a new TAM emulator.
    ///
    /// # Arguments
    /// - `trace`: specify if a trace should be printed during execution
    pub fn new(trace: bool) -> TAM {
        let mut tam = TAM {
            code: [0; MEM_SIZE],
            data: [0; MEM_SIZE],
            registers: [0; 16],
            trace,
        };

        tam.registers[PB] = MEM_SIZE - 29;
        tam.registers[PT] = MEM_SIZE - 1;
        tam.registers[HB] = MEM_SIZE - 1;
        tam.registers[HT] = MEM_SIZE - 1;
        tam
    }

    /// Load a program from a file.
    ///
    /// This method clears the code store before loading.
    pub fn load_program(&mut self, filename: &str) -> std::io::Result<()> {
        let bytes = std::fs::read(filename)?;
        let mut bytes = Cursor::new(bytes);

        self.code.fill(0);
        self.registers[CT] = 0;
        while let Ok(instr) = bytes.read_u32::<BE>() {
            self.code[self.registers[CT]] = instr;
            self.registers[CT] += 1;
        }
        Ok(())
    }

    /// Run the loaded program.
    ///
    /// This method clears the data store before running.
    pub fn run(&mut self) -> TAMResult<()> {
        self.data.fill(0);
        self.registers[CP] = 0;
        loop {
            let instr = self.fetch_decode();
            if self.trace {
                println!("{:08x}: {:?}", self.registers[CP] - 1, instr);
                println!("{:?}", self.data[..self.registers[ST]].to_vec());
                println!(
                    "SB[{:x}] LB[{:x}] ST[{:x}]",
                    self.registers[SB], self.registers[LB], self.registers[ST]
                );
            }

            if instr.op == 15 {
                return Ok(());
            }
            self.execute(instr)?;
        }
    }

    fn fetch_decode(&mut self) -> Instruction {
        let instr = self.code[self.registers[CP]];
        self.registers[CP] += 1;
        Instruction::from(instr)
    }

    fn execute(&mut self, instr: Instruction) -> TAMResult<()> {
        match instr.op {
            0 => self.exec_load(instr),
            1 => self.exec_loada(instr),
            2 => self.exec_loadi(instr),
            3 => self.exec_loadl(instr),
            4 => self.exec_store(instr),
            5 => self.exec_storei(instr),
            6 => self.exec_call(instr),
            7 => self.exec_calli(instr),
            8 => self.exec_return(instr),
            10 => self.exec_push(instr),
            11 => self.exec_pop(instr),
            12 => self.exec_jump(instr),
            13 => self.exec_jumpi(instr),
            14 => self.exec_jumpif(instr),
            _ => Ok(()),
        }
    }

    fn push_data(&mut self, dat: i16) {
        self.data[self.registers[ST]] = dat;
        self.registers[ST] += 1;
    }

    fn pop_data(&mut self) -> i16 {
        self.registers[ST] -= 1;
        self.data[self.registers[ST]]
    }

    fn check_addr(&self, addr: usize) -> TAMResult<()> {
        if addr < self.registers[ST] || addr > self.registers[HT] {
            Ok(())
        } else {
            Err(TAMError::SegmentationFault(self.registers[CP] - 1, addr))
        }
    }

    fn check_stack(&self) -> TAMResult<()> {
        if self.registers[ST] < self.registers[HT] {
            Ok(())
        } else {
            Err(TAMError::StackOverflow(self.registers[CP] - 1))
        }
    }

    fn get_addr(&self, instr: Instruction) -> usize {
        self.registers[instr.r as usize].wrapping_add_signed(instr.d as isize)
    }

    fn exec_load(&mut self, instr: Instruction) -> TAMResult<()> {
        let mut addr = self.get_addr(instr);
        for _ in 0..instr.n {
            self.check_addr(addr)?;
            let dat = self.data[addr];
            self.push_data(dat);
            addr += 1;
        }
        self.check_stack()
    }

    fn exec_loada(&mut self, instr: Instruction) -> TAMResult<()> {
        let addr = self.get_addr(instr);
        self.check_addr(addr)?;
        self.push_data(addr as i16);
        self.check_stack()
    }

    fn exec_loadi(&mut self, instr: Instruction) -> TAMResult<()> {
        let mut addr = self.pop_data() as usize;
        for _ in 0..instr.n {
            self.check_addr(addr)?;
            let dat = self.data[addr];
            self.push_data(dat);
            addr += 1;
        }
        self.check_stack()
    }

    fn exec_loadl(&mut self, instr: Instruction) -> TAMResult<()> {
        self.push_data(instr.d);
        self.check_stack()
    }

    fn exec_store(&mut self, instr: Instruction) -> TAMResult<()> {
        let mut addr = self.get_addr(instr);
        for _ in 0..instr.n {
            self.check_addr(addr)?;
            let dat = self.pop_data();
            self.data[addr] = dat;
            addr += 1;
        }
        self.check_stack()
    }

    fn exec_storei(&mut self, instr: Instruction) -> TAMResult<()> {
        let mut addr = self.pop_data() as usize;
        for _ in 0..instr.n {
            self.check_addr(addr)?;
            let dat = self.pop_data();
            self.data[addr] = dat;
            addr += 1;
        }
        self.check_stack()
    }

    fn exec_call(&mut self, instr: Instruction) -> TAMResult<()> {
        if (instr.r as usize) == PB {
            match instr.d {
                1 => self.call_id(),
                2 => self.call_not(),
                3 => self.call_and(),
                4 => self.call_or(),
                5 => self.call_inc(),
                6 => self.call_dec(),
                7 => self.call_neg(),
                8 => self.call_add(),
                9 => self.call_sub(),
                10 => self.call_mul(),
                11 => self.call_div()?,
                12 => self.call_mod()?,
                13 => self.call_lt(),
                14 => self.call_le(),
                15 => self.call_ge(),
                16 => self.call_gt(),
                21 => self.call_get()?,
                22 => self.call_put(),
                23 => self.call_geteol(),
                24 => self.call_puteol(),
                25 => self.call_getint(),
                26 => self.call_putint(),
                27 => self.call_new(),
                _ => (),
            }
            Ok(())
        } else {
            let addr = self.get_addr(instr);
            if addr >= self.registers[CT] {
                return Err(TAMError::SegmentationFault(self.registers[CP] - 1, addr));
            }

            let static_link = self.registers[instr.n as usize];
            let dynamic_link = self.registers[LB];
            let ret_addr = self.registers[CP];

            self.push_data(static_link as i16);
            self.push_data(dynamic_link as i16);
            self.push_data(ret_addr as i16);
            self.check_stack()?;

            self.registers[LB] = self.registers[ST] - 3;
            self.registers[CP] = addr;

            if self.trace {
                println!("          slnk: {:08x}", self.data[self.registers[LB]]);
                println!("          dlnk: {:08x}", self.data[self.registers[LB] + 1]);
                println!("          radr: {:08x}", self.data[self.registers[LB] + 2]);
            }
            Ok(())
        }
    }

    fn exec_calli(&mut self, _instr: Instruction) -> TAMResult<()> {
        todo!("calli not implemented yet");
    }

    fn exec_return(&mut self, instr: Instruction) -> TAMResult<()> {
        let ret_addr = self.data[self.registers[LB] + 2] as usize;
        if ret_addr >= self.registers[CT] {
            return Err(TAMError::SegmentationFault(
                self.registers[CP] - 1,
                ret_addr,
            ));
        }

        let mut ret_val = Vec::new();
        for _ in 0..instr.n {
            ret_val.push(self.pop_data());
        }

        while self.registers[ST] > self.registers[LB] {
            self.pop_data();
        }

        for _ in 0..instr.d {
            self.pop_data();
        }

        while let Some(val) = ret_val.pop() {
            self.push_data(val);
        }

        self.registers[CP] = ret_addr;
        self.registers[LB] = self.data[self.registers[LB] + 1] as usize;
        Ok(())
    }

    fn exec_push(&mut self, instr: Instruction) -> TAMResult<()> {
        self.registers[ST] += instr.d as usize;
        self.check_stack()
    }

    fn exec_pop(&mut self, _instr: Instruction) -> TAMResult<()> {
        todo!("pop not yet implemented");
    }

    fn exec_jump(&mut self, instr: Instruction) -> TAMResult<()> {
        let addr = self.get_addr(instr);
        if addr >= self.registers[CT] {
            return Err(TAMError::SegmentationFault(self.registers[CP] - 1, addr));
        }

        self.registers[CP] = addr;
        Ok(())
    }

    fn exec_jumpi(&mut self, _: Instruction) -> TAMResult<()> {
        let addr = self.pop_data() as usize;
        if addr >= self.registers[CT] {
            Err(TAMError::SegmentationFault(self.registers[CP] - 1, addr))
        } else {
            self.registers[CP] = addr;
            Ok(())
        }
    }

    fn exec_jumpif(&mut self, instr: Instruction) -> TAMResult<()> {
        let val = self.pop_data();
        if val == instr.n as i16 {
            let addr = self.get_addr(instr);
            if addr >= self.registers[CT] {
                return Err(TAMError::SegmentationFault(self.registers[CP] - 1, addr));
            }
            self.registers[CP] = addr;
        }
        Ok(())
    }

    fn call_id(&mut self) {
        let val = self.pop_data();
        self.push_data(val);
    }

    fn call_not(&mut self) {
        let val = self.pop_data();
        self.push_data(if val == 0 { 1 } else { 0 });
    }

    fn call_and(&mut self) {
        let t2 = self.pop_data();
        let t1 = self.pop_data();
        self.push_data(if t1 * t2 == 0 { 0 } else { 1 });
    }

    fn call_or(&mut self) {
        let t2 = self.pop_data();
        let t1 = self.pop_data();
        self.push_data(if t1 + t2 == 0 { 0 } else { 1 });
    }

    fn call_inc(&mut self) {
        let val = self.pop_data();
        self.push_data(val + 1);
    }

    fn call_dec(&mut self) {
        let val = self.pop_data();
        self.push_data(val - 1);
    }

    fn call_neg(&mut self) {
        let val = self.pop_data() as i16;
        self.push_data(-val);
    }

    fn call_add(&mut self) {
        let t2 = self.pop_data();
        let t1 = self.pop_data();
        self.push_data(t1.overflowing_add(t2).0);
    }

    fn call_sub(&mut self) {
        let t2 = self.pop_data();
        let t1 = self.pop_data();
        self.push_data(t1.overflowing_sub(t2).0);
    }

    fn call_mul(&mut self) {
        let t2 = self.pop_data();
        let t1 = self.pop_data();
        self.push_data(t1.overflowing_mul(t2).0);
    }

    fn call_div(&mut self) -> TAMResult<()> {
        let t2 = self.pop_data();
        if t2 == 0 {
            return Err(TAMError::DivideByZero(self.registers[CP] - 1));
        }

        let t1 = self.pop_data();
        self.push_data(t1.overflowing_div(t2).0);
        Ok(())
    }

    fn call_mod(&mut self) -> TAMResult<()> {
        let t2 = self.pop_data();
        if t2 == 0 {
            return Err(TAMError::DivideByZero(self.registers[CP] - 1));
        }

        let t1 = self.pop_data();
        self.push_data(t1 % t2);
        Ok(())
    }

    fn call_lt(&mut self) {
        let t2 = self.pop_data();
        let t1 = self.pop_data();
        self.push_data(if t1 < t2 { 1 } else { 0 });
    }

    fn call_le(&mut self) {
        let t2 = self.pop_data();
        let t1 = self.pop_data();
        self.push_data(if t1 <= t2 { 1 } else { 0 });
    }

    fn call_ge(&mut self) {
        let t2 = self.pop_data();
        let t1 = self.pop_data();
        self.push_data(if t1 >= t2 { 1 } else { 0 });
    }

    fn call_gt(&mut self) {
        let t2 = self.pop_data();
        let t1 = self.pop_data();
        self.push_data(if t1 > t2 { 1 } else { 0 });
    }

    fn call_getint(&mut self) {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let input = i16::from_str(buffer.trim()).unwrap();
        let addr = self.pop_data() as usize;
        self.data[addr] = input;
    }

    fn call_putint(&mut self) {
        let val = self.pop_data();
        print!("{}", val);
    }

    fn call_puteol(&self) {
        print!("\n");
    }

    fn call_get(&mut self) -> TAMResult<()> {
        let mut input = [0u8; 1];
        std::io::stdin().read(&mut input[..]).unwrap();
        let addr = self.pop_data() as usize;
        self.check_addr(addr)?;
        self.data[addr] = input[0] as i16;
        Ok(())
    }

    fn call_put(&mut self) {
        let c = self.pop_data() as u8;
        print!("{}", c as char);
    }

    fn call_geteol(&self) {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
    }

    fn call_new(&mut self) {
        let n = self.pop_data() as usize;
        self.registers[HT] -= n;
        self.push_data((self.registers[HT] + 1) as i16);
    }
}
