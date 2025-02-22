// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAwsNetworkPerformanceDataOutput {
    /// <p>The list of data responses.</p>
    #[doc(hidden)]
    pub data_responses: ::std::option::Option<::std::vec::Vec<crate::types::DataResponse>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetAwsNetworkPerformanceDataOutput {
    /// <p>The list of data responses.</p>
    pub fn data_responses(&self) -> ::std::option::Option<&[crate::types::DataResponse]> {
        self.data_responses.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetAwsNetworkPerformanceDataOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAwsNetworkPerformanceDataOutput {
    /// Creates a new builder-style object to manufacture [`GetAwsNetworkPerformanceDataOutput`](crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataOutput).
    pub fn builder() -> crate::operation::get_aws_network_performance_data::builders::GetAwsNetworkPerformanceDataOutputBuilder{
        crate::operation::get_aws_network_performance_data::builders::GetAwsNetworkPerformanceDataOutputBuilder::default()
    }
}

/// A builder for [`GetAwsNetworkPerformanceDataOutput`](crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAwsNetworkPerformanceDataOutputBuilder {
    pub(crate) data_responses: ::std::option::Option<::std::vec::Vec<crate::types::DataResponse>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetAwsNetworkPerformanceDataOutputBuilder {
    /// Appends an item to `data_responses`.
    ///
    /// To override the contents of this collection use [`set_data_responses`](Self::set_data_responses).
    ///
    /// <p>The list of data responses.</p>
    pub fn data_responses(mut self, input: crate::types::DataResponse) -> Self {
        let mut v = self.data_responses.unwrap_or_default();
        v.push(input);
        self.data_responses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of data responses.</p>
    pub fn set_data_responses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataResponse>>,
    ) -> Self {
        self.data_responses = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
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
    /// Consumes the builder and constructs a [`GetAwsNetworkPerformanceDataOutput`](crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataOutput
    {
        crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataOutput {
            data_responses: self.data_responses,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
