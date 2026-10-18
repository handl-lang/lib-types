//! `c` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `c/index.hnl`.
pub const C_HNL: &str = include_str!("c/index.hnl");

/// `c/netinet.hnl`.
pub const NETINET_HNL: &str = include_str!("c/netinet.hnl");

/// `c/socket.hnl`.
pub const SOCKET_HNL: &str = include_str!("c/socket.hnl");

/// `c/stddef.hnl`.
pub const STDDEF_HNL: &str = include_str!("c/stddef.hnl");

/// `c/stdint.hnl`.
pub const STDINT_HNL: &str = include_str!("c/stdint.hnl");

/// `c/sys.hnl`.
pub const SYS_HNL: &str = include_str!("c/sys.hnl");

/// `c/time.hnl`.
pub const TIME_HNL: &str = include_str!("c/time.hnl");
