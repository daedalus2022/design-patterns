use init::initialize;

mod gui;
mod html_gui;
mod init;

mod windows_gui;

fn main() {
    let dialog = initialize();

    dialog.render();
    dialog.refresh();
}
