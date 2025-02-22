// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateOriginAccessControlInput {
    /// <p>An origin access control.</p>
    #[doc(hidden)]
    pub origin_access_control_config:
        ::std::option::Option<crate::types::OriginAccessControlConfig>,
    /// <p>The unique identifier of the origin access control that you are updating.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The current version (<code>ETag</code> value) of the origin access control that you are updating.</p>
    #[doc(hidden)]
    pub if_match: ::std::option::Option<::std::string::String>,
}
impl UpdateOriginAccessControlInput {
    /// <p>An origin access control.</p>
    pub fn origin_access_control_config(
        &self,
    ) -> ::std::option::Option<&crate::types::OriginAccessControlConfig> {
        self.origin_access_control_config.as_ref()
    }
    /// <p>The unique identifier of the origin access control that you are updating.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The current version (<code>ETag</code> value) of the origin access control that you are updating.</p>
    pub fn if_match(&self) -> ::std::option::Option<&str> {
        self.if_match.as_deref()
    }
}
impl UpdateOriginAccessControlInput {
    /// Creates a new builder-style object to manufacture [`UpdateOriginAccessControlInput`](crate::operation::update_origin_access_control::UpdateOriginAccessControlInput).
    pub fn builder() -> crate::operation::update_origin_access_control::builders::UpdateOriginAccessControlInputBuilder{
        crate::operation::update_origin_access_control::builders::UpdateOriginAccessControlInputBuilder::default()
    }
}

/// A builder for [`UpdateOriginAccessControlInput`](crate::operation::update_origin_access_control::UpdateOriginAccessControlInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateOriginAccessControlInputBuilder {
    pub(crate) origin_access_control_config:
        ::std::option::Option<crate::types::OriginAccessControlConfig>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) if_match: ::std::option::Option<::std::string::String>,
}
impl UpdateOriginAccessControlInputBuilder {
    /// <p>An origin access control.</p>
    pub fn origin_access_control_config(
        mut self,
        input: crate::types::OriginAccessControlConfig,
    ) -> Self {
        self.origin_access_control_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>An origin access control.</p>
    pub fn set_origin_access_control_config(
        mut self,
        input: ::std::option::Option<crate::types::OriginAccessControlConfig>,
    ) -> Self {
        self.origin_access_control_config = input;
        self
    }
    /// <p>The unique identifier of the origin access control that you are updating.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the origin access control that you are updating.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The current version (<code>ETag</code> value) of the origin access control that you are updating.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.if_match = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The current version (<code>ETag</code> value) of the origin access control that you are updating.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.if_match = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateOriginAccessControlInput`](crate::operation::update_origin_access_control::UpdateOriginAccessControlInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_origin_access_control::UpdateOriginAccessControlInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_origin_access_control::UpdateOriginAccessControlInput {
                origin_access_control_config: self.origin_access_control_config,
                id: self.id,
                if_match: self.if_match,
            },
        )
    }
}
