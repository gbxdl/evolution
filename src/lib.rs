mod agents;
mod grid;

use agents::NeuralNet;
use agents::Predator;
use agents::Prey;
use grid::Grid;

use std::clone::Clone;

pub fn fresh_start() -> SimulationState {
    let config = SimulationConfig {
        prob_prey: 0.1,
        prob_predator: 0.1,
        grid_width: 32,
        grid_height: 32,
        max_energy_prey: 10,
        max_energy_predator: 10,
        view_distance_prey: 3,
        view_distance_predator: 3,
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
                NeuralNet { layers: vec![] },
            ));
        } else if *value == 1 {
            predator_list.push(Predator::new(
                position,
                config.view_distance_predator,
                config.max_energy_predator,
                NeuralNet { layers: vec![] },
            ));
        }
    }
    SimulationState {
        config,
        grid,
        prey_list,
        predator_list,
        running: false,
    }
}

pub struct SimulationConfig {
    prob_prey: f32,
    prob_predator: f32,
    pub grid_width: usize,
    pub grid_height: usize,
    max_energy_prey: usize,
    max_energy_predator: usize,
    view_distance_prey: usize,
    view_distance_predator: usize,
    max_split_count_prey: usize,
    max_split_count_predator: usize,
    energy_gain_predator: usize,
}

pub struct SimulationState {
    pub config: SimulationConfig,
    pub grid: Grid,
    predator_list: Vec<Predator>,
    prey_list: Vec<Prey>,
    pub running: bool,
}

impl SimulationState {
    pub fn update_grid(&mut self) {
        let mut grid = Grid::new(self.config.grid_width, self.config.grid_height, 0.0, 0.0);

        let mut remove_indices_predator = vec![];
        let mut add_positions_nn_predator = vec![];

        for (i, predator) in self.predator_list.iter_mut().enumerate() {
            if predator.split_count == self.config.max_split_count_predator {
                add_positions_nn_predator
                    .push((predator.prev_position, Clone::clone(&predator.neural_net)));
                predator.split_count = 0;
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

        // add predator babies.
        for (pos, nn) in add_positions_nn_predator {
            if grid.ternary[pos] == 0 {
                self.predator_list.push(Predator::new(
                    pos,
                    self.config.view_distance_predator,
                    self.config.max_energy_predator,
                    nn,
                ));
                grid.ternary[pos] = 1;
            }
        }

        let mut remove_indices_prey = vec![];
        let mut add_positions_nn_prey = vec![];
        let mut predator_pos_that_have_eaten = vec![];

        for (i, prey) in self.prey_list.iter_mut().enumerate() {
            if prey.split_count == self.config.max_split_count_prey {
                add_positions_nn_prey.push((prey.prev_position, Clone::clone(&prey.neural_net)));
                prey.split_count = 0;
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
        // add prey babies.
        for (pos, nn) in add_positions_nn_prey {
            if grid.ternary[pos] == 0 {
                self.prey_list.push(Prey::new(
                    pos,
                    self.config.view_distance_prey,
                    self.config.max_energy_prey,
                    nn,
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

pub fn take_step(state: &mut SimulationState) {
    for prey in &mut *state.prey_list {
        prey.take_step(&state.grid);
    }
    for predator in &mut *state.predator_list {
        predator.take_step(&state.grid);
    }
    state.update_grid();
}
