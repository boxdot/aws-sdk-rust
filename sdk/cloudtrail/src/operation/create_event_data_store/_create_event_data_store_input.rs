// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateEventDataStoreInput {
    /// <p>The name of the event data store.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The advanced event selectors to use to select the events for the data store. You can configure up to five advanced event selectors for each event data store.</p>
    /// <p> For more information about how to use advanced event selectors to log CloudTrail events, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#creating-data-event-selectors-advanced">Log events by using advanced event selectors</a> in the CloudTrail User Guide.</p>
    /// <p>For more information about how to use advanced event selectors to include Config configuration items in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-lake-cli.html#lake-cli-create-eds-config">Create an event data store for Config configuration items</a> in the CloudTrail User Guide.</p>
    /// <p>For more information about how to use advanced event selectors to include non-Amazon Web Services events in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-lake-cli.html#lake-cli-create-integration">Create an integration to log events from outside Amazon Web Services</a> in the CloudTrail User Guide.</p>
    #[doc(hidden)]
    pub advanced_event_selectors:
        ::std::option::Option<::std::vec::Vec<crate::types::AdvancedEventSelector>>,
    /// <p>Specifies whether the event data store includes events from all regions, or only from the region in which the event data store is created.</p>
    #[doc(hidden)]
    pub multi_region_enabled: ::std::option::Option<bool>,
    /// <p>Specifies whether an event data store collects events logged for an organization in Organizations.</p>
    #[doc(hidden)]
    pub organization_enabled: ::std::option::Option<bool>,
    /// <p>The retention period of the event data store, in days. You can set a retention period of up to 2557 days, the equivalent of seven years.</p>
    #[doc(hidden)]
    pub retention_period: ::std::option::Option<i32>,
    /// <p>Specifies whether termination protection is enabled for the event data store. If termination protection is enabled, you cannot delete the event data store until termination protection is disabled.</p>
    #[doc(hidden)]
    pub termination_protection_enabled: ::std::option::Option<bool>,
    /// <p>A list of tags.</p>
    #[doc(hidden)]
    pub tags_list: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>Specifies the KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by <code>alias/</code>, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <important>
    /// <p>Disabling or deleting the KMS key, or removing CloudTrail permissions on the key, prevents CloudTrail from logging events to the event data store, and prevents users from querying the data in the event data store that was encrypted with the key. After you associate an event data store with a KMS key, the KMS key cannot be removed or changed. Before you disable or delete a KMS key that you are using with an event data store, delete or back up your event data store.</p>
    /// </important>
    /// <p>CloudTrail also supports KMS multi-Region keys. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>Examples:</p>
    /// <ul>
    /// <li> <p> <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p> <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub kms_key_id: ::std::option::Option<::std::string::String>,
}
impl CreateEventDataStoreInput {
    /// <p>The name of the event data store.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The advanced event selectors to use to select the events for the data store. You can configure up to five advanced event selectors for each event data store.</p>
    /// <p> For more information about how to use advanced event selectors to log CloudTrail events, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#creating-data-event-selectors-advanced">Log events by using advanced event selectors</a> in the CloudTrail User Guide.</p>
    /// <p>For more information about how to use advanced event selectors to include Config configuration items in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-lake-cli.html#lake-cli-create-eds-config">Create an event data store for Config configuration items</a> in the CloudTrail User Guide.</p>
    /// <p>For more information about how to use advanced event selectors to include non-Amazon Web Services events in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-lake-cli.html#lake-cli-create-integration">Create an integration to log events from outside Amazon Web Services</a> in the CloudTrail User Guide.</p>
    pub fn advanced_event_selectors(
        &self,
    ) -> ::std::option::Option<&[crate::types::AdvancedEventSelector]> {
        self.advanced_event_selectors.as_deref()
    }
    /// <p>Specifies whether the event data store includes events from all regions, or only from the region in which the event data store is created.</p>
    pub fn multi_region_enabled(&self) -> ::std::option::Option<bool> {
        self.multi_region_enabled
    }
    /// <p>Specifies whether an event data store collects events logged for an organization in Organizations.</p>
    pub fn organization_enabled(&self) -> ::std::option::Option<bool> {
        self.organization_enabled
    }
    /// <p>The retention period of the event data store, in days. You can set a retention period of up to 2557 days, the equivalent of seven years.</p>
    pub fn retention_period(&self) -> ::std::option::Option<i32> {
        self.retention_period
    }
    /// <p>Specifies whether termination protection is enabled for the event data store. If termination protection is enabled, you cannot delete the event data store until termination protection is disabled.</p>
    pub fn termination_protection_enabled(&self) -> ::std::option::Option<bool> {
        self.termination_protection_enabled
    }
    /// <p>A list of tags.</p>
    pub fn tags_list(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags_list.as_deref()
    }
    /// <p>Specifies the KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by <code>alias/</code>, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <important>
    /// <p>Disabling or deleting the KMS key, or removing CloudTrail permissions on the key, prevents CloudTrail from logging events to the event data store, and prevents users from querying the data in the event data store that was encrypted with the key. After you associate an event data store with a KMS key, the KMS key cannot be removed or changed. Before you disable or delete a KMS key that you are using with an event data store, delete or back up your event data store.</p>
    /// </important>
    /// <p>CloudTrail also supports KMS multi-Region keys. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>Examples:</p>
    /// <ul>
    /// <li> <p> <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p> <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// </ul>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
}
impl CreateEventDataStoreInput {
    /// Creates a new builder-style object to manufacture [`CreateEventDataStoreInput`](crate::operation::create_event_data_store::CreateEventDataStoreInput).
    pub fn builder(
    ) -> crate::operation::create_event_data_store::builders::CreateEventDataStoreInputBuilder {
        crate::operation::create_event_data_store::builders::CreateEventDataStoreInputBuilder::default()
    }
}

