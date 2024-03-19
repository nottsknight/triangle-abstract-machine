use std::collections::HashMap;

use common::instruction::Instruction;

use crate::InstrData;

pub fn gen_code(data: Vec<InstrData>) -> Vec<Instruction> {
    let label_indices = get_label_indies(&data);
    let named_dests = set_named_dests(&label_indices, &data);
    named_dests.iter().map(|d| d.data).collect()
}

fn get_label_indies(data: &Vec<InstrData>) -> HashMap<String, usize> {
    let mut indices = HashMap::new();
    for (i, instr) in data.iter().enumerate() {
        if let Some(lbl) = &instr.label {
            indices.insert(lbl.clone(), i);
        }
    }

    indices
}

fn set_named_dests(
    indices: &HashMap<String, usize>,
    data: &Vec<InstrData>,
) -> Vec<InstrData> {
    let mut new_data = Vec::new();
    for d in data {
        if let None = d.named_dest {
            new_data.push((*d).clone());
            continue;
        }

        let mut d1 = d.clone();
        let offset = indices.get(d1.named_dest.as_ref().unwrap());
        match offset {
            None => panic!("use of undefined location: {}", d1.named_dest.unwrap()),
            Some(i) => {
                d1.data.r = 0;
                d1.data.d = *i as i16;
                new_data.push(d1);
            }
        }
    }

    new_data
}
