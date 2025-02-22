// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AcceptMatchInput {
    /// <p>A unique identifier for a matchmaking ticket. The ticket must be in status <code>REQUIRES_ACCEPTANCE</code>; otherwise this request will fail.</p>
    #[doc(hidden)]
    pub ticket_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for a player delivering the response. This parameter can include one or multiple player IDs.</p>
    #[doc(hidden)]
    pub player_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Player response to the proposed match.</p>
    #[doc(hidden)]
    pub acceptance_type: ::std::option::Option<crate::types::AcceptanceType>,
}
impl AcceptMatchInput {
    /// <p>A unique identifier for a matchmaking ticket. The ticket must be in status <code>REQUIRES_ACCEPTANCE</code>; otherwise this request will fail.</p>
    pub fn ticket_id(&self) -> ::std::option::Option<&str> {
        self.ticket_id.as_deref()
    }
    /// <p>A unique identifier for a player delivering the response. This parameter can include one or multiple player IDs.</p>
    pub fn player_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.player_ids.as_deref()
    }
    /// <p>Player response to the proposed match.</p>
    pub fn acceptance_type(&self) -> ::std::option::Option<&crate::types::AcceptanceType> {
        self.acceptance_type.as_ref()
    }
}
impl AcceptMatchInput {
    /// Creates a new builder-style object to manufacture [`AcceptMatchInput`](crate::operation::accept_match::AcceptMatchInput).
    pub fn builder() -> crate::operation::accept_match::builders::AcceptMatchInputBuilder {
        crate::operation::accept_match::builders::AcceptMatchInputBuilder::default()
    }
}

/// A builder for [`AcceptMatchInput`](crate::operation::accept_match::AcceptMatchInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AcceptMatchInputBuilder {
    pub(crate) ticket_id: ::std::option::Option<::std::string::String>,
    pub(crate) player_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) acceptance_type: ::std::option::Option<crate::types::AcceptanceType>,
}
impl AcceptMatchInputBuilder {
    /// <p>A unique identifier for a matchmaking ticket. The ticket must be in status <code>REQUIRES_ACCEPTANCE</code>; otherwise this request will fail.</p>
    pub fn ticket_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ticket_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for a matchmaking ticket. The ticket must be in status <code>REQUIRES_ACCEPTANCE</code>; otherwise this request will fail.</p>
    pub fn set_ticket_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ticket_id = input;
        self
    }
    /// Appends an item to `player_ids`.
    ///
    /// To override the contents of this collection use [`set_player_ids`](Self::set_player_ids).
    ///
    /// <p>A unique identifier for a player delivering the response. This parameter can include one or multiple player IDs.</p>
    pub fn player_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.player_ids.unwrap_or_default();
        v.push(input.into());
        self.player_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A unique identifier for a player delivering the response. This parameter can include one or multiple player IDs.</p>
    pub fn set_player_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.player_ids = input;
        self
    }
    /// <p>Player response to the proposed match.</p>
    pub fn acceptance_type(mut self, input: crate::types::AcceptanceType) -> Self {
        self.acceptance_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Player response to the proposed match.</p>
    pub fn set_acceptance_type(
        mut self,
        input: ::std::option::Option<crate::types::AcceptanceType>,
    ) -> Self {
        self.acceptance_type = input;
        self
    }
    /// Consumes the builder and constructs a [`AcceptMatchInput`](crate::operation::accept_match::AcceptMatchInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::accept_match::AcceptMatchInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::accept_match::AcceptMatchInput {
            ticket_id: self.ticket_id,
            player_ids: self.player_ids,
            acceptance_type: self.acceptance_type,
        })
    }
}
