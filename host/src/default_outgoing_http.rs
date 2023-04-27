use crate::{
    wasi,
    wasi::types::{FutureIncomingResponse as Response, OutgoingRequest as Request, RequestOptions},
    WasiCtx,
};

impl wasi::default_outgoing_http::Host for WasiCtx {
    /*async*/
    fn handle(
        &mut self,
        _req: Request,
        _options: Option<RequestOptions>,
    ) -> wasmtime::Result<Response> {
        anyhow::bail!("not implemented")
    }
}
