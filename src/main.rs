mod gui;

use gtk::prelude::*;
use gui::build_ui;

fn main() {
    let gui = true;
    if gui {
        let application =
            gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());
        application.connect_activate(build_ui);

        application.run();
    } else {
        println! {"to do: gui-less."}
    }
}
