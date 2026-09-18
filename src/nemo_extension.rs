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

use gdk_sys::{GDK_CONTROL_MASK, GDK_KEY_J, GDK_KEY_j, GDK_SHIFT_MASK, GdkEventKey};
use glib_sys::{GList, GType, gpointer};
use gobject_sys::{
    G_TYPE_OBJECT, GInterfaceInfo, GObject, GObjectClass, GTypeInfo, GTypeInterface, GTypeModule, g_object_get_data, g_object_set_data, g_signal_connect_data, g_type_module_add_interface, g_type_module_register_type
};
use gtk_sys::GtkWidget;
use std::{ffi::c_int, mem::transmute, ptr};

use crate::{nemo::nemo_menu_provider_get_type, zoxide};

pub fn register(module: *mut GTypeModule) -> GType {
    let type_info = GTypeInfo {
        class_size: size_of::<GObjectClass>() as u16,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ptr::null(),
        instance_size: size_of::<GObject>() as u16,
        n_preallocs: 0,
        instance_init: None,
        value_table: ptr::null(),
    };

    let gtype = unsafe {
        g_type_module_register_type(
            module,
            G_TYPE_OBJECT,
            c"nemo-zoxide".as_ptr(),
            &type_info,
            0,
        )
    };

    let iface_info = GInterfaceInfo {
        interface_init: Some(menu_provider_iface_init),
        interface_finalize: None,
        interface_data: ptr::null_mut(),
    };

    unsafe {
        g_type_module_add_interface(module, gtype, nemo_menu_provider_get_type(), &iface_info);
    }

    gtype
}

#[repr(C)]
struct MenuProviderIface {
    g_iface: GTypeInterface,
    get_file_items: Option<extern "C" fn(gpointer, *mut GtkWidget, *mut GList) -> *mut GList>,
    get_background_items: Option<extern "C" fn(gpointer, *mut GtkWidget, gpointer) -> *mut GList>,
    get_item_providers: gpointer,
}

extern "C" fn menu_provider_iface_init(iface: gpointer, _data: gpointer) {
    let iface = iface as *mut MenuProviderIface;

    unsafe {
        (*iface).get_background_items = Some(get_background_items);
    }
}

extern "C" fn get_background_items(
    _provider: gpointer,
    window: *mut GtkWidget,
    _current_folder: gpointer,
) -> *mut GList {
    if !window.is_null() {
        let key = c"nemo-zoxide-shortcut".as_ptr();
        let g_win = window as *mut GObject;

        if unsafe { g_object_get_data(g_win, key) }.is_null() {
            unsafe {
                g_signal_connect_data(
                    g_win,
                    c"key-press-event".as_ptr(),
                    Some(transmute::<usize, extern "C" fn()>(
                        on_key_press as *const () as usize,
                    )),
                    ptr::null_mut(),
                    None,
                    0,
                );

                g_object_set_data(g_win, key, 1 as gpointer);
            }
        }
    }

    ptr::null_mut()
}

extern "C" fn on_key_press(
    widget: *mut GtkWidget,
    event: *mut GdkEventKey,
    _user_data: gpointer,
) -> c_int {
    if event.is_null() {
        return 0;
    }

    let GdkEventKey { keyval, state, .. } = unsafe { *event };

    let ctrl_pressed = (state & GDK_CONTROL_MASK) != 0;
    let shift_pressed = (state & GDK_SHIFT_MASK) != 0;
    let j_pressed = keyval == GDK_KEY_j as u32 || keyval == GDK_KEY_J as u32;

    if ctrl_pressed && !shift_pressed && j_pressed {
        zoxide::start(widget);
        return 1;
    }

    0
}
