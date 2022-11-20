use std::{fs, path::{Path, PathBuf}, collections::HashMap, env};

use crate::neat::datatypes::VType;

use super::{
    datatypes::{PTok, ScopeType, SerializedNode, Token, ValWrap},
    treebuilder::build_tree,
    typeconversion::determine_type,
};

pub fn create_mod_token(raw_mod_string: String, origin_file_path:&str, alias_vec:&mut HashMap<String, Vec<VType>>) -> Box<Token> {
    let mut filepath = PathBuf::from(&fs::canonicalize(Path::new(origin_file_path)).ok().unwrap());
    {
        let mut ans = filepath.ancestors();
        ans.next();
        filepath = PathBuf::from(ans.next().unwrap().to_str().unwrap());
    }
    let mut is_quoting = ' ';
    let mut quote_buffer = String::new();
    let mut path_buffer = String::new();
    let mut is_mod = true;
    let mut last_char = ' ';
    let mut objects:Vec<Vec<String>> = vec![];
    let mut object_buffer:Vec<String> = vec![];
    for chr in raw_mod_string.chars() {
        match chr {
            '"' => {
                if is_quoting == '"' {
                    is_quoting = ' '
                } else if is_quoting == ' ' {
                    is_quoting = '"';
                } else {
                    quote_buffer.push(chr);
                }
            }
            '\'' => {
                if is_quoting == '\'' {
                    is_quoting = ' ';
                } else if is_quoting == ' ' {
                    is_quoting = '\'';
                } else {
                    quote_buffer.push(chr);
                }
            }
            '?' | ':' | '=' => {
                if is_quoting == '\'' {
                    quote_buffer.push(chr);
                } else if is_quoting == '"' {
                    quote_buffer.push(chr);
                } else {
                    is_mod = false;//marks the end of the mod path
                    filepath.push(path_buffer.trim().clone());
                    path_buffer = String::new()
                }
            }
            '.' => {
                if is_mod {
                    if is_quoting == '\'' {
                        quote_buffer.push(chr);
                    } else if is_quoting == '"' {
                        quote_buffer.push(chr);
                    } else if last_char == '.' {
                        let mut ans = filepath.ancestors();
                        ans.next();
                        filepath = PathBuf::from(ans.next().unwrap().to_str().unwrap());
                    } else {
                        filepath.push(path_buffer.trim().clone());
                        path_buffer = String::new();
                    }
                } else {
                    if is_quoting == '\'' {
                        quote_buffer.push(chr);
                    } else if is_quoting == '"' {
                        quote_buffer.push(chr);
                    } else {
                        object_buffer.push(quote_buffer);
                        quote_buffer = String::new();
                    }
                }
            }
            ',' => {
                if is_mod {
                    if is_quoting == '\'' {
                        quote_buffer.push(chr);
                    } else if is_quoting == '"' {
                        quote_buffer.push(chr);
                    }
                } else {
                    if is_quoting == '\'' {
                        quote_buffer.push(chr);
                    } else if is_quoting == '"' {
                        quote_buffer.push(chr);
                    } else {
                        object_buffer.push(quote_buffer);
                        objects.push(object_buffer);
                        quote_buffer = String::new();
                        object_buffer = vec![];
                    }
                }
            }
            _ => {
                if is_quoting == '\'' || is_quoting == '"' {
                    quote_buffer.push(chr);
                } else {
                    path_buffer.push(chr);
                }
            }
        }
        last_char = chr;
    }
    if is_mod {
        filepath.push(path_buffer.trim().clone());
    } else {
        object_buffer.push(quote_buffer);
        objects.push(object_buffer);
    }
    filepath.set_extension("neat");
    return Box::new(Token {
        v_type: VType::Blank,
        tok: PTok::Module(filepath.as_path().to_str().unwrap().to_string(), objects),
    });
}

