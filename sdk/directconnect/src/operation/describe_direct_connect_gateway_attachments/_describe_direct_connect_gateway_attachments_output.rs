// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeDirectConnectGatewayAttachmentsOutput {
    /// <p>The attachments.</p>
    #[doc(hidden)]
    pub direct_connect_gateway_attachments:
        ::std::option::Option<::std::vec::Vec<crate::types::DirectConnectGatewayAttachment>>,
    /// <p>The token to retrieve the next page.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeDirectConnectGatewayAttachmentsOutput {
    /// <p>The attachments.</p>
    pub fn direct_connect_gateway_attachments(
        &self,
    ) -> ::std::option::Option<&[crate::types::DirectConnectGatewayAttachment]> {
        self.direct_connect_gateway_attachments.as_deref()
    }
    /// <p>The token to retrieve the next page.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeDirectConnectGatewayAttachmentsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeDirectConnectGatewayAttachmentsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeDirectConnectGatewayAttachmentsOutput`](crate::operation::describe_direct_connect_gateway_attachments::DescribeDirectConnectGatewayAttachmentsOutput).
    pub fn builder() -> crate::operation::describe_direct_connect_gateway_attachments::builders::DescribeDirectConnectGatewayAttachmentsOutputBuilder{
        crate::operation::describe_direct_connect_gateway_attachments::builders::DescribeDirectConnectGatewayAttachmentsOutputBuilder::default()
    }
}

/// A builder for [`DescribeDirectConnectGatewayAttachmentsOutput`](crate::operation::describe_direct_connect_gateway_attachments::DescribeDirectConnectGatewayAttachmentsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeDirectConnectGatewayAttachmentsOutputBuilder {
    pub(crate) direct_connect_gateway_attachments:
        ::std::option::Option<::std::vec::Vec<crate::types::DirectConnectGatewayAttachment>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeDirectConnectGatewayAttachmentsOutputBuilder {
    /// Appends an item to `direct_connect_gateway_attachments`.
    ///
    /// To override the contents of this collection use [`set_direct_connect_gateway_attachments`](Self::set_direct_connect_gateway_attachments).
    ///
    /// <p>The attachments.</p>
    pub fn direct_connect_gateway_attachments(
        mut self,
        input: crate::types::DirectConnectGatewayAttachment,
    ) -> Self {
        let mut v = self.direct_connect_gateway_attachments.unwrap_or_default();
        v.push(input);
        self.direct_connect_gateway_attachments = ::std::option::Option::Some(v);
        self
    }
    /// <p>The attachments.</p>
    pub fn set_direct_connect_gateway_attachments(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DirectConnectGatewayAttachment>>,
    ) -> Self {
        self.direct_connect_gateway_attachments = input;
        self
    }
    /// <p>The token to retrieve the next page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to retrieve the next page.</p>
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
    /// Consumes the builder and constructs a [`DescribeDirectConnectGatewayAttachmentsOutput`](crate::operation::describe_direct_connect_gateway_attachments::DescribeDirectConnectGatewayAttachmentsOutput).
    pub fn build(self) -> crate::operation::describe_direct_connect_gateway_attachments::DescribeDirectConnectGatewayAttachmentsOutput{
        crate::operation::describe_direct_connect_gateway_attachments::DescribeDirectConnectGatewayAttachmentsOutput {
            direct_connect_gateway_attachments: self.direct_connect_gateway_attachments
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
