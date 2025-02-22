// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetContactMethodsInput {
    /// <p>The protocols used to send notifications, such as <code>Email</code>, or <code>SMS</code> (text messaging).</p>
    /// <p>Specify a protocol in your request to return information about a specific contact method protocol.</p>
    #[doc(hidden)]
    pub protocols: ::std::option::Option<::std::vec::Vec<crate::types::ContactProtocol>>,
}
impl GetContactMethodsInput {
    /// <p>The protocols used to send notifications, such as <code>Email</code>, or <code>SMS</code> (text messaging).</p>
    /// <p>Specify a protocol in your request to return information about a specific contact method protocol.</p>
    pub fn protocols(&self) -> ::std::option::Option<&[crate::types::ContactProtocol]> {
        self.protocols.as_deref()
    }
}
impl GetContactMethodsInput {
    /// Creates a new builder-style object to manufacture [`GetContactMethodsInput`](crate::operation::get_contact_methods::GetContactMethodsInput).
    pub fn builder(
    ) -> crate::operation::get_contact_methods::builders::GetContactMethodsInputBuilder {
        crate::operation::get_contact_methods::builders::GetContactMethodsInputBuilder::default()
    }
}

/// A builder for [`GetContactMethodsInput`](crate::operation::get_contact_methods::GetContactMethodsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetContactMethodsInputBuilder {
    pub(crate) protocols: ::std::option::Option<::std::vec::Vec<crate::types::ContactProtocol>>,
}
impl GetContactMethodsInputBuilder {
    /// Appends an item to `protocols`.
    ///
    /// To override the contents of this collection use [`set_protocols`](Self::set_protocols).
    ///
    /// <p>The protocols used to send notifications, such as <code>Email</code>, or <code>SMS</code> (text messaging).</p>
    /// <p>Specify a protocol in your request to return information about a specific contact method protocol.</p>
    pub fn protocols(mut self, input: crate::types::ContactProtocol) -> Self {
        let mut v = self.protocols.unwrap_or_default();
        v.push(input);
        self.protocols = ::std::option::Option::Some(v);
        self
    }
    /// <p>The protocols used to send notifications, such as <code>Email</code>, or <code>SMS</code> (text messaging).</p>
    /// <p>Specify a protocol in your request to return information about a specific contact method protocol.</p>
    pub fn set_protocols(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ContactProtocol>>,
    ) -> Self {
        self.protocols = input;
        self
    }
    /// Consumes the builder and constructs a [`GetContactMethodsInput`](crate::operation::get_contact_methods::GetContactMethodsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_contact_methods::GetContactMethodsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_contact_methods::GetContactMethodsInput {
                protocols: self.protocols,
            },
        )
    }
}
