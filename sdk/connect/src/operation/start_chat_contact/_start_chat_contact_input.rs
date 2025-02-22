// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartChatContactInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the flow for initiating the chat. To see the ContactFlowId in the Amazon Connect console user interface, on the navigation menu go to <b>Routing</b>, <b>Contact Flows</b>. Choose the flow. On the flow page, under the name of the flow, choose <b>Show additional flow information</b>. The ContactFlowId is the last part of the ARN, shown here in bold: </p>
    /// <p>arn:aws:connect:us-west-2:xxxxxxxxxxxx:instance/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/contact-flow/<b>846ec553-a005-41c0-8341-xxxxxxxxxxxx</b> </p>
    #[doc(hidden)]
    pub contact_flow_id: ::std::option::Option<::std::string::String>,
    /// <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes. They can be accessed in flows just like any other contact attributes. </p>
    /// <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    #[doc(hidden)]
    pub attributes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>Information identifying the participant.</p>
    #[doc(hidden)]
    pub participant_details: ::std::option::Option<crate::types::ParticipantDetails>,
    /// <p>The initial message to be sent to the newly created chat.</p>
    #[doc(hidden)]
    pub initial_message: ::std::option::Option<crate::types::ChatMessage>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The total duration of the newly started chat session. If not specified, the chat session duration defaults to 25 hour. The minimum configurable time is 60 minutes. The maximum configurable time is 10,080 minutes (7 days).</p>
    #[doc(hidden)]
    pub chat_duration_in_minutes: ::std::option::Option<i32>,
    /// <p>The supported chat message content types. Supported types are <code>text/plain</code>, <code>text/markdown</code>, <code>application/json</code>, <code>application/vnd.amazonaws.connect.message.interactive</code>, and <code>application/vnd.amazonaws.connect.message.interactive.response</code>. </p>
    /// <p>Content types must always contain <code>text/plain</code>. You can then put any other supported type in the list. For example, all the following lists are valid because they contain <code>text/plain</code>: <code>[text/plain, text/markdown, application/json]</code>, <code>[text/markdown, text/plain]</code>, <code>[text/plain, application/json, application/vnd.amazonaws.connect.message.interactive.response]</code>. </p> <note>
    /// <p>The type <code>application/vnd.amazonaws.connect.message.interactive</code> is required to use the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/show-view-block.html">Show view</a> flow block.</p>
    /// </note>
    #[doc(hidden)]
    pub supported_messaging_content_types:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Enable persistent chats. For more information about enabling persistent chat, and for example use cases and how to configure for them, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/chat-persistence.html">Enable persistent chat</a>.</p>
    #[doc(hidden)]
    pub persistent_chat: ::std::option::Option<crate::types::PersistentChat>,
    /// <p>The unique identifier for an Amazon Connect contact. This identifier is related to the chat starting.</p> <note>
    /// <p>You cannot provide data for both RelatedContactId and PersistentChat. </p>
    /// </note>
    #[doc(hidden)]
    pub related_contact_id: ::std::option::Option<::std::string::String>,
}
impl StartChatContactInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The identifier of the flow for initiating the chat. To see the ContactFlowId in the Amazon Connect console user interface, on the navigation menu go to <b>Routing</b>, <b>Contact Flows</b>. Choose the flow. On the flow page, under the name of the flow, choose <b>Show additional flow information</b>. The ContactFlowId is the last part of the ARN, shown here in bold: </p>
    /// <p>arn:aws:connect:us-west-2:xxxxxxxxxxxx:instance/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/contact-flow/<b>846ec553-a005-41c0-8341-xxxxxxxxxxxx</b> </p>
    pub fn contact_flow_id(&self) -> ::std::option::Option<&str> {
        self.contact_flow_id.as_deref()
    }
    /// <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes. They can be accessed in flows just like any other contact attributes. </p>
    /// <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    pub fn attributes(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.attributes.as_ref()
    }
    /// <p>Information identifying the participant.</p>
    pub fn participant_details(&self) -> ::std::option::Option<&crate::types::ParticipantDetails> {
        self.participant_details.as_ref()
    }
    /// <p>The initial message to be sent to the newly created chat.</p>
    pub fn initial_message(&self) -> ::std::option::Option<&crate::types::ChatMessage> {
        self.initial_message.as_ref()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The total duration of the newly started chat session. If not specified, the chat session duration defaults to 25 hour. The minimum configurable time is 60 minutes. The maximum configurable time is 10,080 minutes (7 days).</p>
    pub fn chat_duration_in_minutes(&self) -> ::std::option::Option<i32> {
        self.chat_duration_in_minutes
    }
    /// <p>The supported chat message content types. Supported types are <code>text/plain</code>, <code>text/markdown</code>, <code>application/json</code>, <code>application/vnd.amazonaws.connect.message.interactive</code>, and <code>application/vnd.amazonaws.connect.message.interactive.response</code>. </p>
    /// <p>Content types must always contain <code>text/plain</code>. You can then put any other supported type in the list. For example, all the following lists are valid because they contain <code>text/plain</code>: <code>[text/plain, text/markdown, application/json]</code>, <code>[text/markdown, text/plain]</code>, <code>[text/plain, application/json, application/vnd.amazonaws.connect.message.interactive.response]</code>. </p> <note>
    /// <p>The type <code>application/vnd.amazonaws.connect.message.interactive</code> is required to use the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/show-view-block.html">Show view</a> flow block.</p>
    /// </note>
    pub fn supported_messaging_content_types(
        &self,
    ) -> ::std::option::Option<&[::std::string::String]> {
        self.supported_messaging_content_types.as_deref()
    }
    /// <p>Enable persistent chats. For more information about enabling persistent chat, and for example use cases and how to configure for them, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/chat-persistence.html">Enable persistent chat</a>.</p>
    pub fn persistent_chat(&self) -> ::std::option::Option<&crate::types::PersistentChat> {
        self.persistent_chat.as_ref()
    }
    /// <p>The unique identifier for an Amazon Connect contact. This identifier is related to the chat starting.</p> <note>
    /// <p>You cannot provide data for both RelatedContactId and PersistentChat. </p>
    /// </note>
    pub fn related_contact_id(&self) -> ::std::option::Option<&str> {
        self.related_contact_id.as_deref()
    }
}
impl StartChatContactInput {
    /// Creates a new builder-style object to manufacture [`StartChatContactInput`](crate::operation::start_chat_contact::StartChatContactInput).
    pub fn builder() -> crate::operation::start_chat_contact::builders::StartChatContactInputBuilder
    {
        crate::operation::start_chat_contact::builders::StartChatContactInputBuilder::default()
    }
}

