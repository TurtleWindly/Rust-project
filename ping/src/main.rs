use libp2p::swarm::{keep_alive, NetworkBehaviour, Swarm};
use libp2p::{identity, ping, Multiaddr, PeerId};
use std::error::Error;

#[derive(NetworkBehaviour, Default)]
struct Behaivour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create unique peer id for it node
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {}", local_peer_id);

    // transport mean how to send bytes
    let transport = libp2p::development_transport(local_key).await?;

    // behaviour mean what bytes to send
    let behaviour = Behaivour::default();

    // Swarm mean for how to connect
    // Swarm::with_async_std_executor is deprecated, use SwarmBuilder instead
    let mut swarm = Swarm::with_async_std_executor(transport, behaviour, local_peer_id);

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dial: {}", addr);
    }

    Ok(())
}
