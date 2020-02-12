use std::io;

use crate::mapper::MapperOutput;

pub trait ResponseMutation: Send + Sync {
    fn apply_mutation(&self, output: &mut MapperOutput) -> io::Result<()>;
}

pub struct CreateMissing;

impl ResponseMutation for CreateMissing {
    fn apply_mutation(&self, _output: &mut MapperOutput) -> io::Result<()> {
        Ok(())
    }
}
