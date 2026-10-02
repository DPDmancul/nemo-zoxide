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
    GlibLogger, GlibLoggerDomain, GlibLoggerFormat, TypeModule, ffi::GType,
    gobject_ffi::GTypeModule, translate::*,
};
use libc::c_int;
use nemo_extension::NemoZoxide;
use std::panic;
use std::sync::{Once, OnceLock};

mod dialog;
mod nemo;
mod nemo_extension;
mod zoxide;

static INIT: Once = Once::new();
static REGISTERED_TYPE: OnceLock<GType> = OnceLock::new();
static GLIB_LOGGER: GlibLogger =
    GlibLogger::new(GlibLoggerFormat::Plain, GlibLoggerDomain::CrateTarget);

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn nemo_module_initialize(module: *mut GTypeModule) {
    INIT.call_once(|| {
        log::set_logger(&GLIB_LOGGER).expect("cannot setup logger");
        log::set_max_level(log::LevelFilter::Debug);

        panic::set_hook(Box::new(|info| {
            log::error!("{}", info);
        }));
    });

    if let Err(_e) = nemo::api::try_initialize() {
        log::error!("Cannot initialize Nemo API");
    } else {
        let module = unsafe { TypeModule::from_glib_none(module) };
        REGISTERED_TYPE.get_or_init(|| NemoZoxide::register_on(&module).into_glib());
    }
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
