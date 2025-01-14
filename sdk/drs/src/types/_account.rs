// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>AWS account.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Account {
    /// <p>Account ID of AWS account.</p>
    #[doc(hidden)]
    pub account_id: ::std::option::Option<::std::string::String>,
}
impl Account {
    /// <p>Account ID of AWS account.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
}
impl Account {
    /// Creates a new builder-style object to manufacture [`Account`](crate::types::Account).
    pub fn builder() -> crate::types::builders::AccountBuilder {
        crate::types::builders::AccountBuilder::default()
    }
}

/// A builder for [`Account`](crate::types::Account).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AccountBuilder {
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
}
impl AccountBuilder {
    /// <p>Account ID of AWS account.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Account ID of AWS account.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// Consumes the builder and constructs a [`Account`](crate::types::Account).
    pub fn build(self) -> crate::types::Account {
        crate::types::Account {
            account_id: self.account_id,
        }
    }
}
