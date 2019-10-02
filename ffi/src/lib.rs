use glib::object::{Cast, ObjectType};
use hidpi_widget::HiDpiToggle as Toggle;
use std::ptr;

#[no_mangle]
pub struct HiDpiToggle;

#[no_mangle]
pub extern "C" fn hidpi_toggle_new() -> *mut HiDpiToggle {
    unsafe {
        gtk::set_initialized();
    }

    Box::into_raw(Box::new(Toggle::new())) as *mut HiDpiToggle
}

#[no_mangle]
pub extern "C" fn hidpi_toggle_widget(ptr: *const HiDpiToggle) -> *mut gtk_sys::GtkWidget {
    let value = unsafe { (ptr as *const Toggle).as_ref() };
    value.map_or(ptr::null_mut(), |widget| {
        let widget: &gtk::Container = widget.as_ref();
        widget.upcast_ref::<gtk::Widget>().as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn hidpi_toggle_free(widget: *mut HiDpiToggle) {
    unsafe { Box::from_raw(widget as *mut Toggle) };
}
