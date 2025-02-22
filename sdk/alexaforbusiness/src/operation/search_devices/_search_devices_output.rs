// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SearchDevicesOutput {
    /// <p>The devices that meet the specified set of filter criteria, in sort order.</p>
    #[doc(hidden)]
    pub devices: ::std::option::Option<::std::vec::Vec<crate::types::DeviceData>>,
    /// <p>The token returned to indicate that there is more data available.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The total number of devices returned.</p>
    #[doc(hidden)]
    pub total_count: ::std::option::Option<i32>,
    _request_id: Option<String>,
}
impl SearchDevicesOutput {
    /// <p>The devices that meet the specified set of filter criteria, in sort order.</p>
    pub fn devices(&self) -> ::std::option::Option<&[crate::types::DeviceData]> {
        self.devices.as_deref()
    }
    /// <p>The token returned to indicate that there is more data available.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The total number of devices returned.</p>
    pub fn total_count(&self) -> ::std::option::Option<i32> {
        self.total_count
    }
}
impl ::aws_http::request_id::RequestId for SearchDevicesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SearchDevicesOutput {
    /// Creates a new builder-style object to manufacture [`SearchDevicesOutput`](crate::operation::search_devices::SearchDevicesOutput).
    pub fn builder() -> crate::operation::search_devices::builders::SearchDevicesOutputBuilder {
        crate::operation::search_devices::builders::SearchDevicesOutputBuilder::default()
    }
}

/// A builder for [`SearchDevicesOutput`](crate::operation::search_devices::SearchDevicesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SearchDevicesOutputBuilder {
    pub(crate) devices: ::std::option::Option<::std::vec::Vec<crate::types::DeviceData>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) total_count: ::std::option::Option<i32>,
    _request_id: Option<String>,
}
impl SearchDevicesOutputBuilder {
    /// Appends an item to `devices`.
    ///
    /// To override the contents of this collection use [`set_devices`](Self::set_devices).
    ///
    /// <p>The devices that meet the specified set of filter criteria, in sort order.</p>
    pub fn devices(mut self, input: crate::types::DeviceData) -> Self {
        let mut v = self.devices.unwrap_or_default();
        v.push(input);
        self.devices = ::std::option::Option::Some(v);
        self
    }
    /// <p>The devices that meet the specified set of filter criteria, in sort order.</p>
    pub fn set_devices(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DeviceData>>,
    ) -> Self {
        self.devices = input;
        self
    }
    /// <p>The token returned to indicate that there is more data available.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token returned to indicate that there is more data available.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The total number of devices returned.</p>
    pub fn total_count(mut self, input: i32) -> Self {
        self.total_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of devices returned.</p>
    pub fn set_total_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.total_count = input;
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
    /// Consumes the builder and constructs a [`SearchDevicesOutput`](crate::operation::search_devices::SearchDevicesOutput).
    pub fn build(self) -> crate::operation::search_devices::SearchDevicesOutput {
        crate::operation::search_devices::SearchDevicesOutput {
            devices: self.devices,
            next_token: self.next_token,
            total_count: self.total_count,
            _request_id: self._request_id,
        }
    }
}
