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

pub mod type_checker;
pub use type_checker::*;

use leo_ast::{AstPass, Program};
use leo_errors::Result;
use leo_tabi::SymbolTable;

impl AstPass for TypeChecker {
    type Output = SymbolTable;

    fn do_pass(self, ast: Program) -> Result<Self::Output> {
        let type_checker = Self::forward_declarations(&ast.functions);

        type_checker.construct_symbol_table(ast)
    }
}
