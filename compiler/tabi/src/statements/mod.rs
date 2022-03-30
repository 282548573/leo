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

use super::ExpressionSymbol;
use leo_ast::Type;
use leo_span::{Span, Symbol as ID};

mod assign;
pub use assign::*;

mod block;
pub use block::*;

mod conditional;
pub use conditional::*;

mod console;
pub use console::*;

mod definition;
pub use definition::*;

mod expression;
pub use expression::*;

mod iteration;
pub use iteration::*;

mod return_sym;
pub use return_sym::*;

pub enum StatementSymbol {
    Assign(AssignSymbol),
    Block(BlockSymbol),
    Conditional(AssignSymbol),
    Console(ConsoleSymbol),
    Definition(DefinitionSymbol),
    Expression(ExpressionStatementSymbol),
    Iteration(IterationSymbol),
    Return(ReturnSymbol),
}
