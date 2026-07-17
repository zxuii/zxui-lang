use indexmap::IndexMap;

use crate::ast::{BinOp, CompOp, Expr, LogicalOp, Stmt, StmtKind, UnaryOp};
use crate::builtins::{self, *};
use crate::environment::Environment;
use crate::lexer::Lexer;
use crate::object::{ClassData, FunData, InstanceData, Value};
use crate::parser::Parser;

use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;

const MAX_DEPTH: usize = 1000; // 1 call di zxui sama kek 5-6 call di rust

pub struct CallFrame {
    pub fun_name: String,
    pub line: usize,
}

impl CallFrame {
    fn new(fun_name: String, line: usize) -> Self {
        Self { fun_name, line }
    }
}

pub enum Signal {
    None,
    Return(Option<Value>),
    Continue,
    Break,
}

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
    call_stack: Rc<RefCell<Vec<CallFrame>>>,
    filename: Rc<str>,
    code: Rc<str>,
    root_dir: Option<Rc<str>>,
}

impl Interpreter {
    pub fn new(filename: String, code: String) -> Self {
        let mut interp = Self {
            env: Rc::new(RefCell::new(Environment::new())),
            call_stack: Rc::new(RefCell::new(Vec::new())),
            filename: Rc::from(filename),
            code: Rc::from(code),
            root_dir: None,
        };
        interp.define_natives();
        interp
            .call_stack
            .borrow_mut()
            .push(CallFrame::new("<script>".to_string(), 0));

        interp
    }

    pub fn new_env(
        env: Rc<RefCell<Environment>>,
        call_stack: Rc<RefCell<Vec<CallFrame>>>,
        filename: Rc<str>,
        code: Rc<str>,
        root_dir: Option<Rc<str>>,
    ) -> Self {
        Self {
            env,
            call_stack,
            filename,
            code,
            root_dir,
        }
    }

    pub fn new_with_root(filename: Rc<str>, code: Rc<str>, root_dir: Rc<str>) -> Self {
        let mut interp = Self {
            env: Rc::new(RefCell::new(Environment::new())),
            call_stack: Rc::new(RefCell::new(Vec::new())),
            filename: filename,
            code: code,
            root_dir: Some(root_dir),
        };
        interp.define_natives();
        interp
            .call_stack
            .borrow_mut()
            .push(CallFrame::new("<script>".to_string(), 0));
        interp
    }

    fn define_natives(&mut self) {
        self.env.borrow_mut().define(
            "println".to_string(),
            Value::native_fun("println".to_string(), -1, Rc::new(native_println)),
        );
        self.env.borrow_mut().define(
            "print".to_string(),
            Value::native_fun("print".to_string(), -1, Rc::new(native_print)),
        );
        self.env.borrow_mut().define(
            "readline".to_string(),
            Value::native_fun("readline".to_string(), -1, Rc::new(native_readline)),
        );
        self.env.borrow_mut().define(
            "typeof".to_string(),
            Value::native_fun("typeof".to_string(), 1, Rc::new(native_typeof)),
        );
        self.env.borrow_mut().define(
            "number".to_string(),
            Value::native_fun("number".to_string(), 1, Rc::new(native_number)),
        );
        self.env.borrow_mut().define(
            "string".to_string(),
            Value::native_fun("string".to_string(), 1, Rc::new(native_string)),
        );
        self.env.borrow_mut().define(
            "boolean".to_string(),
            Value::native_fun("boolean".to_string(), 1, Rc::new(native_boolean)),
        );
        self.env.borrow_mut().define(
            "push".to_string(),
            Value::native_fun("push".to_string(), 2, Rc::new(native_push)),
        );
        self.env.borrow_mut().define(
            "pop".to_string(),
            Value::native_fun("pop".to_string(), 1, Rc::new(native_pop)),
        );
        self.env.borrow_mut().define(
            "len".to_string(),
            Value::native_fun("len".to_string(), 1, Rc::new(native_len)),
        );
        self.env.borrow_mut().define(
            "remove".to_string(),
            Value::native_fun("remove".to_string(), 2, Rc::new(native_remove)),
        );
        self.env.borrow_mut().define(
            "range".to_string(),
            Value::native_fun("range".to_string(), -1, Rc::new(native_range)),
        );
        self.env.borrow_mut().define(
            "keys".to_string(),
            Value::native_fun("keys".to_string(), 1, Rc::new(native_keys)),
        );
        self.env.borrow_mut().define(
            "values".to_string(),
            Value::native_fun("values".to_string(), 1, Rc::new(native_values)),
        );
        self.env.borrow_mut().define(
            "hasKey".to_string(),
            Value::native_fun("hasKey".to_string(), 2, Rc::new(native_has_key)),
        );
        self.env.borrow_mut().define(
            "clear".to_string(),
            Value::native_fun("clear".to_string(), 2, Rc::new(native_clear)),
        );
    }

