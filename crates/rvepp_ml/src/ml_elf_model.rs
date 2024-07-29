use crate::MachineLearningModel;

use lightgbm::{Booster};

pub struct ElfModel { }

impl MachineLearningModel for ElfModel {
    fn init(&self) {
        let model_result = match Booster::from_file("elf.mdl") {
            Ok(model) => model,
            Err(e) => panic!("{}", e)
        };

        let _ = match model_result.predict(vec![vec![100.0, 1.0]]) {
            Ok(result) => println!("{:?}", result),
            Err(e) => panic!("{}", e)
        };
    }
}