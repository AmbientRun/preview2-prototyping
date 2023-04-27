#![allow(unused_variables)]

use crate::{
    wasi::ip_name_lookup::{self, ResolveAddressStream},
    wasi::network::{Error, IpAddress, IpAddressFamily, Network},
    wasi::poll::Pollable,
    HostResult, WasiCtx,
};

impl ip_name_lookup::Host for WasiCtx {
    /*async*/
    fn resolve_addresses(
        &mut self,
        network: Network,
        name: String,
        address_family: Option<IpAddressFamily>,
        include_unavailable: bool,
    ) -> HostResult<ResolveAddressStream, Error> {
        todo!()
    }

    /*async*/
    fn resolve_next_address(
        &mut self,
        stream: ResolveAddressStream,
    ) -> HostResult<Option<IpAddress>, Error> {
        todo!()
    }

    /*async*/
    fn drop_resolve_address_stream(&mut self, stream: ResolveAddressStream) -> anyhow::Result<()> {
        todo!()
    }

    /*async*/
    fn non_blocking(&mut self, stream: ResolveAddressStream) -> HostResult<bool, Error> {
        todo!()
    }

    /*async*/
    fn set_non_blocking(
        &mut self,
        stream: ResolveAddressStream,
        value: bool,
    ) -> HostResult<(), Error> {
        todo!()
    }

    /*async*/
    fn subscribe(&mut self, stream: ResolveAddressStream) -> anyhow::Result<Pollable> {
        todo!()
    }
}
