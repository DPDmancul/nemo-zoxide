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
    Interface,
    ffi::{GList, gpointer},
    subclass::prelude::*,
    translate::ToGlibPtr,
};

mod ffi {
    use glib::{
        ffi::{GList, GType, gpointer},
        gobject_ffi::GTypeInterface,
    };
    use gtk::subclass::prelude::*;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Interface {
        parent: GTypeInterface,
        pub get_name_and_desc: Option<unsafe extern "C" fn(gpointer) -> *mut GList>,
    }

    unsafe impl InterfaceStruct for Interface {
        type Type = super::imp::NameAndDescProvider;
    }

    unsafe extern "C" {
        pub fn nemo_name_and_desc_provider_get_type() -> GType;
    }
}

mod imp {
    use glib::translate::FromGlib;
    use gtk::subclass::prelude::*;

    pub struct NameAndDescProvider;

    impl ObjectInterface for NameAndDescProvider {
        const NAME: &'static str = "NemoNameAndDescProviderInterface";
        type Prerequisites = ();
        type Instance = ();
        type Interface = super::ffi::Interface;
    }

    unsafe impl ObjectInterfaceType for NameAndDescProvider {
        fn type_() -> glib::Type {
            unsafe { glib::Type::from_glib(super::ffi::nemo_name_and_desc_provider_get_type()) }
        }
    }
}

glib::wrapper! {
    pub struct NameAndDescProvider(ObjectInterface<imp::NameAndDescProvider>);
}

pub trait NameAndDescProviderImpl: ObjectImpl {
    fn get_name(&self) -> String;
    fn get_desc(&self) -> String;
}

unsafe impl<T: NameAndDescProviderImpl> IsImplementable<T> for NameAndDescProvider {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_name_and_desc = Some(get_get_name_and_desc_adapter::<T>);
    }
}

extern "C" fn get_get_name_and_desc_adapter<T: NameAndDescProviderImpl>(
    provider: gpointer,
) -> *mut GList {
    let instance = unsafe { &*(provider as *const T::Instance) };
    let imp = instance.imp();

    let name = imp.get_name();
    let desc = imp.get_desc();

    [format!("{name}:::{desc}")].to_glib_full()
}
