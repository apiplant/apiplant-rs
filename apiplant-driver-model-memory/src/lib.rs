use apiplant_server_lib::{Function, InvocationError, PluginRegistrar};

apiplant_server_lib::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function("random", Box::new(Random));
}

#[derive(Debug, Clone, PartialEq)]
pub struct Random;

impl Function for Random {
    fn call(&self, args: &[f64]) -> Result<f64, InvocationError> {
        run(args)
    }
}

fn run(args: &[f64]) -> Result<f64, InvocationError> {
    match args.len() {
        0 => Ok(0.0),
        1 => Ok(args[0]),
        2 => Ok(args[1]),
        _ => Err("0, 1, or 2 arguments are required".into()),
    }
}