    pub fn eval_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(num) => Ok(Value::Number(*num)),
            Expr::String(str) => Ok(Value::String(str.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Array(exprs) => {
                let mut values: Vec<Value> = Vec::new();
                for expr in exprs {
                    values.push(self.eval_expr(expr)?);
                }
                Ok(Value::Array(Rc::new(RefCell::new(values))))
            }
            Expr::Map(maps) => {
                let mut key_values = IndexMap::new();
                for map in maps {
                    key_values.insert(map.key.clone(), self.eval_expr(&map.val)?);
                }
                Ok(Value::Map(Rc::new(RefCell::new(key_values))))
            }
            Expr::Null => Ok(Value::Null),
            Expr::NoOp => Ok(Value::Null),
            Expr::Identifier(name) => self.env.borrow().get(name.clone()),
            Expr::Unary { op, expr } => {
                let val = self.eval_expr(expr)?;
                match (op, val) {
                    (UnaryOp::Plus, Value::Number(num)) => Ok(Value::Number(num)),
                    (UnaryOp::Minus, Value::Number(num)) => Ok(Value::Number(-num)),
                    (UnaryOp::Not, Value::Boolean(num)) => Ok(Value::Boolean(!num)),
                    _ => Err("unary operation on unsupported type.".into()),
                }
            }
            Expr::BinOp { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => {
                        let result = match op {
                            BinOp::Plus => a + b,
                            BinOp::Minus => a - b,
                            BinOp::Multiply => a * b,
                            BinOp::Divide => {
                                if b == 0.0 {
                                    return Err("division by zero".into());
                                }
                                a / b
                            }
                        };
                        Ok(Value::Number(result))
                    }

                    (Value::String(a), Value::String(b)) if matches!(op, BinOp::Plus) => {
                        Ok(Value::String(a + &b))
                    }

                    (Value::String(a), Value::Number(b)) if matches!(op, BinOp::Multiply) => {
                        Ok(Value::String(a.repeat(b as usize)))
                    }

                    _ => Err("binary operation on unsupported type".into()),
                }
            }

            Expr::CompOp { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => {
                        let result = match op {
                            CompOp::Lt => a < b,
                            CompOp::Gt => a > b,
                            CompOp::LtEq => a <= b,
                            CompOp::GtEq => a >= b,
                            CompOp::EqEq => a == b,
                            CompOp::NotEq => a != b,
                        };
                        Ok(Value::Boolean(result))
                    }
                    (Value::Boolean(a), Value::Boolean(b)) => {
                        let result = match op {
                            CompOp::Lt => a < b,
                            CompOp::Gt => a > b,
                            CompOp::LtEq => a <= b,
                            CompOp::GtEq => a >= b,
                            CompOp::EqEq => a == b,
                            CompOp::NotEq => a != b,
                        };
                        Ok(Value::Boolean(result))
                    }
                    (Value::String(a), Value::String(b)) => {
                        let result = match op {
                            CompOp::Lt => a < b,
                            CompOp::Gt => a > b,
                            CompOp::LtEq => a <= b,
                            CompOp::GtEq => a >= b,
                            CompOp::EqEq => a == b,
                            CompOp::NotEq => a != b,
                        };
                        Ok(Value::Boolean(result))
                    }
                    (Value::Null, Value::Null) => match op {
                        CompOp::EqEq => Ok(Value::Boolean(true)),
                        CompOp::NotEq => Ok(Value::Boolean(false)),
                        _ => Err(
                            "cannot use ordering comparison ('<', '>', '<=', '>=') on null".into(),
                        ),
                    },
                    (Value::Array(a), Value::Array(b)) => match op {
                        CompOp::EqEq => Ok(Value::Boolean(Rc::ptr_eq(&a, &b))),
                        CompOp::NotEq => Ok(Value::Boolean(!Rc::ptr_eq(&a, &b))),
                        _ => Err(
                            "cannot use ordering comparison ('<', '>', '<=', '>=') on array".into(),
                        ),
                    },
                    _ => Err("comparison operation on unsupported type".into()),
                }
            }

