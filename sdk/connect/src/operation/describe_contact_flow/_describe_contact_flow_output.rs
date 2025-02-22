// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeContactFlowOutput {
    /// <p>Information about the flow.</p>
    #[doc(hidden)]
    pub contact_flow: ::std::option::Option<crate::types::ContactFlow>,
    _request_id: Option<String>,
}
impl DescribeContactFlowOutput {
    /// <p>Information about the flow.</p>
    pub fn contact_flow(&self) -> ::std::option::Option<&crate::types::ContactFlow> {
        self.contact_flow.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeContactFlowOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeContactFlowOutput {
    /// Creates a new builder-style object to manufacture [`DescribeContactFlowOutput`](crate::operation::describe_contact_flow::DescribeContactFlowOutput).
    pub fn builder(
    ) -> crate::operation::describe_contact_flow::builders::DescribeContactFlowOutputBuilder {
        crate::operation::describe_contact_flow::builders::DescribeContactFlowOutputBuilder::default(
        )
    }
}

/// A builder for [`DescribeContactFlowOutput`](crate::operation::describe_contact_flow::DescribeContactFlowOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeContactFlowOutputBuilder {
    pub(crate) contact_flow: ::std::option::Option<crate::types::ContactFlow>,
    _request_id: Option<String>,
}
impl DescribeContactFlowOutputBuilder {
    /// <p>Information about the flow.</p>
    pub fn contact_flow(mut self, input: crate::types::ContactFlow) -> Self {
        self.contact_flow = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the flow.</p>
    pub fn set_contact_flow(
        mut self,
        input: ::std::option::Option<crate::types::ContactFlow>,
    ) -> Self {
        self.contact_flow = input;
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
    /// Consumes the builder and constructs a [`DescribeContactFlowOutput`](crate::operation::describe_contact_flow::DescribeContactFlowOutput).
    pub fn build(self) -> crate::operation::describe_contact_flow::DescribeContactFlowOutput {
        crate::operation::describe_contact_flow::DescribeContactFlowOutput {
            contact_flow: self.contact_flow,
            _request_id: self._request_id,
        }
    }
}
