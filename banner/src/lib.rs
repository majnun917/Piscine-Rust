use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag<'a> {
    pub short_hand: String,
    pub long_hand: String,
    pub desc:  &'a str
}

impl<'a> Flag<'a> {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Self {
            short_hand: format!("-{}", &name[..1]),
            long_hand: format!("--{}", name),
            desc: d
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        match self.flags.get(input) {
            Some(func) => {
                if argv.len() < 2 {
                    return Err("".to_string());
                }
                match func(argv[0], argv[1]) {
                    Ok(res) => Ok(res),
                    Err(e) => Err(format!("{}", e)),
                }
            }
            None => Err(format!("{}", input)),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a1 =  a.parse::<f64>()?;
    let b1 = b.parse::<f64>()?;
    Ok((a1/b1).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a1 =  a.parse::<f64>()?;
    let b1 = b.parse::<f64>()?;
    Ok((a1%b1).to_string())
}
