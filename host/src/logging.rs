use crate::wasi::console;
use crate::WasiCtx;

impl console::Host for WasiCtx {
    /*async*/
    fn log(
        &mut self,
        level: console::Level,
        context: String,
        message: String,
    ) -> anyhow::Result<()> {
        println!("{:?} {}: {}", level, context, message);
        Ok(())
    }
}
