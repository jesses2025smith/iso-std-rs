//! Table 1 — Vehicle identification parameter values (value not set)
//!
//! Table 2 — Payload type vehicle identification request message — No message parameters
//!
//! Table 3 — Payload type vehicle identification request message with EID
//!
//! Table 4 — Payload type vehicle identification request message with VIN
//!
//! Table 5 — Payload type vehicle announcement/identification response message
//!
//! Table 6 — Definition of further action code values
//!
//! Table 7 — Definition of VIN/GID synchronization status code values
//!
//! Table 8 — Diagnostic power mode information request
//!
//! Table 9 — Diagnostic power mode information response
//!
//! Table 10 — DoIP entity status request
//!
//! Table 11 — DoIP entity status response
//!
//! Table 12 — DoIP timing and communication parameters
//!
//! Table 13 — Logical address overview
//!
//! Table 14 — DHCP on OSI layers
//!
//! Table 15 — IETF RFC 3927 adapted timings
//!
//! Table 16 — Generic DoIP header structure
//!
//! Table 17 — Overview of DoIP payload types
//!
//! Table 18 — Generic DoIP header negative acknowledge structure
//!
//! Table 19 — Generic DoIP header NACK codes
//!
//! Table 20 — UDP and TCP port usage
//!
//! Table 21 — Payload type diagnostic message structure
//!
//! Table 22 — Example of ISO 27145-3 request message transported by a DoIP message frame
//!
//! Table 23 — Payload type diagnostic message positive acknowledgment structure
//!
//! Table 24 — Diagnostic message positive acknowledge codes
//!
//! Table 25 — Payload type diagnostic message negative acknowledgment structure
//!
//! Table 26 — Diagnostic message negative acknowledge codes
//!
//! Table 27 — Payload type alive check request structure
//!
//! Table 28 — Payload type alive check response structure
//!
//! Table 29 — TLS authentication type
//!
//! Table 30 — TLS 1.2 version cipher suites
//!
//! Table 31 — TLS 1.3 version cipher suites
//!
//! Table 32 — TLS 1.2 version supported TLS extensions
//!
//! Table 33 — TLS 1.2 version optional TLS extensions
//!
//! Table 34 — TLS 1.2 version not supported TLS extensions
//!
//! Table 35 — TLS 1.3 version supported TLS extensions
//!
//! Table 36 — TLS 1.3 version optional TLS extensions
//!
//! Table 37 — TLS 1.3 version not supported TLS extensions
//!
//! Table 38 — TCP on OSI layers
//!
//! Table 39 — Supported TCP ports
//!
//! Table 40 — UDP on OSI layers
//!
//! Table 41 — UDP ports
//!
//! Table 42 — IPv4/IPv6 on OSI layers
//!
//! Table 43 — ARP on OSI layers
//!
//! Table 44 — NDP on OSI layers
//!
//! Table 45 — ICMP on OSI layers
//!
//! Table 46 — Payload type routing activation request
//!
//! Table 47 — Routing activation request activation types
//!
//! Table 48 — Payload type routing activation response
//!
//! Table 49 — Routing activation response code values
mod common;
mod constants;
mod error;

pub mod request;
pub mod response;

pub(crate) mod utils;

mod id;
mod payload;

pub type Eid = id::Id;
pub type Gid = id::Id;
pub use self::{common::*, constants::*, error::Error as Iso13400Error, payload::*};

/// It will be removed in a future version. Use [NodeType] instead
#[deprecated(
    since = "0.1.0",
    note = "It will be removed in a future version. Use 'NodeType` instead"
)]
pub type Entity = NodeType;
