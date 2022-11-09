use std::fs;

use serde::Serialize;

use serde_json::{Error, Value, value::Serializer};

use crate::lib::datatypes::VType;

use super::{datatypes::{PTok, ScopeType, Token, ValWrap}, treebuilder::build_tree, typeconversion::determine_type};

pub fn serialize(file_path: &str) -> Result<Value, Error> {
    let file_content = fs::read_to_string(file_path).unwrap();
    let mut token_vec: Vec<Box<Token>> = vec![];
    let mut val_wrap = ValWrap::None;
    let scope_type_stack = vec![ScopeType::None];
    //this acts as a buffer for the current token
    let mut string_buffer = String::new();
    let mut last_character = '\t';
    let mut is_dict = true;
    for (ln, raw_line) in file_content.clone().split("\n").enumerate() {
    	let temp_line = format!("{} ", raw_line.trim());
		let line = temp_line.as_str();
        if line == "" || line.starts_with("|") {
            continue;
        }
        //Handle section/list enders
        else if vec!["[-] ", "# "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::ESection
            }));
            continue;
        } else if vec!["<-> ", "~ "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::EList
            }));
            continue;
        } else if vec!["< ", "( "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::SList
            }));
            continue;
        } else if vec!["> ", ") "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::EList
            }));
            continue;
        } else if vec!["[ ","{ "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::SSection
            }));
            continue;
        } else if vec!["] ","} "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::ESection
            }));
            continue;
        } else if vec!["~list ", "~l ", "~<> ", "~> ", "~< ", "~vec ", "~vector ", "~v ", "~array ", "~a ", "~() ", "~) ", "~( "].contains(&line) {
            is_dict = false;
            continue;
        } else if vec!["~dict ", "~section ", "~sect ", "~sec ", "~s ", "~d ", "~[] ", "~{} ", "~{ ", "~} ", "~[ ", "~] ", "~section "].contains(&line) {
            is_dict = true;
            continue;
        }
        //

        //LINE LOOP
        for (cn, curr_character) in line.clone().chars().enumerate() {
            if val_wrap == ValWrap::None {
                if curr_character.is_alphanumeric()
                    || curr_character == '?'
                    || curr_character == '/'
                    || (curr_character == '.' && string_buffer.chars().all(char::is_numeric))
                {
                    string_buffer.push(curr_character.clone());
                    continue;
                } else if string_buffer != "" {
                    if vec![
                        "true",
                        "t",
                        "yes",
                        "y",
                        "yup",
                        "affirmative",
                        "yep",
                        "correct",
                        "right",
                        "positive",
                    ]
                    .contains(&string_buffer.clone().to_lowercase().as_str())
                    {
                        token_vec.push(Box::new(Token {
                            v_type: VType::Bool(true),
                            tok: PTok::Literal
                        }));
                    } else if vec![
                        "false", "f", "no", "n", "nope", "nada", "never", "not", "wrong",
                        "negative",
                    ]
                    .contains(&string_buffer.clone().to_lowercase().as_str())
                    {
                        token_vec.push(Box::new(Token {
                            v_type: VType::Bool(false),
                            tok: PTok::Literal,
                        }));
                    } else if vec![
                        "idk",
                        "?",
                        "null",
                        "/",
                        "na",
                        "none",
                        "untitled",
                        "empty",
                        "nonapplicable",
                    ]
                    .contains(&string_buffer.clone().to_lowercase().as_str())
                    {
                        token_vec.push(Box::new(Token {
                            v_type: VType::Null,
                            tok: PTok::Literal
                        }));
                    }else if string_buffer
                        .chars()
                        .all(|il_char| il_char.is_numeric())
                    {
                        token_vec.push(Box::new(Token {
                            v_type: determine_type(VType::Int(0), string_buffer.clone()),
                            tok: PTok::Literal,
                        }));
                    } else if string_buffer
                        .chars()
                        .all(|il_char| il_char.is_numeric() || il_char == '.')
                    {
                        token_vec.push(Box::new(Token {
                            v_type: determine_type(VType::Float(0.0), string_buffer.clone()),
                            tok: PTok::Literal,
                        }));
                    }
                    string_buffer = String::new();
                }
                match curr_character {
                    '[' => val_wrap = ValWrap::Section,
                    '<' => val_wrap = ValWrap::ListSection,
                    '(' => {
                        if token_vec.last().clone().unwrap().tok == PTok::Setter {
                            let new_tok = Box::new(Token {
                                v_type: token_vec[token_vec.len() - 1].v_type.clone(),
                                tok: PTok::SList
                            });
                            token_vec.pop();
                            token_vec.push(new_tok);
                        } else {
                            token_vec.push(Box::new(Token {
                                v_type: VType::Blank,
                                tok: PTok::SList,
                            }));
                        }
                    }
                    ')' => {
                        token_vec.push(Box::new(Token {
                            v_type: VType::Blank,
                            tok: PTok::EList,
                        }));
                    }
                    '{' => {
                        if token_vec.last().clone().unwrap().tok == PTok::Setter {
                            let new_tok = Box::new(Token {
                                v_type: token_vec[token_vec.len() - 1].v_type.clone(),
                                tok: PTok::SSection
                            });
                            token_vec.pop();
                            token_vec.push(new_tok);
                        } else {
                            token_vec.push(Box::new(Token {
                                v_type: VType::Blank,
                                tok: PTok::SSection
                            }));
                        }
                    }
                    '}' => {
                        token_vec.push(Box::new(Token {
                            v_type: VType::Blank,
                            tok: PTok::ESection
                        }));
                    }
                    '"' => {
                        val_wrap = ValWrap::StringDouble;
                    }
                    '\'' => {
                        val_wrap = ValWrap::StringSingle;
                    }
                    ',' => {}
                    '-' => {
						token_vec.push(Box::new(Token {
                            v_type: VType::Blank,
                            tok: PTok::AutoInc
                        }));
					}
                    ':' | '=' => {
                        let new_tok = Box::new(Token {
                            v_type: token_vec[token_vec.len() - 1].v_type.clone(),
                            tok: PTok::Setter
                        });
                        token_vec.pop();
                        token_vec.push(new_tok);
                    }
                    ' ' => {}
                    _ => {
                        // Handle Keywords
                    }
                }
            } else {
                match val_wrap {
                    ValWrap::Section => {
                        if curr_character == ']' {
                            //end section
                            if string_buffer == "-" {
                                token_vec.push(Box::new(Token {
                                    v_type: VType::Blank,
                                    tok: PTok::ESection
                                }));
                            }
                            else {
                                token_vec.push(Box::new(Token {
                                    v_type: VType::String(string_buffer.clone()),
                                    tok: PTok::SSection,
                                }));
                            }
                            string_buffer = String::new();
                            val_wrap = ValWrap::None;
                        } else {
                            string_buffer.push(curr_character.clone());
                        }
                    }
                    ValWrap::ListSection => {
                        if curr_character == '>' {
                            //end section
                            if string_buffer == "-" {
                                token_vec.push(Box::new(Token {
                                    v_type: VType::Blank,
                                    tok: PTok::ESection
                                }));
                            }
                            else {
                                token_vec.push(Box::new(Token {
                                    v_type: VType::String(string_buffer.clone()),
                                    tok: PTok::SList,
                                }));
                            }
                            string_buffer = String::new();
                            val_wrap = ValWrap::None;
                        } else {
                            string_buffer.push(curr_character.clone());
                        }
                    }
                    ValWrap::StringSingle => {
                        if curr_character == '\'' {
                            //end section
                            token_vec.push(Box::new(Token {
                                v_type: VType::String(string_buffer.clone()),
                                tok: PTok::Literal,
                            }));
                            string_buffer = String::new();
                            val_wrap = ValWrap::None;
                        } else {
                            string_buffer.push(curr_character.clone());
                        }
                    }
                    ValWrap::StringDouble => {
                        if curr_character == '"' {
                            //end section
                            token_vec.push(Box::new(Token {
                                v_type: VType::String(string_buffer.clone()),
                                tok: PTok::Literal,
                            }));
                            string_buffer = String::new();
                            val_wrap = ValWrap::None;
                        } else {
                            string_buffer.push(curr_character.clone());
                        }
                    }
                    _ => {}
                }
            }
            last_character = curr_character.clone();
        }
    }
    //println!("{:?}",token_vec);
    return build_tree(token_vec, is_dict).clone().serialize(Serializer);
}
