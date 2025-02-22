// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a queue resource for which metrics are returned.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct QueueReference {
    /// <p>The identifier of the queue.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the queue.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
}
impl QueueReference {
    /// <p>The identifier of the queue.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the queue.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl QueueReference {
    /// Creates a new builder-style object to manufacture [`QueueReference`](crate::types::QueueReference).
    pub fn builder() -> crate::types::builders::QueueReferenceBuilder {
        crate::types::builders::QueueReferenceBuilder::default()
    }
}

/// A builder for [`QueueReference`](crate::types::QueueReference).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct QueueReferenceBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
}
impl QueueReferenceBuilder {
    /// <p>The identifier of the queue.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the queue.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the queue.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the queue.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Consumes the builder and constructs a [`QueueReference`](crate::types::QueueReference).
    pub fn build(self) -> crate::types::QueueReference {
        crate::types::QueueReference {
            id: self.id,
            arn: self.arn,
        }
    }
}