            Expr::LogicalOp { left, op, right } => {
                let l = self.eval_expr(left)?;
                match (op, &l) {
                    (LogicalOp::Or, Value::Boolean(true)) => return Ok(Value::Boolean(true)),
                    (LogicalOp::And, Value::Boolean(false)) => return Ok(Value::Boolean(false)),
                    _ => {}
                }
                let r = self.eval_expr(right)?;
                match (l, r) {
                    (Value::Boolean(_), Value::Boolean(b)) => Ok(Value::Boolean(b)),
                    _ => Err("logical operation on non-boolean".into()),
                }
            }

            Expr::Call { callee, args, line } => {
                let fun = self.eval_expr(callee)?;

                let evaluated_args: Result<Vec<Value>, String> =
                    args.iter().map(|arg| self.eval_expr(arg)).collect();
                let evaluated_args = evaluated_args?;

                match fun {
                    Value::Function(f) => self.call_user_function(
                        f.name,
                        f.params,
                        f.body,
                        f.closure,
                        evaluated_args,
                        *line,
                    ),

                    Value::NativeFunction(f) => {
                        if f.arity != -1 && evaluated_args.len() != f.arity as usize {
                            return Err(self.format_error(
                                &format!(
                                    "function {}() expects {} args but got {}",
                                    f.name,
                                    f.arity,
                                    evaluated_args.len()
                                ),
                                *line,
                            ));
                        }
                        (f.fun)(evaluated_args)
                    }

                    Value::Class(class) => {
                        let instance = Rc::new(InstanceData {
                            class: Rc::clone(&class),
                            fields: RefCell::new(IndexMap::new()),
                        });

                        if let Some(init_fun) = find_method(&class, "init") {
                            let bound =
                                bind_method(&init_fun, Value::Instance(Rc::clone(&instance)));
                            if let Value::Function(f) = bound {
                                self.call_user_function(
                                    f.name,
                                    f.params,
                                    f.body,
                                    f.closure,
                                    evaluated_args,
                                    *line,
                                )?;
                            }
                        } else if !evaluated_args.is_empty() {
                            return Err(self.format_error(
                                &format!(
                                    "class '{}' has no 'init' method but got {} arg(s).",
                                    class.name,
                                    evaluated_args.len()
                                ),
                                *line,
                            ));
                        }

                        Ok(Value::Instance(instance))
                    }

                    _ => Err(self.format_error("attempted to call a non-function value", *line)),
                }
            }
            Expr::Index { target, index } => {
                let (arr, i) = self.resolve_array_index(target, index)?;
                Ok(arr.borrow()[i].clone())
            }

            Expr::Get { target, name } => {
                let prop = self.resolve_property(target)?;
                if let Some(val) = prop.get(name) {
                    return Ok(val);
                }
                if let PropertyTarget::Instance(inst) = &prop {
                    if let Some(fun) = find_method(&inst.class, name) {
                        return Ok(bind_method(&fun, Value::Instance(Rc::clone(inst))));
                    }
                }
                Err(format!(
                    "property '{}' does not exist on this object.",
                    name
                ))
            }

            Expr::SelfExpr => self.env.borrow().get("self".to_string()),

