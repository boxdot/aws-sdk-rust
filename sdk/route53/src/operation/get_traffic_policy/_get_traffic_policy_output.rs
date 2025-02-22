// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the response information for the request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTrafficPolicyOutput {
    /// <p>A complex type that contains settings for the specified traffic policy.</p>
    #[doc(hidden)]
    pub traffic_policy: ::std::option::Option<crate::types::TrafficPolicy>,
    _request_id: Option<String>,
}
impl GetTrafficPolicyOutput {
    /// <p>A complex type that contains settings for the specified traffic policy.</p>
    pub fn traffic_policy(&self) -> ::std::option::Option<&crate::types::TrafficPolicy> {
        self.traffic_policy.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetTrafficPolicyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetTrafficPolicyOutput {
    /// Creates a new builder-style object to manufacture [`GetTrafficPolicyOutput`](crate::operation::get_traffic_policy::GetTrafficPolicyOutput).
    pub fn builder() -> crate::operation::get_traffic_policy::builders::GetTrafficPolicyOutputBuilder
    {
        crate::operation::get_traffic_policy::builders::GetTrafficPolicyOutputBuilder::default()
    }
}

/// A builder for [`GetTrafficPolicyOutput`](crate::operation::get_traffic_policy::GetTrafficPolicyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetTrafficPolicyOutputBuilder {
    pub(crate) traffic_policy: ::std::option::Option<crate::types::TrafficPolicy>,
    _request_id: Option<String>,
}
impl GetTrafficPolicyOutputBuilder {
    /// <p>A complex type that contains settings for the specified traffic policy.</p>
    pub fn traffic_policy(mut self, input: crate::types::TrafficPolicy) -> Self {
        self.traffic_policy = ::std::option::Option::Some(input);
        self
    }
    /// <p>A complex type that contains settings for the specified traffic policy.</p>
    pub fn set_traffic_policy(
        mut self,
        input: ::std::option::Option<crate::types::TrafficPolicy>,
    ) -> Self {
        self.traffic_policy = input;
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
    /// Consumes the builder and constructs a [`GetTrafficPolicyOutput`](crate::operation::get_traffic_policy::GetTrafficPolicyOutput).
    pub fn build(self) -> crate::operation::get_traffic_policy::GetTrafficPolicyOutput {
        crate::operation::get_traffic_policy::GetTrafficPolicyOutput {
            traffic_policy: self.traffic_policy,
            _request_id: self._request_id,
        }
    }
}
