use adw::prelude::*;
use adw::{ActionRow, ApplicationWindow, HeaderBar};
use gtk::{Align, Application, Box, ListBox, Orientation};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_startup(|_| {
        let _ = adw::init();
    });

    application.connect_activate(|app| {
        let switch = gtk::Switch::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .vexpand_set(false)
            .build();
        switch.connect_state_set(|_, is_enabled| {
            println!("Switch! {}", is_enabled);
            false.into()
        });

        // ActionRows are only available in Adwaita
        let row = ActionRow::builder()
            .activatable(false)
            .selectable(false)
            .title("Title")
            .subtitle("Subtitle")
            .build();
        row.add_suffix(&switch);
        row.connect_activated(|_| {
            println!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(16)
            .margin_end(16)
            .margin_bottom(16)
            .margin_start(16)
            // the content class makes the list look nicer
            .css_classes(vec![String::from("content")])
            .build();
        list.append(&row);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("My App", "Subtitle"))
                .build(),
        );
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.show();
    });

    application.run();
}
