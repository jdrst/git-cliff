use serde::{
	Deserialize,
	Serialize,
};

use crate::commit::Commit;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Commit range (from..to or from_short..to_short)
pub struct CommitRange {
	from:       String,
	to:         String,
	from_short: String,
	to_short:   String,
}

impl CommitRange {
	/// Creates a new [`CommitRange`] from [`crate::commit::Commit`].
	pub fn new(from: &Commit, to: &Commit) -> Self {
		Self {
			from:       from.id.clone(),
			to:         to.id.clone(),
			from_short: from.short_id.clone(),
			to_short:   to.short_id.clone(),
		}
	}
}
