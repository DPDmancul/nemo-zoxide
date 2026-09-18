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

use glib_sys::GType;
use gobject_sys::GTypeModule;
use libc::c_int;
use std::sync::RwLock;

mod nemo;
mod nemo_extension;
mod zoxide;

static REGISTERED_TYPE: RwLock<GType> = RwLock::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn nemo_module_initialize(module: *mut GTypeModule) {
    *REGISTERED_TYPE.write().unwrap() = nemo_extension::register(module);
}

#[unsafe(no_mangle)]
pub extern "C" fn nemo_module_shutdown() {}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn nemo_module_list_types(types: *mut *const GType, num_types: *mut c_int) {
    let registered_type = REGISTERED_TYPE.read().unwrap();

    unsafe {
        *types = &*registered_type;
        *num_types = 1;
    }
}