pub fn create_alias_token(mut raw_alias_string: String, mut alias_vec:&mut HashMap<String, Vec<VType>>){
    let mut alias_name = String::new();
    let mut is_quoting = ' ';
    let mut last_char = ' ';
    let mut alias_name = String::new();
    let mut is_lhs = true;
    let mut quote_buffer = String::new();
    let mut object_path:Vec<VType> = vec![];
    raw_alias_string.push(' ');
    for chr in raw_alias_string.chars() {
        match chr {
            '"' => {
                if is_quoting == '"' {
                    is_quoting = ' ';
                    object_path.push(VType::String(quote_buffer.clone()));
                    quote_buffer = String::new();
                } else if is_quoting == ' ' {
                    is_quoting = '"';
                } else {
                    quote_buffer.push(chr);
                }
            }
            '\'' => {
                if is_quoting == '\'' {
                    is_quoting = ' ';
                    object_path.push(VType::String(quote_buffer.clone()));
                    quote_buffer = String::new();
                } else if is_quoting == ' ' {
                    is_quoting = '\'';
                } else {
                    quote_buffer.push(chr);
                }
            }
            '['|'{' => {
                if is_quoting == ' ' {
                    is_quoting = '[';
                } else {
                    quote_buffer.push(chr);
                }
            }
            ']'|'}' => {
                if is_quoting == '[' {
                    is_quoting = ' ';
                    object_path.push(VType::String(quote_buffer.clone()));
                    quote_buffer = String::new();
                } else {
                    quote_buffer.push(chr);
                }
            }
            '<'|'(' => {
                if is_quoting == ' ' {
                    is_quoting = '<';
                } else {
                    quote_buffer.push(chr);
                }
            }
            '>'|')' => {
                if is_quoting == '<' {
                    is_quoting = ' ';
                    object_path.push(VType::String(quote_buffer.clone()));
                    quote_buffer = String::new();
                } else {
                    quote_buffer.push(chr);
                }
            }
            ' ' => {
                is_lhs = false;
                if is_quoting != ' ' {
                    quote_buffer.push(chr);
                } else if is_quoting == ' ' && quote_buffer.trim() != "" {
                    if quote_buffer.chars().all(|charr| charr.is_numeric()) {
                        object_path.push(VType::Int(quote_buffer.parse::<i64>().unwrap()));
                        quote_buffer = String::new();
                    } else if vec![
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
                    .contains(&quote_buffer.clone().to_lowercase().as_str())
                    {
                        object_path.push(VType::Bool(true));
                        quote_buffer = String::new();
                    } else if vec![
                        "false", "f", "no", "n", "nope", "nada", "never", "not", "wrong",
                        "negative",
                    ]
                    .contains(&quote_buffer.clone().to_lowercase().as_str())
                    {
                        object_path.push(VType::Bool(false));
                        quote_buffer = String::new();
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
                    .contains(&quote_buffer.clone().to_lowercase().as_str())
                    {
                        object_path.push(VType::Null);
                        quote_buffer = String::new();
                    }
                }
            }
            '\t' => {
                is_lhs = false;
                if is_quoting != ' ' {
                    quote_buffer.push(chr);
                }
            }
            '=' | ':' => {
                is_lhs = false;
                if is_quoting != ' ' {
                    quote_buffer.push(chr);
                }
            }
            _ => {
                if is_quoting != ' ' {
                    quote_buffer.push(chr);
                } else if is_lhs {
                    alias_name.push(chr);
                } else {
                    quote_buffer.push(chr);
                }
            }
        }
        last_char = chr;
    }
    
    //println!("{}: {:?}\n\n", alias_name,object_path);
    alias_vec.insert(alias_name.clone(), object_path);
}

pub fn serialize(file_path: &str, alias_vec:&HashMap<String, Vec<VType>>) -> Box<SerializedNode> {
    let file_content = fs::read_to_string(file_path).unwrap();
    let mut token_vec: Vec<Box<Token>> = vec![];
    let mut val_wrap = ValWrap::None;
    let mut last_val_wrap = ValWrap::None;
    let scope_type_stack = vec![ScopeType::None];
    //this acts as a buffer for the current token
    let mut string_buffer = String::new();
    let mut env_buffer = String::new();
    let mut last_character = '\t';
    let mut is_dict = true;
    let mut alias_list:HashMap<String, Vec<VType>> = alias_vec.clone();
    let mut escaping = false;
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
                tok: PTok::ESection,
            }));
            continue;
        } else if vec!["<-> ", "~ "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::EList,
            }));
            continue;
        } else if line.trim() == "/-/" {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::EAlias,
            }));
            continue;
        } else if vec!["< ", "( "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::SList,
            }));
            continue;
        } else if vec!["> ", ") "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::EList,
            }));
            continue;
        } else if vec!["[ ", "{ "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::SSection,
            }));
            continue;
        } else if vec!["] ", "} "].contains(&line) {
            token_vec.push(Box::new(Token {
                v_type: VType::Blank,
                tok: PTok::ESection,
            }));
            continue;
        } else if vec![
            "~list ", "~l ", "~<> ", "~> ", "~< ", "~vec ", "~vector ", "~v ", "~array ", "~a ",
            "~() ", "~) ", "~( ",
        ]
        .contains(&line)
        {
            is_dict = false;
            continue;
        } else if vec![
            "~dict ",
            "~section ",
            "~sect ",
            "~sec ",
            "~s ",
            "~d ",
            "~[] ",
            "~{} ",
            "~{ ",
            "~} ",
            "~[ ",
            "~] ",
            "~section ",
        ]
        .contains(&line)
        {
            is_dict = true;
            continue;
        } else if line.starts_with("mod ") {
            token_vec.push(create_mod_token(
                line.strip_prefix("mod ")
                    .unwrap()
                    .trim()
                    .replace("\t", ""),
                    file_path.clone(),
                    &mut alias_list
            ));
            continue;
        } else if line.starts_with("* ") {
            token_vec.push(create_mod_token(
                line.strip_prefix("* ")
                    .unwrap()
                    .trim()
                    .replace("\t", ""),
                    file_path.clone(),
                    &mut alias_list
            ));
            continue;
        } else if line.starts_with("alias ") {
            create_alias_token(
                line.strip_prefix("alias ")
                    .unwrap()
                    .trim()
                    .replace("\t", ""),
                    &mut alias_list
                );
            continue;
        } else if line.starts_with("@ ") {
            create_alias_token(
                line.strip_prefix("@ ")
                    .unwrap()
                    .trim()
                    .replace("\t", ""),
                    &mut alias_list
                );
            continue;
        } else if alias_list.keys().collect::<Vec<&String>>().contains(&&raw_line.trim().to_string()) {
            token_vec.push(Box::new(Token {
                v_type: VType::Alias(raw_line.trim().to_string().clone()),
                tok: PTok::SAlias,
            }));
            continue;
        }
        //

        //LINE LOOP
        for (cn, curr_character) in line.clone().chars().enumerate() {
            if val_wrap == ValWrap::None {
                if curr_character ==' ' && if string_buffer.chars().collect::<Vec<char>>().len() > 0 {string_buffer.chars().last().unwrap() == '-'} else {false} {
                    string_buffer.pop();
                }
                if curr_character.is_alphanumeric()
                    || curr_character == '?'
                    || curr_character == '/'
                    || curr_character == '.'
                {
                    //println!("current char : {curr_character}");
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
                            tok: PTok::Literal,
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
                            tok: PTok::Literal,
                        }));
                    } else if string_buffer.chars().enumerate().all(|(ilcn, il_char)| il_char.is_numeric() || (ilcn == 0 && il_char == '-')) {
                        //println!("{string_buffer} {last_character}");
                        token_vec.push(Box::new(Token {
                            v_type: determine_type(VType::Int(0), string_buffer.clone()),
                            tok: PTok::Literal,
                        }));
                    } else if string_buffer
                        .chars().enumerate().all(|(ilcn, il_char)| il_char.is_numeric() || (ilcn == 0 && il_char == '-') || il_char=='.')
                    {
                        //println!("{string_buffer} {last_character}");
                        token_vec.push(Box::new(Token {
                            v_type: determine_type(VType::Float(0.0), string_buffer.clone()),
                            tok: PTok::Literal,
                        }));
                    } else {
                        token_vec.push(Box::new(Token {
                            v_type: VType::Alias(string_buffer.clone()),
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
                                tok: PTok::SList,
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
                                tok: PTok::SSection,
                            });
                            token_vec.pop();
                            token_vec.push(new_tok);
                        } else {
                            token_vec.push(Box::new(Token {
                                v_type: VType::Blank,
                                tok: PTok::SSection,
                            }));
                        }
                    }
                    '}' => {
                        token_vec.push(Box::new(Token {
                            v_type: VType::Blank,
                            tok: PTok::ESection,
                        }));
                    }
                    '"' => {
                        val_wrap = ValWrap::StringDouble;
                    }
                    '\'' => {
                        val_wrap = ValWrap::StringSingle;
                    }
                    ',' => {}
                    '-' => {string_buffer.push(curr_character)}
                    ':' | '=' => {
                        let new_tok = Box::new(Token {
                            v_type: token_vec[token_vec.len() - 1].v_type.clone(),
                            tok: PTok::Setter,
                        });
                        token_vec.pop();
                        token_vec.push(new_tok);
                    }
                    ' ' => {
                        if if cn > 0 {line.chars().collect::<Vec<char>>()[cn - 1] == '-'} else {false} {
                            string_buffer.pop();
                            token_vec.push(Box::new(Token {
                                v_type: VType::Blank,
                                tok: PTok::AutoInc,
                            }));
                        }
                    }
                    _ => {
                        // Handle Keywords
                    }
                }
            } else {
                if curr_character == '\\' {
                    if !escaping {
                        escaping = true;
                        continue;
                    }
                }
                //println!("eb:{env_buffer}:ln:{ln}:cn:{cn}");
                if last_character == ':' && curr_character == '{' && escaping { //backslash
                    if val_wrap == ValWrap::EnvVar {
                        env_buffer.pop();
                        env_buffer += ":{";
                        //println!("eb:{string_buffer}");
                    } else {
                        string_buffer.pop();
                        string_buffer += ":{";
                        //println!("sb:{string_buffer}");
                    }
                    escaping = false;
                    continue;
                } else if last_character == '}' && curr_character == ':' && escaping { //backslash
                    if val_wrap == ValWrap::EnvVar {
                        env_buffer.pop();
                        env_buffer += "}:";
                        //println!("eb:{string_buffer}");
                    } else {
                        string_buffer.pop();
                        string_buffer += "}:";
                        //println!("sb:{string_buffer}");
                    }
                    escaping = false;
                    continue;
                } else if last_character == ':' && curr_character == '{' && !escaping {//&& if len_str >= 2 {string_buffer.chars().collect::<Vec<char>>()[len_str - 2] != '\\'} else {true} { //beginning
                    string_buffer.pop();
                    last_val_wrap = val_wrap.clone();
                    val_wrap = ValWrap::EnvVar;
                    continue;
                } else if last_character == '}' && curr_character == ':' && !escaping {// && (val_wrap != ValWrap::EnvVar || if env_buffer.len() >= 2 {env_buffer.chars().collect::<Vec<char>>()[env_buffer.len() - 2] != '\\'} else {true}) { //end
                    env_buffer.pop();
                    //println!("eb:{env_buffer}");
                    string_buffer += env::var(env_buffer.clone()).expect(format!("Unable to find the specified environment variable {env_buffer}").as_str()).as_str();
                    val_wrap = last_val_wrap.clone();
                    last_val_wrap = ValWrap::None;
                    env_buffer = String::new();
                    continue;
                } else if escaping {
                    let mut buffer_ref:&mut String = &mut String::new();
                    if val_wrap == ValWrap::EnvVar {
                        buffer_ref = &mut env_buffer;
                    } else {
                        buffer_ref = &mut string_buffer;
                    }
                    //buffer_ref.pop();
                    match curr_character {
                        'n' => {
                            escaping = false;
                            buffer_ref.push('\n');
                            continue;
                        }
                        '\\' => {
                            escaping = false;
                            buffer_ref.push(curr_character.clone());
                            continue;
                        }
                        't' => {
                            escaping = false;
                            buffer_ref.push('\t');
                            continue;
                        }
                        '[' => {
                            escaping = false;
                            buffer_ref.push(curr_character.clone());
                            continue;
                        }
                        ']' => {
                            escaping = false;
                            buffer_ref.push(curr_character.clone());
                            continue;
                        }
                        '(' => {
                            escaping = false;
                            buffer_ref.push(curr_character.clone());
                            continue;
                        }
                        ')' => {
                            escaping = false;
                            buffer_ref.push(curr_character.clone());
                            continue;
                        }
                        '<' => {
                            escaping = false;
                            buffer_ref.push(curr_character.clone());
                            continue;
                        }
                        '"' => {
                            escaping = false;
                            buffer_ref.push(curr_character.clone());
                            continue;
                        }
                        '\'' => {
                            escaping = false;
                            buffer_ref.push(curr_character.clone());
                            continue;
                        }
                        '>' => {
                            escaping = false;
                            buffer_ref.push(curr_character.clone());
                            continue;
                        }
                        _ => {

                        }
                    }
                }
                match val_wrap {
                    ValWrap::Section => {
                        if curr_character == ']' {
                            //end section
                            if string_buffer == "-" {
                                token_vec.push(Box::new(Token {
                                    v_type: VType::Blank,
                                    tok: PTok::ESection,
                                }));
                            } else {
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
                                    tok: PTok::ESection,
                                }));
                            } else {
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
                    ValWrap::EnvVar => { //append to env var buffer
                        env_buffer.push(curr_character.clone())
                    }
                    _ => {}
                }
            }
            last_character = curr_character.clone();
        }
    }
    //println!("{:?}", token_vec);
    return build_tree(token_vec, is_dict, file_path, &alias_list)
}
