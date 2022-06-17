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

use crate::algorithms::{CoreFunction, ALL_TYPES};
use leo_ast::Type;

pub struct BHP256Hash;

impl CoreFunction for BHP256Hash {
    const NUM_ARGS: usize = 1;

    fn first_arg_types() -> &'static [Type] {
        &ALL_TYPES
    }

    fn return_type() -> Type {
        Type::Field
    }
}
