use std::{collections::HashMap, slice::Iter};

use indexmap::IndexMap;

use crate::neat::datatypes::VType;

use super::{datatypes::{Token, SerializedNode, NDSType, PTok, NDSKeyType}, tokenizer::serialize};

fn alias_get(alias:String, aliases:HashMap<String, Vec<VType>>, mut curr_scope:&mut Vec<Box<SerializedNode>>){
	let curr_a_list = aliases[&alias].clone();
	//println!("{:?}", curr_a_list);
	for curr_key in curr_a_list.iter() {
		match &curr_scope[0].value {
			NDSType::Hashmap(val) => {
				match curr_key {
					VType::Bool(v) => {
						curr_scope[0] = val[&NDSKeyType::Bool(v.clone())].clone();
					},
					VType::Int(v) => {
						curr_scope[0] = val[&NDSKeyType::Int(v.clone())].clone();
					},
					VType::String(v) => {
						//println!("{:?}, k={}",val, v);
						curr_scope[0] = val[&NDSKeyType::Str(v.clone())].clone();
					},
					VType::Null => {
						curr_scope[0] = val[&NDSKeyType::Null].clone();
					},
					_ => {}
				}
			},
			NDSType::List(val) => {
				match curr_key {
					VType::Int(v) => {
						curr_scope[0] = val[*v as usize].clone();
					},
					_ => {}
				}
			},
			_ => {},
		}
	}
}

fn alias_set(aliases:Vec<VType>, mut curr_scope:&mut Box<SerializedNode>, set_scope:SerializedNode) {
	
	fn search(scope_ref: &mut SerializedNode, mut curr_a_list:Iter<VType>, set_scope:SerializedNode){
		let mut a_list = VType::Blank;
		if !curr_a_list.clone().next().is_none() {
			a_list = curr_a_list.next().unwrap().clone();
		}
		else {
			*scope_ref = set_scope;
			return;
		}
		match &mut scope_ref.value {
			NDSType::Hashmap(val) => {
				match a_list {
					VType::Bool(v) => {
						search(&mut val[&NDSKeyType::Bool(v.clone())], curr_a_list, set_scope);
					},
					VType::Int(v) => {
						search(&mut val[&NDSKeyType::Int(v.clone())], curr_a_list, set_scope);
					},
					VType::String(v) => {
						search(&mut val[&NDSKeyType::Str(v.clone())], curr_a_list, set_scope);
					},
					VType::Null => {
						search(&mut val[&NDSKeyType::Null], curr_a_list, set_scope);
					},
					_ => {}
				}
			},
			NDSType::List(val) => {
				match a_list {
					VType::Int(v) => {
						search(&mut val[v as usize], curr_a_list, set_scope);
					},
					_ => {}
				}
			},
			_ => {},
		}
	}

	let mut curr_a_list = aliases.iter();

	search(&mut curr_scope, curr_a_list, set_scope);
	//println!("{:?}", curr_scope);
}

