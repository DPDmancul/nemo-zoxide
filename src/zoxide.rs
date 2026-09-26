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

use gtk::{Window, gio::File};
use std::path::PathBuf;

use crate::nemo;

pub fn start(window: &Window) {
    if let Some(path) = open_modal() {
        let location = File::for_path(path);
        nemo::change_location(window, &location);
    }
}

fn open_modal() -> Option<PathBuf> {
    Some(PathBuf::from("/")) // TODO
}
