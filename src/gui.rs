use evolution::SimulationState;
use evolution::{fresh_start, take_step};

use cairo;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Evolution");
    // window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(600, 630);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let drawing_area = Rc::new(RefCell::new(gtk::DrawingArea::new()));
    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    // let life_button = gtk::Button::with_label("Create life");
    let start_button = gtk::Button::with_label("Start");
    let take_step_button = gtk::Button::with_label("Step");

    window.add(&vbox);

    let state = Rc::new(RefCell::new(fresh_start()));
    update_drawing_area(&state, &drawing_area.borrow_mut());

    // button_box.pack_start(&life_button, true, true, 0);
    button_box.pack_start(&start_button, true, true, 0);
    button_box.pack_start(&take_step_button, true, true, 0);

    vbox.pack_start(&*drawing_area.borrow(), true, true, 0);

    let state2 = Rc::clone(&state);
    let drawing_area2 = Rc::clone(&drawing_area);

    take_step_button.connect_clicked(move |_| {
        take_step(&mut state2.borrow_mut());
        update_drawing_area(&state2, &drawing_area2.borrow_mut())
    });

    start_button.connect_clicked(move |_| {
        take_step(&mut state.borrow_mut());
        update_drawing_area(&state, &drawing_area.borrow_mut());
    });

    vbox.pack_start(&button_box, false, false, 0);
    window.show_all();
}

fn update_drawing_area(state: &Rc<RefCell<SimulationState>>, drawing_area: &gtk::DrawingArea) {
    let width = state.borrow().config.grid_width;
    let height = state.borrow().config.grid_height;

    for (pos, val) in state.borrow().grid.ternary.iter().enumerate() {
        let color = match val {
            1 => (255.0, 0.0, 0.0),
            -1 => (0.0, 255.0, 0.0),
            _ => (255.0, 255.0, 255.0),
        };
        drawing_area.connect_draw(move |_, ctx| draw_square(ctx, pos, color, width, height));
    }
}

fn draw_square(
    ctx: &cairo::Context,
    position: usize,
    color: (f64, f64, f64),
    width: usize,
    height: usize,
) -> gtk::Inhibit {
    // ctx.scale(500f64, 500f64);

    ctx.set_line_width(0.5);

    let rect_width = 600.0 / width as f64;
    let rect_height = 600.0 / height as f64;

    ctx.rectangle(
        (position % width) as f64 * rect_width,
        (position / width) as f64 * rect_height,
        rect_width,
        rect_height,
    );
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.stroke().expect("problems stroking squares.");
    ctx.rectangle(
        (position % width) as f64 * rect_width,
        (position / width) as f64 * rect_height,
        rect_width,
        rect_height,
    );
    ctx.set_source_rgb(color.0, color.1, color.2);
    ctx.fill().expect("Problems filling squares.");

    Inhibit(false)
}
