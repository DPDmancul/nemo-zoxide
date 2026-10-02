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

use std::{
    env,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
        mpsc,
    },
};

use glib::{ControlFlow, GString, Propagation, clone, idle_add_local, object::ObjectExt};
use gtk::{
    Align, Dialog, Label, ListBox, ListBoxRow, ScrolledWindow, SearchEntry, Window,
    gdk::keys::{self},
    gio::{File, spawn_blocking},
    prelude::{
        BoxExt, ContainerExt, DialogExt, EntryExt, GtkWindowExt, ListBoxExt, ListBoxRowExt,
        SearchEntryExt, WidgetExt,
    },
};

use crate::{nemo, zoxide};

const PATH_DATA_KEY: &str = "path";

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

    // Channels

    let (search_sender, search_receiver) = mpsc::channel();
    let generation_counter = Arc::new(AtomicU32::new(0));

    search(search_sender.clone(), generation_counter.clone(), None);

    // Query signals

    search_entry.connect_search_changed(clone!(
        #[weak]
        generation_counter,
        move |x| search(
            search_sender.clone(),
            generation_counter.clone(),
            Some(x.text())
        )
    ));

    // Navigation signals

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

    search_entry.connect_key_press_event(clone!(
        #[weak]
        list_box,
        #[upgrade_or]
        Propagation::Proceed,
        move |_, event| match event.keyval() {
            keys::constants::Up => {
                select_prev(&list_box);
                Propagation::Stop
            }
            keys::constants::Down => {
                select_next(&list_box);
                Propagation::Stop
            }
            _ => Propagation::Proceed,
        }
    ));

    // Accept signals

    search_entry.connect_activate(clone!(
        #[weak]
        list_box,
        #[weak]
        window,
        #[weak]
        dialog,
        move |_| change_location(
            &window,
            &dialog,
            list_box.selected_row().as_ref().and_then(get_row_location)
        )
    ));

    list_box.connect_row_activated(clone!(
        #[weak]
        window,
        #[weak]
        dialog,
        move |_, row| change_location(&window, &dialog, get_row_location(row))
    ));

    // Rejection signals

    search_entry.connect_stop_search(clone!(
        #[weak]
        dialog,
        move |_| dialog.close()
    ));

    // Draw

    dialog.show_all();

    idle_add_local(clone!(
        #[weak]
        list_box,
        #[upgrade_or]
        ControlFlow::Break,
        move || {
            match search_receiver.try_recv() {
                Ok((generation, entries)) => {
                    if generation == generation_counter.load(Ordering::Relaxed) {
                        set_entries(&list_box, entries);
                    }
                    ControlFlow::Continue
                }
                Err(mpsc::TryRecvError::Empty) => ControlFlow::Continue,
                _ => ControlFlow::Break,
            }
        }
    ));
}

fn current_index(list_box: &ListBox) -> i32 {
    list_box
        .selected_row()
        .as_ref()
        .map(|r| r.index())
        .unwrap_or(-1)
}

fn select_prev(list_box: &ListBox) {
    if let Some(row) = list_box.row_at_index((current_index(list_box) - 1).max(0)) {
        select_row(list_box, Some(&row));
    }
}

fn select_next(list_box: &ListBox) {
    if let Some(row) = list_box.row_at_index(current_index(list_box) + 1) {
        select_row(list_box, Some(&row));
    }
}

fn select_row(list_box: &ListBox, row: Option<&ListBoxRow>) {
    list_box.select_row(row);
    if let Some(row) = row {
        row.grab_focus();
    }
}

fn search(
    sender: mpsc::Sender<(u32, Vec<PathBuf>)>,
    generation_counter: Arc<AtomicU32>,
    query: Option<GString>,
) {
    let generation = generation_counter.fetch_add(1, Ordering::Relaxed) + 1;

    spawn_blocking(move || {
        let zoxide_res = zoxide::query(query)
            .inspect_err(|e| log::error!("Failed to query zoxide: {}", e))
            .unwrap_or_default();

        sender
            .send((generation, zoxide_res))
            .unwrap_or_else(|e| log::error!("Failed to send zoxide query result: {}", e));
    });
}

fn set_entries(list_box: &ListBox, entries: Vec<PathBuf>) {
    while let Some(row) = list_box.row_at_index(0) {
        list_box.remove(&row);
    }

    let home = env::var_os("HOME");

    for entry in entries {
        list_box.add(&{
            let row = ListBoxRow::builder().can_focus(false).build();
            let label = Label::builder()
                .label(
                    if let Some(home) = &home
                        && let Ok(rest) = entry.strip_prefix(home)
                    {
                        format!("~/{}", rest.to_string_lossy()).into()
                    } else {
                        entry.to_string_lossy()
                    },
                )
                .halign(Align::Start)
                .hexpand(true)
                .margin(5)
                .build();

            unsafe {
                row.set_data(PATH_DATA_KEY, entry);
            }
            row.add(&label);

            row
        });
    }

    select_row(list_box, list_box.row_at_index(0).as_ref());
    list_box.show_all();
}

fn get_row_location(row: &ListBoxRow) -> Option<&PathBuf> {
    unsafe { row.data::<PathBuf>(PATH_DATA_KEY).map(|x| x.as_ref()) }
}

fn change_location(window: &Window, dialog: &Dialog, location: Option<&PathBuf>) {
    if let Some(path) = location {
        nemo::api::change_location(window, &File::for_path(path));
        if let Err(e) = zoxide::add(path) {
            log::error!("Failed to add entry to zoxide: {}", e);
        }
    }
    dialog.close();
}
