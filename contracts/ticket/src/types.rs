use soroban_sdk::{contracttype, Address, Symbol};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum TicketStatus {
    Valid,
    Used,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub ticket_id: u64,
    pub event_id: Symbol,
    pub organizer: Address,
    pub owner: Address,
    pub issued_at: u64,
    pub status: TicketStatus,
    pub is_transferable: bool,
    pub is_used: bool,
}

/// Maximum number of tickets that can be minted in a single batch operation.
///
/// Each mint operation performs multiple persistent storage write operations:
/// - `DataKey::Ticket(ticket_id)`
/// - `DataKey::OwnerTicket(owner, ticket_id)`
/// - `DataKey::OwnerTicketIndex(owner, count)`
/// - `DataKey::OwnerTicketsCount(owner)`
/// - `DataKey::EventTicket(event_id, ticket_id)`
/// - `DataKey::EventTicketIndex(event_id, count)`
/// - `DataKey::EventTicketsCount(event_id)`
///
/// In addition to `NextTicketId` and event emissions.
/// Capping the batch at 30 keeps total persistent storage writes (~180 writes) safely
/// within Soroban's ledger write limit of 200 operations per transaction.
pub const MAX_BATCH_TICKET_MINT: u32 = 30;
