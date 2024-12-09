// Copyright (C) 2024 [Kulpreet Singh]
//
//  This file is part of P2Poolv2
//
// P2Poolv2 is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free 
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// P2Poolv2 is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS 
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with 
// P2Poolv2. If not, see <https://www.gnu.org/licenses/>. 

use libp2p::{
    gossipsub, identify, 
    identity::Keypair, kad::{self, store::MemoryStore, RecordKey}, 
    ping, swarm::NetworkBehaviour, Multiaddr, PeerId,
};
use std::error::Error;
use tracing::{debug, info};
use tokio::time::Duration;

// Combine the behaviors we want to use
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "P2PoolBehaviourEvent")]
pub struct P2PoolBehaviour {
    // pub gossipsub: gossipsub::Behaviour,
    pub kademlia: kad::Behaviour<MemoryStore>,
    // pub ping: ping::Behaviour,
    pub identify: identify::Behaviour,
}

/// The interval at which the node will send ping messages to peers
const HEARTBEAT_INTERVAL: u64 = 15;

// Define the events that can be emitted by our behavior
#[derive(Debug)]
pub enum P2PoolBehaviourEvent {
    // Gossipsub(gossipsub::Event),
    Kademlia(kad::Event),
    // Ping(ping::Event),
    Identify(identify::Event),
}

impl P2PoolBehaviour {
    pub fn new(
        local_key: &Keypair,
    ) -> Result<Self, Box<dyn Error>> {
        // Initialize gossipsub
        // let gossipsub_config = gossipsub::ConfigBuilder::default()
        //     .heartbeat_interval(std::time::Duration::from_secs(HEARTBEAT_INTERVAL))
        //     .validation_mode(gossipsub::ValidationMode::Strict)
        //     .build()
        //     .expect("Valid config");

        // let gossipsub_behaviour = gossipsub::Behaviour::new(
        //     gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        //     gossipsub_config,
        // )?;

        // Initialize Kademlia
        let store = MemoryStore::new(local_key.public().to_peer_id());
        let mut kad_config = kad::Config::default();
        kad_config.set_query_timeout(tokio::time::Duration::from_secs(60));

        let kademlia_behaviour = kad::Behaviour::with_config(
            local_key.public().to_peer_id(),
            store,
            kad_config,
        );

        let identify_behaviour = identify::Behaviour::new(
            identify::Config::new(
                "/p2pool/1.0.0".to_string(),
                local_key.public(),
            )
        );

        Ok(P2PoolBehaviour {
            // gossipsub: gossipsub_behaviour,
            kademlia: kademlia_behaviour,
            // ping: libp2p::ping::Behaviour::new(ping::Config::new().with_interval(tokio::time::Duration::from_secs(HEARTBEAT_INTERVAL))),
            identify: identify_behaviour,
        })
    }

    pub fn add_peer_address(&mut self, peer_id: PeerId, addr: Multiaddr) {
        // Add the peer's address to Kademlia's routing table
        self.kademlia.add_address(&peer_id, addr);
    }

    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        // Remove the peer from Kademlia's routing table
        self.kademlia.remove_peer(peer_id);
    }
}

// impl From<gossipsub::Event> for P2PoolBehaviourEvent {
//     fn from(event: gossipsub::Event) -> Self {
//         P2PoolBehaviourEvent::Gossipsub(event)
//     }
// }

impl From<kad::Event> for P2PoolBehaviourEvent {
    fn from(event: kad::Event) -> Self {
        P2PoolBehaviourEvent::Kademlia(event)
    }
}

// impl From<ping::Event> for P2PoolBehaviourEvent {
//     fn from(event: ping::Event) -> Self {
//         P2PoolBehaviourEvent::Ping(event)
//     }
// }

impl From<identify::Event> for P2PoolBehaviourEvent {
    fn from(event: identify::Event) -> Self {
        P2PoolBehaviourEvent::Identify(event)
    }
}
