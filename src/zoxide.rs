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

use std::{ffi::OsStr, io, os::unix::ffi::OsStrExt, path::PathBuf, process::Command};

pub fn add(path: impl AsRef<OsStr>) -> io::Result<()> {
    Command::new("zoxide")
        .arg("add")
        .arg("--")
        .arg(path)
        .spawn()
        .map(|_| ())
}

pub fn query(query: Option<impl AsRef<OsStr>>) -> Vec<PathBuf> {
    Command::new("zoxide")
        .arg("query")
        .arg("-l")
        .arg("--")
        .args(query)
        .output()
        .expect("failed to execute zoxide query")
        .stdout
        .split(|c| *c == b'\n' || *c == b'\r')
        .filter(|x| !x.is_empty())
        .map(OsStr::from_bytes)
        .map(PathBuf::from)
        .collect()
}
