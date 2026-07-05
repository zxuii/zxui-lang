use crate::ast::{BinOp, Expr, Stmt, UnaryOp};
use crate::environment::Environment;
use crate::object::Value;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new()))
        }
    }

    pub fn eval_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(num) => Ok(Value::Number(*num)),
            Expr::Null => Ok(Value::Null),
            Expr::NoOp => Ok(Value::Null),
            Expr::Identifier(name) => {
                self.env.borrow().get(name.clone())
            }
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
                                if b == 0.0 { return Err("Division by zero".into())}
                                a / b
                            },
                        };
                        Ok(Value::Number(result))
                    }
                    _ => Err("Binary operation on non-numbers".into())
                }
            }

            Expr::Call { callee, args} => {
                todo!()
            }
        }
    }

    pub fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::Program(stmts) => {
                let mut ret = None;
                for stmt in stmts {
                    ret = self.exec_stmt(stmt)?;
                    if ret.is_some() { break; }
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
                    if ret.is_some() { break; }
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
                let val = self.eval_expr(expr)?;
                Ok(Some(val))
            }

            Stmt::FunDecl { name, params, body } => todo!()

        }
    }
}