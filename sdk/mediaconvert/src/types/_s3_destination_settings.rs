// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Settings associated with S3 destination
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3DestinationSettings {
    /// Optional. Have MediaConvert automatically apply Amazon S3 access control for the outputs in this output group. When you don't use this setting, S3 automatically applies the default access control list PRIVATE.
    #[doc(hidden)]
    pub access_control: ::std::option::Option<crate::types::S3DestinationAccessControl>,
    /// Settings for how your job outputs are encrypted as they are uploaded to Amazon S3.
    #[doc(hidden)]
    pub encryption: ::std::option::Option<crate::types::S3EncryptionSettings>,
}
impl S3DestinationSettings {
    /// Optional. Have MediaConvert automatically apply Amazon S3 access control for the outputs in this output group. When you don't use this setting, S3 automatically applies the default access control list PRIVATE.
    pub fn access_control(
        &self,
    ) -> ::std::option::Option<&crate::types::S3DestinationAccessControl> {
        self.access_control.as_ref()
    }
    /// Settings for how your job outputs are encrypted as they are uploaded to Amazon S3.
    pub fn encryption(&self) -> ::std::option::Option<&crate::types::S3EncryptionSettings> {
        self.encryption.as_ref()
    }
}
impl S3DestinationSettings {
    /// Creates a new builder-style object to manufacture [`S3DestinationSettings`](crate::types::S3DestinationSettings).
    pub fn builder() -> crate::types::builders::S3DestinationSettingsBuilder {
        crate::types::builders::S3DestinationSettingsBuilder::default()
    }
}

/// A builder for [`S3DestinationSettings`](crate::types::S3DestinationSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct S3DestinationSettingsBuilder {
    pub(crate) access_control: ::std::option::Option<crate::types::S3DestinationAccessControl>,
    pub(crate) encryption: ::std::option::Option<crate::types::S3EncryptionSettings>,
}
impl S3DestinationSettingsBuilder {
    /// Optional. Have MediaConvert automatically apply Amazon S3 access control for the outputs in this output group. When you don't use this setting, S3 automatically applies the default access control list PRIVATE.
    pub fn access_control(mut self, input: crate::types::S3DestinationAccessControl) -> Self {
        self.access_control = ::std::option::Option::Some(input);
        self
    }
    /// Optional. Have MediaConvert automatically apply Amazon S3 access control for the outputs in this output group. When you don't use this setting, S3 automatically applies the default access control list PRIVATE.
    pub fn set_access_control(
        mut self,
        input: ::std::option::Option<crate::types::S3DestinationAccessControl>,
    ) -> Self {
        self.access_control = input;
        self
    }
    /// Settings for how your job outputs are encrypted as they are uploaded to Amazon S3.
    pub fn encryption(mut self, input: crate::types::S3EncryptionSettings) -> Self {
        self.encryption = ::std::option::Option::Some(input);
        self
    }
    /// Settings for how your job outputs are encrypted as they are uploaded to Amazon S3.
    pub fn set_encryption(
        mut self,
        input: ::std::option::Option<crate::types::S3EncryptionSettings>,
    ) -> Self {
        self.encryption = input;
        self
    }
    /// Consumes the builder and constructs a [`S3DestinationSettings`](crate::types::S3DestinationSettings).
    pub fn build(self) -> crate::types::S3DestinationSettings {
        crate::types::S3DestinationSettings {
            access_control: self.access_control,
            encryption: self.encryption,
        }
    }
}
