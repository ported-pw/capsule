/*
* Copyright 2019 Comcast Cable Communications Management, LLC
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*
* SPDX-License-Identifier: Apache-2.0
*/

use anyhow::Result;
use capsule::packets::ethernet::Ethernet;
use capsule::packets::icmp::v4::{EchoReply, EchoRequest};
use capsule::packets::ip::v4::Ipv4;
use capsule::packets::{Mbuf, Packet, Postmark};
use capsule::runtime::{self, Runtime};
use signal_hook::consts;
use signal_hook::flag;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::fmt;

fn reply_echo(packet: Mbuf) -> Result<Postmark> {
    let reply = Mbuf::new()?;

    let ethernet = packet.peek::<Ethernet>()?;
    let mut reply = reply.push::<Ethernet>()?;
    reply.set_src(ethernet.dst());
    reply.set_dst(ethernet.src());

    let ipv4 = ethernet.peek::<Ipv4>()?;
    let mut reply = reply.push::<Ipv4>()?;
    reply.set_src(ipv4.dst());
    reply.set_dst(ipv4.src());
    reply.set_ttl(255);

    let request = ipv4.peek::<EchoRequest>()?;
    let mut reply = reply.push::<EchoReply>()?;
    reply.set_identifier(request.identifier());
    reply.set_seq_no(request.seq_no());
    reply.set_data(request.data())?;
    reply.reconcile_all();

    info!(?request);
    info!(?reply);

    Ok(Postmark::emit_and_drop(reply, packet))
}

fn main() -> Result<()> {
    let subscriber = fmt::Subscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let config = runtime::load_config()?;
    let runtime = Runtime::from_config(config)?;

    runtime.ports()
        .get("cap0")?
        .spawn_rx_tx_pipeline(
            runtime.lcores(),
            |mbuf, _| reply_echo(mbuf),
            || (),
            None)?;

    let _guard = runtime.execute()?;

    let term = Arc::new(AtomicBool::new(false));
    flag::register(consts::SIGINT, Arc::clone(&term))?;
    info!("ctrl-c to quit ...");
    while !term.load(Ordering::Relaxed) {}

    Ok(())
}
