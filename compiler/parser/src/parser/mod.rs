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

//! The parser to convert Leo code text into an [`Program`] AST type.
//!
//! This module contains the [`parse()`] method which calls the underlying [`tokenize()`]
//! method to create a new program ast.

use crate::{tokenizer::*, Token};

use leo_ast::*;
use leo_errors::emitter::Handler;
use leo_errors::{ParserError, Result};
use leo_span::Span;

use indexmap::IndexMap;
use leo_span::span::BytePos;
use std::unreachable;

mod context;
pub use context::*;

pub mod expression;
pub mod file;
pub mod input;
pub mod statement;
pub mod type_;

pub(crate) fn assert_no_whitespace(left_span: &Span, right_span: &Span, left: &str, right: &str) -> Result<()> {
    if left_span.hi != right_span.lo {
        let error_span = Span::new(left_span.hi, right_span.lo); // The span between them.
        return Err(ParserError::unexpected_whitespace(left, right, &error_span).into());
    }

    Ok(())
}

/// Creates a new program from a given file path and source code text.
pub fn parse(handler: &Handler, source: &str, start_pos: BytePos) -> Result<Program> {
    let mut tokens = ParserContext::new(handler, crate::tokenize(source, start_pos)?);

    tokens.parse_program()
}

/// Parses an input file at the given file `path` and `source` code text.
pub fn parse_input(handler: &Handler, source: &str, start_pos: BytePos) -> Result<InputAst> {
    let mut tokens = ParserContext::new(handler, crate::tokenize(source, start_pos)?);

    tokens.parse_input()
}
