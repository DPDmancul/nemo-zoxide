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

use glib::{
    ffi::{GList, gpointer},
    object::ObjectType,
};
use gtk::{
    Window,
    gio::{File, ffi::GFile},
};
use std::{ffi::c_int, ptr};

pub mod menu_provider;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct NemoWindowSlot(gpointer);

unsafe extern "C" {
    fn nemo_window_get_active_slot(window: gpointer) -> NemoWindowSlot;

    fn nemo_window_slot_open_location_full(
        slot: NemoWindowSlot,
        location: *mut GFile,
        flags: c_int,
        new_selection: *mut GList,
        callback: Option<unsafe extern "C" fn()>,
        user_data: gpointer,
    );
}

pub fn change_location(window: &Window, location: &File) {
    let slot = unsafe { nemo_window_get_active_slot(window.as_ptr() as gpointer) };
    let location = location.as_ptr();

    if !slot.0.is_null() {
        unsafe {
            nemo_window_slot_open_location_full(
                slot,
                location,
                0, // open in current tab
                ptr::null_mut(),
                None,
                ptr::null_mut(),
            )
        };
    }
}
