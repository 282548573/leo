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

use indexmap::{IndexMap, IndexSet};
use leo_ast::*;
use leo_errors::Result;
use leo_span::Symbol;
use leo_tabi::*;

#[derive(Default)]
pub struct TypeChecker {
    functions: IndexSet<Symbol>,
    vars: IndexSet<Symbol>,
}

impl TypeChecker {
    pub(crate) fn forward_declarations(functions: &IndexMap<Identifier, Function>) -> Self {
        Self {
            functions: functions.iter().map(|(id, _)| id.name).collect(),
            vars: IndexSet::new(),
        }
    }

    pub(crate) fn construct_symbol_table(&self, ast: Program) -> Result<SymbolTable> {
        let functions = ast
            .functions
            .into_iter()
            .map(|(id, f)| Ok((id.name, self.convert_function(f)?)))
            .collect::<Result<IndexMap<_, _>>>()?;

        Ok(SymbolTable { functions })
    }

    pub(crate) fn convert_function(&self, f: Function) -> Result<FunctionSymbol> {
        let vars = IndexMap::new();
        Ok(FunctionSymbol {
            id: f.identifier.name,
            signature: FunctionSignature {
                const_: f.const_,
                inputs: f
                    .input
                    .into_iter()
                    .map(|input| self.convert_function_input(input))
                    .collect::<Result<IndexMap<_, _>>>()?,
                // TODO: unwrap needs to be removed when types are required.
                outputs: f.output.unwrap(),
            },
            statements: self.convert_block(f.block)?,
            span: f.span,
            vars,
        })
    }

    pub(crate) fn convert_function_input(&self, input: FunctionInput) -> Result<(Symbol, FunctionInputSymbol)> {
        Ok((
            todo!(),
            FunctionInputSymbol {
                const_: todo!(),
                id: todo!(),
                type_: todo!(),
            },
        ))
    }

    pub(crate) fn convert_statement(&self, statement: Statement) -> Result<StatementSymbol> {
        Ok(
            todo!(), /* match statement {
                         Statement::Assign(assign) =>
                     } */
        )
    }

    pub(crate) fn convert_assign(&self, block: AssignStatement) -> Result<AssignSymbol> {
        Ok(todo!())
    }

    pub(crate) fn convert_block(&self, block: Block) -> Result<BlockSymbol> {
        Ok(BlockSymbol {
            statements: block
                .statements
                .into_iter()
                .map(|s| self.convert_statement(s))
                .collect::<Result<Vec<_>>>()?,
            span: block.span,
        })
    }
}
