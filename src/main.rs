mod agents;
mod grid;

use agents::Predator;
use agents::Prey;
use grid::Grid;

fn main() {
    let prob_prey = 0.1;
    let prob_predator = 0.1;
    let width = 8;
    let height = 8;

    // let max_energy_prey = 10;
    // let max_energy_praditor = 10;
    let view_distance_prey = 5;
    let view_distance_predator = 5;

    let grid = Grid::new(width, height, prob_prey, prob_predator);
    let mut predator_list: Vec<Predator> = vec![];
    let mut prey_list: Vec<Prey> = vec![];

    for (position, value) in grid.ternary.iter().enumerate() {
        if *value == -1 {
            prey_list.push(Prey::new(position, view_distance_prey));
        } else if *value == 1 {
            predator_list.push(Predator::new(position, view_distance_predator));
        }
    }

    report(grid, prey_list, predator_list);
}

fn report(grid: Grid, prey_list: Vec<Prey>, predator_list: Vec<Predator>) {
    grid.show();
    println!("number of preys: {}", prey_list.len());
    println!("number of predicators: {}", predator_list.len());
}
// #[derive(Clone, Copy)]
// struct Point {
//     x: usize,
//     y: usize,
// }
