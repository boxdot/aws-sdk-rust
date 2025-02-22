// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the message configuration settings for a campaign.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MessageConfiguration {
    /// <p>The message that the campaign sends through the ADM (Amazon Device Messaging) channel. If specified, this message overrides the default message.</p>
    #[doc(hidden)]
    pub adm_message: ::std::option::Option<crate::types::Message>,
    /// <p>The message that the campaign sends through the APNs (Apple Push Notification service) channel. If specified, this message overrides the default message.</p>
    #[doc(hidden)]
    pub apns_message: ::std::option::Option<crate::types::Message>,
    /// <p>The message that the campaign sends through the Baidu (Baidu Cloud Push) channel. If specified, this message overrides the default message.</p>
    #[doc(hidden)]
    pub baidu_message: ::std::option::Option<crate::types::Message>,
    /// <p>The message that the campaign sends through a custom channel, as specified by the delivery configuration (CustomDeliveryConfiguration) settings for the campaign. If specified, this message overrides the default message.</p>
    #[doc(hidden)]
    pub custom_message: ::std::option::Option<crate::types::CampaignCustomMessage>,
    /// <p>The default message that the campaign sends through all the channels that are configured for the campaign.</p>
    #[doc(hidden)]
    pub default_message: ::std::option::Option<crate::types::Message>,
    /// <p>The message that the campaign sends through the email channel. If specified, this message overrides the default message.</p>
    #[doc(hidden)]
    pub email_message: ::std::option::Option<crate::types::CampaignEmailMessage>,
    /// <p>The message that the campaign sends through the GCM channel, which enables Amazon Pinpoint to send push notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service. If specified, this message overrides the default message.</p>
    #[doc(hidden)]
    pub gcm_message: ::std::option::Option<crate::types::Message>,
    /// <p>The message that the campaign sends through the SMS channel. If specified, this message overrides the default message.</p>
    #[doc(hidden)]
    pub sms_message: ::std::option::Option<crate::types::CampaignSmsMessage>,
    /// <p>The in-app message configuration.</p>
    #[doc(hidden)]
    pub in_app_message: ::std::option::Option<crate::types::CampaignInAppMessage>,
}
impl MessageConfiguration {
    /// <p>The message that the campaign sends through the ADM (Amazon Device Messaging) channel. If specified, this message overrides the default message.</p>
    pub fn adm_message(&self) -> ::std::option::Option<&crate::types::Message> {
        self.adm_message.as_ref()
    }
    /// <p>The message that the campaign sends through the APNs (Apple Push Notification service) channel. If specified, this message overrides the default message.</p>
    pub fn apns_message(&self) -> ::std::option::Option<&crate::types::Message> {
        self.apns_message.as_ref()
    }
    /// <p>The message that the campaign sends through the Baidu (Baidu Cloud Push) channel. If specified, this message overrides the default message.</p>
    pub fn baidu_message(&self) -> ::std::option::Option<&crate::types::Message> {
        self.baidu_message.as_ref()
    }
    /// <p>The message that the campaign sends through a custom channel, as specified by the delivery configuration (CustomDeliveryConfiguration) settings for the campaign. If specified, this message overrides the default message.</p>
    pub fn custom_message(&self) -> ::std::option::Option<&crate::types::CampaignCustomMessage> {
        self.custom_message.as_ref()
    }
    /// <p>The default message that the campaign sends through all the channels that are configured for the campaign.</p>
    pub fn default_message(&self) -> ::std::option::Option<&crate::types::Message> {
        self.default_message.as_ref()
    }
    /// <p>The message that the campaign sends through the email channel. If specified, this message overrides the default message.</p>
    pub fn email_message(&self) -> ::std::option::Option<&crate::types::CampaignEmailMessage> {
        self.email_message.as_ref()
    }
    /// <p>The message that the campaign sends through the GCM channel, which enables Amazon Pinpoint to send push notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service. If specified, this message overrides the default message.</p>
    pub fn gcm_message(&self) -> ::std::option::Option<&crate::types::Message> {
        self.gcm_message.as_ref()
    }
    /// <p>The message that the campaign sends through the SMS channel. If specified, this message overrides the default message.</p>
    pub fn sms_message(&self) -> ::std::option::Option<&crate::types::CampaignSmsMessage> {
        self.sms_message.as_ref()
    }
    /// <p>The in-app message configuration.</p>
    pub fn in_app_message(&self) -> ::std::option::Option<&crate::types::CampaignInAppMessage> {
        self.in_app_message.as_ref()
    }
}
impl MessageConfiguration {
    /// Creates a new builder-style object to manufacture [`MessageConfiguration`](crate::types::MessageConfiguration).
    pub fn builder() -> crate::types::builders::MessageConfigurationBuilder {
        crate::types::builders::MessageConfigurationBuilder::default()
    }
}

