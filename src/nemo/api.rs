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
use std::{
    ffi::c_int,
    fs,
    io::{self, BufRead, BufReader},
    path::Path,
    ptr,
    sync::OnceLock,
};

static NEMO_API: OnceLock<NemoApi> = OnceLock::new();

#[derive(Debug, thiserror::Error)]
pub enum NemoApiError {
    #[error("failed to read Nemo executable")]
    ReadExe(#[from] io::Error),
    #[error("failed to parse Nemo executable")]
    ParseElf(#[from] goblin::error::Error),
    #[error("could not determine Nemo PIE base address")]
    PieBaseNotFound,
    #[error("Nemo symbol `{0}` was not found")]
    MissingSymbo(&'static str),
}

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

pub fn try_initialize() -> Result<(), NemoApiError> {
    // TODO use get_or_try_init when stable

    if NEMO_API.get().is_some() {
        return Ok(());
    }

    let api = load_nemo_api()?;
    let _ = NEMO_API.set(api);

    Ok(())
}

pub fn change_location(window: &Window, location: &File) {
    let slot = unsafe { (get_api().window_get_active_slot)(window.as_ptr() as gpointer) };
    let location = location.as_ptr();

    if !slot.0.is_null() {
        unsafe {
            (get_api().window_slot_open_location_full)(
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

fn get_api() -> &'static NemoApi {
    NEMO_API.get().expect("Didn't call nemo::try_initialize")
}

fn load_nemo_api() -> Result<NemoApi, NemoApiError> {
    let exe = fs::read("/proc/self/exe")?;
    let elf = Elf::parse(&exe)?;
    let base = find_pie_base().ok_or(NemoApiError::PieBaseNotFound)?;

    Ok(NemoApi {
        window_get_active_slot: resolve_symbol::<NemoWindowGetActiveSlot>(
            &elf,
            base,
            "nemo_window_get_active_slot",
        )?,
        window_slot_open_location_full: resolve_symbol::<NemoWindowSlotOpenLocationFull>(
            &elf,
            base,
            "nemo_window_slot_open_location_full",
        )?,
    })
}

fn find_pie_base() -> Option<usize> {
    let exe_path = fs::read_link("/proc/self/exe").ok()?;

    let maps = BufReader::new(fs::File::open("/proc/self/maps").ok()?);
    for line in maps.lines() {
        let line = line.ok()?;
        let mut fields = line.split_whitespace();

        let range = fields.next()?;
        let _perms = fields.next()?;
        let offset = fields.next()?;
        let _dev = fields.next()?;
        let _inode = fields.next()?;

        let path = fields.next().unwrap_or("");

        if Path::new(path) != exe_path {
            continue;
        }

        // We want the mapping corresponding to ELF virtual address 0.
        if offset.parse() != Ok(0usize) {
            continue;
        }

        let start = range.split('-').next()?;

        return usize::from_str_radix(start, 16).ok();
    }

    None
}

fn resolve_symbol<T>(elf: &Elf<'_>, base: usize, name: &'static str) -> Result<T, NemoApiError> {
    let sym = elf
        .syms
        .iter()
        .find(|sym| {
            sym.st_value != 0
                && sym.st_type() == sym::STT_FUNC
                && elf.strtab.get_at(sym.st_name) == Some(name)
        })
        .ok_or(NemoApiError::MissingSymbo(name))?;

    let address = base
        .checked_add(sym.st_value as usize)
        .unwrap_or_else(|| panic!("address overflow for {name}"));

    Ok(unsafe { std::mem::transmute_copy::<usize, T>(&address) })
}
