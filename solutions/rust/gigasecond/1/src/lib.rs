use time::PrimitiveDateTime as DateTime;
use time::SignedDuration;

// Returns a DateTime one billion seconds after start.

const GIGASECOND: SignedDuration = SignedDuration::seconds(1_000_000_000);
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(GIGASECOND)
}
