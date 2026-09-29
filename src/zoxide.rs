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

use std::{io, process::Command};

pub fn add(path: &str) -> Option<io::Error> {
    Command::new("zoxide")
        .arg("add")
        .arg("--")
        .arg(path)
        .spawn()
        .err()
}

pub fn query(query: &str) -> Vec<String> {
    String::from_utf8(
        Command::new("zoxide")
            .arg("query")
            .arg("-l")
            .arg("--")
            .arg(query)
            .output()
            .expect("failed to execute zoxide query")
            .stdout,
    )
    .expect("Zoxide returned a non UTF-8 result")
    .lines()
    .map(str::to_owned)
    .collect()
}
