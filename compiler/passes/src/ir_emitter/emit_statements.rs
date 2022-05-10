use leo_ast::{Declare, StatementVisitor};
use snarkvm_bytecode::Program;

use crate::IrEmitter;

impl<'a, P: Program> StatementVisitor<'a> for IrEmitter<'a, P> {
    fn visit_statement(&mut self, input: &'a leo_ast::Statement) -> leo_ast::VisitResult {
        match input {
            leo_ast::Statement::Return(s) => self.visit_return(s),
            leo_ast::Statement::Definition(s) => self.visit_definition(s),
            leo_ast::Statement::Assign(s) => self.visit_assign(s),
            leo_ast::Statement::Conditional(s) => self.visit_conditional(s),
            leo_ast::Statement::Iteration(s) => self.visit_iteration(s),
            leo_ast::Statement::Console(s) => self.visit_console(s),
            leo_ast::Statement::Expression(s) => self.visit_expression_statement(s),
            leo_ast::Statement::Block(s) => self.visit_block(s),
        }
    }

    fn visit_return(&mut self, _input: &'a leo_ast::ReturnStatement) -> leo_ast::VisitResult {
        todo!()
    }

    fn visit_definition(&mut self, input: &'a leo_ast::DefinitionStatement) -> leo_ast::VisitResult {
        let constant = input.declaration_type == Declare::Const;
        // todo left off here
        todo!()
    }

    fn visit_assign(&mut self, _input: &'a leo_ast::AssignStatement) -> leo_ast::VisitResult {
        todo!()
    }

    fn visit_conditional(&mut self, _input: &'a leo_ast::ConditionalStatement) -> leo_ast::VisitResult {
        unimplemented!("conditional statements don't exits in IR")
    }

    fn visit_iteration(&mut self, _input: &'a leo_ast::IterationStatement) -> leo_ast::VisitResult {
        unimplemented!("iteration statements don't exist in IR")
    }

    fn visit_console(&mut self, _input: &'a leo_ast::ConsoleStatement) -> leo_ast::VisitResult {
        unimplemented!("console statements don't exist in IR")
    }

    fn visit_expression_statement(&mut self, input: &'a leo_ast::ExpressionStatement) -> leo_ast::VisitResult {
        self.emit_expr(&input.expression);
        self.drop_reg();
        Default::default()
    }

    fn visit_block(&mut self, input: &'a leo_ast::Block) -> leo_ast::VisitResult {
        for s in &input.statements {
            self.visit_statement(s);
        }
        todo!()
    }
}
