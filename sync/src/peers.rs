#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Peer {
    pub multiaddr: &'static str,
}

pub fn bootstrap_peers() -> &'static [Peer] {
    &[
        Peer { multiaddr: "/dns/bootstrap1.voidblock.local/tcp/443" },
        Peer { multiaddr: "/dns/bootstrap2.voidblock.local/tcp/443" },
    ]
}
