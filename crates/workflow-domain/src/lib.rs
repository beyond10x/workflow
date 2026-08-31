//! Pure workflow domain vocabulary.

/// Stable identity of a workflow within one server-derived tenant.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WorkflowId(String);

impl WorkflowId {
    /// Create an identifier after rejecting empty input.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidWorkflowId`] when the supplied value is empty or whitespace-only.
    pub fn parse(value: impl Into<String>) -> Result<Self, InvalidWorkflowId> {
        let value = value.into();
        if value.trim().is_empty() {
            Err(InvalidWorkflowId)
        } else {
            Ok(Self(value))
        }
    }

    /// Borrow the transport-neutral value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An empty workflow identifier is invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidWorkflowId;

impl std::fmt::Display for InvalidWorkflowId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("workflow id must be non-empty")
    }
}

impl std::error::Error for InvalidWorkflowId {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_identity_refuses_empty_input() {
        assert_eq!(WorkflowId::parse(" "), Err(InvalidWorkflowId));
        assert_eq!(
            WorkflowId::parse("release").expect("id").as_str(),
            "release"
        );
    }
}
