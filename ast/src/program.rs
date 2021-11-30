// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! A Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of ast statements and expressions.

use crate::{Alias, Circuit, CircuitMember, DefinitionStatement, Function, FunctionInput, Identifier, ImportStatement};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program abstract syntax tree.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub expected_input: Vec<FunctionInput>,
    pub import_statements: Vec<ImportStatement>,
    #[serde(with = "crate::common::imported_modules")]
    pub imports: IndexMap<Vec<String>, Program>,
    pub aliases: IndexMap<Identifier, Alias>,
    pub circuits: IndexMap<Identifier, Circuit>,
    #[serde(with = "crate::common::global_consts_json")]
    pub global_consts: IndexMap<Vec<Identifier>, DefinitionStatement>,
    pub functions: IndexMap<Identifier, Function>,
}

impl AsRef<Program> for Program {
    fn as_ref(&self) -> &Program {
        self
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for import in self.import_statements.iter() {
            import.fmt(f)?;
            writeln!(f,)?;
        }
        writeln!(f,)?;
        for (_, alias) in self.aliases.iter() {
            alias.fmt(f)?;
            writeln!(f,)?;
        }
        writeln!(f,)?;
        for (_, import) in self.imports.iter() {
            import.fmt(f)?;
            writeln!(f,)?;
        }
        writeln!(f,)?;
        for (_, circuit) in self.circuits.iter() {
            circuit.fmt(f)?;
            writeln!(f,)?;
        }
        writeln!(f,)?;
        for (_, function) in self.functions.iter() {
            function.fmt(f)?;
            writeln!(f,)?;
        }
        write!(f, "")
    }
}

impl Program {
    pub fn new(name: String) -> Self {
        Self {
            name,
            expected_input: vec![],
            import_statements: vec![],
            imports: IndexMap::new(),
            aliases: IndexMap::new(),
            circuits: IndexMap::new(),
            global_consts: IndexMap::new(),
            functions: IndexMap::new(),
        }
    }

    pub fn handle_internal_annotations(&mut self) {
        self.circuits
            .iter_mut()
            .flat_map(|(_, circuit)| &mut circuit.members)
            .filter_map(|member| {
                if let CircuitMember::CircuitFunction(function) = member {
                    Some(function)
                } else {
                    None
                }
            })
            .into_iter()
            .for_each(|function| {
                function.annotations.clone().into_iter().for_each(|(name, _)| {
                    match (name.as_str(), function.annotations.remove(&name)) {
                        ("CoreFunction", Some(core_map)) => {
                            function.core_mapping.replace(
                                core_map
                                    .arguments
                                    .get(0)
                                    .or(Some(&function.identifier.name))
                                    .map(|f| f.to_string()),
                            );
                        }
                        ("AlwaysConst", Some(_)) => {
                            function.const_ = true;
                        }
                        _ => todo!("we should handle re-entrant parsing"),
                    }
                })
            });

        /* for (_, circuit) in self.circuits.iter_mut() {
            for member in circuit.members.iter_mut() {
                if let CircuitMember::CircuitFunction(function) = member {
                    for (name, annotation) in function.annotations.iter_mut() {
                        match name.as_str() {
                            "CoreFunction" if core_mapping_annotation => {
                                if let Some(core_map) = function.annotations.remove("CoreFunction") {
                                    function.core_mapping.replace(
                                        core_map
                                            .arguments
                                            .get(0)
                                            .or(Some(&function.identifier.name))
                                            .map(|f| f.to_string()),
                                    );
                                }
                            }
                            "AlwaysConst" => {}

                            _ => todo!("we should handle re-entrant parsing"),
                        }
                    }
                }
            }
        } */
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
