// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeLifecycleHookTypesOutput {
    /// <p>The lifecycle hook types.</p>
    #[doc(hidden)]
    pub lifecycle_hook_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl DescribeLifecycleHookTypesOutput {
    /// <p>The lifecycle hook types.</p>
    pub fn lifecycle_hook_types(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.lifecycle_hook_types.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeLifecycleHookTypesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeLifecycleHookTypesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeLifecycleHookTypesOutput`](crate::operation::describe_lifecycle_hook_types::DescribeLifecycleHookTypesOutput).
    pub fn builder() -> crate::operation::describe_lifecycle_hook_types::builders::DescribeLifecycleHookTypesOutputBuilder{
        crate::operation::describe_lifecycle_hook_types::builders::DescribeLifecycleHookTypesOutputBuilder::default()
    }
}

/// A builder for [`DescribeLifecycleHookTypesOutput`](crate::operation::describe_lifecycle_hook_types::DescribeLifecycleHookTypesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeLifecycleHookTypesOutputBuilder {
    pub(crate) lifecycle_hook_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl DescribeLifecycleHookTypesOutputBuilder {
    /// Appends an item to `lifecycle_hook_types`.
    ///
    /// To override the contents of this collection use [`set_lifecycle_hook_types`](Self::set_lifecycle_hook_types).
    ///
    /// <p>The lifecycle hook types.</p>
    pub fn lifecycle_hook_types(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.lifecycle_hook_types.unwrap_or_default();
        v.push(input.into());
        self.lifecycle_hook_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>The lifecycle hook types.</p>
    pub fn set_lifecycle_hook_types(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.lifecycle_hook_types = input;
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
    /// Consumes the builder and constructs a [`DescribeLifecycleHookTypesOutput`](crate::operation::describe_lifecycle_hook_types::DescribeLifecycleHookTypesOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_lifecycle_hook_types::DescribeLifecycleHookTypesOutput {
        crate::operation::describe_lifecycle_hook_types::DescribeLifecycleHookTypesOutput {
            lifecycle_hook_types: self.lifecycle_hook_types,
            _request_id: self._request_id,
        }
    }
}
