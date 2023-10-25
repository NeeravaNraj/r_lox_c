use crate::{backend::vm::Vm, parse_args::Options};

pub struct Runner {
    vm: Vm,
    options: Options,
}

impl Runner {
    pub fn new(options: Options) -> Self {
        if !options.file_path.exists() {
            panic!("Error: file does not exist!")
        }
        Self {
            options: options.clone(),
            vm: Vm::new(options.clone()),
        }
    }

    pub fn run(&mut self) {
        let Some(path) = self.options.file_path.to_str() else {
            panic!("Error: file name not found");
        };
        let source = std::fs::read_to_string(self.options.file_path.clone())
            .expect(format!("Unable to read source file: {path}").as_str());
        self.vm.interpret(path.into(), source);
    }
}
