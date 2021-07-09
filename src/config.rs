use std::path::PathBuf;
use std::process::exit;

use atty::Stream;
use getopts::{Matches, Options};

#[derive(Debug, Clone)]
pub struct Conf {
    pub src_file: Vec<PathBuf>,
    pub expression: String,
    pub interactive: bool,
    pub stdin_piped: bool,
    pub debug: bool,
}

impl Default for Conf {
    fn default() -> Conf {
        Conf { src_file: Vec::new(), expression: "".to_string(), interactive: false, stdin_piped: false, debug: false }
    }
}

impl Conf {
    fn new() -> Self {
        Default::default()
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [OPTIONS] [\"expression\" | --file filename]", program);
    print!("{}", opts.usage(&brief));
}

fn preprocess_args(args: &[String]) -> Vec<String> {
    let opt_list = ["-h", "-v", "-i"];
    let mut res: Vec<String> = Vec::new();
    'outer: for arg in args {
        if !arg.starts_with('-') || arg.starts_with("-f") || arg.starts_with("--") {
            res.push(arg.to_string());
            continue;
        }
        for opt in opt_list {
            if opt == arg {
                res.push(arg.to_string());
                continue 'outer;
            }
        }
        res.push(" ".to_string() + arg);
    }
    res
}

pub fn parse_args(args: &[String]) -> Conf {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this help");
    opts.optflag("v", "version", "Print application version");
    opts.optflag("i", "interactive", "Force interactive mode");
    opts.optflag("", "debug", "Show extra information while calculating");
    opts.optmulti("f", "file", "Path to file with expressions to calculate one by one", "SRC FILE PATH");

    let args = preprocess_args(args);
    let program = args[0].clone();
    let mut conf = Conf::new();
    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, &opts);
            exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        exit(0);
    }

    if matches.opt_count("file") > 0 {
        for f in matches.opt_strs("file").iter() {
            let pb = PathBuf::from(f);
            conf.src_file.push(pb);
        }
    }
    if matches.opt_present("version") {
        let version = env!("CARGO_PKG_VERSION");
        println!("RionaCalc Version {}", version);
        exit(0);
    }

    for expr in &matches.free {
        conf.expression += &expr;
    }

    conf.interactive = (conf.expression.is_empty() && conf.src_file.is_empty()) || matches.opt_present("interactive");
    conf.interactive = conf.interactive && atty::is(Stream::Stdout) && atty::is(Stream::Stdin);
    conf.stdin_piped = !atty::is(Stream::Stdin);
    conf.debug = matches.opt_present("debug");

    conf
}