/// A builder for [`StartChatContactInput`](crate::operation::start_chat_contact::StartChatContactInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartChatContactInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) contact_flow_id: ::std::option::Option<::std::string::String>,
    pub(crate) attributes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) participant_details: ::std::option::Option<crate::types::ParticipantDetails>,
    pub(crate) initial_message: ::std::option::Option<crate::types::ChatMessage>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) chat_duration_in_minutes: ::std::option::Option<i32>,
    pub(crate) supported_messaging_content_types:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) persistent_chat: ::std::option::Option<crate::types::PersistentChat>,
    pub(crate) related_contact_id: ::std::option::Option<::std::string::String>,
}
impl StartChatContactInputBuilder {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The identifier of the flow for initiating the chat. To see the ContactFlowId in the Amazon Connect console user interface, on the navigation menu go to <b>Routing</b>, <b>Contact Flows</b>. Choose the flow. On the flow page, under the name of the flow, choose <b>Show additional flow information</b>. The ContactFlowId is the last part of the ARN, shown here in bold: </p>
    /// <p>arn:aws:connect:us-west-2:xxxxxxxxxxxx:instance/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/contact-flow/<b>846ec553-a005-41c0-8341-xxxxxxxxxxxx</b> </p>
    pub fn contact_flow_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.contact_flow_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the flow for initiating the chat. To see the ContactFlowId in the Amazon Connect console user interface, on the navigation menu go to <b>Routing</b>, <b>Contact Flows</b>. Choose the flow. On the flow page, under the name of the flow, choose <b>Show additional flow information</b>. The ContactFlowId is the last part of the ARN, shown here in bold: </p>
    /// <p>arn:aws:connect:us-west-2:xxxxxxxxxxxx:instance/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/contact-flow/<b>846ec553-a005-41c0-8341-xxxxxxxxxxxx</b> </p>
    pub fn set_contact_flow_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.contact_flow_id = input;
        self
    }
    /// Adds a key-value pair to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes. They can be accessed in flows just like any other contact attributes. </p>
    /// <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    pub fn attributes(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.attributes.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes. They can be accessed in flows just like any other contact attributes. </p>
    /// <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    pub fn set_attributes(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.attributes = input;
        self
    }
    /// <p>Information identifying the participant.</p>
    pub fn participant_details(mut self, input: crate::types::ParticipantDetails) -> Self {
        self.participant_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information identifying the participant.</p>
    pub fn set_participant_details(
        mut self,
        input: ::std::option::Option<crate::types::ParticipantDetails>,
    ) -> Self {
        self.participant_details = input;
        self
    }
    /// <p>The initial message to be sent to the newly created chat.</p>
    pub fn initial_message(mut self, input: crate::types::ChatMessage) -> Self {
        self.initial_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The initial message to be sent to the newly created chat.</p>
    pub fn set_initial_message(
        mut self,
        input: ::std::option::Option<crate::types::ChatMessage>,
    ) -> Self {
        self.initial_message = input;
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The total duration of the newly started chat session. If not specified, the chat session duration defaults to 25 hour. The minimum configurable time is 60 minutes. The maximum configurable time is 10,080 minutes (7 days).</p>
    pub fn chat_duration_in_minutes(mut self, input: i32) -> Self {
        self.chat_duration_in_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total duration of the newly started chat session. If not specified, the chat session duration defaults to 25 hour. The minimum configurable time is 60 minutes. The maximum configurable time is 10,080 minutes (7 days).</p>
    pub fn set_chat_duration_in_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.chat_duration_in_minutes = input;
        self
    }
    /// Appends an item to `supported_messaging_content_types`.
    ///
    /// To override the contents of this collection use [`set_supported_messaging_content_types`](Self::set_supported_messaging_content_types).
    ///
    /// <p>The supported chat message content types. Supported types are <code>text/plain</code>, <code>text/markdown</code>, <code>application/json</code>, <code>application/vnd.amazonaws.connect.message.interactive</code>, and <code>application/vnd.amazonaws.connect.message.interactive.response</code>. </p>
    /// <p>Content types must always contain <code>text/plain</code>. You can then put any other supported type in the list. For example, all the following lists are valid because they contain <code>text/plain</code>: <code>[text/plain, text/markdown, application/json]</code>, <code>[text/markdown, text/plain]</code>, <code>[text/plain, application/json, application/vnd.amazonaws.connect.message.interactive.response]</code>. </p> <note>
    /// <p>The type <code>application/vnd.amazonaws.connect.message.interactive</code> is required to use the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/show-view-block.html">Show view</a> flow block.</p>
    /// </note>
    pub fn supported_messaging_content_types(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.supported_messaging_content_types.unwrap_or_default();
        v.push(input.into());
        self.supported_messaging_content_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>The supported chat message content types. Supported types are <code>text/plain</code>, <code>text/markdown</code>, <code>application/json</code>, <code>application/vnd.amazonaws.connect.message.interactive</code>, and <code>application/vnd.amazonaws.connect.message.interactive.response</code>. </p>
    /// <p>Content types must always contain <code>text/plain</code>. You can then put any other supported type in the list. For example, all the following lists are valid because they contain <code>text/plain</code>: <code>[text/plain, text/markdown, application/json]</code>, <code>[text/markdown, text/plain]</code>, <code>[text/plain, application/json, application/vnd.amazonaws.connect.message.interactive.response]</code>. </p> <note>
    /// <p>The type <code>application/vnd.amazonaws.connect.message.interactive</code> is required to use the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/show-view-block.html">Show view</a> flow block.</p>
    /// </note>
    pub fn set_supported_messaging_content_types(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.supported_messaging_content_types = input;
        self
    }
    /// <p>Enable persistent chats. For more information about enabling persistent chat, and for example use cases and how to configure for them, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/chat-persistence.html">Enable persistent chat</a>.</p>
    pub fn persistent_chat(mut self, input: crate::types::PersistentChat) -> Self {
        self.persistent_chat = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable persistent chats. For more information about enabling persistent chat, and for example use cases and how to configure for them, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/chat-persistence.html">Enable persistent chat</a>.</p>
    pub fn set_persistent_chat(
        mut self,
        input: ::std::option::Option<crate::types::PersistentChat>,
    ) -> Self {
        self.persistent_chat = input;
        self
    }
    /// <p>The unique identifier for an Amazon Connect contact. This identifier is related to the chat starting.</p> <note>
    /// <p>You cannot provide data for both RelatedContactId and PersistentChat. </p>
    /// </note>
    pub fn related_contact_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.related_contact_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for an Amazon Connect contact. This identifier is related to the chat starting.</p> <note>
    /// <p>You cannot provide data for both RelatedContactId and PersistentChat. </p>
    /// </note>
    pub fn set_related_contact_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.related_contact_id = input;
        self
    }
    /// Consumes the builder and constructs a [`StartChatContactInput`](crate::operation::start_chat_contact::StartChatContactInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_chat_contact::StartChatContactInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::start_chat_contact::StartChatContactInput {
                instance_id: self.instance_id,
                contact_flow_id: self.contact_flow_id,
                attributes: self.attributes,
                participant_details: self.participant_details,
                initial_message: self.initial_message,
                client_token: self.client_token,
                chat_duration_in_minutes: self.chat_duration_in_minutes,
                supported_messaging_content_types: self.supported_messaging_content_types,
                persistent_chat: self.persistent_chat,
                related_contact_id: self.related_contact_id,
            },
        )
    }
}
