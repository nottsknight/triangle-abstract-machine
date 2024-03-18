use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub op: u8,
    pub r: u8,
    pub n: u8,
    pub d: i16,
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let op = (value & 0xf0000000) >> 28;
        let r = (value & 0x0f000000) >> 24;
        let n = (value & 0x00ff0000) >> 16;
        let d = value & 0x0000ffff;
        Instruction {
            op: op as u8,
            r: r as u8,
            n: n as u8,
            d: d as i16,
        }
    }
}

fn get_reg_name(r: u8) -> String {
    let rname = match r {
        0 => "cb",
        1 => "ct",
        2 => "pb",
        3 => "pt",
        4 => "sb",
        5 => "st",
        6 => "hb",
        7 => "ht",
        8 => "lb",
        9 => "l1",
        10 => "l2",
        11 => "l3",
        12 => "l4",
        13 => "l5",
        14 => "l6",
        15 => "cp",
        _ => "",
    };
    String::from(rname)
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Instruction { op, r, n, d } = self;
        let reg_name = get_reg_name(*r);
        match op {
            0 => write!(f, "load    {n}, [{reg_name}{d:+}]"),
            1 => write!(f, "loada   [{reg_name}{d:+}]"),
            2 => write!(f, "loadi   {n}"),
            3 => write!(f, "loadl   {d}"),
            4 => write!(f, "store   {n}, [{reg_name}{d:+}]"),
            5 => write!(f, "storei"),
            6 => write!(f, "call    [{reg_name}{d:+}]"),
            7 => write!(f, "calli"),
            8 => write!(f, "return  {n}, {d}"),
            10 => write!(f, "push    {d}"),
            11 => write!(f, "pop"),
            12 => write!(f, "jump    [{reg_name}{d:+}]"),
            13 => write!(f, "jumpi"),
            14 => write!(f, "jumpif  {n}, [{reg_name}{d:+}]"),
            15 => write!(f, "halt"),
            _ => write!(f, ""),
        }
    }
}
