// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteChannelBan`](crate::operation::delete_channel_ban::builders::DeleteChannelBanFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_channel_ban::builders::DeleteChannelBanFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::delete_channel_ban::builders::DeleteChannelBanFluentBuilder::set_channel_arn): <p>The ARN of the channel from which the <code>AppInstanceUser</code> was banned.</p>
    ///   - [`member_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_channel_ban::builders::DeleteChannelBanFluentBuilder::member_arn) / [`set_member_arn(Option<String>)`](crate::operation::delete_channel_ban::builders::DeleteChannelBanFluentBuilder::set_member_arn): <p>The ARN of the <code>AppInstanceUser</code> that you want to reinstate.</p>
    ///   - [`chime_bearer(impl ::std::convert::Into<String>)`](crate::operation::delete_channel_ban::builders::DeleteChannelBanFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::delete_channel_ban::builders::DeleteChannelBanFluentBuilder::set_chime_bearer): <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    /// - On success, responds with [`DeleteChannelBanOutput`](crate::operation::delete_channel_ban::DeleteChannelBanOutput)
    /// - On failure, responds with [`SdkError<DeleteChannelBanError>`](crate::operation::delete_channel_ban::DeleteChannelBanError)
    pub fn delete_channel_ban(
        &self,
    ) -> crate::operation::delete_channel_ban::builders::DeleteChannelBanFluentBuilder {
        crate::operation::delete_channel_ban::builders::DeleteChannelBanFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
