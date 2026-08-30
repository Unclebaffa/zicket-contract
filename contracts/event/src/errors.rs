use soroban_sdk::contracterror;

/// Event contract error codes.
///
/// Discriminants follow the unified numbering scheme shared by `EventError`,
/// `PaymentError`, and `TicketError`: errors matching a category defined in
/// `common_utils::errors::CommonErrorCode` use that category's canonical number
/// (see the `// CommonErrorCode::*` comment on each such variant) when this
/// contract has only one variant in that category. Same-category duplicates
/// fill the remaining slots of that category's band -- those slots identify
/// the shared category only, not a specific universal error, so decoding one
/// still requires this enum's context. Errors with no common-category
/// equivalent live in this contract's reserved extension range, 200-299
/// (payments: 300-399, ticket: 400-499), so a raw discriminant alone
/// identifies its semantic category and, for domain-specific errors, its
/// originating contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EventError {
    EventNotFound = 1,            // CommonErrorCode::NotFound
    EventAlreadyExists = 2,       // CommonErrorCode::AlreadyExists
    InvalidStatusTransition = 23, // CommonErrorCode::InvalidStatusTransition
    Unauthorized = 11,            // CommonErrorCode::Unauthorized
    InvalidInput = 21,            // CommonErrorCode::InvalidInput
    EventNotActive = 41,          // CommonErrorCode::NotActive
    InvalidEventDate = 200,
    InvalidTicketCount = 201,
    InvalidPrice = 22, // CommonErrorCode::InvalidAmount
    EventNotUpdatable = 202,
    EventSoldOut = 83,      // CommonErrorCode::SoldOut
    AlreadyRegistered = 43, // CommonErrorCode::AlreadyProcessed
    TierNotFound = 203,
    TierSoldOut = 81,                // CommonErrorCode::SoldOut
    ContractLinksNotConfigured = 62, // CommonErrorCode::NotConfigured
    RefundFailed = 204,
    ReservationNotFound = 205,
    ReservationExpired = 206,
    InvalidOrganizer = 207,
    InvalidPayoutToken = 208,
    MigrationFailed = 121,          // CommonErrorCode::MigrationFailed
    UnsupportedVersion = 122,       // CommonErrorCode::UnsupportedVersion
    UnauthorizedPrivateAccess = 12, // CommonErrorCode::Unauthorized
    PrivacyViolation = 209,
    ClaimLimitExceeded = 82, // CommonErrorCode::MaxLimitReached
    ClaimCooldownActive = 210,
    AnonCommitmentReused = 211,
    AnonClaimWindowFull = 84, // CommonErrorCode::MaxLimitReached
    AnonymousClaimsNotEnabled = 212,
    /// The requested refund-choice window is shorter than the mandatory minimum
    /// (`MIN_POSTPONEMENT_CHOICE_WINDOW_LEDGERS`).
    PostponementWindowTooShort = 213,
    /// The proposed new event date is not strictly after the close of the
    /// refund-choice window, or is in the past.
    InvalidPostponementDate = 214,
    /// The event has already been postponed the maximum number of times
    /// (`MAX_POSTPONEMENTS`); the organizer must run or cancel it instead.
    MaxPostponementsReached = 85, // CommonErrorCode::MaxLimitReached
    /// `finalize_postponement` was called while the refund-choice window is still open.
    PostponementWindowOpen = 215,
    /// The operation requires the event to be in the `Postponed` state.
    EventNotPostponed = 216,
    /// The caller holds no revocable (valid, unused) ticket for the event, so a
    /// postponement refund cannot be issued (e.g. the ticket was already used or
    /// transferred away).
    NoRefundableTicket = 217,
    /// Revenue split is malformed: wrong basis-point sum, too many recipients,
    /// a zero/duplicate recipient, or index 0 is not the primary organizer.
    InvalidRevenueSplit = 24, // CommonErrorCode::InvalidInput
    // -- zkPassport errors ----------------------------------------------------
    /// The proof's `expiry_ledger` is less than the current ledger sequence.
    ZkProofExpired = 218,
    /// This nullifier has already been recorded for this event -- proof reuse
    /// is not allowed.
    ZkNullifierReused = 219,
    /// The event `requires_verification` is `true` but the ZkVerificationConfig
    /// has not been enabled by the organizer.
    ZkVerificationRequired = 220,
    /// Reserved for future on-chain verifier integration. Currently signals that
    /// the provided proof bytes are structurally invalid.
    ZkProofInvalid = 221,
    /// The submitted `ZkPassportClaim.claim_type` does not match the type
    /// required by the event's `ZkVerificationConfig`.
    ZkClaimTypeMismatch = 222,
    /// The event is configured for Private/Anonymous payment privacy, which the
    /// cross-contract `register_for_event` path cannot settle because it has no
    /// client-generated privacy material (stealth key / nullifier commitment).
    /// Use the payments contract's privacy-aware entry point directly.
    PaymentPrivacyUnsupported = 223,
    AnonymousProofExpired = 224,
    AnonymousProofInvalid = 225,
    AnonymousClaimVerifierNotConfigured = 226,
    AnonymousNullifierReused = 227,
    AnonymousClaimVerifierAlreadyConfigured = 228,
    AnonymousProofExpiryTooFar = 229,
}
