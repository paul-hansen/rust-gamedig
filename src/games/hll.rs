use std::net::IpAddr;
use crate::{
    protocols::valve::{self, game, SteamApp},
    GDResult,
};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<game::Response> {
    let valve_response = valve::query(
        address,
        port.unwrap_or(26420),
        SteamApp::HLL.as_engine(),
        None,
        None,
    )?;

    Ok(game::Response::new_from_valve_response(valve_response))
}