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

use glib::{TypeModule, ffi::GType, gobject_ffi::GTypeModule, translate::*};
use libc::c_int;
use nemo_extension::NemoZoxide;
use std::sync::OnceLock;

mod nemo;
mod nemo_extension;
mod zoxide;

static REGISTERED_TYPE: OnceLock<GType> = OnceLock::new();

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn nemo_module_initialize(module: *mut GTypeModule) {
    let module = unsafe { TypeModule::from_glib_none(module) };
    REGISTERED_TYPE.get_or_init(|| NemoZoxide::register_on(&module).into_glib());
}

#[unsafe(no_mangle)]
pub extern "C" fn nemo_module_shutdown() {}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn nemo_module_list_types(types: *mut *const GType, num_types: *mut c_int) {
    if let Some(registered_type) = REGISTERED_TYPE.get() {
        unsafe {
            *types = registered_type;
            *num_types = 1;
        }
    }
}
