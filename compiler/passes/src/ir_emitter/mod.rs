// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

pub mod emit_expressions;
pub use emit_expressions::*;

pub mod emit_file;
pub use emit_file::*;

pub mod emit_statements;
pub use emit_statements::*;

pub mod emitter;
pub use emitter::*;
use snarkvm_bytecode::Program;

use crate::{Pass, SymbolTable};

use leo_ast::{Ast, VisitorDirector};
use leo_errors::Result;
impl<'a, P: Program> Pass<'a> for IrEmitter<'a, P> {
    type Input = (&'a Ast, &'a mut SymbolTable<'a>);
    type Output = Result<()>;

    fn do_pass((ast, symbol_table): Self::Input) -> Self::Output {
        let mut visitor = VisitorDirector::new(IrEmitter::<P>::new(symbol_table));
        visitor.visit_program(ast.as_repr());
        Ok(())
    }
}
