use leo_ast::ProgramVisitor;
use snarkvm_bytecode::Program;

use crate::IrEmitter;

impl<'a, P: Program> ProgramVisitor<'a> for IrEmitter<'a, P> {
    fn visit_function(&mut self, _input: &'a leo_ast::Function) -> leo_ast::VisitResult {
        todo!()
    }
}
