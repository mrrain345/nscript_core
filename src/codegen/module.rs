use crate::{
    builder,
    nscript::{self, environment::Environment, module::StateMutRef},
    parser::Expression,
};

use super::*;

pub struct Module(builder::Module);

pub struct ModuleEnv<'a> {
    pub builder: builder::Module,
    pub state: StateMutRef<'a>,
    pub op: builder::Op,
    pub int32: builder::Int32,
}

impl Module {
    pub fn compile(
        env: &Environment,
        path: String,
        expressions: Vec<Expression>,
        is_main: bool,
    ) -> Self {
        let builder = builder::Module::new();
        let module = nscript::Module::new(path);

        let builder = {
            let state = module.state_mut();

            // Create module environment
            let mut module_env = ModuleEnv {
                state,
                op: builder.op(),
                int32: builder.int32(),
                builder,
            };

            // Execute global instructions
            let mut body = Vec::with_capacity(expressions.len());
            for expr in expressions {
                body.push(codegen::codegen(&mut module_env, expr).expr);
            }

            // Add main function to the builder module
            if is_main {
                module_env
                    .builder
                    .add_function("main".into(), &[], &[], &[], &body);

                module_env.builder.add_export("main".into(), None);
            }

            module_env.builder
        };

        env.add_module(module);

        Self(builder)
    }

    pub fn optimize(&self) {
        self.0.optimize()
    }

    pub fn print(&self) {
        self.0.print()
    }
}
