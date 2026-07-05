use crate::ast::{BinOp, Expr, Stmt, UnaryOp};
use crate::environment::Environment;
use crate::object::Value;
use crate::builtins::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}



impl Interpreter {
    pub fn new() -> Self {
        let mut interp = Self {
            env: Rc::new(RefCell::new(Environment::new())),
        };
        interp.define_natives();
        interp
    }

    pub fn new_env(env: Rc<RefCell<Environment>>) -> Self {
        Self {
            env,
        }
    }

    fn define_natives(&mut self) {
        self.env.borrow_mut().define("println".to_string(), Value::native_fun("println".to_string(), -1, Rc::new(native_println)));
        self.env.borrow_mut().define("print".to_string(), Value::native_fun("print".to_string(), -1, Rc::new(native_print)));
        self.env.borrow_mut().define("readline".to_string(), Value::native_fun("print".to_string(), -1, Rc::new(native_readline)));
    }

    pub fn eval_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(num) => Ok(Value::Number(*num)),
            Expr::Null => Ok(Value::Null),
            Expr::NoOp => Ok(Value::Null),
            Expr::Identifier(name) => self.env.borrow().get(name.clone()),
            Expr::Unary { op, expr } => {
                let val = self.eval_expr(expr)?;
                match (op, val) {
                    (UnaryOp::Plus, Value::Number(num)) => Ok(Value::Number(num)),
                    (UnaryOp::Minus, Value::Number(num)) => Ok(Value::Number(-num)),
                    _ => Err("Unary op on non-number".into()),
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
                                    return Err("Division by zero".into());
                                }
                                a / b
                            }
                        };
                        Ok(Value::Number(result))
                    }
                    _ => Err("Binary operation on non-numbers".into()),
                }
            }

            Expr::Call { callee, args } => {
                let fun = self.env.borrow_mut().get(callee.clone())?;

                let evaluated_args: Result<Vec<Value>, String> =
                    args.iter().map(|arg| self.eval_expr(arg)).collect();
                let evaluated_args = evaluated_args?; // entah kenapa ga ke infer, jadi gini aja :V

                match fun {
                    Value::Function {
                        name: _,
                        params,
                        body,
                        closure,
                    } => {
                        if evaluated_args.len() != params.len() {
                            return Err(format!(
                                "function '{}' expects {} args but got {}",
                                callee,
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

                        let mut interp = Interpreter::new_env(call_env);
                        let mut return_val = Value::Null;
                        for stmt in &body {
                            match interp.exec_stmt(stmt)? {
                                Some(val) => {
                                    return_val = val;
                                    break;
                                }
                                None => {}
                            }
                        }

                        Ok(return_val)
                    }

                    Value::NativeFunction { fun, arity, name } => {
                        if arity != -1 && evaluated_args.len() != arity as usize {
                            return Err(format!(
                                "function '{}' expects {} args but got {}",
                                name,
                                arity,
                                evaluated_args.len()
                            ));
                        }
                        fun(evaluated_args)
                    }

                    _ => Err(format!("'{callee}' is not a function.")),
                }
            }
        }
    }

    pub fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::Program(stmts) => {
                let mut ret = None;
                for stmt in stmts {
                    ret = self.exec_stmt(stmt)?;
                    if ret.is_some() {
                        break;
                    }
                }
                Ok(ret)
            }

            Stmt::Block(stmts) => {
                let child = Environment::new_enclosing(Some(Rc::clone(&self.env)));
                let prev = Rc::clone(&self.env);
                self.env = Rc::new(RefCell::new(child));

                let mut ret = None;
                for stmt in stmts {
                    ret = self.exec_stmt(stmt)?;
                    if ret.is_some() {
                        break;
                    }
                }

                self.env = prev;
                Ok(ret)
            }

            Stmt::Let { name, expr } => {
                let val = self.eval_expr(expr)?;
                self.env.borrow_mut().define(name.clone(), val);
                Ok(None)
            }

            Stmt::Assign { name, expr } => {
                let val = self.eval_expr(expr)?;
                self.env.borrow_mut().assign(name.clone(), val)?;
                Ok(None)
            }

            Stmt::Return(expr) => {
                let val = self.eval_expr(expr)?;
                Ok(Some(val))
            }

            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }

            Stmt::FunDecl { name, params, body } => {
                let fun = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: Rc::clone(&self.env),
                };
                self.env.borrow_mut().define(name.clone(), fun);
                Ok(None)
            }
        }
    }
}
