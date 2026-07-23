mod ast;
mod builtins;
mod environment;
mod ffi;
mod filesystem;
mod interpreter;
mod lexer;
mod object;
mod parser;
mod tokens;
mod system;
mod types;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::{
    cell::RefCell,
    env,
    fs::{self, read_to_string},
    path::{Path, PathBuf},
    process::exit,
    rc::Rc,
};

use crate::{environment::Environment, object::Value};

fn main() {
    let child = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB untuk call stack, gede sih awokawok
        .spawn(run) // jalanin run function
        .unwrap();

    child.join().unwrap();
}

fn run_file(path: &str) {
    let file = read_to_string(path);
    match file {
        Ok(f) => match Lexer::new(path.to_string(), f.clone()).tokenize() {
            Ok(tokens) => match Parser::new(path.to_string(), f.clone(), tokens).parse() {
                Ok(stmt) => {
                    // println!("{:#?}", stmt);
                    match Interpreter::new(path.to_string(), f).exec_stmt(&stmt) {
                        Ok(_) => {}

                        Err(e) => {
                            eprintln!("Runtime Error: {e}");
                            exit(1)
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Parse Error: {e}");
                    exit(1)
                }
            },
            Err(e) => {
                eprintln!("Lexing Error: {e}");
                exit(1)
            }
        },

        Err(e) => {
            eprintln!("Error when opening file '{}': {}", path, e);
            exit(1);
        }
    }
}

fn run_project(dir: &str) {
    let root_path = Path::new(dir).join("root.zxui");

    let code = match read_to_string(&root_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error when opening file '{}': {}", root_path.display(), e);
            exit(1);
        }
    };

    let root_str = root_path.to_string_lossy().to_string();

    let tokens = match Lexer::new(root_str.clone(), code.clone()).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexing Error: {e}");
            exit(1);
        }
    };
    let stmt = match Parser::new(root_str.clone(), code.clone(), tokens).parse() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Parse Error: {e}");
            exit(1);
        }
    };

    let root_env = Rc::new(RefCell::new(Environment::new()));
    let mut root_interp = Interpreter::new_env(
        root_env.clone(),
        Rc::new(RefCell::new(vec![])),
        Rc::from(root_str),
        Rc::from(code),
        None,
        Rc::new(types::build_type_registry()),
    );

    if let Err(e) = root_interp.exec_stmt(&stmt) {
        eprintln!("Runtime Error: {e}");
        exit(1);
    }

    let main_rel = {
        let root_ref = root_env.borrow();
        let project = match root_ref.get("project".to_string()) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Error: 'project' not defined in root.zxui.");
                exit(1);
            }
        };
        match project {
            Value::Map(m) => match m.borrow().get("main") {
                Some(Value::String(s)) => s.clone(),
                _ => {
                    eprintln!("Error: 'main' field missing or not a string in project map.");
                    exit(1);
                }
            },
            _ => {
                eprintln!("Error: 'project' must be a map.");
                exit(1);
            }
        }
    };

    let main_file = Path::new(dir).join(&main_rel).to_string_lossy().to_string();

    let dir_abs = Path::new(dir)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(dir))
        .to_string_lossy()
        .trim_start_matches("\\\\?\\")
        .to_string();

    run_project_file(&main_file, &dir_abs);
}

fn run_project_file(path: &str, root_dir: &str) {
    let code = match read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error when opening file '{}': {}", path, e);
            exit(1);
        }
    };
    match Lexer::new(path.to_string(), code.clone()).tokenize() {
        Ok(tokens) => match Parser::new(path.to_string(), code.clone(), tokens).parse() {
            Ok(stmt) => {
                match Interpreter::new_with_root(Rc::from(path), Rc::from(code), Rc::from(root_dir))
                    .exec_stmt(&stmt)
                {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Runtime Error: {e}");
                        exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Parse Error: {e}");
                exit(1)
            }
        },
        Err(e) => {
            eprintln!("Lexing Error: {e}");
            exit(1);
        }
    }
}

fn init_project(dir: &str) {
    let path = Path::new(dir);
    if !path.exists() {
        match fs::create_dir(dir) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error when creating directory '{dir}': {e}");
                exit(1);
            }
        }
    }
    let main_file = "// Modify me too!\nprintln(\"Hello, Zxui!\")\n";
    let root_file =
        "// Modify me!\nlet project = {\n    name = \"myproject\",\n    main = \"main.zxui\",\n}\n";
    match fs::write(path.join("main.zxui"), main_file) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "Error when creating file '{}': {}",
                path.join("main.zxui").display(),
                e
            );
            exit(1);
        }
    }
    match fs::write(path.join("root.zxui"), root_file) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "Error when creating file '{}': {}",
                path.join("root.zxui").display(),
                e
            );
            exit(1);
        }
    }
    println!("Successfully initialized a project in '{dir}'!")
}

fn print_usage() {
    eprintln!("usage: zxui <command or file>");
    eprintln!("Commands:");
    eprintln!("run  [path] - run project. if path is empty,");
    eprintln!("              it will run project in current dir.");
    eprintln!("init [path] - init project. if path is empty,");
    eprintln!("              it will init on current dir.");
    eprintln!();
    eprintln!("Args:");
    eprintln!("<file>      - run single file and cannot be using");
    eprintln!("              import feature.");
}

fn run() {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        match path.as_str() {
            "run" => {
                let dir = args.get(2).map(|s| s.as_str()).unwrap_or(".");
                run_project(dir);
            }
            "init" => {
                let dir = args.get(2).map(|s| s.as_str()).unwrap_or(".");
                init_project(dir);
            }
            "help" => print_usage(),
            _ => run_file(path),
        }
    } else {
        print_usage();
        exit(1);
    }
}
