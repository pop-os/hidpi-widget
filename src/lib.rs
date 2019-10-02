#[macro_use]
extern crate gtk_extras;

use gio::SettingsExt;
use gtk_extras::{settings, ToggleVariant, VariantToggler};

#[derive(Clone, Copy, Debug)]
enum HiDpiEvent {
    /// Defines if the daemon should be enabled.
    Daemon,
    /// Defines if LoDPI displays should be rendered at HiDPI resolutions.
    LoRenderHi,
}

#[derive(Shrinkwrap)]
pub struct HiDpiToggle(gtk::Container);

impl HiDpiToggle {
    /// This method returns `None` if the `com.system76.hidpi` schema could not be found.
    pub fn new() -> Option<Self> {
        let settings = settings::new_checked("com.system76.hidpi")?;

        let variants = [
            ToggleVariant {
                name:        "Enabled",
                description: "Enable or disable the HiDPI daemon.",
                active:      settings.get_boolean("enable"),
                event:       HiDpiEvent::Daemon,
            },
            ToggleVariant {
                name:        "Mode",
                description: "Enable to render LoDPI displays at HiDPI resolution.",
                active:      settings.get_string("mode").map_or(false, |string| string == "hidpi"),
                event:       HiDpiEvent::LoRenderHi,
            },
        ];

        let event_handler = move |event, active| match event {
            HiDpiEvent::Daemon => {
                settings.set_boolean("enable", active);
            }
            HiDpiEvent::LoRenderHi => {
                settings.set_string("mode", if active { "hidpi" } else { "lodpi" });
            }
        };

        Some(Self(VariantToggler::new(&variants, event_handler).into()))
    }
}
