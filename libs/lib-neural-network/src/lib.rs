use std::fmt::format;

use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct NeuralNetwork {
    layers: Vec<Layer>
}

impl NeuralNetwork {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers.iter().fold(inputs, |acc, l| l.propagate(acc))
    }

    fn get_layers(&self) -> &[Layer] {
        &self.layers
    }

    fn random(layers: Vec<LayerTopology>, rng: &mut dyn RngCore) -> Result<Self, String> {
        if layers.len() <= 1 {
            return Err(format!("Network must have at least 2 or more layers"))
        }
        let built_layers = layers.windows(2).map(|layer_flow| {
            Layer::random(layer_flow[0].neurons, layer_flow[1].neurons, rng)
        }).collect();
        Ok(NeuralNetwork { layers: built_layers })
    }
}

pub struct LayerTopology {
    neurons: usize
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons.iter().map(|n| {
            n.propagate(inputs.clone()).map_err(|e| eprintln!("[ERR] {}", e)).unwrap()
        }).collect()
    }

    fn get_neurons(&self) -> &[Neuron] {
        &self.neurons   
    }

    fn random(weights: usize, neurons: usize, rng: &mut dyn RngCore) -> Self {
        Self { neurons: (0..neurons).map(|_| Neuron::random(weights, rng)).collect() }
    }
}

// Standard Neuron with a ReLU activation function
#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>
}

impl Neuron {
    fn propagate(&self, inputs: Vec<f32>) -> Result<f32, String> {
        if inputs.len() != self.weights.len() {
            return Err(format!("Input length {} does not match weights length {}", inputs.len(), self.weights.len()));
        }
        Ok((inputs.iter().zip(self.weights.iter()).map(|(&i, &w)| i * w).sum::<f32>() + self.bias).max(0.0))
    }

    fn get_weights(&self) -> &[f32] {
        self.weights.as_slice()
    }

    fn random(weights: usize, rng: &mut dyn RngCore) -> Self {
        Self {
            bias: rng.gen(),
            weights: (0..weights).map(|_| rng.gen()).collect()
        }
    }
}


#[cfg(test)]
mod test {
    use std::vec;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use crate::{Layer, Neuron};

    mod neurons {
        use  super::*;



        #[test]
        fn can_create_random_neurons() {
            let mut rng = ChaCha8Rng::from_seed(ChaCha8Rng::seed_from_u64(10495 as u64).get_seed());
            let first_neuron = Neuron::random(2, &mut rng);
            let second_neuron = Neuron::random(3, &mut rng);

            approx::assert_relative_eq!(first_neuron.bias, 0.036300004);
            approx::assert_relative_eq!(first_neuron.get_weights(), [0.81902564, 0.21760976].as_slice());

            approx::assert_relative_eq!(second_neuron.bias, 0.0117205980);
            approx::assert_relative_eq!(second_neuron.get_weights(), [0.97990996, 0.6928334, 0.29998326].as_slice());
        }
        
    }

    mod layers {
        use super::*;

        #[test]
        fn can_create_layers_from_topology() {
            let mut rng = ChaCha8Rng::from_seed(ChaCha8Rng::seed_from_u64(140516 as u64).get_seed());
            let first_layer = Layer::random(2, 3, &mut rng);
            let second_layer = Layer::random(5, 2, &mut rng);

            assert_eq!(first_layer.get_neurons().len(), 3);
            for neuron in first_layer.get_neurons() {
                assert_eq!(neuron.get_weights().len(), 2)
            }


            assert_eq!(second_layer.get_neurons().len(), 2);
            for neuron in second_layer.get_neurons() {
                assert_eq!(neuron.get_weights().len(), 5)
            }
        }
    }

    mod network {
        use crate::{LayerTopology, NeuralNetwork};

        use super::*;

        #[test]
        fn can_not_create_network_without_layers_topology() {
            let mut rng = ChaCha8Rng::from_seed(ChaCha8Rng::seed_from_u64(91295 as u64).get_seed());
            let invalid_network = NeuralNetwork::random(vec![], &mut rng);

            assert!(invalid_network.is_err());
        }


        #[test]
        fn can_not_create_network_with_one_layer_topology() {
            let mut rng = ChaCha8Rng::from_seed(ChaCha8Rng::seed_from_u64(91295 as u64).get_seed());
            let invalid_network = NeuralNetwork::random(vec![LayerTopology{neurons: 42}], &mut rng);

            assert!(invalid_network.is_err());
        }


        #[test]
        fn can_create_network_with_enough_layers() {
            let mut rng = ChaCha8Rng::from_seed(ChaCha8Rng::seed_from_u64(91295 as u64).get_seed());
            let network = NeuralNetwork::random(vec![LayerTopology{neurons: 1}, LayerTopology{ neurons: 4}, LayerTopology{ neurons: 2}], &mut rng);
            /*
             * 
             * Final Network should be something like this
             *                    ┌──┐                          
             *                  --│  │ --                       
             *                --- └──┘  ----                    
             *              ---         --  ----  ┌──┐          
             *           ---      ┌──┐ -----------│  │ ---      
             *        ----      --│  │--- --  --- └──┘   ------ 
             *     ----    ------ └──┘  -- ----               --
             *   ---   -----             -- --                  
             * --------           ┌──┐    -----             ----
             *       ---------    │  │----------- ┌──┐   ----   
             *        ---     ----└──┘        ----│  │ --       
             *           --                 ------└──┘          
             *             ---    ┌──┐  -----                   
             *               ---- │  │---                       
             *                    └──┘                                        
             *
             */

            assert!(network.is_ok());
            let network = network.unwrap();
            assert_eq!(network.get_layers().len(), 2);
            let mut layers = network.get_layers().into_iter();
            let layer = layers.next().unwrap();
            assert_eq!(layer.get_neurons().len(), 4);
            let mut layers_neurons = layer.get_neurons().into_iter();
            let first_neuron = layers_neurons.next().unwrap();
            approx::assert_relative_eq!(first_neuron.bias, 0.024323463);
            assert_eq!(first_neuron.get_weights().len(), 1);
            approx::assert_relative_eq!(first_neuron.get_weights(), &[0.55383307].as_slice());
            let layer = layers.next().unwrap();
            assert_eq!(layer.get_neurons().len(), 2);
            let mut layers_neurons = layer.get_neurons().into_iter();
            let neuron = layers_neurons.next().unwrap();
            assert_eq!(neuron.get_weights().len(), 4);
            approx::assert_relative_eq!(neuron.bias, 0.65961576);
            approx::assert_relative_eq!(neuron.get_weights(), &[0.40742087, 0.89908165, 0.85216343, 0.22683924].as_slice());

            
        }
    }
}