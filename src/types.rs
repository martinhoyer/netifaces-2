#![allow(dead_code)]
use std::collections::HashMap;

pub type AddrPairs = HashMap<String, String>;
pub type IfAddrs = HashMap<u32, Vec<AddrPairs>>;

pub const ADDR_ADDR: &str = "addr";
pub const MASK_ADDR: &str = "mask";
pub const BROADCAST_ADDR: &str = "broadcast";
pub const PEER_ADDR: &str = "peer";

pub const AF_UNSPEC: u8 = 0;
pub const AF_UNIX: u8 = 1;
pub const AF_LOCAL: u8 = 1;
pub const AF_INET: u8 = 2;
pub const AF_AX25: u8 = 3;
pub const AF_IPX: u8 = 4;
pub const AF_APPLETALK: u8 = 5;
pub const AF_NETROM: u8 = 6;
pub const AF_BRIDGE: u8 = 7;
pub const AF_ATMPVC: u8 = 8;
pub const AF_X25: u8 = 9;
pub const AF_INET6: u8 = 10;
pub const AF_ROSE: u8 = 11;
pub const AF_DECNET: u8 = 12;
pub const AF_NETBEUI: u8 = 13;
pub const AF_SECURITY: u8 = 14;
pub const AF_KEY: u8 = 15;
pub const AF_NETLINK: u8 = 16;
pub const AF_ROUTE: u8 = 16;
pub const AF_PACKET: u8 = 17;
pub const AF_ASH: u8 = 18;
pub const AF_ECONET: u8 = 19;
pub const AF_ATMSVC: u8 = 20;
pub const AF_RDS: u8 = 21;
pub const AF_SNA: u8 = 22;
pub const AF_IRDA: u8 = 23;
pub const AF_PPPOX: u8 = 24;
pub const AF_WANPIPE: u8 = 25;
pub const AF_LLC: u8 = 26;
pub const AF_IB: u8 = 27;
pub const AF_MPLS: u8 = 28;
pub const AF_CAN: u8 = 29;
pub const AF_TIPC: u8 = 30;
pub const AF_BLUETOOTH: u8 = 31;
pub const AF_IUCV: u8 = 32;
pub const AF_RXRPC: u8 = 33;
pub const AF_ISDN: u8 = 34;
pub const AF_PHONET: u8 = 35;
pub const AF_IEEE802154: u8 = 36;
pub const AF_CAIF: u8 = 37;
pub const AF_ALG: u8 = 38;
pub const AF_NFC: u8 = 39;
pub const AF_VSOCK: u8 = 40;
pub const AF_KCM: u8 = 41;
pub const AF_QIPCRTR: u8 = 42;
pub const AF_SMC: u8 = 43;
pub const AF_XDP: u8 = 44;
pub const AF_MCTP: u8 = 45;
pub const AF_MAX: u8 = 46;