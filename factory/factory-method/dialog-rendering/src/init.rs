use crate::{gui::Dialog, html_gui::HtmlDialog, windows_gui::WindowsDialog};

pub fn initialize() -> &'static dyn Dialog {
    if cfg!(unix) {
        println!("-- Windows detected, create Windows GUI --");
        &WindowsDialog
    } else {
        println!("-- No OS detected, create the HTML GUI --");
        &HtmlDialog
    }
}
