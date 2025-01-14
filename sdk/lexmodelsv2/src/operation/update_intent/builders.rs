// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_intent::_update_intent_output::UpdateIntentOutputBuilder;

pub use crate::operation::update_intent::_update_intent_input::UpdateIntentInputBuilder;

/// Fluent builder constructing a request to `UpdateIntent`.
///
/// <p>Updates the settings for an intent.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateIntentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_intent::builders::UpdateIntentInputBuilder,
}
impl UpdateIntentFluentBuilder {
    /// Creates a new `UpdateIntent`.
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
            crate::operation::update_intent::UpdateIntent,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_intent::UpdateIntentError>,
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
        crate::operation::update_intent::UpdateIntentOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_intent::UpdateIntentError>,
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
        crate::operation::update_intent::UpdateIntentOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_intent::UpdateIntentError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_intent::UpdateIntent,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_intent::UpdateIntentError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier of the intent to update.</p>
    pub fn intent_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.intent_id(input.into());
        self
    }
    /// <p>The unique identifier of the intent to update.</p>
    pub fn set_intent_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_intent_id(input);
        self
    }
    /// <p>The new name for the intent.</p>
    pub fn intent_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.intent_name(input.into());
        self
    }
    /// <p>The new name for the intent.</p>
    pub fn set_intent_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_intent_name(input);
        self
    }
    /// <p>The new description of the intent.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The new description of the intent.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The signature of the new built-in intent to use as the parent of this intent.</p>
    pub fn parent_intent_signature(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.parent_intent_signature(input.into());
        self
    }
    /// <p>The signature of the new built-in intent to use as the parent of this intent.</p>
    pub fn set_parent_intent_signature(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_parent_intent_signature(input);
        self
    }
    /// Appends an item to `sampleUtterances`.
    ///
    /// To override the contents of this collection use [`set_sample_utterances`](Self::set_sample_utterances).
    ///
    /// <p>New utterances used to invoke the intent.</p>
    pub fn sample_utterances(mut self, input: crate::types::SampleUtterance) -> Self {
        self.inner = self.inner.sample_utterances(input);
        self
    }
    /// <p>New utterances used to invoke the intent.</p>
    pub fn set_sample_utterances(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SampleUtterance>>,
    ) -> Self {
        self.inner = self.inner.set_sample_utterances(input);
        self
    }
    /// <p>The new Lambda function to use between each turn of the conversation with the bot.</p>
    pub fn dialog_code_hook(mut self, input: crate::types::DialogCodeHookSettings) -> Self {
        self.inner = self.inner.dialog_code_hook(input);
        self
    }
    /// <p>The new Lambda function to use between each turn of the conversation with the bot.</p>
    pub fn set_dialog_code_hook(
        mut self,
        input: ::std::option::Option<crate::types::DialogCodeHookSettings>,
    ) -> Self {
        self.inner = self.inner.set_dialog_code_hook(input);
        self
    }
    /// <p>The new Lambda function to call when all of the intents required slots are provided and the intent is ready for fulfillment.</p>
    pub fn fulfillment_code_hook(
        mut self,
        input: crate::types::FulfillmentCodeHookSettings,
    ) -> Self {
        self.inner = self.inner.fulfillment_code_hook(input);
        self
    }
    /// <p>The new Lambda function to call when all of the intents required slots are provided and the intent is ready for fulfillment.</p>
    pub fn set_fulfillment_code_hook(
        mut self,
        input: ::std::option::Option<crate::types::FulfillmentCodeHookSettings>,
    ) -> Self {
        self.inner = self.inner.set_fulfillment_code_hook(input);
        self
    }
    /// Appends an item to `slotPriorities`.
    ///
    /// To override the contents of this collection use [`set_slot_priorities`](Self::set_slot_priorities).
    ///
    /// <p>A new list of slots and their priorities that are contained by the intent.</p>
    pub fn slot_priorities(mut self, input: crate::types::SlotPriority) -> Self {
        self.inner = self.inner.slot_priorities(input);
        self
    }
    /// <p>A new list of slots and their priorities that are contained by the intent.</p>
    pub fn set_slot_priorities(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SlotPriority>>,
    ) -> Self {
        self.inner = self.inner.set_slot_priorities(input);
        self
    }
    /// <p>New prompts that Amazon Lex sends to the user to confirm the completion of an intent.</p>
    pub fn intent_confirmation_setting(
        mut self,
        input: crate::types::IntentConfirmationSetting,
    ) -> Self {
        self.inner = self.inner.intent_confirmation_setting(input);
        self
    }
    /// <p>New prompts that Amazon Lex sends to the user to confirm the completion of an intent.</p>
    pub fn set_intent_confirmation_setting(
        mut self,
        input: ::std::option::Option<crate::types::IntentConfirmationSetting>,
    ) -> Self {
        self.inner = self.inner.set_intent_confirmation_setting(input);
        self
    }
    /// <p>The new response that Amazon Lex sends the user when the intent is closed.</p>
    pub fn intent_closing_setting(mut self, input: crate::types::IntentClosingSetting) -> Self {
        self.inner = self.inner.intent_closing_setting(input);
        self
    }
    /// <p>The new response that Amazon Lex sends the user when the intent is closed.</p>
    pub fn set_intent_closing_setting(
        mut self,
        input: ::std::option::Option<crate::types::IntentClosingSetting>,
    ) -> Self {
        self.inner = self.inner.set_intent_closing_setting(input);
        self
    }
    /// Appends an item to `inputContexts`.
    ///
    /// To override the contents of this collection use [`set_input_contexts`](Self::set_input_contexts).
    ///
    /// <p>A new list of contexts that must be active in order for Amazon Lex to consider the intent.</p>
    pub fn input_contexts(mut self, input: crate::types::InputContext) -> Self {
        self.inner = self.inner.input_contexts(input);
        self
    }
    /// <p>A new list of contexts that must be active in order for Amazon Lex to consider the intent.</p>
    pub fn set_input_contexts(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::InputContext>>,
    ) -> Self {
        self.inner = self.inner.set_input_contexts(input);
        self
    }
    /// Appends an item to `outputContexts`.
    ///
    /// To override the contents of this collection use [`set_output_contexts`](Self::set_output_contexts).
    ///
    /// <p>A new list of contexts that Amazon Lex activates when the intent is fulfilled.</p>
    pub fn output_contexts(mut self, input: crate::types::OutputContext) -> Self {
        self.inner = self.inner.output_contexts(input);
        self
    }
    /// <p>A new list of contexts that Amazon Lex activates when the intent is fulfilled.</p>
    pub fn set_output_contexts(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OutputContext>>,
    ) -> Self {
        self.inner = self.inner.set_output_contexts(input);
        self
    }
    /// <p>New configuration settings for connecting to an Amazon Kendra index.</p>
    pub fn kendra_configuration(mut self, input: crate::types::KendraConfiguration) -> Self {
        self.inner = self.inner.kendra_configuration(input);
        self
    }
    /// <p>New configuration settings for connecting to an Amazon Kendra index.</p>
    pub fn set_kendra_configuration(
        mut self,
        input: ::std::option::Option<crate::types::KendraConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_kendra_configuration(input);
        self
    }
    /// <p>The identifier of the bot that contains the intent.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The identifier of the bot that contains the intent.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The version of the bot that contains the intent. Must be <code>DRAFT</code>.</p>
    pub fn bot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_version(input.into());
        self
    }
    /// <p>The version of the bot that contains the intent. Must be <code>DRAFT</code>.</p>
    pub fn set_bot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_version(input);
        self
    }
    /// <p>The identifier of the language and locale where this intent is used. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The identifier of the language and locale where this intent is used. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p></p>
    pub fn initial_response_setting(mut self, input: crate::types::InitialResponseSetting) -> Self {
        self.inner = self.inner.initial_response_setting(input);
        self
    }
    /// <p></p>
    pub fn set_initial_response_setting(
        mut self,
        input: ::std::option::Option<crate::types::InitialResponseSetting>,
    ) -> Self {
        self.inner = self.inner.set_initial_response_setting(input);
        self
    }
}
