#[macro_use]
extern crate derive_more;

use cascade::cascade;
use gio::SettingsExt;
use glib::clone;
use gtk::prelude::*;
use gtk_extras::settings;
use libhandy::prelude::*;

#[derive(AsRef, Deref)]
#[as_ref]
#[deref]
pub struct HiDpiToggle(gtk::Container);

impl HiDpiToggle {
    /// This method returns `None` if the `com.system76.hidpi` schema could not be found.
    pub fn new() -> Option<Self> {
        let settings = settings::new_checked("com.system76.hidpi")?;

        let enable_switch = cascade! {
            gtk::Switch::new();
            ..set_valign(gtk::Align::Center);
        };
        let enable_row = cascade! {
            libhandy::ActionRow::new();
            ..set_title(Some("Enabled"));
            ..set_subtitle(Some("Enable or disable the HiDPI daemon."));
            ..add(&enable_switch);
        };
        settings.bind("enable", &enable_switch, "active", gio::SettingsBindFlags::DEFAULT);

        let mode_switch = cascade! {
            gtk::Switch::new();
            ..set_valign(gtk::Align::Center);
        };
        let mode_row = cascade! {
            libhandy::ActionRow::new();
            ..set_title(Some("Mode"));
            ..set_subtitle(Some("Enable to render LoDPI displays at HiDPI resolution."));
            ..add(&mode_switch);
        };
        // TODO: Update once gtk-rs is released with binding for `g_settings_bind_with_mapping`
        // https://github.com/gtk-rs/gtk-rs/pull/210
        let value = settings.get_string("mode").map(|s| s == "hidpi").unwrap_or(false);
        mode_switch.set_active(value);
        settings.connect_changed(clone!(@weak mode_switch => move |settings, key| {
            if key == "mode" {
                let value = settings.get_string("mode").map(|s| s == "hidpi").unwrap_or(false);
                mode_switch.set_active(value);
            }
        }));
        mode_switch.connect_property_active_notify(clone!(@strong settings => move |switch| {
            let value = if switch.get_active() {
                "hidpi"
            } else {
                "lodpi"
            };
            let _ = settings.set_string("mode", value);
        }));

        let list_box = cascade! {
            gtk::ListBox::new();
            ..set_selection_mode(gtk::SelectionMode::None);
            ..add(&enable_row);
            ..add(&mode_row);
        };

        Some(Self(list_box.upcast()))
    }
}
