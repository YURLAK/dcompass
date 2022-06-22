// Copyright 2020 LEXUGE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! This module provides universal error type used in the library. The error type uses `thiserror`.

pub use super::router::{script::ScriptError, upstreams::error::UpstreamError};
use std::fmt::Debug;
use thiserror::Error;

// We don't expose this as this is useless for external
pub(crate) type Result<T> = std::result::Result<T, DrouteError>;

/// DrouteError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum DrouteError {
    /// Error related to the `script` section.
    #[error(transparent)]
    ScriptError(#[from] ScriptError),

    /// Error related to the `upstreams` section.
    #[error(transparent)]
    UpstreamError(#[from] UpstreamError),

    /// The buffer is too short
    #[error(transparent)]
    ShortBuf(#[from] domain::base::ShortBuf),
}
