// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListVpcConnectorsOutput {
    /// <p>A list of information records for VPC connectors. In a paginated request, the request returns up to <code>MaxResults</code> records for each call.</p>
    #[doc(hidden)]
    pub vpc_connectors: ::std::option::Option<::std::vec::Vec<crate::types::VpcConnector>>,
    /// <p>The token that you can pass in a subsequent request to get the next result page. It's returned in a paginated request.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListVpcConnectorsOutput {
    /// <p>A list of information records for VPC connectors. In a paginated request, the request returns up to <code>MaxResults</code> records for each call.</p>
    pub fn vpc_connectors(&self) -> ::std::option::Option<&[crate::types::VpcConnector]> {
        self.vpc_connectors.as_deref()
    }
    /// <p>The token that you can pass in a subsequent request to get the next result page. It's returned in a paginated request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListVpcConnectorsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListVpcConnectorsOutput {
    /// Creates a new builder-style object to manufacture [`ListVpcConnectorsOutput`](crate::operation::list_vpc_connectors::ListVpcConnectorsOutput).
    pub fn builder(
    ) -> crate::operation::list_vpc_connectors::builders::ListVpcConnectorsOutputBuilder {
        crate::operation::list_vpc_connectors::builders::ListVpcConnectorsOutputBuilder::default()
    }
}

/// A builder for [`ListVpcConnectorsOutput`](crate::operation::list_vpc_connectors::ListVpcConnectorsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListVpcConnectorsOutputBuilder {
    pub(crate) vpc_connectors: ::std::option::Option<::std::vec::Vec<crate::types::VpcConnector>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListVpcConnectorsOutputBuilder {
    /// Appends an item to `vpc_connectors`.
    ///
    /// To override the contents of this collection use [`set_vpc_connectors`](Self::set_vpc_connectors).
    ///
    /// <p>A list of information records for VPC connectors. In a paginated request, the request returns up to <code>MaxResults</code> records for each call.</p>
    pub fn vpc_connectors(mut self, input: crate::types::VpcConnector) -> Self {
        let mut v = self.vpc_connectors.unwrap_or_default();
        v.push(input);
        self.vpc_connectors = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of information records for VPC connectors. In a paginated request, the request returns up to <code>MaxResults</code> records for each call.</p>
    pub fn set_vpc_connectors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::VpcConnector>>,
    ) -> Self {
        self.vpc_connectors = input;
        self
    }
    /// <p>The token that you can pass in a subsequent request to get the next result page. It's returned in a paginated request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token that you can pass in a subsequent request to get the next result page. It's returned in a paginated request.</p>
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
    /// Consumes the builder and constructs a [`ListVpcConnectorsOutput`](crate::operation::list_vpc_connectors::ListVpcConnectorsOutput).
    pub fn build(self) -> crate::operation::list_vpc_connectors::ListVpcConnectorsOutput {
        crate::operation::list_vpc_connectors::ListVpcConnectorsOutput {
            vpc_connectors: self.vpc_connectors,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