            Expr::Super { method } => {
                let superclass = match self.env.borrow().get("super".to_string())? {
                    Value::Class(c) => c,
                    _ => return Err("'super' resolved to non-class value.".into()),
                };
                let instance = self.env.borrow().get("self".to_string())?;

                match find_method(&superclass, method) {
                    Some(fun_data) => Ok(bind_method(&fun_data, instance)),
                    None => Err(format!("undefined method '{method}' in superclass")),
                }
            }
        }
    }

    fn call_user_function(
        &self,
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
        evaluated_args: Vec<Value>,
        line: usize,
    ) -> Result<Value, String> {
        if self.call_stack.borrow().len() >= MAX_DEPTH {
            let trace = self.build_stack_trace();
            return Err(self.format_error(
                &format!("stack overflow: maximum recursion depth exceed.\n{trace}"),
                line,
            ));
        }

        if evaluated_args.len() != params.len() {
            return Err(self.format_error(
                &format!(
                    "function <closure> expects {} args but got {}",
                    params.len(),
                    evaluated_args.len()
                ),
                line,
            ));
        }

        let call_env = Rc::new(RefCell::new(Environment::new_enclosing(Some(Rc::clone(
            &closure,
        )))));
        for (param, arg) in params.iter().zip(evaluated_args) {
            call_env.borrow_mut().define(param.clone(), arg);
        }

        self.call_stack
            .borrow_mut()
            .push(CallFrame::new(name.clone(), line));

        let mut interp = Interpreter::new_env(
            call_env,
            self.call_stack.clone(),
            self.filename.clone(),
            self.code.clone(),
            self.root_dir.clone(),
        );
        let mut return_val = Value::Null;
        let mut error = None;
        for stmt in &body {
            match interp.exec_stmt(stmt) {
                Ok(Signal::Return(val)) => {
                    return_val = val.unwrap_or(Value::Null);
                    break;
                }
                Ok(Signal::Break) | Ok(Signal::Continue) => {
                    unreachable!(
                        "Harusnya ini ga akan pernah tercapai karena sudah di handle di parser. jaga-jaga aja."
                    )
                }
                Ok(Signal::None) => {}
                Err(e) => {
                    error = Some(e);
                    break;
                }
            }
        }

        match &mut error {
            Some(e) if !e.contains("stack trace:") => {
                let trace = self.build_stack_trace();
                *e = format!("{}\n{}", e, trace);
            }
            _ => {}
        }

        self.call_stack.borrow_mut().pop();

        match error {
            Some(e) => Err(e),
            None => Ok(return_val),
        }
    }

    fn exec_stmts(&mut self, stmts: &[Stmt]) -> Result<Signal, String> {
        for stmt in stmts {
            let signal = self.exec_stmt(stmt)?;
            if !matches!(signal, Signal::None) {
                return Ok(signal);
            }
        }
        Ok(Signal::None)
    }

    fn exec_block(&mut self, stmts: &[Stmt]) -> Result<Signal, String> {
        let child = Environment::new_enclosing(Some(Rc::clone(&self.env)));
        let prev = Rc::clone(&self.env);
        self.env = Rc::new(RefCell::new(child));

        let signal = self.exec_stmts(stmts);

        self.env = prev;
        signal
    }

    fn exec_block_with(
        &mut self,
        stmts: &[Stmt],
        bind_name: &str,
        bind_val: Value,
    ) -> Result<Signal, String> {
        let child = Environment::new_enclosing(Some(Rc::clone(&self.env)));
        let prev = Rc::clone(&self.env);
        self.env = Rc::new(RefCell::new(child));
        self.env
            .borrow_mut()
            .define(bind_name.to_string(), bind_val);

        let signal = self.exec_stmts(stmts);

        self.env = prev;
        signal
    }

    pub fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Signal, String> {
        self.exec_stmt_kind(&stmt.kind)
            .map_err(|e| self.attach_line(e, stmt.line))
    }

    fn attach_line(&self, e: String, line: usize) -> String {
        if e.contains(" at ") {
            e
        } else {
            self.format_error(&e, line)
        }
    }

    fn format_error(&self, msg: &str, line: usize) -> String {
        let snippet = self.code.lines().nth(line.saturating_sub(1)).unwrap_or("");
        format!("{} at {}:{}\n    {}", msg, self.filename, line, snippet)
    }

    fn exec_stmt_kind(&mut self, kind: &StmtKind) -> Result<Signal, String> {
        match kind {
            StmtKind::Program(stmts) => {
                let ret = self.exec_stmts(stmts)?;
                Ok(ret)
            }

            StmtKind::Block(stmts) => {
                let ret = self.exec_block(stmts)?;
                Ok(ret)
            }

            StmtKind::Let { name, expr } => {
                let val = self.eval_expr(expr)?;
                self.env.borrow_mut().define(name.clone(), val);
                Ok(Signal::None)
            }

            StmtKind::Assign { target, expr } => {
                let val = self.eval_expr(expr)?;
                match target {
                    Expr::Identifier(name) => {
                        self.env.borrow_mut().assign(name.clone(), val)?;
                    }
                    Expr::Index { target, index } => {
                        let (arr, i) = self.resolve_array_index(target, index)?;
                        arr.borrow_mut()[i] = val;
                    }
                    Expr::Get { target, name } => {
                        let prop = self.resolve_property(target)?;
                        prop.set(name.clone(), val)?;
                    }
                    _ => return Err("invalid assignment target".into()),
                }
                Ok(Signal::None)
            }

            StmtKind::CompAssign { target, op, expr } => {
                let rhs = self.eval_expr(expr)?;
                match target {
                    Expr::Identifier(name) => {
                        let current = self.env.borrow().get(name.clone())?;
                        let new_val = Self::apply_comp_op(current, op, rhs)?;
                        self.env.borrow_mut().assign(name.clone(), new_val)?;
                    }
                    Expr::Index {
                        target: arr_target,
                        index,
                    } => {
                        let (arr, i) = self.resolve_array_index(arr_target, index)?;
                        let current = arr.borrow()[i].clone();
                        let new_val = Self::apply_comp_op(current, op, rhs)?;
                        arr.borrow_mut()[i] = new_val;
                    }
                    Expr::Get {
                        target: obj_target,
                        name,
                    } => {
                        let prop = self.resolve_property(obj_target)?;
                        let current = match prop.get(name) {
                            Some(val) => val,
                            None => {
                                return Err(format!(
                                    "property '{}' does not exist on this object.",
                                    name
                                ));
                            }
                        };
                        let new_val = Self::apply_comp_op(current, op, rhs)?;
                        prop.set(name.clone(), new_val)?;
                    }
                    _ => return Err("invalid assignment target".into()),
                }
                Ok(Signal::None)
            }

            StmtKind::If {
                expr,
                block,
                else_block,
            } => match self.eval_expr(expr)? {
                Value::Boolean(b) => {
                    if b {
                        self.exec_block(block)
                    } else if let Some(else_stmts) = else_block {
                        self.exec_block(else_stmts)
                    } else {
                        Ok(Signal::None)
                    }
                }

                _ => Err("if statement on non-boolean type".into()),
            },

            StmtKind::While { expr, block } => {
                loop {
                    match self.eval_expr(expr)? {
                        Value::Boolean(true) => match self.exec_block(block)? {
                            Signal::Break => break,
                            Signal::Continue => continue,
                            Signal::None => {}
                            ret @ Signal::Return(_) => return Ok(ret),
                        },
                        Value::Boolean(false) => break,
                        _ => return Err("while condition must be boolean".into()),
                    }
                }
                Ok(Signal::None)
            }

            StmtKind::For { name, expr, block } => {
                let val = self.eval_expr(expr)?;

                let items: Vec<Value> = match val {
                    Value::Array(arr) => arr.borrow().clone(),

                    Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),

                    Value::Map(map) => map
                        .borrow()
                        .iter()
                        .map(|(k, v)| {
                            let mut pair = indexmap::IndexMap::new();
                            pair.insert("key".to_string(), Value::String(k.clone()));
                            pair.insert("val".to_string(), v.clone());
                            Value::Map(Rc::new(RefCell::new(pair)))
                        })
                        .collect(),

                    _ => return Err("for-in loop requires an array, string, or map.".into()),
                };

                for item in items {
                    match self.exec_block_with(block, name, item)? {
                        Signal::Break => break,
                        Signal::Continue => continue,
                        Signal::None => {}
                        ret @ Signal::Return(_) => return Ok(ret),
                    }
                }
                Ok(Signal::None)
            }

            StmtKind::Return(expr) => {
                let val = self.eval_expr(expr)?;
                Ok(Signal::Return(Some(val)))
            }

            StmtKind::Break => Ok(Signal::Break),
            StmtKind::Continue => Ok(Signal::Continue),

            StmtKind::ExprStmt(expr) => {
                self.eval_expr(expr)?;
                Ok(Signal::None)
            }

            StmtKind::FunDecl {
                name,
                params,
                body,
                is_static: _,
            } => {
                let fun = Value::Function(FunData::new(
                    name.clone(),
                    params.clone(),
                    body.clone(),
                    Rc::clone(&self.env),
                ));
                self.env.borrow_mut().define(name.clone(), fun);
                Ok(Signal::None)
            }
            StmtKind::Import(path) => {
                // ngeparse antara "root:src/math" atau "root:math" misalnya
                let parts: Vec<&str> = path.splitn(2, ':').collect();
                if parts.len() != 2 {
                    return Err(format!("invalid import path '{}'.", path));
                }

                let root = match &self.root_dir {
                    Some(r) => r.clone(),
                    None => {
                        return Err(
                            "cannot use 'import' statement without a project, please run `zxui init`."
                                .into(),
                        );
                    }
                };

                match parts[0] {
                    "builtin" => {
                        let map = match parts[1] {
                            "raylib" => builtins::module_raylib(),
                            other => {
                                return Err(format!("unknown builtin module named '{}'", other));
                            }
                        };
                        self.env
                            .borrow_mut()
                            .define(parts[1].to_string(), Value::Map(Rc::new(RefCell::new(map))));
                    }

                    "root" => {
                        let module_rel = parts[1].replace(':', "/");
                        let module_file =
                            Path::new(&root.to_string()).join(format!("{}.zxui", module_rel));
                        let module_path_str = module_file.to_string_lossy().to_string();

                        let code = fs::read_to_string(&module_file)
                            .map_err(|e| format!("cannot import '{}': {}", module_path_str, e))?;

                        let tokens = Lexer::new(module_path_str.clone(), code.clone())
                            .tokenize()
                            .map_err(|e| format!("In Import '{}' Lexing Error: {}", path, e))?;

                        let stmts = Parser::new(module_path_str.clone(), code.clone(), tokens)
                            .parse()
                            .map_err(|e| format!("In Import '{}' Parse Error: {}", path, e))?;

                        let module_env = Rc::new(RefCell::new(Environment::new()));

                        let mut module_interp = Interpreter::new_env(
                            module_env.clone(),
                            self.call_stack.clone(),
                            Rc::from(module_path_str),
                            Rc::from(code),
                            Some(root),
                        );

                        module_interp.define_natives();
                        module_interp.exec_stmt(&stmts)?;

                        let mut map = IndexMap::new();
                        for (k, v) in module_env.borrow().values.iter() {
                            map.insert(k.clone(), v.clone());
                        }

                        // nama utk variable yang nanti akan teresolve diambil dari
                        // bagian terakhir path, misal, "src/math", yang diambil "math"
                        let var_name = module_rel
                            .split('/')
                            .last()
                            .unwrap_or(&module_rel)
                            .to_string();

                        self.env
                            .borrow_mut()
                            .define(var_name, Value::Map(Rc::new(RefCell::new(map))));
                    }

                    // "std" => {
                    //     let map = match parts[1] {
                    //         "ffi" => {

                    //         }
                    //         other => return Err(format!("unknown standard library module named '{}'", other)),
                    //     };

                    //     self.env
                    //         .borrow_mut()
                    //         .define(parts[1].to_string(), Value::Map(Rc::new(RefCell::new(map))));
                    // }
                    other => return Err(format!("unknown import scheme '{}'", other)),
                }

                Ok(Signal::None)
            }

            StmtKind::ClassDecl {
                name,
                methods,
                superclass,
            } => {
                let superclass = match superclass {
                    Some(supername) => {
                        let val = self.env.borrow().get(supername.clone())?;
                        match val {
                            Value::Class(c) => Some(c),
                            _ => return Err(format!("superclass '{supername}' is not a class.")),
                        }
                    }
                    None => None,
                };

                let method_env = if let Some(ref sc) = superclass {
                    let mut env = Environment::new_enclosing(Some(Rc::clone(&self.env)));
                    env.define("super".to_string(), Value::Class(Rc::clone(sc)));
                    Rc::new(RefCell::new(env))
                } else {
                    Rc::clone(&self.env)
                };

                let mut method_map = IndexMap::new();
                let mut static_method_map = IndexMap::new();

                for method_stmt in methods {
                    if let StmtKind::FunDecl {
                        name: m_name,
                        params,
                        body,
                        is_static,
                    } = &method_stmt.kind
                    {
                        let fun_data = Rc::new(FunData::new(
                            m_name.clone(),
                            params.clone(),
                            body.clone(),
                            Rc::clone(&method_env),
                        ));

                        if *is_static {
                            static_method_map.insert(m_name.clone(), fun_data);
                        } else {
                            method_map.insert(m_name.clone(), fun_data);
                        }
                    }
                }

                let class = Rc::new(ClassData::new(
                    name.clone(),
                    method_map,
                    static_method_map,
                    superclass,
                ));

                self.env
                    .borrow_mut()
                    .define(name.clone(), Value::Class(class));
                Ok(Signal::None)
            }
        }
    }

    fn build_stack_trace(&self) -> String {
        let stack = self.call_stack.borrow();
        let mut trace = String::from("stack trace:\n");
        for (i, frame) in stack.iter().rev().enumerate() {
            if frame.fun_name == "<script>" {
                trace.push_str(&format!("  {}: at {}\n", i, frame.fun_name));
            } else {
                trace.push_str(&format!(
                    "  {}: at fun {}() (line {})\n",
                    i, frame.fun_name, frame.line
                ));
            }
        }
        trace
    }

    fn apply_comp_op(current: Value, op: &BinOp, rhs: Value) -> Result<Value, String> {
        match (current, rhs) {
            (Value::Number(a), Value::Number(b)) => {
                let result = match op {
                    BinOp::Plus => a + b,
                    BinOp::Minus => a - b,
                    BinOp::Multiply => a * b,
                    BinOp::Divide => {
                        if b == 0.0 {
                            return Err("division by zero".into());
                        }
                        a / b
                    }
                };
                Ok(Value::Number(result))
            }
            (Value::String(a), Value::String(b)) if matches!(op, BinOp::Plus) => {
                Ok(Value::String(a + &b))
            }
            (Value::String(a), Value::Number(b)) if matches!(op, BinOp::Multiply) => {
                Ok(Value::String(a.repeat(b as usize)))
            }
            _ => Err("binary operation on unsupported type".into()),
        }
    }

    fn validate_index(num: f64, len: usize) -> Result<usize, String> {
        if num < 0.0 {
            return Err("index cannot be negatives number".into());
        }
        let i = num as usize;
        if i >= len {
            return Err(format!(
                "index out of bounds. need index of {}, but only has {} indices.",
                i, len
            ));
        }
        Ok(i)
    }

    fn resolve_array_index(
        &self,
        target: &Expr,
        index: &Expr,
    ) -> Result<(Rc<RefCell<Vec<Value>>>, usize), String> {
        let var = self.eval_expr(target)?;
        let i = self.eval_expr(index)?;

        match (var, i) {
            (Value::Array(arr), Value::Number(num)) => {
                let idx = Self::validate_index(num, arr.borrow().len())?;
                Ok((arr, idx))
            }
            (Value::Array(_), _) => Err("index must be a number.".into()),
            _ => Err("cannot indexing of non-array type.".into()),
        }
    }

    fn resolve_property(&self, target: &Expr) -> Result<PropertyTarget, String> {
        let obj = self.eval_expr(target)?;
        match obj {
            Value::Map(map) => Ok(PropertyTarget::Map(map)),
            Value::Instance(inst) => Ok(PropertyTarget::Instance(inst)),
            Value::Class(class) => Ok(PropertyTarget::Class(class)),
            _ => Err("cannot access property on this type.".into()),
        }
    }
}

