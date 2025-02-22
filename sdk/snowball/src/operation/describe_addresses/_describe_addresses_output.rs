// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAddressesOutput {
    /// <p>The Snow device shipping addresses that were created for this account.</p>
    #[doc(hidden)]
    pub addresses: ::std::option::Option<::std::vec::Vec<crate::types::Address>>,
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>DescribeAddresses</code> call, your list of returned addresses will start from this point in the array.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeAddressesOutput {
    /// <p>The Snow device shipping addresses that were created for this account.</p>
    pub fn addresses(&self) -> ::std::option::Option<&[crate::types::Address]> {
        self.addresses.as_deref()
    }
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>DescribeAddresses</code> call, your list of returned addresses will start from this point in the array.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeAddressesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeAddressesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAddressesOutput`](crate::operation::describe_addresses::DescribeAddressesOutput).
    pub fn builder(
    ) -> crate::operation::describe_addresses::builders::DescribeAddressesOutputBuilder {
        crate::operation::describe_addresses::builders::DescribeAddressesOutputBuilder::default()
    }
}

/// A builder for [`DescribeAddressesOutput`](crate::operation::describe_addresses::DescribeAddressesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeAddressesOutputBuilder {
    pub(crate) addresses: ::std::option::Option<::std::vec::Vec<crate::types::Address>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeAddressesOutputBuilder {
    /// Appends an item to `addresses`.
    ///
    /// To override the contents of this collection use [`set_addresses`](Self::set_addresses).
    ///
    /// <p>The Snow device shipping addresses that were created for this account.</p>
    pub fn addresses(mut self, input: crate::types::Address) -> Self {
        let mut v = self.addresses.unwrap_or_default();
        v.push(input);
        self.addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Snow device shipping addresses that were created for this account.</p>
    pub fn set_addresses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Address>>,
    ) -> Self {
        self.addresses = input;
        self
    }
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>DescribeAddresses</code> call, your list of returned addresses will start from this point in the array.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>DescribeAddresses</code> call, your list of returned addresses will start from this point in the array.</p>
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
    /// Consumes the builder and constructs a [`DescribeAddressesOutput`](crate::operation::describe_addresses::DescribeAddressesOutput).
    pub fn build(self) -> crate::operation::describe_addresses::DescribeAddressesOutput {
        crate::operation::describe_addresses::DescribeAddressesOutput {
            addresses: self.addresses,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
