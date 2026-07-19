use indexmap::IndexMap;

use crate::ast::{BinOp, CompOp, Expr, LogicalOp, Stmt, StmtKind, UnaryOp};
use crate::builtins::*;
use crate::environment::Environment;
use crate::lexer::Lexer;
use crate::object::{ClassData, FunData, InstanceData, MethodKind, Value};
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

pub enum IndexTarget {
    Array(Rc<RefCell<Vec<Value>>>, usize),
    StringChar(String, usize),
}

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
    call_stack: Rc<RefCell<Vec<CallFrame>>>,
    filename: Rc<str>,
    code: Rc<str>,
    root_dir: Option<Rc<str>>,
    types: Rc<crate::object::TypeRegistry>,
}

impl Interpreter {
    pub fn new(filename: String, code: String) -> Self {
        let mut interp = Self {
            env: Rc::new(RefCell::new(Environment::new())),
            call_stack: Rc::new(RefCell::new(Vec::new())),
            filename: Rc::from(filename),
            code: Rc::from(code),
            root_dir: None,
            types: Rc::new(crate::types::build_type_registry()),
        };
        interp.define_natives();
        interp.define_types();
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
        types: Rc<crate::object::TypeRegistry>,
    ) -> Self {
        Self {
            env,
            call_stack,
            filename,
            code,
            root_dir,
            types,
        }
    }

    pub fn new_with_root(filename: Rc<str>, code: Rc<str>, root_dir: Rc<str>) -> Self {
        let mut interp = Self {
            env: Rc::new(RefCell::new(Environment::new())),
            call_stack: Rc::new(RefCell::new(Vec::new())),
            filename: filename,
            code: code,
            root_dir: Some(root_dir),
            types: Rc::new(crate::types::build_type_registry()),
        };
        interp.define_natives();
        interp.define_types();
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
            "range".to_string(),
            Value::native_fun("range".to_string(), -1, Rc::new(native_range)),
        );

