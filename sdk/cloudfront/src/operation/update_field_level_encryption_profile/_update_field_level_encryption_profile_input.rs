// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateFieldLevelEncryptionProfileInput {
    /// <p>Request to update a field-level encryption profile.</p>
    #[doc(hidden)]
    pub field_level_encryption_profile_config:
        ::std::option::Option<crate::types::FieldLevelEncryptionProfileConfig>,
    /// <p>The ID of the field-level encryption profile request.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the profile identity to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    #[doc(hidden)]
    pub if_match: ::std::option::Option<::std::string::String>,
}
impl UpdateFieldLevelEncryptionProfileInput {
    /// <p>Request to update a field-level encryption profile.</p>
    pub fn field_level_encryption_profile_config(
        &self,
    ) -> ::std::option::Option<&crate::types::FieldLevelEncryptionProfileConfig> {
        self.field_level_encryption_profile_config.as_ref()
    }
    /// <p>The ID of the field-level encryption profile request.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the profile identity to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub fn if_match(&self) -> ::std::option::Option<&str> {
        self.if_match.as_deref()
    }
}
impl UpdateFieldLevelEncryptionProfileInput {
    /// Creates a new builder-style object to manufacture [`UpdateFieldLevelEncryptionProfileInput`](crate::operation::update_field_level_encryption_profile::UpdateFieldLevelEncryptionProfileInput).
    pub fn builder() -> crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileInputBuilder{
        crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileInputBuilder::default()
    }
}

/// A builder for [`UpdateFieldLevelEncryptionProfileInput`](crate::operation::update_field_level_encryption_profile::UpdateFieldLevelEncryptionProfileInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateFieldLevelEncryptionProfileInputBuilder {
    pub(crate) field_level_encryption_profile_config:
        ::std::option::Option<crate::types::FieldLevelEncryptionProfileConfig>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) if_match: ::std::option::Option<::std::string::String>,
}
impl UpdateFieldLevelEncryptionProfileInputBuilder {
    /// <p>Request to update a field-level encryption profile.</p>
    pub fn field_level_encryption_profile_config(
        mut self,
        input: crate::types::FieldLevelEncryptionProfileConfig,
    ) -> Self {
        self.field_level_encryption_profile_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Request to update a field-level encryption profile.</p>
    pub fn set_field_level_encryption_profile_config(
        mut self,
        input: ::std::option::Option<crate::types::FieldLevelEncryptionProfileConfig>,
    ) -> Self {
        self.field_level_encryption_profile_config = input;
        self
    }
    /// <p>The ID of the field-level encryption profile request.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the field-level encryption profile request.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the profile identity to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.if_match = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the profile identity to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.if_match = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateFieldLevelEncryptionProfileInput`](crate::operation::update_field_level_encryption_profile::UpdateFieldLevelEncryptionProfileInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::update_field_level_encryption_profile::UpdateFieldLevelEncryptionProfileInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::update_field_level_encryption_profile::UpdateFieldLevelEncryptionProfileInput {
                field_level_encryption_profile_config: self.field_level_encryption_profile_config
                ,
                id: self.id
                ,
                if_match: self.if_match
                ,
            }
        )
    }
}
