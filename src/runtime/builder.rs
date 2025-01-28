use std::io;
use tokio::runtime::Builder as TBuilder;
use crate::runtime::runtime::Runtime;

pub struct Builder {
    builder: TBuilder
}

impl Builder {
    pub fn new_current_thread() -> Self {
        Self {
            builder: TBuilder::new_current_thread()
        }
    }

    pub fn new_multi_thread() -> Self {
        Self {
            builder: TBuilder::new_current_thread()
        }
    }

    pub fn enable_all(&mut self) -> &mut Self {
        self.builder.enable_all();
        self
    }

    pub fn build(&mut self) -> io::Result<Runtime> {
        self.builder.build().map(|runtime| Runtime {
            runtime
        })
    }
}