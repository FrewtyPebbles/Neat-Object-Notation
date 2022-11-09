use std::collections::HashMap;

use crate::lib::datatypes::VType;

use super::datatypes::{Token, SerializedNode, NDSType, PTok, NDSKeyType};

pub fn build_tree(token_list: Vec<Box<Token>>, is_dict:bool) -> Box<SerializedNode> {
	let mut tree_stack:Vec<Box<SerializedNode>> = vec![];
	let mut key_stack:Vec<NDSKeyType> = vec![NDSKeyType::Blank];
	if is_dict {
		let inner_hm:HashMap<NDSKeyType, Box<SerializedNode>> = HashMap::new();
		tree_stack.push(Box::new(SerializedNode { value: NDSType::Hashmap(inner_hm) }));
	}
	else {
		let inner_vec:Vec<Box<SerializedNode>> = Vec::new();
		tree_stack.push(Box::new(SerializedNode { value: NDSType::List(inner_vec) }));
	}

	for (tn, tok) in token_list.iter().enumerate() {
		let curr_tok = *tok.clone();

		match curr_tok.tok {
			PTok::SList => {
				match curr_tok.v_type {
					VType::Blank => {
						key_stack.push(NDSKeyType::Blank);
					},
					VType::Bool(val) => {
						key_stack.push(NDSKeyType::Bool(val));
					},
					VType::Int(val) => {
						key_stack.push(NDSKeyType::Int(val));
					},
					VType::Float(_) => {},
					VType::String(val) => {
						key_stack.push(NDSKeyType::Str(val));
					},
					VType::Null => {
						key_stack.push(NDSKeyType::Null);
					},
				}
				let inner_vec:Vec<Box<SerializedNode>> = Vec::new();
				tree_stack.push(Box::new(SerializedNode { value: NDSType::List(inner_vec) }))
			},
			PTok::EList => {
				let stack_len = tree_stack.len();
				if stack_len > 1 {
					let new_value = Box::new(*tree_stack[stack_len-1].clone());
					match &mut tree_stack[stack_len - 2].value {
						
						NDSType::List(vector) =>{
							vector.push(new_value);
							key_stack.pop();
							tree_stack.pop();
						},
						NDSType::Hashmap(hashmap) =>{
							hashmap.insert(key_stack.last().unwrap().clone(), new_value);
							key_stack.pop();
							tree_stack.pop();
						},
						_ => {}
					}
				}
			},
			PTok::SSection => {
				match curr_tok.v_type {
					VType::Blank => {
						key_stack.push(NDSKeyType::Blank);
					},
					VType::Bool(val) => {
						key_stack.push(NDSKeyType::Bool(val));
					},
					VType::Int(val) => {
						key_stack.push(NDSKeyType::Int(val));
					},
					VType::Float(_) => {},
					VType::String(val) => {
						key_stack.push(NDSKeyType::Str(val));
					},
					VType::Null => key_stack.push(NDSKeyType::Null),
				}
				let inner_hm:HashMap<NDSKeyType, Box<SerializedNode>> = HashMap::new();
				tree_stack.push(Box::new(SerializedNode { value: NDSType::Hashmap(inner_hm) }));
			},
			PTok::ESection => {
				let stack_len = tree_stack.len();
				if stack_len > 1 {
					let new_value = Box::new(*tree_stack[stack_len-1].clone());
					match &mut tree_stack[stack_len - 2].value {
						
						NDSType::List(vector) =>{
							if key_stack.last().unwrap().clone() == NDSKeyType::Blank {
								vector.push(new_value);
							}
							else {
								let inner_hm:HashMap<NDSKeyType, Box<SerializedNode>> = HashMap::new();
								let mut new_hm = Box::new(SerializedNode { value: NDSType::Hashmap(inner_hm) });
								match &mut new_hm.value {
									NDSType::Hashmap(hm) => {
										hm.insert(key_stack.last().unwrap().clone(), new_value);
									},
									_ => {

									}
								}
								vector.push(new_hm);
							}
						},
						NDSType::Hashmap(hashmap) =>{
							hashmap.insert(key_stack.last().unwrap().clone(), new_value);
							
						},
						_ => {}
					}
					key_stack.pop();
					tree_stack.pop();
				}
			},
			PTok::Setter => {
				match curr_tok.v_type {
					VType::Blank => key_stack.push(NDSKeyType::Null),
					VType::Bool(val) => key_stack.push(NDSKeyType::Bool(val)),
					VType::Int(val) => key_stack.push(NDSKeyType::Int(val)),
					VType::String(val) => key_stack.push(NDSKeyType::Str(val)),
					VType::Null => key_stack.push(NDSKeyType::Null),
					VType::Float(_) => {
						//add an error handling system
						eprintln!("Error: Float is not a valid key type.")
					},
				}
			},
			PTok::Literal => {
				//if the last token is a setter then make it a single hashed item
				let tlist_len = token_list.len();
				let stack_len = tree_stack.len();
				if tn > 1 {
					if tlist_len > 1 && token_list[tn - 2].tok == PTok::Setter {
						match &mut tree_stack[stack_len-1].value {
							NDSType::Hashmap(hm) => {
								match curr_tok.v_type {
									VType::Blank => {},
									VType::Bool(val) => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Bool(val) }));
									},
									VType::Int(val) => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Int(val) }));
									},
									VType::Float(val) => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Float(val) }));
									},
									VType::String(val) => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Str(val) }));
									},
									VType::Null => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Null }));
									},
								}
								key_stack.pop();
							},
							univ => {
								eprintln!("Error: Attempted to supply a key inside a list. {:?}", univ);
							}
						}
					}
					else {
						match &mut tree_stack[stack_len-1].value {
							NDSType::List(vec) => {
								match curr_tok.v_type {
									VType::Blank => {},
									VType::Bool(val) => {
										vec.push(Box::new(SerializedNode { value: NDSType::Bool(val) }));
									},
									VType::Int(val) => {
										vec.push(Box::new(SerializedNode { value: NDSType::Int(val) }));
									},
									VType::Float(val) => {
										vec.push(Box::new(SerializedNode { value: NDSType::Float(val) }));
									},
									VType::String(val) => {
										vec.push(Box::new(SerializedNode { value: NDSType::Str(val) }));
									},
									VType::Null => {
										vec.push(Box::new(SerializedNode { value: NDSType::Null }));
									},
								}
							},
							NDSType::Hashmap(hm) => {
								match curr_tok.v_type {
									VType::Blank => {},
									VType::Bool(val) => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Bool(val) }));
									},
									VType::Int(val) => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Int(val) }));
									},
									VType::Float(val) => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Float(val) }));
									},
									VType::String(val) => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Str(val) }));
									},
									VType::Null => {
										hm.insert(key_stack.last().unwrap().clone(), Box::new(SerializedNode { value: NDSType::Null }));
									},
								}
								key_stack.pop();
							},
							univ => {
								eprintln!("Error: Key for {:?} supplied in a list. {:?}", curr_tok.v_type, univ);
							}
						}
					}
				}
				//else append it to the top list
			},
			_ => {}
		}
	}
	return tree_stack[0].clone();
}