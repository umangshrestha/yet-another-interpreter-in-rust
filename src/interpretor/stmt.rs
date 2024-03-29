use std::{cell::RefCell, rc::Rc};

use crate::{
    object::Function, visitor, Environment, Error, ErrorInfo, Expr, Interpretor, Object, Span, Stmt,
};

impl visitor::Stmt for Interpretor {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), ErrorInfo> {
        let out = self.eval(expr)?;
        println!("{}", out);
        Ok(())
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), ErrorInfo> {
        self.eval(expr)?;
        Ok(())
    }

    fn visit_let_stmt(
        &mut self,
        name: &String,
        value: &Option<Expr>,
        is_const: bool,
        span: &Span,
    ) -> Result<(), ErrorInfo> {
        let value = value
            .as_ref()
            .map(|v| self.eval(v))
            .unwrap_or(Ok(Object::Nil))?;

        self.environment
            .borrow_mut()
            .define(name.to_owned(), value, is_const)
            .map_err(|e| ErrorInfo::new_with_span(e, span.to_owned()))
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Box<Stmt>) -> Result<(), ErrorInfo> {
        let mut flag = self.eval(condition)?;
        while flag.to_boolean() {
            self.exec(body)?;
            flag = self.eval(condition)?;
        }
        Ok(())
    }

    fn visit_return_stmt(&mut self, value: &Option<Expr>, span: &Span) -> Result<(), ErrorInfo> {
        Err(ErrorInfo::new_with_span(
            Error::Return(if let Some(expr) = value {
                self.eval(expr)?
            } else {
                Object::Nil
            }),
            span.to_owned(),
        ))
    }
    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), ErrorInfo> {
        self.exec_block(
            stmts,
            Rc::new(RefCell::new(Environment::new_from_closure(
                &self.environment,
            ))),
        )
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        truthy: &Box<Stmt>,
        falsy: &Option<Box<Stmt>>,
    ) -> Result<(), ErrorInfo> {
        if self.eval(condition)?.to_boolean() {
            self.exec(truthy)
        } else if let Some(expr) = falsy {
            self.exec(expr)
        } else {
            Ok(())
        }
    }

    fn visit_function_stmt(
        &mut self,
        name: &String,
        params: &Vec<String>,
        body: &Vec<Stmt>,
        span: &Span,
    ) -> Result<(), ErrorInfo> {
        let function = Function::User {
            name: name.to_owned(),
            span: span.to_owned(),
            params: params.to_owned(),
            body: body.to_owned(),
            closure: self.environment.clone(),
            is_initializer: false,
        };
        self.environment
            .borrow_mut()
            .define(name.to_owned(), Object::Function(function), false)
            .map_err(|e| ErrorInfo::new_with_span(e, span.to_owned()))
    }

    fn visit_class_stmt(
        &mut self,
        name: &String,
        super_class: &Option<String>,
        methods: &Vec<Stmt>,
        span: &Span,
    ) -> Result<(), ErrorInfo> {
        todo!();
    }

    fn visit_break_stmt(&mut self, span: &Span) -> Result<(), ErrorInfo> {
        todo!();
    }

    fn visit_continue_stmt(&mut self, span: &Span) -> Result<(), ErrorInfo> {
        todo!();
    }
}
