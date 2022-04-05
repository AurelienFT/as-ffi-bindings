use wasmer::{Function, HostEnvInitError, Instance, LazyInit, Memory, WasmerEnv, ExportError};

#[derive(Clone, Default)]
pub struct Env {
    pub memory: LazyInit<Memory>,
    pub fn_new: Option<Function>,
    pub fn_pin: Option<Function>,
    pub fn_unpin: Option<Function>,
    pub fn_collect: Option<Function>,
}

impl Env {
    pub fn new(
        arg_memory: Memory,
        fn_new: Option<Function>,
        fn_pin: Option<Function>,
        fn_unpin: Option<Function>,
        fn_collect: Option<Function>,
    ) -> Env {
        let mut memory = LazyInit::<Memory>::default();
        memory.initialize(arg_memory);
        Env {
            memory,
            fn_new,
            fn_pin,
            fn_unpin,
            fn_collect,
        }
    }

    pub fn init(&mut self, instance: &Instance) -> anyhow::Result<()> {
        Ok(self.init_with_instance(instance)?)
    }
}

impl WasmerEnv for Env {
    fn init_with_instance(&mut self, instance: &Instance) -> Result<(), HostEnvInitError> {
        let mem: Memory = instance
            .exports
            .get_with_generics_weak("memory")
            .map_err(HostEnvInitError::from)?;
        let new: Result<Function, ExportError> = instance.exports.get_with_generics_weak("__new");
        let pin: Result<Function, ExportError> = instance.exports.get_with_generics_weak("__pin");
        let unpin: Result<Function, ExportError> = instance.exports.get_with_generics_weak("__unpin");
        let collect: Result<Function, ExportError> = instance.exports.get_with_generics_weak("__collect");
        if let Ok(func) = new {
            self.fn_new = Some(func)
        }
        if let Ok(func) = pin {
            self.fn_pin = Some(func)
        }
        if let Ok(func) = unpin {
            self.fn_unpin = Some(func)
        }
        if let Ok(func) = collect {
            self.fn_collect = Some(func)
        }
        self.memory.initialize(mem);
        Ok(())
    }
}
