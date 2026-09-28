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

use glib::{SignalHandlerId, clone, idle_add_local_once};
use gtk::{
    Align, Dialog, Label, ListBox, ListBoxRow, ResponseType, ScrolledWindow, SearchEntry, Window, builders::GridBuilder, gio::File, prelude::{
        BoxExt, BuildableExt, ContainerExt, DialogExt, EntryExt, GtkWindowExt, LabelExt,
        ListBoxExt, ListBoxRowExt, SearchEntryExt, WidgetExt,
    }
};
use std::path::PathBuf;

use crate::nemo;

pub fn start(window: &Window) {
    // Layout

    let dialog = Dialog::builder()
        .title("Zoxide")
        .transient_for(window)
        .modal(true)
        .destroy_with_parent(true)
        .default_width(600)
        .default_height(700)
        .build();

    let search_entry = SearchEntry::new();
    let list_box = ListBox::new();

    let scroll_view = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    scroll_view.add(&list_box);

    dialog
        .content_area()
        .pack_start(&search_entry, false, false, 0);
    dialog
        .content_area()
        .pack_start(&scroll_view, true, true, 0);

    // Search entry signals

    search_entry.connect_previous_match(clone!(
        #[weak]
        list_box,
        move |_| select_prev(&list_box)
    ));
    search_entry.connect_next_match(clone!(
        #[weak]
        list_box,
        move |_| select_next(&list_box)
    ));
    search_entry.connect_stop_search(clone!(#[weak] dialog, move |_| dialog.close()));
    search_entry.connect_search_changed(clone!(
        #[weak]
        list_box,
        move |x| {
            list_box.add(&{
                let row = ListBoxRow::new();
                let label = Label::builder()
                    .label(x.text())
                    .halign(Align::Start)
                    .hexpand(true)
                    .margin(5)
                    .build();

                row.add(&label);

                row
            });
            list_box.show_all();
        }
    ));

    // dialog.connect_response(clone!(
    //     #[weak]
    //     window,
    //     move |dialog, response| {
    //         if response == ResponseType::Ok {
    //             let location = File::for_path(PathBuf::from("/"));
    //             nemo::change_location(&window, &location);
    //         }
    //         dialog.close();
    //     }
    // ));

    dialog.show_all();
}

fn current_index(list_box: &ListBox) -> i32 {
    list_box
        .selected_row()
        .as_ref()
        .map(|r| r.index())
        .unwrap_or(-1)
}

fn select_prev(list_box: &ListBox) {
    if let Some(row) = list_box.row_at_index(current_index(list_box) - 1) {
        list_box.select_row(Some(&row));
    }
}

fn select_next(list_box: &ListBox) {
    if let Some(row) = list_box.row_at_index(current_index(list_box) + 1) {
        list_box.select_row(Some(&row));
    }
}
