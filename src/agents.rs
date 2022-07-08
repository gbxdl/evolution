use fastapprox::faster::tanh;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use std::iter::zip;

struct NeuralNet {
    layers: Vec<Layer>, //list of number of neurons per layer
}

impl NeuralNet {
    fn new(architecture: Vec<usize>) -> NeuralNet {
        let mut layers = vec![];
        let mut n_inputs = 0;
        for n_neurons in architecture {
            if n_inputs == 0 {
                continue;
            }
            layers.push(Layer::new(n_inputs, n_neurons));
            n_inputs = n_neurons;
        }
        NeuralNet { layers }
    }

    fn compute(&self, inputs: &Vec<f32>) -> Vec<f32> {
        let mut outputs = vec![0.0];
        for layer in &self.layers {
            let outputs = layer.compute(inputs);
            let inputs = &outputs;
        }
        outputs
    }
}

struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn new(n_inputs: usize, n_neurons: usize) -> Layer {
        let mut neurons = vec![];

        for _ in 0..n_neurons {
            neurons.push(Neuron::new(n_inputs));
        }
        Layer { neurons }
    }

    fn compute(&self, inputs: &Vec<f32>) -> Vec<f32> {
        let mut outputs = vec![0.0];
        for neuron in &self.neurons {
            outputs.push(neuron.compute(inputs));
        }
        outputs
    }
}

struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

impl Neuron {
    fn new(n_inputs: usize) -> Neuron {
        let neuron = Neuron {
            weights: vec![0.0; n_inputs],
            bias: 0.0,
        };
        // call random mutation here
        neuron
    }

    fn random_mutation(&mut self) {
        let prob = 0.1;
        for weigth in &mut self.weights {
            let coin = rand::thread_rng().gen::<f32>();
            if coin < prob {
                *weigth += 0.1 * thread_rng().sample::<f32, _>(StandardNormal);
            }
        }
    }

    fn compute(&self, inputs: &Vec<f32>) -> f32 {
        let mut output = 0.0;
        for (input, weight) in zip(inputs, &self.weights) {
            output += input * weight + self.bias;
        }
        tanh(output)
    }
}

// impl NeuralNet {
//     fn new() -> NeuralNet {
//         NeuralNet
//     }
// }

pub struct Prey {
    position: usize,
    view_distance: usize,
    energy: usize,
    split_count: usize,
    neural_net: NeuralNet,
}

pub struct Predator {
    position: usize,
    view_distance: usize,
    energy: usize,
    split_count: usize,
    neural_net: NeuralNet,
}

impl Prey {
    pub fn new(position: usize, view_distance: usize) -> Prey {
        Prey {
            position,
            view_distance,
            energy: 0,
            split_count: 0,
            neural_net: NeuralNet::new(vec![view_distance * 4 + 1, 5, 5]),
        }
    }
}

impl Predator {
    pub fn new(position: usize, view_distance: usize) -> Predator {
        Predator {
            position,
            view_distance,
            energy: 0,
            split_count: 0,
            neural_net: NeuralNet::new(vec![view_distance + 1, 5, 5]),
        }
    }
}
