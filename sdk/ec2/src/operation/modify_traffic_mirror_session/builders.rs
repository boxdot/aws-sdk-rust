// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_traffic_mirror_session::_modify_traffic_mirror_session_output::ModifyTrafficMirrorSessionOutputBuilder;

pub use crate::operation::modify_traffic_mirror_session::_modify_traffic_mirror_session_input::ModifyTrafficMirrorSessionInputBuilder;

/// Fluent builder constructing a request to `ModifyTrafficMirrorSession`.
///
/// <p>Modifies a Traffic Mirror session.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyTrafficMirrorSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::modify_traffic_mirror_session::builders::ModifyTrafficMirrorSessionInputBuilder,
}
impl ModifyTrafficMirrorSessionFluentBuilder {
    /// Creates a new `ModifyTrafficMirrorSession`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::modify_traffic_mirror_session::ModifyTrafficMirrorSession,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_traffic_mirror_session::ModifyTrafficMirrorSessionError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_traffic_mirror_session::ModifyTrafficMirrorSessionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_traffic_mirror_session::ModifyTrafficMirrorSessionError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_traffic_mirror_session::ModifyTrafficMirrorSessionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_traffic_mirror_session::ModifyTrafficMirrorSessionError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::modify_traffic_mirror_session::ModifyTrafficMirrorSession,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_traffic_mirror_session::ModifyTrafficMirrorSessionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the Traffic Mirror session.</p>
    pub fn traffic_mirror_session_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.traffic_mirror_session_id(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror session.</p>
    pub fn set_traffic_mirror_session_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_traffic_mirror_session_id(input);
        self
    }
    /// <p>The Traffic Mirror target. The target must be in the same VPC as the source, or have a VPC peering connection with the source.</p>
    pub fn traffic_mirror_target_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.traffic_mirror_target_id(input.into());
        self
    }
    /// <p>The Traffic Mirror target. The target must be in the same VPC as the source, or have a VPC peering connection with the source.</p>
    pub fn set_traffic_mirror_target_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_traffic_mirror_target_id(input);
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn traffic_mirror_filter_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.traffic_mirror_filter_id(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn set_traffic_mirror_filter_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_traffic_mirror_filter_id(input);
        self
    }
    /// <p>The number of bytes in each packet to mirror. These are bytes after the VXLAN header. To mirror a subset, set this to the length (in bytes) to mirror. For example, if you set this value to 100, then the first 100 bytes that meet the filter criteria are copied to the target. Do not specify this parameter when you want to mirror the entire packet.</p>
    pub fn packet_length(mut self, input: i32) -> Self {
        self.inner = self.inner.packet_length(input);
        self
    }
    /// <p>The number of bytes in each packet to mirror. These are bytes after the VXLAN header. To mirror a subset, set this to the length (in bytes) to mirror. For example, if you set this value to 100, then the first 100 bytes that meet the filter criteria are copied to the target. Do not specify this parameter when you want to mirror the entire packet.</p>
    pub fn set_packet_length(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_packet_length(input);
        self
    }
    /// <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
    /// <p>Valid values are 1-32766.</p>
    pub fn session_number(mut self, input: i32) -> Self {
        self.inner = self.inner.session_number(input);
        self
    }
    /// <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
    /// <p>Valid values are 1-32766.</p>
    pub fn set_session_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_session_number(input);
        self
    }
    /// <p>The virtual network ID of the Traffic Mirror session.</p>
    pub fn virtual_network_id(mut self, input: i32) -> Self {
        self.inner = self.inner.virtual_network_id(input);
        self
    }
    /// <p>The virtual network ID of the Traffic Mirror session.</p>
    pub fn set_virtual_network_id(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_virtual_network_id(input);
        self
    }
    /// <p>The description to assign to the Traffic Mirror session.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description to assign to the Traffic Mirror session.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `RemoveFields`.
    ///
    /// To override the contents of this collection use [`set_remove_fields`](Self::set_remove_fields).
    ///
    /// <p>The properties that you want to remove from the Traffic Mirror session.</p>
    /// <p>When you remove a property from a Traffic Mirror session, the property is set to the default.</p>
    pub fn remove_fields(mut self, input: crate::types::TrafficMirrorSessionField) -> Self {
        self.inner = self.inner.remove_fields(input);
        self
    }
    /// <p>The properties that you want to remove from the Traffic Mirror session.</p>
    /// <p>When you remove a property from a Traffic Mirror session, the property is set to the default.</p>
    pub fn set_remove_fields(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorSessionField>>,
    ) -> Self {
        self.inner = self.inner.set_remove_fields(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