/// A builder for [`CreateEventDataStoreInput`](crate::operation::create_event_data_store::CreateEventDataStoreInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateEventDataStoreInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) advanced_event_selectors:
        ::std::option::Option<::std::vec::Vec<crate::types::AdvancedEventSelector>>,
    pub(crate) multi_region_enabled: ::std::option::Option<bool>,
    pub(crate) organization_enabled: ::std::option::Option<bool>,
    pub(crate) retention_period: ::std::option::Option<i32>,
    pub(crate) termination_protection_enabled: ::std::option::Option<bool>,
    pub(crate) tags_list: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
}
impl CreateEventDataStoreInputBuilder {
    /// <p>The name of the event data store.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the event data store.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `advanced_event_selectors`.
    ///
    /// To override the contents of this collection use [`set_advanced_event_selectors`](Self::set_advanced_event_selectors).
    ///
    /// <p>The advanced event selectors to use to select the events for the data store. You can configure up to five advanced event selectors for each event data store.</p>
    /// <p> For more information about how to use advanced event selectors to log CloudTrail events, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#creating-data-event-selectors-advanced">Log events by using advanced event selectors</a> in the CloudTrail User Guide.</p>
    /// <p>For more information about how to use advanced event selectors to include Config configuration items in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-lake-cli.html#lake-cli-create-eds-config">Create an event data store for Config configuration items</a> in the CloudTrail User Guide.</p>
    /// <p>For more information about how to use advanced event selectors to include non-Amazon Web Services events in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-lake-cli.html#lake-cli-create-integration">Create an integration to log events from outside Amazon Web Services</a> in the CloudTrail User Guide.</p>
    pub fn advanced_event_selectors(mut self, input: crate::types::AdvancedEventSelector) -> Self {
        let mut v = self.advanced_event_selectors.unwrap_or_default();
        v.push(input);
        self.advanced_event_selectors = ::std::option::Option::Some(v);
        self
    }
    /// <p>The advanced event selectors to use to select the events for the data store. You can configure up to five advanced event selectors for each event data store.</p>
    /// <p> For more information about how to use advanced event selectors to log CloudTrail events, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#creating-data-event-selectors-advanced">Log events by using advanced event selectors</a> in the CloudTrail User Guide.</p>
    /// <p>For more information about how to use advanced event selectors to include Config configuration items in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-lake-cli.html#lake-cli-create-eds-config">Create an event data store for Config configuration items</a> in the CloudTrail User Guide.</p>
    /// <p>For more information about how to use advanced event selectors to include non-Amazon Web Services events in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-lake-cli.html#lake-cli-create-integration">Create an integration to log events from outside Amazon Web Services</a> in the CloudTrail User Guide.</p>
    pub fn set_advanced_event_selectors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AdvancedEventSelector>>,
    ) -> Self {
        self.advanced_event_selectors = input;
        self
    }
    /// <p>Specifies whether the event data store includes events from all regions, or only from the region in which the event data store is created.</p>
    pub fn multi_region_enabled(mut self, input: bool) -> Self {
        self.multi_region_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the event data store includes events from all regions, or only from the region in which the event data store is created.</p>
    pub fn set_multi_region_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.multi_region_enabled = input;
        self
    }
    /// <p>Specifies whether an event data store collects events logged for an organization in Organizations.</p>
    pub fn organization_enabled(mut self, input: bool) -> Self {
        self.organization_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether an event data store collects events logged for an organization in Organizations.</p>
    pub fn set_organization_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.organization_enabled = input;
        self
    }
    /// <p>The retention period of the event data store, in days. You can set a retention period of up to 2557 days, the equivalent of seven years.</p>
    pub fn retention_period(mut self, input: i32) -> Self {
        self.retention_period = ::std::option::Option::Some(input);
        self
    }
    /// <p>The retention period of the event data store, in days. You can set a retention period of up to 2557 days, the equivalent of seven years.</p>
    pub fn set_retention_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.retention_period = input;
        self
    }
    /// <p>Specifies whether termination protection is enabled for the event data store. If termination protection is enabled, you cannot delete the event data store until termination protection is disabled.</p>
    pub fn termination_protection_enabled(mut self, input: bool) -> Self {
        self.termination_protection_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether termination protection is enabled for the event data store. If termination protection is enabled, you cannot delete the event data store until termination protection is disabled.</p>
    pub fn set_termination_protection_enabled(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.termination_protection_enabled = input;
        self
    }
    /// Appends an item to `tags_list`.
    ///
    /// To override the contents of this collection use [`set_tags_list`](Self::set_tags_list).
    ///
    /// <p>A list of tags.</p>
    pub fn tags_list(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags_list.unwrap_or_default();
        v.push(input);
        self.tags_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of tags.</p>
    pub fn set_tags_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags_list = input;
        self
    }
    /// <p>Specifies the KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by <code>alias/</code>, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <important>
    /// <p>Disabling or deleting the KMS key, or removing CloudTrail permissions on the key, prevents CloudTrail from logging events to the event data store, and prevents users from querying the data in the event data store that was encrypted with the key. After you associate an event data store with a KMS key, the KMS key cannot be removed or changed. Before you disable or delete a KMS key that you are using with an event data store, delete or back up your event data store.</p>
    /// </important>
    /// <p>CloudTrail also supports KMS multi-Region keys. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>Examples:</p>
    /// <ul>
    /// <li> <p> <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p> <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// </ul>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by <code>alias/</code>, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <important>
    /// <p>Disabling or deleting the KMS key, or removing CloudTrail permissions on the key, prevents CloudTrail from logging events to the event data store, and prevents users from querying the data in the event data store that was encrypted with the key. After you associate an event data store with a KMS key, the KMS key cannot be removed or changed. Before you disable or delete a KMS key that you are using with an event data store, delete or back up your event data store.</p>
    /// </important>
    /// <p>CloudTrail also supports KMS multi-Region keys. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>Examples:</p>
    /// <ul>
    /// <li> <p> <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p> <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// </ul>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateEventDataStoreInput`](crate::operation::create_event_data_store::CreateEventDataStoreInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_event_data_store::CreateEventDataStoreInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_event_data_store::CreateEventDataStoreInput {
                name: self.name,
                advanced_event_selectors: self.advanced_event_selectors,
                multi_region_enabled: self.multi_region_enabled,
                organization_enabled: self.organization_enabled,
                retention_period: self.retention_period,
                termination_protection_enabled: self.termination_protection_enabled,
                tags_list: self.tags_list,
                kms_key_id: self.kms_key_id,
            },
        )
    }
}
