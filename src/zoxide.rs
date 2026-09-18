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

use gio_sys::g_file_new_for_path;
use gobject_sys::{GObject, g_object_unref};
use gtk_sys::GtkWidget;
use std::ffi::CString;

use crate::nemo;

pub fn start(window: *mut GtkWidget) {
    if let Some(path) = open_modal() {
        let location = unsafe { g_file_new_for_path(path.as_ptr()) };
        if !location.is_null() {
            nemo::change_location(window, location);
            unsafe {
                g_object_unref(location as *mut GObject);
            }
        }
    }
}

fn open_modal() -> Option<CString> {
    Some(CString::from(c"/")) // TODO
}
