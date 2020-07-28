use git2::Delta;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
	/// Entry does not exist in old version
	Added,
	/// Entry does not exist in new version
	Deleted,
	/// Entry content changed between old and new
	Modified,
	/// Entry was renamed between old and new
	Renamed,
	/// Entry was copied from another old entry
	Copied,
	/// Type of entry changed between old and new
	Typechange,
	/// Other type of change not normally found in a rebase
	Other,
}

impl Status {
	pub(super) fn new_from_git_delta(delta: Delta) -> Self {
		match delta {
			Delta::Added => Self::Added,
			Delta::Conflicted => Self::Other,
			Delta::Copied => Self::Copied,
			Delta::Deleted => Self::Deleted,
			Delta::Ignored => Self::Other,
			Delta::Modified => Self::Modified,
			Delta::Renamed => Self::Renamed,
			Delta::Typechange => Self::Typechange,
			Delta::Unmodified => Self::Other,
			Delta::Unreadable => Self::Other,
			Delta::Untracked => Self::Other,
		}
	}
}