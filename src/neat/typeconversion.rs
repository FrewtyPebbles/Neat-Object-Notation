use super::datatypes::VType;

pub fn determine_type(v_type: VType, raw_type: String) -> VType {
    let mut ret_type: VType = VType::Blank;
    //println!("{}", raw_type);
    match v_type {
        VType::Int(_) => {
			ret_type = VType::Int(raw_type.parse::<i64>().unwrap());
		},
        VType::Float(_) => {
			ret_type = VType::Float(raw_type.parse::<f64>().unwrap());
		},
        _ => todo!()
    }

    return ret_type;
}
