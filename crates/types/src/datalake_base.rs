pub enum DataPoint {
    Int(i32),
    Str(String),
}

pub struct DatalakeBase {
    pub identifier: String,
    pub compilation_pipeline: Vec<Box<dyn Fn() -> Vec<DataPoint>>>,
    pub datapoints: Vec<DataPoint>,
}

impl DatalakeBase {
    pub fn new<F>(identifier: &str, compiler: F) -> Self
    where
        F: Fn() -> Vec<DataPoint> + 'static,
    {
        Self {
            identifier: identifier.to_string(),
            compilation_pipeline: vec![Box::new(compiler)],
            datapoints: Vec::new(),
        }
    }

    // TODO: decide if we want to merge datalakes
    // fn merge(&mut self, other: DatalakeBase) {
    //     self.compilation_pipeline.extend(other.compilation_pipeline);
    //     self.identifier = format!("{}{}", self.identifier, other.identifier);
    // }

    pub fn compile(&mut self) {
        for compiler in &self.compilation_pipeline {
            self.datapoints.extend(compiler());
        }
    }
}
