use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

use rcalc_lib::parse;

mod config;

fn read_expr(prompt: &str) -> String {
    print!("{}> ", prompt);
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("error: unable to read user input");
    input.trim().to_string()
}

// splits expression 'var = expression' into expression and variable name
// Variable name must start with 'a'..'z' and contain only '_', '0'..'9', 'a'..'z'
fn detect_variable(expr: &str) -> (String, Option<String>) {
    let expr = expr.trim().to_lowercase();
    if let Some(p) = expr.find(|c: char| ('a'..='z').contains(&c)) {
        if p != 0 {
            return (expr, None);
        }
    } else {
        return (expr, None);
    }

    if let Some(p) = expr.find('=') {
        let v = expr[..p].trim();
        let e = expr[p + 1..].trim();
        let p = v.find(|c: char| c != '_' && !('0'..='9').contains(&c) && !('a'..='z').contains(&c));
        if p.is_none() {
            return (e.to_string(), Some(v.to_string()));
        }
    }

    (expr, None)
}

fn calc_expr(expr: &str, state: &mut parse::CalcState) -> Result<String, String> {
    let (expr, var) = detect_variable(expr);
    match parse::eval(&expr, state) {
        Ok(v) => {
            if state.has_alt {
                return Ok(state.alt_result.clone());
            }
            let res = format!("{}", v);
            if let Some(vname) = var {
                match state.variable_name_validate(&vname) {
                    Err(s) => return Err(s.to_string()),
                    Ok(..) => state.add_variable(&vname, v),
                }
            }
            Ok(res)
        }
        Err(e) => Err(format!("{}", e)),
    }
}

fn run_interactive_loop(cstate: &mut parse::CalcState, conf: &config::Conf) {
    loop {
        let s = read_expr("");
        if s.is_empty() {
            continue;
        }
        let cmd = s.trim().to_lowercase();
        if cmd == "quit" || cmd == "exit" {
            break;
        } else if cmd.starts_with("load ") {
            let fname = cmd.trim_start_matches("load ").trim();
            let err = load_file(&PathBuf::from(fname), cstate, conf);
            if err.is_empty() {
                if let Some(v) = cstate.result() {
                    println!("= {}", v);
                } else {
                    println!("File {} successfully loaded", fname);
                }
            } else {
                println!("Error while loading '{:?}': {}", fname, err);
            }
            continue;
        }

        match calc_expr(&s, cstate) {
            Ok(res) => println!("= {}", res),
            Err(err) => println!("ERROR: {}", err),
        }
    }
}

fn load_file(pb: &Path, cstate: &mut parse::CalcState, conf: &config::Conf) -> String {
    let f = match File::open(pb) {
        Ok(fl) => fl,
        Err(e) => return format!("error opening file: {}", e),
    };

    const BOM: [u8; 3] = [0xef, 0xbb, 0xbf];
    let bom = if let Ok(s) = String::from_utf8(BOM.to_vec()) {
        s
    } else {
        "".to_string()
    };

    let file = BufReader::new(&f);
    let mut err: String = "".to_string();
    for line in file.lines() {
        let l = match line {
            Ok(ll) => ll,
            Err(e) => return format!("error reading file: {}", e),
        };
        let l = l.trim_start_matches(&bom).to_string();
        if l.starts_with('#') || l.starts_with("//") {
            continue;
        }
        if conf.debug {
            println!("Line: {}", l);
        }
        match calc_expr(&l, cstate) {
            Err(e) => {
                if err.is_empty() {
                    err = e;
                } else {
                    err = err + "\n" + &e;
                }
            }
            Ok(v) => {
                if conf.debug {
                    println!("      {}", v);
                }
            }
        }
    }

    err
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = config::parse_args(&args);
    let mut cstate = parse::CalcState::new();

    if !conf.expression.is_empty() {
        let r = calc_expr(&conf.expression, &mut cstate);
        if !conf.interactive && conf.src_file.is_empty() {
            match r {
                Ok(res) => {
                    if conf.debug {
                        println!("{} = \n{}", conf.expression, res)
                    } else {
                        println!("{}", res)
                    }
                }
                Err(err) => eprintln!("ERROR: {}", err),
            }
        }
    }

    for pb in conf.src_file.iter() {
        let err = load_file(pb, &mut cstate, &conf);
        if !err.is_empty() {
            println!("Error while loading '{:?}': {}", pb, err);
        } else if !conf.interactive {
            if let Some(v) = cstate.result() {
                println!("{}", v);
            }
        }
    }

    if conf.stdin_piped {
        loop {
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    let s = input.trim_matches(|c: char| c == '"' || c == '\'' || c == '\n' || c == '\r' || c == ' ');
                    if s.starts_with('#') || s.starts_with("//") {
                        continue;
                    }
                    let r = calc_expr(s, &mut cstate);
                    match r {
                        Ok(res) => {
                            if conf.debug {
                                println!("{} = \n{}", s, res)
                            } else {
                                println!("{}", res)
                            }
                        }
                        Err(err) => println!("ERROR: {}", err),
                    }
                }
                Err(error) => {
                    println!("error: {}", error);
                    break;
                }
            }
        }
    } else if conf.interactive {
        run_interactive_loop(&mut cstate, &conf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_detect() {
        let s = "";
        let (e, v) = detect_variable(s);
        assert_eq!(e, "");
        assert_eq!(v, None);
        let s = "20=30";
        let (e, v) = detect_variable(s);
        assert_eq!(e, "20=30");
        assert_eq!(v, None);
        let s = "20+30";
        let (e, v) = detect_variable(s);
        assert_eq!(e, "20+30");
        assert_eq!(v, None);
        let s = "20+90=30";
        let (e, v) = detect_variable(s);
        assert_eq!(e, "20+90=30");
        assert_eq!(v, None);
        let s = "abc=30";
        let (e, v) = detect_variable(s);
        assert_eq!(e, "30");
        assert_eq!(v, Some("abc".to_string()));
        let s = " abc  =  30";
        let (e, v) = detect_variable(s);
        assert_eq!(e, "30");
        assert_eq!(v, Some("abc".to_string()));
        let s = "  a1bc_0=   30 == 89";
        let (e, v) = detect_variable(s);
        assert_eq!(e, "30 == 89");
        assert_eq!(v, Some("a1bc_0".to_string()));
    }
}
