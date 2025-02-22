// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_participant_token::_create_participant_token_output::CreateParticipantTokenOutputBuilder;

pub use crate::operation::create_participant_token::_create_participant_token_input::CreateParticipantTokenInputBuilder;

/// Fluent builder constructing a request to `CreateParticipantToken`.
///
/// <p>Creates an additional token for a specified stage. This can be done after stage creation or when tokens expire. Tokens always are scoped to the stage for which they are created.</p>
/// <p>Encryption keys are owned by Amazon IVS and never used directly by your application.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateParticipantTokenFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_participant_token::builders::CreateParticipantTokenInputBuilder,
}
impl CreateParticipantTokenFluentBuilder {
    /// Creates a new `CreateParticipantToken`.
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
            crate::operation::create_participant_token::CreateParticipantToken,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_participant_token::CreateParticipantTokenError,
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
        crate::operation::create_participant_token::CreateParticipantTokenOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_participant_token::CreateParticipantTokenError,
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
        crate::operation::create_participant_token::CreateParticipantTokenOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_participant_token::CreateParticipantTokenError,
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
            crate::operation::create_participant_token::CreateParticipantToken,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_participant_token::CreateParticipantTokenError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>ARN of the stage to which this token is scoped.</p>
    pub fn stage_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stage_arn(input.into());
        self
    }
    /// <p>ARN of the stage to which this token is scoped.</p>
    pub fn set_stage_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stage_arn(input);
        self
    }
    /// <p>Duration (in minutes), after which the token expires. Default: 720 (12 hours).</p>
    pub fn duration(mut self, input: i32) -> Self {
        self.inner = self.inner.duration(input);
        self
    }
    /// <p>Duration (in minutes), after which the token expires. Default: 720 (12 hours).</p>
    pub fn set_duration(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_duration(input);
        self
    }
    /// <p>Name that can be specified to help identify the token. This can be any UTF-8 encoded text. <i>This field is exposed to all stage participants and should not be used for personally identifying, confidential, or sensitive information.</i> </p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>Name that can be specified to help identify the token. This can be any UTF-8 encoded text. <i>This field is exposed to all stage participants and should not be used for personally identifying, confidential, or sensitive information.</i> </p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// Adds a key-value pair to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>Application-provided attributes to encode into the token and attach to a stage. Map keys and values can contain UTF-8 encoded text. The maximum length of this field is 1 KB total. <i>This field is exposed to all stage participants and should not be used for personally identifying, confidential, or sensitive information.</i> </p>
    pub fn attributes(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.attributes(k.into(), v.into());
        self
    }
    /// <p>Application-provided attributes to encode into the token and attach to a stage. Map keys and values can contain UTF-8 encoded text. The maximum length of this field is 1 KB total. <i>This field is exposed to all stage participants and should not be used for personally identifying, confidential, or sensitive information.</i> </p>
    pub fn set_attributes(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_attributes(input);
        self
    }
    /// Appends an item to `capabilities`.
    ///
    /// To override the contents of this collection use [`set_capabilities`](Self::set_capabilities).
    ///
    /// <p>Set of capabilities that the user is allowed to perform in the stage. Default: <code>PUBLISH, SUBSCRIBE</code>.</p>
    pub fn capabilities(mut self, input: crate::types::ParticipantTokenCapability) -> Self {
        self.inner = self.inner.capabilities(input);
        self
    }
    /// <p>Set of capabilities that the user is allowed to perform in the stage. Default: <code>PUBLISH, SUBSCRIBE</code>.</p>
    pub fn set_capabilities(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ParticipantTokenCapability>>,
    ) -> Self {
        self.inner = self.inner.set_capabilities(input);
        self
    }
}
