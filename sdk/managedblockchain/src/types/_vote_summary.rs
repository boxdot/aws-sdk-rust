// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Properties of an individual vote that a member cast for a proposal. </p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VoteSummary {
    /// <p> The vote value, either <code>YES</code> or <code>NO</code>. </p>
    #[doc(hidden)]
    pub vote: ::std::option::Option<crate::types::VoteValue>,
    /// <p> The name of the member that cast the vote. </p>
    #[doc(hidden)]
    pub member_name: ::std::option::Option<::std::string::String>,
    /// <p> The unique identifier of the member that cast the vote. </p>
    #[doc(hidden)]
    pub member_id: ::std::option::Option<::std::string::String>,
}
impl VoteSummary {
    /// <p> The vote value, either <code>YES</code> or <code>NO</code>. </p>
    pub fn vote(&self) -> ::std::option::Option<&crate::types::VoteValue> {
        self.vote.as_ref()
    }
    /// <p> The name of the member that cast the vote. </p>
    pub fn member_name(&self) -> ::std::option::Option<&str> {
        self.member_name.as_deref()
    }
    /// <p> The unique identifier of the member that cast the vote. </p>
    pub fn member_id(&self) -> ::std::option::Option<&str> {
        self.member_id.as_deref()
    }
}
impl VoteSummary {
    /// Creates a new builder-style object to manufacture [`VoteSummary`](crate::types::VoteSummary).
    pub fn builder() -> crate::types::builders::VoteSummaryBuilder {
        crate::types::builders::VoteSummaryBuilder::default()
    }
}

/// A builder for [`VoteSummary`](crate::types::VoteSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VoteSummaryBuilder {
    pub(crate) vote: ::std::option::Option<crate::types::VoteValue>,
    pub(crate) member_name: ::std::option::Option<::std::string::String>,
    pub(crate) member_id: ::std::option::Option<::std::string::String>,
}
impl VoteSummaryBuilder {
    /// <p> The vote value, either <code>YES</code> or <code>NO</code>. </p>
    pub fn vote(mut self, input: crate::types::VoteValue) -> Self {
        self.vote = ::std::option::Option::Some(input);
        self
    }
    /// <p> The vote value, either <code>YES</code> or <code>NO</code>. </p>
    pub fn set_vote(mut self, input: ::std::option::Option<crate::types::VoteValue>) -> Self {
        self.vote = input;
        self
    }
    /// <p> The name of the member that cast the vote. </p>
    pub fn member_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.member_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the member that cast the vote. </p>
    pub fn set_member_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.member_name = input;
        self
    }
    /// <p> The unique identifier of the member that cast the vote. </p>
    pub fn member_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.member_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The unique identifier of the member that cast the vote. </p>
    pub fn set_member_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.member_id = input;
        self
    }
    /// Consumes the builder and constructs a [`VoteSummary`](crate::types::VoteSummary).
    pub fn build(self) -> crate::types::VoteSummary {
        crate::types::VoteSummary {
            vote: self.vote,
            member_name: self.member_name,
            member_id: self.member_id,
        }
    }
}