/// A builder for [`MessageConfiguration`](crate::types::MessageConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MessageConfigurationBuilder {
    pub(crate) adm_message: ::std::option::Option<crate::types::Message>,
    pub(crate) apns_message: ::std::option::Option<crate::types::Message>,
    pub(crate) baidu_message: ::std::option::Option<crate::types::Message>,
    pub(crate) custom_message: ::std::option::Option<crate::types::CampaignCustomMessage>,
    pub(crate) default_message: ::std::option::Option<crate::types::Message>,
    pub(crate) email_message: ::std::option::Option<crate::types::CampaignEmailMessage>,
    pub(crate) gcm_message: ::std::option::Option<crate::types::Message>,
    pub(crate) sms_message: ::std::option::Option<crate::types::CampaignSmsMessage>,
    pub(crate) in_app_message: ::std::option::Option<crate::types::CampaignInAppMessage>,
}
impl MessageConfigurationBuilder {
    /// <p>The message that the campaign sends through the ADM (Amazon Device Messaging) channel. If specified, this message overrides the default message.</p>
    pub fn adm_message(mut self, input: crate::types::Message) -> Self {
        self.adm_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The message that the campaign sends through the ADM (Amazon Device Messaging) channel. If specified, this message overrides the default message.</p>
    pub fn set_adm_message(mut self, input: ::std::option::Option<crate::types::Message>) -> Self {
        self.adm_message = input;
        self
    }
    /// <p>The message that the campaign sends through the APNs (Apple Push Notification service) channel. If specified, this message overrides the default message.</p>
    pub fn apns_message(mut self, input: crate::types::Message) -> Self {
        self.apns_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The message that the campaign sends through the APNs (Apple Push Notification service) channel. If specified, this message overrides the default message.</p>
    pub fn set_apns_message(mut self, input: ::std::option::Option<crate::types::Message>) -> Self {
        self.apns_message = input;
        self
    }
    /// <p>The message that the campaign sends through the Baidu (Baidu Cloud Push) channel. If specified, this message overrides the default message.</p>
    pub fn baidu_message(mut self, input: crate::types::Message) -> Self {
        self.baidu_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The message that the campaign sends through the Baidu (Baidu Cloud Push) channel. If specified, this message overrides the default message.</p>
    pub fn set_baidu_message(
        mut self,
        input: ::std::option::Option<crate::types::Message>,
    ) -> Self {
        self.baidu_message = input;
        self
    }
    /// <p>The message that the campaign sends through a custom channel, as specified by the delivery configuration (CustomDeliveryConfiguration) settings for the campaign. If specified, this message overrides the default message.</p>
    pub fn custom_message(mut self, input: crate::types::CampaignCustomMessage) -> Self {
        self.custom_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The message that the campaign sends through a custom channel, as specified by the delivery configuration (CustomDeliveryConfiguration) settings for the campaign. If specified, this message overrides the default message.</p>
    pub fn set_custom_message(
        mut self,
        input: ::std::option::Option<crate::types::CampaignCustomMessage>,
    ) -> Self {
        self.custom_message = input;
        self
    }
    /// <p>The default message that the campaign sends through all the channels that are configured for the campaign.</p>
    pub fn default_message(mut self, input: crate::types::Message) -> Self {
        self.default_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The default message that the campaign sends through all the channels that are configured for the campaign.</p>
    pub fn set_default_message(
        mut self,
        input: ::std::option::Option<crate::types::Message>,
    ) -> Self {
        self.default_message = input;
        self
    }
    /// <p>The message that the campaign sends through the email channel. If specified, this message overrides the default message.</p>
    pub fn email_message(mut self, input: crate::types::CampaignEmailMessage) -> Self {
        self.email_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The message that the campaign sends through the email channel. If specified, this message overrides the default message.</p>
    pub fn set_email_message(
        mut self,
        input: ::std::option::Option<crate::types::CampaignEmailMessage>,
    ) -> Self {
        self.email_message = input;
        self
    }
    /// <p>The message that the campaign sends through the GCM channel, which enables Amazon Pinpoint to send push notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service. If specified, this message overrides the default message.</p>
    pub fn gcm_message(mut self, input: crate::types::Message) -> Self {
        self.gcm_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The message that the campaign sends through the GCM channel, which enables Amazon Pinpoint to send push notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service. If specified, this message overrides the default message.</p>
    pub fn set_gcm_message(mut self, input: ::std::option::Option<crate::types::Message>) -> Self {
        self.gcm_message = input;
        self
    }
    /// <p>The message that the campaign sends through the SMS channel. If specified, this message overrides the default message.</p>
    pub fn sms_message(mut self, input: crate::types::CampaignSmsMessage) -> Self {
        self.sms_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The message that the campaign sends through the SMS channel. If specified, this message overrides the default message.</p>
    pub fn set_sms_message(
        mut self,
        input: ::std::option::Option<crate::types::CampaignSmsMessage>,
    ) -> Self {
        self.sms_message = input;
        self
    }
    /// <p>The in-app message configuration.</p>
    pub fn in_app_message(mut self, input: crate::types::CampaignInAppMessage) -> Self {
        self.in_app_message = ::std::option::Option::Some(input);
        self
    }
    /// <p>The in-app message configuration.</p>
    pub fn set_in_app_message(
        mut self,
        input: ::std::option::Option<crate::types::CampaignInAppMessage>,
    ) -> Self {
        self.in_app_message = input;
        self
    }
    /// Consumes the builder and constructs a [`MessageConfiguration`](crate::types::MessageConfiguration).
    pub fn build(self) -> crate::types::MessageConfiguration {
        crate::types::MessageConfiguration {
            adm_message: self.adm_message,
            apns_message: self.apns_message,
            baidu_message: self.baidu_message,
            custom_message: self.custom_message,
            default_message: self.default_message,
            email_message: self.email_message,
            gcm_message: self.gcm_message,
            sms_message: self.sms_message,
            in_app_message: self.in_app_message,
        }
    }
}
