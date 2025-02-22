// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListServiceVersionsOutput {
    /// <p>A list of supported versions.</p>
    #[doc(hidden)]
    pub service_versions: ::std::option::Option<::std::vec::Vec<crate::types::ServiceVersion>>,
    /// <p>The name of the service for which the system provided supported versions.</p>
    #[doc(hidden)]
    pub service_name: ::std::option::Option<crate::types::ServiceName>,
    /// <p>A list of names and versions of dependant services of the service for which the system provided supported versions.</p>
    #[doc(hidden)]
    pub dependent_services: ::std::option::Option<::std::vec::Vec<crate::types::DependentService>>,
    /// <p>Because HTTP requests are stateless, this is the starting point of the next list of returned <code>ListServiceVersionsResult</code> results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListServiceVersionsOutput {
    /// <p>A list of supported versions.</p>
    pub fn service_versions(&self) -> ::std::option::Option<&[crate::types::ServiceVersion]> {
        self.service_versions.as_deref()
    }
    /// <p>The name of the service for which the system provided supported versions.</p>
    pub fn service_name(&self) -> ::std::option::Option<&crate::types::ServiceName> {
        self.service_name.as_ref()
    }
    /// <p>A list of names and versions of dependant services of the service for which the system provided supported versions.</p>
    pub fn dependent_services(&self) -> ::std::option::Option<&[crate::types::DependentService]> {
        self.dependent_services.as_deref()
    }
    /// <p>Because HTTP requests are stateless, this is the starting point of the next list of returned <code>ListServiceVersionsResult</code> results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListServiceVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListServiceVersionsOutput {
    /// Creates a new builder-style object to manufacture [`ListServiceVersionsOutput`](crate::operation::list_service_versions::ListServiceVersionsOutput).
    pub fn builder(
    ) -> crate::operation::list_service_versions::builders::ListServiceVersionsOutputBuilder {
        crate::operation::list_service_versions::builders::ListServiceVersionsOutputBuilder::default(
        )
    }
}

/// A builder for [`ListServiceVersionsOutput`](crate::operation::list_service_versions::ListServiceVersionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListServiceVersionsOutputBuilder {
    pub(crate) service_versions:
        ::std::option::Option<::std::vec::Vec<crate::types::ServiceVersion>>,
    pub(crate) service_name: ::std::option::Option<crate::types::ServiceName>,
    pub(crate) dependent_services:
        ::std::option::Option<::std::vec::Vec<crate::types::DependentService>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListServiceVersionsOutputBuilder {
    /// Appends an item to `service_versions`.
    ///
    /// To override the contents of this collection use [`set_service_versions`](Self::set_service_versions).
    ///
    /// <p>A list of supported versions.</p>
    pub fn service_versions(mut self, input: crate::types::ServiceVersion) -> Self {
        let mut v = self.service_versions.unwrap_or_default();
        v.push(input);
        self.service_versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of supported versions.</p>
    pub fn set_service_versions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ServiceVersion>>,
    ) -> Self {
        self.service_versions = input;
        self
    }
    /// <p>The name of the service for which the system provided supported versions.</p>
    pub fn service_name(mut self, input: crate::types::ServiceName) -> Self {
        self.service_name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the service for which the system provided supported versions.</p>
    pub fn set_service_name(
        mut self,
        input: ::std::option::Option<crate::types::ServiceName>,
    ) -> Self {
        self.service_name = input;
        self
    }
    /// Appends an item to `dependent_services`.
    ///
    /// To override the contents of this collection use [`set_dependent_services`](Self::set_dependent_services).
    ///
    /// <p>A list of names and versions of dependant services of the service for which the system provided supported versions.</p>
    pub fn dependent_services(mut self, input: crate::types::DependentService) -> Self {
        let mut v = self.dependent_services.unwrap_or_default();
        v.push(input);
        self.dependent_services = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of names and versions of dependant services of the service for which the system provided supported versions.</p>
    pub fn set_dependent_services(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DependentService>>,
    ) -> Self {
        self.dependent_services = input;
        self
    }
    /// <p>Because HTTP requests are stateless, this is the starting point of the next list of returned <code>ListServiceVersionsResult</code> results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Because HTTP requests are stateless, this is the starting point of the next list of returned <code>ListServiceVersionsResult</code> results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListServiceVersionsOutput`](crate::operation::list_service_versions::ListServiceVersionsOutput).
    pub fn build(self) -> crate::operation::list_service_versions::ListServiceVersionsOutput {
        crate::operation::list_service_versions::ListServiceVersionsOutput {
            service_versions: self.service_versions,
            service_name: self.service_name,
            dependent_services: self.dependent_services,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
