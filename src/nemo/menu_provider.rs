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
    Interface, Type, TypeModule, ffi::gpointer, prelude::*, subclass::prelude::*,
    translate::FromGlibPtrNone,
};
use gtk::{Widget, Window, ffi::GtkWidget};

mod ffi {
    use glib::{ffi::gpointer, gobject_ffi::GTypeInterface};
    use gtk::{ffi::GtkWidget, subclass::prelude::*};

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Interface {
        parent: GTypeInterface,
        get_file_items: gpointer,
        pub get_background_items:
            Option<unsafe extern "C" fn(gpointer, *mut GtkWidget, gpointer) -> gpointer>,
        get_item_providers: gpointer,
    }

    unsafe impl InterfaceStruct for Interface {
        type Type = super::imp::MenuProvider;
    }
}

mod imp {
    use gtk::subclass::prelude::*;

    pub struct MenuProvider;

    #[glib::object_interface]
    #[object_interface_dynamic]
    impl ObjectInterface for MenuProvider {
        const NAME: &'static str = "NemoMenuProviderIface";
        type Interface = super::ffi::Interface;
    }
}

glib::wrapper! {
    pub struct MenuProvider(ObjectInterface<imp::MenuProvider>);
}

impl MenuProvider {
    pub fn register_on(module: &TypeModule) -> Type {
        imp::MenuProvider::on_implementation_load(module);
        imp::MenuProvider::type_()
    }
}

#[allow(unused_variables)]
pub trait MenuProviderImpl: ObjectImpl {
    fn on_get_background_items(&self, window: &Window);
}

unsafe impl<T: MenuProviderImpl> IsImplementable<T> for MenuProvider {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_background_items = Some(get_background_items_adapter::<T>);
    }
}

extern "C" fn get_background_items_adapter<T: MenuProviderImpl>(
    provider: gpointer,
    window: *mut GtkWidget,
    _current_folder: gpointer,
) -> gpointer {
    if !window.is_null() {
        let instance = unsafe { &*(provider as *const T::Instance) };
        let imp = instance.imp();
        let window = unsafe { &Widget::from_glib_none(window).downcast::<Window>().unwrap() };
        imp.on_get_background_items(window);
    }
    std::ptr::null_mut()
}
