#[macro_use]
extern crate gtk_extras;

use gio::prelude::*;
use gtk::prelude::*;
use hidpi_widget::HiDpiToggle;

pub const APP_ID: &str = "com.system76.HiDpiWidget";

fn main() {
    glib::set_program_name(APP_ID.into());
    gtk::init().expect("failed to init GTK");

    let application = gtk::ApplicationBuilder::new().application_id(APP_ID).build();

    application.connect_activate(|app| {
        if let Some(window) = app.get_window_by_id(0) {
            window.present();
        }
    });

    application.connect_startup(|app| {
        let widget = cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 12);
            ..add(&*cascade! {
                HiDpiToggle::new().unwrap();
                ..set_border_width(12);
            });
        };

        let headerbar =
            gtk::HeaderBarBuilder::new().title("HiDPI Widget").show_close_button(true).build();

        let _window = cascade! {
            gtk::ApplicationWindowBuilder::new()
                .application(app)
                .icon_name("firmware-manager")
                .window_position(gtk::WindowPosition::Center)
                .build();
            ..set_titlebar(Some(&headerbar));
            ..add(&widget);
            ..show_all();
            ..connect_delete_event(move |window, _| {
                window.close();

                // Allow this closure to attain ownership of our firmware widget,
                // so that this widget will exist for as long as the window exists.
                let _widget = &widget;

                Inhibit(false)
            });
        };
    });

    application.run(&[]);
}
