// Copyright (C) 2026 Davide Peressoni
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::{
    ffi::OsStr,
    os::unix::ffi::OsStrExt,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use thiserror::Error;

const ZOXIDE_CMD: &str = match option_env!("ZOXIDE_CMD") {
    Some(x) => x,
    None => "zoxide",
};

#[derive(Debug, Error)]
pub enum ZoxideError {
    #[error("failed to execute zoxide")]
    Io(#[from] std::io::Error),
    #[error("zoxide exited with status {0}")]
    ExitStatus(ExitStatus),
}

pub fn add(path: impl AsRef<OsStr>) -> Result<(), ZoxideError> {
    Command::new(ZOXIDE_CMD)
        .arg("add")
        .arg("--")
        .arg(path)
        .spawn()?;

    Ok(())
}

pub fn query(query: Option<impl AsRef<OsStr>>) -> Result<Vec<PathBuf>, ZoxideError> {
    let output = Command::new(ZOXIDE_CMD)
        .arg("query")
        .arg("-l")
        .arg("--")
        .args(query)
        .output()?;

    if !output.status.success() {
        return Err(ZoxideError::ExitStatus(output.status));
    }

    Ok(output
        .stdout
        .split(|c| *c == b'\n' || *c == b'\r')
        .filter(|x| !x.is_empty())
        .map(OsStr::from_bytes)
        .map(PathBuf::from)
        .collect())
}