fn find_method(class: &Rc<ClassData>, name: &str) -> Option<Rc<FunData>> {
    if let Some(m) = class.methods.get(name) {
        return Some(Rc::clone(m));
    }

    match &class.superclass {
        Some(sc) => find_method(sc, name),
        None => None,
    }
}

fn find_static_method(class: &Rc<ClassData>, name: &str) -> Option<Value> {
    if let Some(m) = class.static_methods.get(name) {
        return Some(Value::Function(m.as_ref().clone()));
    }
    match &class.superclass {
        Some(sc) => find_static_method(sc, name),
        None => None,
    }
}

fn bind_method(fun: &Rc<FunData>, instance: Value) -> Value {
    let mut env = Environment::new_enclosing(Some(Rc::clone(&fun.closure)));
    env.define("self".to_string(), instance);
    Value::Function(FunData::new(
        fun.name.clone(),
        fun.params.clone(),
        fun.body.clone(),
        Rc::new(RefCell::new(env)),
    ))
}

enum PropertyTarget {
    Map(Rc<RefCell<IndexMap<String, Value>>>),
    Instance(Rc<InstanceData>),
    Class(Rc<ClassData>),
}

impl PropertyTarget {
    fn get(&self, key: &str) -> Option<Value> {
        match self {
            PropertyTarget::Map(map) => map.borrow().get(key).cloned(),
            PropertyTarget::Instance(inst) => inst.fields.borrow().get(key).cloned(),
            PropertyTarget::Class(class) => find_static_method(class, key),
        }
    }

    fn set(&self, key: String, val: Value) -> Result<(), String> {
        match self {
            PropertyTarget::Map(map) => {
                map.borrow_mut().insert(key, val);
                Ok(())
            }
            PropertyTarget::Instance(inst) => {
                inst.fields.borrow_mut().insert(key, val);
                Ok(())
            }
            PropertyTarget::Class(_) => Err("cannot set property on a class.".into()),
        }
    }
}
