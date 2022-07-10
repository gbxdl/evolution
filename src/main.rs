mod agents;

use agents::Predator;
use agents::Prey;
use evolution::grid::Grid;

fn main() {
    let prob_prey = 0.1;
    let prob_predator = 0.1;
    let width = 8;
    let height = 8;

    let max_energy_prey = 10;
    let max_energy_predator = 10;
    let view_distance_prey = 5;
    let view_distance_predator = 5;

    let mut grid = Grid::new(width, height, prob_prey, prob_predator);
    let mut predator_list: Vec<Predator> = vec![];
    let mut prey_list: Vec<Prey> = vec![];

    for (position, value) in grid.ternary.iter().enumerate() {
        if *value == -1 {
            prey_list.push(Prey::new(position, view_distance_prey, max_energy_prey));
        } else if *value == 1 {
            predator_list.push(Predator::new(
                position,
                view_distance_predator,
                max_energy_predator,
            ));
        }
    }

    // report(&grid, &prey_list, &predator_list);
    // prey_list[0].take_step(&grid);
    // grid = update_grid(&prey_list, &predator_list, width, height);
    // report(&grid, &prey_list, &predator_list);

    loop {
        report(&grid, &prey_list, &predator_list);
        for prey in &mut prey_list {
            prey.take_step(&grid);
        }
        for predator in &mut predator_list {
            predator.take_step(&grid);
        }
        grid = update_grid(&prey_list, &predator_list, width, height);
    }
}

fn report(grid: &Grid, prey_list: &Vec<Prey>, predator_list: &Vec<Predator>) {
    grid.show();
    println!("number of preys: {}", prey_list.len());
    println!("number of predicators: {}", predator_list.len());
}

pub fn update_grid(
    prey_list: &Vec<Prey>,
    predator_list: &Vec<Predator>,
    width: usize,
    height: usize,
) -> Grid {
    let mut grid = Grid::new(width, height, 0.0, 0.0);
    for prey in prey_list {
        grid.ternary[prey.position] = -1;
    }
    for predator in predator_list {
        grid.ternary[predator.position] = 1;
    }
    grid
}
