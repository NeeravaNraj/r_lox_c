
use std::{path::Path, rc::Rc};

use crate::backend::vm::Vm;

pub struct Runner<'a> {
    file: &'a Path,
    vm: Vm,
}

impl<'a> Runner<'a> {
    pub fn new(file: &'a Path) -> Self {
        if !file.exists() {
            panic!("Error: file doesn't exist!")
        }
        Self {
            file,
            vm: Vm::new(),
        }
    }

    pub fn run(&mut self) {
        let Some(path) = self.file.to_str() else {
            panic!("Error: file name not found");
        };
        let source = std::fs::read_to_string(self.file)
            .expect(format!("Unable to read source file: {path}").as_str());
        self.vm.interpret(path.into(), source);
    }
}
