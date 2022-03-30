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

use super::{BlockSymbol, ExpressionSymbol, Span, Type, ID};

pub struct IterationSymbol {
    pub var: ID,
    pub start: ExpressionSymbol,
    pub stop: ExpressionSymbol,
    pub inclusive: bool,
    pub block: BlockSymbol,
    pub span: Span,
    pub type_: Type,
}
