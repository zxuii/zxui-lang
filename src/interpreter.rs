use indexmap::IndexMap;

use crate::ast::{BinOp, CompOp, Expr, LogicalOp, Stmt, UnaryOp};
use crate::builtins::*;
use crate::environment::Environment;
use crate::object::Value;

use std::cell::RefCell;
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
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interp = Self {
            env: Rc::new(RefCell::new(Environment::new())),
            call_stack: Rc::new(RefCell::new(Vec::new())),
        };
        interp.define_natives();
        interp
            .call_stack
            .borrow_mut()
            .push(CallFrame::new("<script>".to_string(), 0));

        interp
    }

    pub fn new_env(env: Rc<RefCell<Environment>>, call_stack: Rc<RefCell<Vec<CallFrame>>>) -> Self {
        Self { env, call_stack }
    }

    fn define_natives(&mut self) {
        // raylibs
        let ray = Rc::new(
            Raylib::new("./raylib/lib/raylib.dll".to_string()).expect("failed to load raylib.dll"),
        );

        self.env
            .borrow_mut()
            .define("initWindow".to_string(), raylib_init_window(ray.clone()));
        self.env.borrow_mut().define(
            "windowShouldClose".to_string(),
            raylib_windows_should_close(ray.clone()),
        );
        self.env.borrow_mut().define(
            "beginDrawing".to_string(),
            raylib_begin_drawing(ray.clone()),
        );
        self.env
            .borrow_mut()
            .define("endDrawing".to_string(), raylib_end_drawing(ray.clone()));
        self.env
            .borrow_mut()
            .define("closeWindow".to_string(), raylib_close_window(ray.clone()));
        self.env.borrow_mut().define(
            "clearBackground".to_string(),
            raylib_clear_background(ray.clone()),
        );
        self.env.borrow_mut().define(
            "drawRectangle".to_string(),
            raylib_draw_rectangle(ray.clone()),
        );
        self.env
            .borrow_mut()
            .define("isKeyDown".to_string(), raylib_is_key_down(ray.clone()));
        self.env
            .borrow_mut()
            .define("getFrameTime".to_string(), raylib_get_frame_time(ray));

        // langs
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
                    Value::Function {
                        name,
                        params,
                        body,
                        closure,
                    } => {
                        if self.call_stack.borrow().len() >= MAX_DEPTH {
                            let trace = self.build_stack_trace();
                            return Err(format!(
                                "stack overflow: maximum recursion depth exceed.\n{trace}"
                            ));
                        }

                        if evaluated_args.len() != params.len() {
                            return Err(format!(
                                "function <closure> expects {} args but got {}",
                                params.len(),
                                evaluated_args.len()
                            ));
                        }

                        let call_env = Rc::new(RefCell::new(Environment::new_enclosing(Some(
                            Rc::clone(&closure),
                        ))));
                        for (param, arg) in params.iter().zip(evaluated_args) {
                            call_env.borrow_mut().define(param.clone(), arg);
                        }

                        self.call_stack
                            .borrow_mut()
                            .push(CallFrame::new(name.clone(), *line));

                        let mut interp =
                            Interpreter::new_env(call_env, Rc::clone(&self.call_stack));
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

                    Value::NativeFunction { fun, arity, name } => {
                        if arity != -1 && evaluated_args.len() != arity as usize {
                            return Err(format!(
                                "function {}() expects {} args but got {}",
                                name,
                                arity,
                                evaluated_args.len()
                            ));
                        }
                        fun(evaluated_args)
                    }

                    _ => Err("attempted to call a non-function value".into()),
                }
            }

            Expr::Index { target, index } => {
                let (arr, i) = self.resolve_array_index(target, index)?;
                Ok(arr.borrow()[i].clone())
            }

            Expr::Get { target, name } => {
                let (map, key) = self.resolve_map_property(target, name)?;
                match map.borrow().get(&key) {
                    Some(val) => Ok(val.clone()),
                    None => Err(format!("property '{}' does not exist on this object.", key)),
                }
            }
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
        match stmt {
            Stmt::Program(stmts) => {
                let ret = self.exec_stmts(stmts)?;
                Ok(ret)
            }

            Stmt::Block(stmts) => {
                let ret = self.exec_block(stmts)?;
                Ok(ret)
            }

            Stmt::Let { name, expr } => {
                let val = self.eval_expr(expr)?;
                self.env.borrow_mut().define(name.clone(), val);
                Ok(Signal::None)
            }

            Stmt::Assign { target, expr } => {
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
                        let target_val = self.eval_expr(target)?;
                        match target_val {
                            Value::Map(map) => {
                                map.borrow_mut().insert(name.clone(), val);
                            }
                            _ => return Err("cannot set property on non-map type.".into()),
                        }
                    }
                    _ => return Err("invalid assignment target".into()),
                }
                Ok(Signal::None)
            }

            Stmt::CompAssign { target, op, expr } => {
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
                        let (map, key) = self.resolve_map_property(obj_target, name)?;
                        let current = match map.borrow().get(&key) {
                            Some(val) => val.clone(),
                            None => {
                                return Err(format!(
                                    "property '{}' does not exist on this object.",
                                    key
                                ));
                            }
                        };
                        let new_val = Self::apply_comp_op(current, op, rhs)?;
                        map.borrow_mut().insert(key, new_val);
                    }
                    _ => return Err("invalid assignment target".into()),
                }
                Ok(Signal::None)
            }

            Stmt::If {
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

            Stmt::While { expr, block } => {
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

            Stmt::For { name, expr, block } => {
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

            Stmt::Return(expr) => {
                let val = self.eval_expr(expr)?;
                Ok(Signal::Return(Some(val)))
            }

            Stmt::Break => Ok(Signal::Break),
            Stmt::Continue => Ok(Signal::Continue),

            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr)?;
                Ok(Signal::None)
            }

            Stmt::FunDecl { name, params, body } => {
                let fun = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: Rc::clone(&self.env),
                };
                self.env.borrow_mut().define(name.clone(), fun);
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

    fn resolve_map_property(
        &self,
        target: &Expr,
        name: &str,
    ) -> Result<(Rc<RefCell<IndexMap<String, Value>>>, String), String> {
        let obj = self.eval_expr(target)?;

        match obj {
            Value::Map(map) => Ok((map, name.to_string())),
            _ => Err(format!("cannot access property on non-map type.")),
        }
    }
}
