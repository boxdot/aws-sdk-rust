// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The root cause of a trace summary error.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ErrorRootCause {
    /// <p>A list of services corresponding to an error. A service identifies a segment and it contains a name, account ID, type, and inferred flag.</p>
    #[doc(hidden)]
    pub services: ::std::option::Option<::std::vec::Vec<crate::types::ErrorRootCauseService>>,
    /// <p>A flag that denotes that the root cause impacts the trace client.</p>
    #[doc(hidden)]
    pub client_impacting: ::std::option::Option<bool>,
}
impl ErrorRootCause {
    /// <p>A list of services corresponding to an error. A service identifies a segment and it contains a name, account ID, type, and inferred flag.</p>
    pub fn services(&self) -> ::std::option::Option<&[crate::types::ErrorRootCauseService]> {
        self.services.as_deref()
    }
    /// <p>A flag that denotes that the root cause impacts the trace client.</p>
    pub fn client_impacting(&self) -> ::std::option::Option<bool> {
        self.client_impacting
    }
}
impl ErrorRootCause {
    /// Creates a new builder-style object to manufacture [`ErrorRootCause`](crate::types::ErrorRootCause).
    pub fn builder() -> crate::types::builders::ErrorRootCauseBuilder {
        crate::types::builders::ErrorRootCauseBuilder::default()
    }
}

/// A builder for [`ErrorRootCause`](crate::types::ErrorRootCause).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ErrorRootCauseBuilder {
    pub(crate) services:
        ::std::option::Option<::std::vec::Vec<crate::types::ErrorRootCauseService>>,
    pub(crate) client_impacting: ::std::option::Option<bool>,
}
impl ErrorRootCauseBuilder {
    /// Appends an item to `services`.
    ///
    /// To override the contents of this collection use [`set_services`](Self::set_services).
    ///
    /// <p>A list of services corresponding to an error. A service identifies a segment and it contains a name, account ID, type, and inferred flag.</p>
    pub fn services(mut self, input: crate::types::ErrorRootCauseService) -> Self {
        let mut v = self.services.unwrap_or_default();
        v.push(input);
        self.services = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of services corresponding to an error. A service identifies a segment and it contains a name, account ID, type, and inferred flag.</p>
    pub fn set_services(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ErrorRootCauseService>>,
    ) -> Self {
        self.services = input;
        self
    }
    /// <p>A flag that denotes that the root cause impacts the trace client.</p>
    pub fn client_impacting(mut self, input: bool) -> Self {
        self.client_impacting = ::std::option::Option::Some(input);
        self
    }
    /// <p>A flag that denotes that the root cause impacts the trace client.</p>
    pub fn set_client_impacting(mut self, input: ::std::option::Option<bool>) -> Self {
        self.client_impacting = input;
        self
    }
    /// Consumes the builder and constructs a [`ErrorRootCause`](crate::types::ErrorRootCause).
    pub fn build(self) -> crate::types::ErrorRootCause {
        crate::types::ErrorRootCause {
            services: self.services,
            client_impacting: self.client_impacting,
        }
    }
}
