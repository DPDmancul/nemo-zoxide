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
use goblin::{elf::Elf, elf::sym};
use gtk::{
    Window,
    gio::{File, ffi::GFile},
};
use std::{ffi::c_int, fs, ptr, sync::LazyLock};

pub mod menu_provider;

static NEMO_API: LazyLock<NemoApi> = LazyLock::new(load_nemo_api);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct NemoWindowSlot(gpointer);

type NemoWindowGetActiveSlot = unsafe extern "C" fn(window: gpointer) -> NemoWindowSlot;
type NemoWindowSlotOpenLocationFull = unsafe extern "C" fn(
    slot: NemoWindowSlot,
    location: *mut GFile,
    flags: c_int,
    new_selection: *mut GList,
    callback: Option<unsafe extern "C" fn()>,
    user_data: gpointer,
);

struct NemoApi {
    pub window_get_active_slot: NemoWindowGetActiveSlot,
    pub window_slot_open_location_full: NemoWindowSlotOpenLocationFull,
}

pub fn change_location(window: &Window, location: &File) {
    let slot = unsafe { (NEMO_API.window_get_active_slot)(window.as_ptr() as gpointer) };
    let location = location.as_ptr();

    if !slot.0.is_null() {
        unsafe {
            (NEMO_API.window_slot_open_location_full)(
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

fn load_nemo_api() -> NemoApi {
    let exe = fs::read("/proc/self/exe").expect("failed to read Nemo executable");
    let elf = Elf::parse(&exe).expect("failed to parse Nemo ELF");
    let base = find_pie_base().expect("failed to determine Nemo PIE load base");

    NemoApi {
        window_get_active_slot: resolve_symbol::<NemoWindowGetActiveSlot>(
            &elf,
            base,
            "nemo_window_get_active_slot",
        ),
        window_slot_open_location_full: resolve_symbol::<NemoWindowSlotOpenLocationFull>(
            &elf,
            base,
            "nemo_window_slot_open_location_full",
        ),
    }
}

fn find_pie_base() -> Option<usize> {
    let maps = fs::read_to_string("/proc/self/maps").ok()?;

    let exe = fs::read_link("/proc/self/exe").ok()?;
    let exe = exe.to_string_lossy();

    for line in maps.lines() {
        let mut fields = line.split_whitespace();

        let range = fields.next()?;
        let _perms = fields.next()?;
        let offset = fields.next()?;
        let _dev = fields.next()?;
        let _inode = fields.next()?;

        let path = fields.next().unwrap_or("");

        if path != exe {
            continue;
        }

        // We want the mapping corresponding to ELF virtual address 0.
        if offset != "00000000" {
            continue;
        }

        let start = range.split('-').next()?;

        return usize::from_str_radix(start, 16).ok();
    }

    None
}

fn resolve_symbol<T>(elf: &Elf<'_>, base: usize, name: &str) -> T {
    let sym = elf
        .syms
        .iter()
        .find(|sym| {
            sym.st_value != 0
                && sym.st_type() == sym::STT_FUNC
                && elf.strtab.get_at(sym.st_name) == Some(name)
        })
        .unwrap_or_else(|| panic!("Nemo symbol not found: {name}"));

    let address = base
        .checked_add(sym.st_value as usize)
        .unwrap_or_else(|| panic!("address overflow for {name}"));

    unsafe { std::mem::transmute_copy::<usize, T>(&address) }
}
