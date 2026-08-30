use soroban_sdk::contracterror;

/// Ticket contract error codes.
///
/// Discriminants follow the unified numbering scheme shared by `EventError`,
/// `PaymentError`, and `TicketError`: errors matching a category defined in
/// `common_utils::errors::CommonErrorCode` use that category's canonical number
/// (see the `// CommonErrorCode::*` comment on each such variant) when this
/// contract has only one variant in that category. Same-category duplicates
/// fill the remaining slots of that category's band -- those slots identify
/// the shared category only, not a specific universal error, so decoding one
/// still requires this enum's context. Errors with no common-category
/// equivalent live in this contract's reserved extension range, 400-499
/// (event: 200-299, payments: 300-399), so a raw discriminant alone
/// identifies its semantic category and, for domain-specific errors, its
/// originating contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TicketError {
    TicketNotFound = 1,           // CommonErrorCode::NotFound
    TicketAlreadyExists = 2,      // CommonErrorCode::AlreadyExists
    InvalidStatusTransition = 23, // CommonErrorCode::InvalidStatusTransition
    Unauthorized = 11,            // CommonErrorCode::Unauthorized
    InvalidInput = 21,            // CommonErrorCode::InvalidInput
    TicketNotActive = 41,         // CommonErrorCode::NotActive
    InvalidTicketDate = 400,
    InvalidTicketCount = 401,
    InvalidPrice = 22, // CommonErrorCode::InvalidAmount
    TicketNotUpdatable = 402,
    TicketNotTransferable = 403,
    TransferToSelf = 404,
    TicketAlreadyUsed = 43,        // CommonErrorCode::AlreadyProcessed
    EventNotActive = 42,           // CommonErrorCode::NotActive
    MigrationFailed = 121,         // CommonErrorCode::MigrationFailed
    UnsupportedVersion = 122,      // CommonErrorCode::UnsupportedVersion
    RecoveryKeyNotFound = 3,       // CommonErrorCode::NotFound
    InvalidRecoverySignature = 24, // CommonErrorCode::InvalidInput
    TicketNotUsed = 405,
}
