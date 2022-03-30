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

use indexmap::IndexMap;

use super::{Span, StatementSymbol, Type, ID};

pub struct FunctionSymbol {
    pub id: ID,
    pub signature: FunctionSignature,
    pub statements: Vec<StatementSymbol>,
    pub span: Span,
}

pub struct FunctionSignature {
    pub const_: bool,
    pub inputs: IndexMap<ID, FunctionInput>,
    pub outputs: Vec<Type>,
}

pub struct FunctionInput {
    pub const_: bool,
    pub id: ID,
    pub type_: Type,
}
