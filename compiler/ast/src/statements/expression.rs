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

use crate::{Expression, Node};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An expression statement `expr;`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ExpressionStatement {
    /// The expression to evaluate purely for its side-effects.
    pub expression: Expression,
    /// The span excluding the semicolon.
    pub span: Span,
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{};", self.expression)
    }
}

crate::simple_node_impl!(ExpressionStatement);
