use ethportal_api::types::discv5::Enr;
use ethportal_api::types::portal::DataRadius;
use ethportal_api::{HistoryContentItem, HistoryContentKey};

/// Discv5 JSON-RPC endpoints. Start with "discv5_" prefix
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Discv5Endpoint {
    NodeInfo,
    RoutingTableInfo,
}

/// State network JSON-RPC endpoints. Start with "portal_state" prefix
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StateEndpoint {
    DataRadius,
    FindContent,
    FindNodes,
    LocalContent,
    SendOffer,
    Store,
    Ping,
    RecursiveFindContent,
    RoutingTableInfo,
}

/// History network JSON-RPC endpoints. Start with "portal_history" prefix
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HistoryEndpoint {
    /// params: None
    DataRadius,
    /// params: [enr, content_key]
    FindContent(Enr, HistoryContentKey),
    /// params: [enr, distances]
    FindNodes(Enr, Vec<u16>),
    /// params: content_key
    LocalContent(HistoryContentKey),
    /// params: [content_key, content_value]
    Gossip(HistoryContentKey, HistoryContentItem),
    /// params: [enr, content_key]
    Offer(Enr, HistoryContentKey, Option<HistoryContentItem>),
    /// params: [enr, data_radius]
    Ping(Enr, Option<DataRadius>),
    /// params: content_key
    RecursiveFindContent(HistoryContentKey),
    /// params: content_key
    TraceRecursiveFindContent(HistoryContentKey),
    /// params: [content_key, content_value]
    Store(HistoryContentKey, HistoryContentItem),
    /// params: None
    RoutingTableInfo,
    // This endpoint is not History network specific
    /// params: [offset, limit]
    PaginateLocalContentKeys(u64, u64),
}

/// Ethereum JSON-RPC endpoints not currently supported by portal network requests, proxied to
/// trusted provider
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TrustedProviderEndpoint {
    BlockNumber,
}

/// Ethereum JSON-RPC endpoints supported by portal network requests
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PortalEndpoint {
    ClientVersion, // Doesn't actually rely on portal network data, but it makes sense to live here
    GetBlockByHash,
    GetBlockByNumber,
}

/// Global portal network endpoints supported by trin, including trusted providers, Discv5, Ethereum and all overlay network endpoints supported by portal network requests
// When adding a json-rpc endpoint, make sure to...
// - Update `docs/jsonrpc_api.md`
// - Add tests to ethportal-peertest
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TrinEndpoint {
    Discv5Endpoint(Discv5Endpoint),
    HistoryEndpoint(HistoryEndpoint),
    StateEndpoint(StateEndpoint),
    TrustedProviderEndpoint(TrustedProviderEndpoint),
    PortalEndpoint(PortalEndpoint),
}