        let types = self.types.clone();
        self.env.borrow_mut().define(
            "typeof".to_string(),
            Value::native_fun(
                "typeof".to_string(),
                1,
                Rc::new(move |args| Ok(Value::Class(types.class_for(&args[0])))),
            ),
        );
    }

    fn define_types(&mut self) {
        self.env.borrow_mut().define(
            "Number".to_string(),
            Value::Class(Rc::clone(&self.types.number)),
        );
        self.env.borrow_mut().define(
            "String".to_string(),
            Value::Class(Rc::clone(&self.types.string)),
        );
        self.env.borrow_mut().define(
            "Boolean".to_string(),
            Value::Class(Rc::clone(&self.types.boolean)),
        );
        self.env.borrow_mut().define(
            "Array".to_string(),
            Value::Class(Rc::clone(&self.types.array)),
        );
        self.env
            .borrow_mut()
            .define("Map".to_string(), Value::Class(Rc::clone(&self.types.map)));
        self.env.borrow_mut().define(
            "Null".to_string(),
            Value::Class(Rc::clone(&self.types.null)),
        );
        self.env.borrow_mut().define(
            "Function".to_string(),
            Value::Class(Rc::clone(&self.types.function)),
        );
        self.env.borrow_mut().define(
            "NativeFunction".to_string(),
            Value::Class(Rc::clone(&self.types.native_function)),
        );
        self.env.borrow_mut().define(
            "Class".to_string(),
            Value::Class(Rc::clone(&self.types.class)),
        );
    }

    fn try_native_type_conversion(
        &self,
        class: &Rc<ClassData>,
        args: &[Value],
    ) -> Result<Option<Value>, String> {
        if Rc::ptr_eq(class, &self.types.number) {
            if args.len() != 1 {
                return Err(format!(
                    "Number(): expects 1 argument but got {}",
                    args.len()
                ));
            }
            return Ok(Some(crate::types::convert_to_number(&args[0])?));
        }
        if Rc::ptr_eq(class, &self.types.string) {
            if args.len() != 1 {
                return Err(format!(
                    "String(): expects 1 argument but got {}",
                    args.len()
                ));
            }
            return Ok(Some(Value::String(format!("{}", args[0]))));
        }
        if Rc::ptr_eq(class, &self.types.boolean) {
            if args.len() != 1 {
                return Err(format!(
                    "Boolean(): expects 1 argument but got {}",
                    args.len()
                ));
            }
            return Ok(Some(crate::types::convert_to_boolean(&args[0])));
        }
        if Rc::ptr_eq(class, &self.types.array) {
            if args.len() != 1 {
                return Err(format!(
                    "Array(): expects 1 argument but got {}",
                    args.len()
                ));
            }
            return Ok(Some(crate::types::convert_to_array(&args[0])?));
        }

        Ok(None)
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

                if matches!(op, CompOp::EqEq | CompOp::NotEq) {
                    let is_eq = values_equal(&l, &r);
                    let result = if matches!(op, CompOp::EqEq) {
                        is_eq
                    } else {
                        !is_eq
                    };
                    return Ok(Value::Boolean(result));
                }

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
                    (a, b) => Err(format!(
                        "cannot use ordering comparison ('<','>','<=','>=') between '{}' and '{}'",
                        a, b,
                    )),
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
                        if let Some(converted) =
                            self.try_native_type_conversion(&class, &evaluated_args)?
                        {
                            return Ok(converted);
                        }

                        let instance = Rc::new(InstanceData {
                            class: Rc::clone(&class),
                            fields: RefCell::new(IndexMap::new()),
                            native: RefCell::new(None),
                        });
                        if let Some(init_fun) = find_method(&class, "init") {
                            let bound =
                                bind_method(&init_fun, Value::Instance(Rc::clone(&instance)));
                            match bound {
                                Value::Function(f) => {
                                    self.call_user_function(
                                        f.name,
                                        f.params,
                                        f.body,
                                        f.closure,
                                        evaluated_args,
                                        *line,
                                    )?;
                                }
                                Value::NativeFunction(f) => {
                                    (f.fun)(evaluated_args)?;
                                }
                                _ => unreachable!(),
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
            Expr::Index { target, index } => match self.resolve_index(target, index)? {
                IndexTarget::Array(arr, i) => Ok(arr.borrow()[i].clone()),
                IndexTarget::StringChar(s, i) => {
                    let ch = s.chars().nth(i).unwrap();
                    Ok(Value::String(ch.to_string()))
                }
            },

            Expr::Get { target, name } => {
                let target_val = self.eval_expr(target)?;

                match &target_val {
                    Value::Map(map) => {
                        if let Some(val) = map.borrow().get(name).cloned() {
                            return Ok(val);
                        }
                    }
                    Value::Instance(inst) => {
                        if let Some(getter) = &inst.class.native_get {
                            if let Some(val) = getter(inst, name) {
                                return Ok(val);
                            }
                        }
                        if let Some(val) = inst.fields.borrow().get(name).cloned() {
                            return Ok(val);
                        }
                        if let Some(fun) = find_method(&inst.class, name) {
                            return Ok(bind_method(&fun, Value::Instance(Rc::clone(inst))));
                        }
                    }
                    Value::Class(class) => {
                        if let Some(m) = find_static_method(class, name) {
                            return Ok(bind_static(&m));
                        }
                    }
                    _ => {}
                }

                let type_class = self.types.class_for(&target_val);
                if let Some(fun) = find_method(&type_class, name) {
                    return Ok(bind_method(&fun, target_val));
                }

                Err(format!("property '{}' does not exist on this value.", name))
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
            self.types.clone(),
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
                    Expr::Index { target, index } => match self.resolve_index(target, index)? {
                        IndexTarget::Array(arr, i) => {
                            arr.borrow_mut()[i] = val;
                        }
                        IndexTarget::StringChar(_, _) => {
                            return Err(
                                "cannot assign to a string index; strings are immutable".into()
                            );
                        }
                    },
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
                    } => match self.resolve_index(arr_target, index)? {
                        IndexTarget::Array(arr, i) => {
                            let current = arr.borrow()[i].clone();
                            let new_val = Self::apply_comp_op(current, op, rhs)?;
                            arr.borrow_mut()[i] = new_val;
                        }
                        IndexTarget::StringChar(_, _) => {
                            return Err(
                                "cannot assign to a string index; strings are immutable".into()
                            );
                        }
                    },
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
                            self.types.clone(),
                        );
                        module_interp.define_natives();
                        module_interp.define_types();

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

                    "std" => {
                        let map = match parts[1] {
                            "ffi" => {
                                let root = match &self.root_dir {
                                    Some(r) => r.clone(),
                                    None => {
                                        return Err(
                        "cannot use 'std:ffi' without a project, please run `zxui init`."
                            .into(),
                    );
                                    }
                                };
                                crate::ffi::make_ffi_module(root)
                            }
                            other => {
                                return Err(format!(
                                    "unknown standard library module named '{}'",
                                    other
                                ));
                            }
                        };
                        self.env
                            .borrow_mut()
                            .define(parts[1].to_string(), Value::Map(Rc::new(RefCell::new(map))));
                    }
                    other => return Err(format!("unknown import schema '{}'", other)),
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
                            static_method_map.insert(m_name.clone(), MethodKind::User(fun_data));
                        } else {
                            method_map.insert(m_name.clone(), MethodKind::User(fun_data));
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
                    "  {}: at fun {}() at {}:{}\n",
                    i, frame.fun_name, self.filename, frame.line
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

    fn resolve_index(&self, target: &Expr, index: &Expr) -> Result<IndexTarget, String> {
        let var = self.eval_expr(target)?;
        let i = self.eval_expr(index)?;

        match (var, i) {
            (Value::Array(arr), Value::Number(num)) => {
                let idx = Self::validate_index(num, arr.borrow().len())?;
                Ok(IndexTarget::Array(arr, idx))
            }
            (Value::Array(_), _) => Err("index must be a number.".into()),

            (Value::String(s), Value::Number(num)) => {
                let char_len = s.chars().count();
                let idx = Self::validate_index(num, char_len)?;
                Ok(IndexTarget::StringChar(s, idx))
            }
            (Value::String(_), _) => Err("index must be a number.".into()),

            _ => Err("can only indexing array and string type.".into()),
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

fn find_method(class: &Rc<ClassData>, name: &str) -> Option<MethodKind> {
    if let Some(m) = class.methods.get(name) {
        return Some(m.clone());
    }
    match &class.superclass {
        Some(sc) => find_method(sc, name),
        None => None,
    }
}

fn find_static_method(class: &Rc<ClassData>, name: &str) -> Option<MethodKind> {
    if let Some(m) = class.static_methods.get(name) {
        return Some(m.clone());
    }
    match &class.superclass {
        Some(sc) => find_static_method(sc, name),
        None => None,
    }
}

fn bind_method(method: &MethodKind, instance: Value) -> Value {
    match method {
        MethodKind::User(fun) => {
            let mut env = Environment::new_enclosing(Some(Rc::clone(&fun.closure)));
            env.define("self".to_string(), instance);
            Value::Function(FunData::new(
                fun.name.clone(),
                fun.params.clone(),
                fun.body.clone(),
                Rc::new(RefCell::new(env)),
            ))
        }
        MethodKind::Native(native) => {
            let native = Rc::clone(native);
            Value::native_fun(
                native.name.clone(),
                native.arity,
                Rc::new(move |args| (native.fun)(instance.clone(), args)),
            )
        }
    }
}

fn bind_static(method: &MethodKind) -> Value {
    match method {
        MethodKind::User(fun) => Value::Function(fun.as_ref().clone()),
        MethodKind::Native(native) => {
            let native = Rc::clone(native);
            Value::native_fun(
                native.name.clone(),
                native.arity,
                Rc::new(move |args| (native.fun)(Value::Null, args)),
            )
        }
    }
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
            PropertyTarget::Instance(inst) => {
                if let Some(getter) = &inst.class.native_get {
                    if let Some(val) = getter(inst, key) {
                        return Some(val);
                    }
                }
                inst.fields.borrow().get(key).cloned()
            }
            PropertyTarget::Class(class) => find_static_method(class, key).map(|m| bind_static(&m)),
        }
    }

    fn set(&self, key: String, val: Value) -> Result<(), String> {
        match self {
            PropertyTarget::Map(map) => {
                map.borrow_mut().insert(key, val);
                Ok(())
            }
            PropertyTarget::Instance(inst) => {
                if let Some(setter) = &inst.class.native_set {
                    return setter(inst, &key, val);
                }
                inst.fields.borrow_mut().insert(key, val);
                Ok(())
            }
            PropertyTarget::Class(_) => Err("cannot set property on a class.".into()),
        }
    }
}

fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Number(x), Value::Number(y)) => x == y,
        (Value::Boolean(x), Value::Boolean(y)) => x == y,
        (Value::String(x), Value::String(y)) => x == y,
        (Value::Array(x), Value::Array(y)) => Rc::ptr_eq(x, y),
        (Value::Map(x), Value::Map(y)) => Rc::ptr_eq(x, y),
        (Value::Class(x), Value::Class(y)) => Rc::ptr_eq(x, y),
        (Value::Instance(x), Value::Instance(y)) => Rc::ptr_eq(x, y),
        // beda tipe apapun (termasuk lawannya null) -> otomatis false, BUKAN error
        _ => false,
    }
}
