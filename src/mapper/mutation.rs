use std::io;

use crate::mapper::MapperOutput;

pub trait ResponseMutation: Send + Sync {
    fn apply_mutation(&mut self, output: &mut MapperOutput) -> io::Result<()>;
}

pub struct CreateMissing;

impl ResponseMutation for CreateMissing {
    fn apply_mutation(&mut self, _output: &mut MapperOutput) -> io::Result<()> {
        Ok(())
    }
}
