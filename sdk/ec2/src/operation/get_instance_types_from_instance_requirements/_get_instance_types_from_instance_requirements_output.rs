// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetInstanceTypesFromInstanceRequirementsOutput {
    /// <p>The instance types with the specified instance attributes.</p>
    #[doc(hidden)]
    pub instance_types: ::std::option::Option<
        ::std::vec::Vec<crate::types::InstanceTypeInfoFromInstanceRequirements>,
    >,
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetInstanceTypesFromInstanceRequirementsOutput {
    /// <p>The instance types with the specified instance attributes.</p>
    pub fn instance_types(
        &self,
    ) -> ::std::option::Option<&[crate::types::InstanceTypeInfoFromInstanceRequirements]> {
        self.instance_types.as_deref()
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetInstanceTypesFromInstanceRequirementsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetInstanceTypesFromInstanceRequirementsOutput {
    /// Creates a new builder-style object to manufacture [`GetInstanceTypesFromInstanceRequirementsOutput`](crate::operation::get_instance_types_from_instance_requirements::GetInstanceTypesFromInstanceRequirementsOutput).
    pub fn builder() -> crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsOutputBuilder{
        crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsOutputBuilder::default()
    }
}

/// A builder for [`GetInstanceTypesFromInstanceRequirementsOutput`](crate::operation::get_instance_types_from_instance_requirements::GetInstanceTypesFromInstanceRequirementsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetInstanceTypesFromInstanceRequirementsOutputBuilder {
    pub(crate) instance_types: ::std::option::Option<
        ::std::vec::Vec<crate::types::InstanceTypeInfoFromInstanceRequirements>,
    >,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetInstanceTypesFromInstanceRequirementsOutputBuilder {
    /// Appends an item to `instance_types`.
    ///
    /// To override the contents of this collection use [`set_instance_types`](Self::set_instance_types).
    ///
    /// <p>The instance types with the specified instance attributes.</p>
    pub fn instance_types(
        mut self,
        input: crate::types::InstanceTypeInfoFromInstanceRequirements,
    ) -> Self {
        let mut v = self.instance_types.unwrap_or_default();
        v.push(input);
        self.instance_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>The instance types with the specified instance attributes.</p>
    pub fn set_instance_types(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::InstanceTypeInfoFromInstanceRequirements>,
        >,
    ) -> Self {
        self.instance_types = input;
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
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
    /// Consumes the builder and constructs a [`GetInstanceTypesFromInstanceRequirementsOutput`](crate::operation::get_instance_types_from_instance_requirements::GetInstanceTypesFromInstanceRequirementsOutput).
    pub fn build(self) -> crate::operation::get_instance_types_from_instance_requirements::GetInstanceTypesFromInstanceRequirementsOutput{
        crate::operation::get_instance_types_from_instance_requirements::GetInstanceTypesFromInstanceRequirementsOutput {
            instance_types: self.instance_types
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