pub fn build_tree(token_list: Vec<Box<Token>>, is_dict:bool, file_path:&str, alias_vec:&HashMap<String, Vec<VType>>) -> Box<SerializedNode> {
	let mut tree_stack:Vec<Box<SerializedNode>> = vec![];
	let mut key_stack:Vec<NDSKeyType> = vec![];
	let mut aliases:HashMap<String, Vec<VType>> = alias_vec.clone();
	let mut current_used_stack = &mut tree_stack;
	let mut alias_scope:(Vec<Vec<VType>>, Vec<Vec<Box<SerializedNode>>>) = (vec![], vec![]);// if not empty then use this scope instead
	let mut num_stack:Vec<i64> = vec![-1];
	if is_dict {
		let inner_hm:IndexMap<NDSKeyType, Box<SerializedNode>> = IndexMap::new();
		current_used_stack.push(Box::new(SerializedNode { value: NDSType::Hashmap(inner_hm) }));
	}
	else {
		let inner_vec:Vec<Box<SerializedNode>> = Vec::new();
		current_used_stack.push(Box::new(SerializedNode { value: NDSType::List(inner_vec) }));
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
						let num_stack_len = num_stack.len();
						num_stack[num_stack_len - 1] = val.clone();
						key_stack.push(NDSKeyType::Int(val));
					},
					VType::Float(_) => {},
					VType::String(val) => {
						key_stack.push(NDSKeyType::Str(val));
					},
					VType::Null => {
						key_stack.push(NDSKeyType::Null);
					},
					VType::Alias(_) => todo!()
				}
				let inner_vec:Vec<Box<SerializedNode>> = Vec::new();
				current_used_stack.push(Box::new(SerializedNode { value: NDSType::List(inner_vec) }))
			},
			PTok::EList => {
				let stack_len = current_used_stack.len();
				if stack_len > 1 {
					let new_value = Box::new(*current_used_stack[stack_len-1].clone());
					match &mut current_used_stack[stack_len - 2].value {
						
						NDSType::List(vector) =>{
							vector.push(new_value);
							key_stack.pop();
							current_used_stack.pop();
						},
						NDSType::Hashmap(hashmap) =>{
							if key_stack.len() > 0 {
								hashmap.insert(key_stack.last().unwrap().clone(), new_value);
								key_stack.pop();
								current_used_stack.pop();
							}
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
						let num_stack_len = num_stack.len();
						num_stack[num_stack_len - 1] = val.clone();
						key_stack.push(NDSKeyType::Int(val));
					},
					VType::Float(_) => {},
					VType::String(val) => {
						key_stack.push(NDSKeyType::Str(val));
					},
					VType::Null => key_stack.push(NDSKeyType::Null),
					VType::Alias(val) => {}
				}
				num_stack.push(-1);
				let inner_hm:IndexMap<NDSKeyType, Box<SerializedNode>> = IndexMap::new();
				current_used_stack.push(Box::new(SerializedNode { value: NDSType::Hashmap(inner_hm) }));
			},
			PTok::ESection => {
				let stack_len = current_used_stack.len();
				if stack_len > 1 {
					let new_value = Box::new(*current_used_stack[stack_len-1].clone());
					match &mut current_used_stack[stack_len - 2].value {
						
						NDSType::List(vector) =>{
							if key_stack.last().unwrap().clone() == NDSKeyType::Blank {
								vector.push(new_value);
							}
							else {
								let inner_hm:IndexMap<NDSKeyType, Box<SerializedNode>> = IndexMap::new();
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
					current_used_stack.pop();
				}
				num_stack.pop();
			},
			PTok::Setter => {
				match curr_tok.v_type {
					VType::Blank => key_stack.push(NDSKeyType::Null),
					VType::Bool(val) => key_stack.push(NDSKeyType::Bool(val)),
					VType::Int(val) => {
						let num_stack_len = num_stack.len();
						num_stack[num_stack_len - 1] = val.clone();
						//println!("{:?} {:?}", key_stack, NDSKeyType::Int(num_stack[num_stack_len - 1].clone()));
						key_stack.push(NDSKeyType::Int(val));
					},
					VType::String(val) => key_stack.push(NDSKeyType::Str(val)),
					VType::Null => key_stack.push(NDSKeyType::Null),
					VType::Float(_) => {
						//add an error handling system
						eprintln!("Error: Float is not a valid key type.")
					},
					VType::Alias(val) => todo!()
				}
			},
			PTok::Literal => {
				//if the last token is a setter then make it a single hashed item
				let tlist_len = token_list.len();
				let stack_len = current_used_stack.len();
				let last_tok = token_list[tlist_len - 2].clone();
				if tn >= 1 {
				
					match &mut current_used_stack[stack_len-1].value {
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
								VType::Alias(_) => todo!()
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
								VType::Alias(_) => todo!()
							}
							key_stack.pop();
						},
						univ => {
							eprintln!("Error: Key for {:?} supplied in a list. {:?}", curr_tok.v_type, univ);
						}
					}
				}
				//else append it to the top list
			},
			PTok::Module(path, objects) => {
				let ts_last_ind = current_used_stack.len()-1;
				let module = serialize(path.as_str(), alias_vec);
				let mut current_key = path.split("\\").collect::<Vec<&str>>().last().unwrap().strip_suffix(".neat").unwrap().to_string();
				if objects.len() == 0 {
					match &mut current_used_stack[ts_last_ind].value {
						NDSType::Hashmap(val) => {val.insert(NDSKeyType::Str(current_key), module.clone());},
						NDSType::List(val) => {val.push(module.clone());},
						_ => {}
					}
				}
				for object_path in objects.iter() {
					let mut current_ds = module.clone();
					current_key = path.split("\\").collect::<Vec<&str>>().last().unwrap().strip_suffix(".neat").unwrap().to_string();
					for object in object_path.iter() {
						current_key = object.to_string();
						current_ds = match current_ds.value {
							NDSType::Hashmap(val_in) => val_in[&NDSKeyType::Str(current_key.clone())].clone(),
							NDSType::List(val_in) => val_in[current_key.parse::<usize>().unwrap()].clone(),
							NDSType::Int(val_in) => Box::new(SerializedNode{value:NDSType::Int(val_in)}),
							NDSType::Str(val_in) => Box::new(SerializedNode{value:NDSType::Str(val_in)}),
							NDSType::Float(val_in) => Box::new(SerializedNode{value:NDSType::Float(val_in)}),
							NDSType::Bool(val_in) => Box::new(SerializedNode{value:NDSType::Bool(val_in)}),
							NDSType::Null => Box::new(SerializedNode{value:NDSType::Null}),
						};
						
					}
					match &mut current_used_stack[ts_last_ind].value {
						NDSType::Hashmap(val) => {val.insert(NDSKeyType::Str(current_key), current_ds);},
						NDSType::List(val) => {val.push(current_ds.clone());},
						_ => {}
					}
				}
			},
			PTok::SAlias => {
				match curr_tok.v_type {
					VType::Alias(val) => {
						let alias_scope_1_len = alias_scope.1.len();
						if alias_scope_1_len != 0 {
							alias_scope.1.push(alias_scope.1[alias_scope_1_len - 1].clone());
						} else {
							alias_scope.1.push(vec![tree_stack[0].clone()]);
						}
						alias_scope.0.push(aliases[&val].clone());
						current_used_stack = &mut alias_scope.1[alias_scope_1_len];
						alias_get(val, aliases.clone(), &mut current_used_stack);
					},
					_ => {}
				}
			},
			PTok::EAlias => {
				//println!("{:?}", alias_scope);
				let mut alias_scope_1_len = alias_scope.1.len();
				let mut alias_scope_0_len = alias_scope.0.len();
				let mut sets_scope = *alias_scope.1[alias_scope_1_len - 1][0].clone();
				let mut curr_aliases = alias_scope.0[alias_scope_0_len - 1].clone();
				alias_scope.0.pop();
				alias_scope.1.pop();
				alias_scope_1_len = alias_scope.1.len();
				alias_scope_0_len = alias_scope.0.len();
				if alias_scope_1_len != 0 {
					current_used_stack = &mut alias_scope.1[alias_scope_1_len - 1];
				} else {
					current_used_stack = &mut tree_stack;
				}
				alias_set(curr_aliases, &mut current_used_stack[0], sets_scope);
				//println!("alias[0] {:?}",alias_scope.1[0]);
				
			}
			PTok::AutoInc => {
				let num_stack_len = num_stack.len();
				let num = num_stack[num_stack_len - 1].clone();
				//println!("{num}");
				num_stack[num_stack_len - 1] += 1;
				key_stack.push(NDSKeyType::Int(num_stack[num_stack_len - 1].clone()));
				//println!("{tn} {:?} {:?}", key_stack, NDSKeyType::Int(num_stack[num_stack_len - 1].clone()));
				//[Token { v_type: String("section"), tok: SSection }, Token { v_type: Blank, tok: AutoInc }, Token { v_type: String("foo"), tok: Literal }, Token { v_type: Blank, tok: AutoInc }, Token { v_type: String("- bar"), tok: Literal }, Token { v_type: Blank, tok: AutoInc }, Token { v_type: Int(-123), tok: Literal }, Token { v_type: Blank, tok: AutoInc }, Token { v_type: Int(7), tok: Setter }, Token { v_type: Bool(true), tok: Literal }, Token { v_type: Blank, tok: AutoInc }, Token { v_type: Blank, tok: SList }, Token { v_type: Float(-0.12), tok: Literal }, Token { v_type: Int(1), tok: Literal }, Token { v_type: Blank, tok: EList }, Token { v_type: Blank, tok: AutoInc }, Token { v_type: Float(-22.2323), tok: Literal }, Token { v_type: Blank, tok: AutoInc }, Token { v_type: Blank, tok: ESection }]
			}
			_ => {}
		}
	}
	return tree_stack[0].clone();
}