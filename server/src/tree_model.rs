use std::io::BufReader;
use std::fs::File;
use std::path::Path;


use serde::{Deserialize, Serialize};
use linfa::traits::Predict;
use linfa_trees::DecisionTree;
use linfa::prelude::Error as LinfaError;

use anyhow::Result;
use ndarray::{Array, ArrayBase, Dim, Ix2, OwnedRepr};
use serde_json::from_reader;


#[derive(Debug, Deserialize)]
pub struct TreeModelInput{
    pub records: Array<f64,Ix2>
}

#[derive(Debug, Serialize)]
pub struct TreeModelPrediction{
    pub label: ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>>,
}

pub struct TreeModel{
    model: DecisionTree<f64, bool>
}

impl TreeModel{
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self>{
    
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let model: DecisionTree<f64, bool> = from_reader(reader)
    .unwrap();

    Ok(Self{model})
    }

    pub fn predict_input(&self,
        input: TreeModelInput) -> Result<TreeModelPrediction, LinfaError>
    {

        let label = self.model
        .predict(&input.records);

        Ok(TreeModelPrediction{label})
    }
}