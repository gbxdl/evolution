mod agents;

use agents::Predator;
use agents::Prey;
use evolution::grid::Grid;

use cairo;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

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

struct SimulationConfig {
    prob_prey: f32,
    prob_predator: f32,
    grid_width: usize,
    grid_height: usize,
    max_energy_prey: usize,
    max_energy_predator: usize,
    view_distance_prey: usize,
    view_distance_predator: usize,
    max_split_count_prey: usize,
    max_split_count_predator: usize,
    energy_gain_predator: usize,
}

pub struct SimulationState {
    config: SimulationConfig,
    grid: Grid,
    predator_list: Vec<Predator>,
    prey_list: Vec<Prey>,
}

impl SimulationState {
    pub fn update_grid(&mut self) {
        let mut grid = Grid::new(self.config.grid_width, self.config.grid_height, 0.0, 0.0);

        let mut remove_indices_predator = vec![];
        let mut add_positions_predator = vec![];

        for (i, predator) in self.predator_list.iter().enumerate() {
            if predator.split_count == self.config.max_split_count_predator {
                add_positions_predator.push(predator.prev_position);
            }
            if predator.energy == 0 {
                remove_indices_predator.push(i);
            } else {
                grid.ternary[predator.position] = 1;
            }
        }
        // remove predators if out of energy (backwards so indices exist)
        for i in remove_indices_predator.iter().rev() {
            self.predator_list.remove(*i);
        }

        // add predator babies @todo same NN.
        for pos in add_positions_predator {
            if grid.ternary[pos] == 0 {
                self.predator_list.push(Predator::new(
                    pos,
                    self.config.view_distance_predator,
                    self.config.max_energy_predator,
                ));
                grid.ternary[pos] = 1;
            }
        }

        let mut remove_indices_prey = vec![];
        let mut add_positions_prey = vec![];
        let mut predator_pos_that_have_eaten = vec![];

        for (i, prey) in self.prey_list.iter().enumerate() {
            if prey.split_count == self.config.max_split_count_prey {
                add_positions_prey.push(prey.prev_position);
            }
            if grid.ternary[prey.position] == 1 {
                remove_indices_prey.push(i);
                predator_pos_that_have_eaten.push(prey.position);
            } else {
                grid.ternary[prey.position] = -1;
            }
        }

        // delete prey if it runs into a predator.
        for i in remove_indices_prey.iter().rev() {
            self.prey_list.remove(*i);
        }
        // add prey babies @todo same NN.
        for pos in add_positions_prey {
            if grid.ternary[pos] == 0 {
                self.prey_list.push(Prey::new(
                    pos,
                    self.config.view_distance_prey,
                    self.config.max_energy_prey,
                ));
                grid.ternary[pos] = -1;
            }
        }

        // give predators that eat energy and increments split count
        for predator in &mut self.predator_list {
            if predator_pos_that_have_eaten.contains(&predator.position) {
                predator.energy += self.config.energy_gain_predator;
                predator.split_count += 1;
            }
        }

        self.grid = grid;
    }
}

fn fresh_start() -> SimulationState {
    let config = SimulationConfig {
        prob_prey: 0.1,
        prob_predator: 0.1,
        grid_width: 16,
        grid_height: 16,
        max_energy_prey: 10,
        max_energy_predator: 10,
        view_distance_prey: 5,
        view_distance_predator: 5,
        max_split_count_prey: 10,
        max_split_count_predator: 2,
        energy_gain_predator: 10,
    };
    let grid = Grid::new(
        config.grid_width,
        config.grid_height,
        config.prob_prey,
        config.prob_predator,
    );
    let mut predator_list: Vec<Predator> = vec![];
    let mut prey_list: Vec<Prey> = vec![];

    for (position, value) in grid.ternary.iter().enumerate() {
        if *value == -1 {
            prey_list.push(Prey::new(
                position,
                config.view_distance_prey,
                config.max_energy_prey,
            ));
        } else if *value == 1 {
            predator_list.push(Predator::new(
                position,
                config.view_distance_predator,
                config.max_energy_predator,
            ));
        }
    }
    SimulationState {
        config,
        grid,
        prey_list,
        predator_list,
    }
}

fn take_step(state: &mut SimulationState) {
    for prey in &mut *state.prey_list {
        prey.take_step(&state.grid);
    }
    for predator in &mut *state.predator_list {
        predator.take_step(&state.grid);
    }
    state.update_grid();
}

fn build_ui(application: &gtk::Application) {
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
