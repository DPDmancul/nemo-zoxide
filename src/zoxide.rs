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

use glib::{clone, idle_add_local_once};
use gtk::{
    Dialog, ResponseType, Window,
    gio::File,
    prelude::{DialogExt, GtkWindowExt, WidgetExt},
};
use std::path::PathBuf;

use crate::nemo;

pub fn start(window: &Window) {
    let dialog = Dialog::builder()
        .title("Zoxide")
        .transient_for(window)
        .modal(true)
        .destroy_with_parent(true)
        .build();

    dialog.add_button("Test", gtk::ResponseType::Ok);

    dialog.connect_response(clone!(
        #[weak]
        window,
        move |dialog, response| {
            if response == ResponseType::Ok {
                let location = File::for_path(PathBuf::from("/"));
                nemo::change_location(&window, &location);
            }
            dialog.close();
        }
    ));

    dialog.show_all();
}
