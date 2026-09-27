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

use glib::{Type, TypeModule, subclass::prelude::*};
use gtk::glib;

mod imp {
    use glib::{Propagation, object::ObjectExt, subclass::prelude::*};
    use gtk::{
        Window,
        gdk::{EventKey, ModifierType},
        glib,
        prelude::WidgetExt,
    };

    use crate::{
        nemo::menu_provider::{MenuProvider, MenuProviderImpl},
        zoxide,
    };

    #[derive(Default)]
    pub struct NemoZoxide;

    #[glib::object_subclass]
    #[object_subclass_dynamic]
    impl ObjectSubclass for NemoZoxide {
        const NAME: &'static str = "NemoZoxide";
        type Type = super::NemoZoxide;
        type ParentType = glib::Object;
        type Interfaces = (MenuProvider,);
    }

    impl ObjectImpl for NemoZoxide {}

    impl MenuProviderImpl for NemoZoxide {
        fn on_get_background_items(&self, window: &Window) {
            let key = "nemo-zoxide-shortcut";

            if unsafe { window.data::<()>(key) }.is_none() {
                window.connect_key_press_event(on_key_press);
                unsafe {
                    window.set_data(key, ());
                }
            }
        }
    }

    fn on_key_press(window: &Window, event: &EventKey) -> Propagation {
        let ctrl_pressed = event.state().contains(ModifierType::CONTROL_MASK);
        let shift_pressed = event.state().contains(ModifierType::SHIFT_MASK);
        let j_pressed = event.keyval().to_lower().to_unicode() == Some('j');

        if ctrl_pressed && !shift_pressed && j_pressed {
            zoxide::start(window);
            return Propagation::Stop;
        }

        Propagation::Proceed
    }
}

glib::wrapper! {
    pub struct NemoZoxide(ObjectSubclass<imp::NemoZoxide>);
}

impl NemoZoxide {
    pub fn register_on(module: &TypeModule) -> Type {
        if !imp::NemoZoxide::on_implementation_load(module) {
            eprintln!("Failed to register NemoZoxide");
            return glib::Type::INVALID;
        }

        imp::NemoZoxide::type_()
    }
}
