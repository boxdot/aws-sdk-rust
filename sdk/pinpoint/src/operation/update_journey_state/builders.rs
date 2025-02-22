// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_journey_state::_update_journey_state_output::UpdateJourneyStateOutputBuilder;

pub use crate::operation::update_journey_state::_update_journey_state_input::UpdateJourneyStateInputBuilder;

/// Fluent builder constructing a request to `UpdateJourneyState`.
///
/// <p>Cancels (stops) an active journey.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateJourneyStateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_journey_state::builders::UpdateJourneyStateInputBuilder,
}
impl UpdateJourneyStateFluentBuilder {
    /// Creates a new `UpdateJourneyState`.
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
            crate::operation::update_journey_state::UpdateJourneyState,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_journey_state::UpdateJourneyStateError,
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
        crate::operation::update_journey_state::UpdateJourneyStateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_journey_state::UpdateJourneyStateError,
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
        crate::operation::update_journey_state::UpdateJourneyStateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_journey_state::UpdateJourneyStateError,
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
            crate::operation::update_journey_state::UpdateJourneyState,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_journey_state::UpdateJourneyStateError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The unique identifier for the journey.</p>
    pub fn journey_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.journey_id(input.into());
        self
    }
    /// <p>The unique identifier for the journey.</p>
    pub fn set_journey_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_journey_id(input);
        self
    }
    /// <p>Changes the status of a journey.</p>
    pub fn journey_state_request(mut self, input: crate::types::JourneyStateRequest) -> Self {
        self.inner = self.inner.journey_state_request(input);
        self
    }
    /// <p>Changes the status of a journey.</p>
    pub fn set_journey_state_request(
        mut self,
        input: ::std::option::Option<crate::types::JourneyStateRequest>,
    ) -> Self {
        self.inner = self.inner.set_journey_state_request(input);
        self
    }
}
