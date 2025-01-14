// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetRoomInput {
    /// <p>The Amazon Chime account ID.</p>
    #[doc(hidden)]
    pub account_id: ::std::option::Option<::std::string::String>,
    /// <p>The room ID.</p>
    #[doc(hidden)]
    pub room_id: ::std::option::Option<::std::string::String>,
}
impl GetRoomInput {
    /// <p>The Amazon Chime account ID.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The room ID.</p>
    pub fn room_id(&self) -> ::std::option::Option<&str> {
        self.room_id.as_deref()
    }
}
impl GetRoomInput {
    /// Creates a new builder-style object to manufacture [`GetRoomInput`](crate::operation::get_room::GetRoomInput).
    pub fn builder() -> crate::operation::get_room::builders::GetRoomInputBuilder {
        crate::operation::get_room::builders::GetRoomInputBuilder::default()
    }
}

/// A builder for [`GetRoomInput`](crate::operation::get_room::GetRoomInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetRoomInputBuilder {
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
    pub(crate) room_id: ::std::option::Option<::std::string::String>,
}
impl GetRoomInputBuilder {
    /// <p>The Amazon Chime account ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Chime account ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The room ID.</p>
    pub fn room_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.room_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The room ID.</p>
    pub fn set_room_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.room_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetRoomInput`](crate::operation::get_room::GetRoomInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_room::GetRoomInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_room::GetRoomInput {
            account_id: self.account_id,
            room_id: self.room_id,
        })
    }
}
