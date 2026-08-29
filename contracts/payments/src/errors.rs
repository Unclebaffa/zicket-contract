use soroban_sdk::contracterror;

/// Payment contract error codes.
///
/// Discriminants follow the unified numbering scheme shared by `EventError`,
/// `PaymentError`, and `TicketError`: errors matching a category defined in
/// `common_utils::errors::CommonErrorCode` use that category's canonical number
/// (see the `// CommonErrorCode::*` comment on each such variant), with
/// same-category duplicates packed into the remaining slots of that category's
/// band. Errors with no common-category equivalent live in this contract's
/// reserved extension range, 300-399 (event: 200-299, ticket: 400-499),
/// so a raw discriminant alone identifies both its semantic category and,
/// for domain-specific errors, its originating contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PaymentError {
    PaymentNotFound = 1,          // CommonErrorCode::NotFound
    TicketNotFound = 2,           // CommonErrorCode::NotFound
    InsufficientFunds = 81,       // CommonErrorCode::InsufficientFunds
    Unauthorized = 11,            // CommonErrorCode::Unauthorized
    PaymentAlreadyProcessed = 43, // CommonErrorCode::AlreadyProcessed
    InvalidAmount = 22,           // CommonErrorCode::InvalidAmount
    RefundFailed = 300,
    NotInitialized = 61,         // CommonErrorCode::NotInitialized
    PaymentAlreadyRefunded = 41, // CommonErrorCode::AlreadyProcessed
    NoRevenue = 301,
    AnonymousPaymentsDisabled = 302,
    VerificationRequired = 303,
    UnauthorizedWithdrawal = 12, // CommonErrorCode::Unauthorized
    InvalidOrganizer = 304,
    InvalidPayoutToken = 305,
    EventNotActive = 42,    // CommonErrorCode::NotActive
    EventNotCompleted = 44, // CommonErrorCode::NotCompleted
    RefundNotAllowed = 306,
    EscrowNotExpired = 307,
    EscrowAlreadyReleased = 45, // CommonErrorCode::AlreadyProcessed
    EscrowNotConfigured = 62,   // CommonErrorCode::NotConfigured
    AccountingMismatch = 103,   // CommonErrorCode::AccountingMismatch
    InvalidFeeBps = 24,         // CommonErrorCode::InvalidFeeBps
    NoPlatformRevenue = 308,
    DuplicateRequest = 309,
    MigrationFailed = 121,    // CommonErrorCode::MigrationFailed
    UnsupportedVersion = 122, // CommonErrorCode::UnsupportedVersion
    MaxTicketsReached = 82,   // CommonErrorCode::MaxLimitReached
    EventSoldOut = 83,        // CommonErrorCode::SoldOut
    NonceRequired = 310,
    ContractPaused = 101, // CommonErrorCode::ContractPaused
    /// Token transfer via the Soroban token interface failed unexpectedly.
    TransferFailed = 102, // CommonErrorCode::TransferFailed
    PostponementWindowClosed = 311,
    EventNotPostponed = 312,
    /// Revenue split configuration is invalid (bad sum, too many recipients,
    /// duplicate or empty recipient, or an attempt to mutate an existing config).
    InvalidSplitConfig = 21, // CommonErrorCode::InvalidInput
    /// No revenue split has been configured for this event.
    SplitsNotConfigured = 63, // CommonErrorCode::NotConfigured
    /// The caller is not one of the configured split recipients.
    NotASplitRecipient = 313,
    /// This recipient has already withdrawn (or had reassigned) its split share.
    SplitAlreadyWithdrawn = 46, // CommonErrorCode::AlreadyProcessed
    /// The recipient's share is frozen because the wallet has been flagged.
    RecipientFlagged = 314,
    /// The recipient is not currently flagged.
    RecipientNotFlagged = 315,
    /// A zkEmail commitment is already bound to this payment; commitments are
    /// write-once and cannot be overwritten.
    CommitmentAlreadySet = 47, // CommonErrorCode::AlreadyProcessed
    /// The payment is in a state that no longer accepts a commitment
    /// (e.g. it has been refunded).
    CommitmentNotAllowed = 316,
    /// Anonymous payment is missing its required nullifier commitment
    MissingNullifierCommitment = 317,
    /// Private payment is missing its required stealth delivery key
    MissingStealthDeliveryKey = 318,
    /// The supplied privacy data does not match the declared privacy level
    PrivacyLevelMismatch = 319,
    /// The dispute window is closed or not open yet.
    DisputeWindowClosed = 320,
    /// A dispute already exists for this ticket.
    DisputeAlreadyExists = 321,
    /// No dispute record was found for this ticket.
    DisputeNotFound = 322,
    /// The dispute resolution timeout has expired.
    DisputeExpired = 323,
    /// Invalid reason code provided for dispute.
    InvalidDisputeReason = 324,
    /// Organizer cannot withdraw while disputes are active.
    /// Organizer cannot withdraw while disputes are active.
    ActiveDisputes = 325,
    /// Token escrow balance invariant violated
    RevenueInvariantViolated = 326,}
