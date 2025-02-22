// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateSamlProviderInput {
    /// <p>An XML document generated by an identity provider (IdP) that supports SAML 2.0. The document includes the issuer's name, expiration information, and keys that can be used to validate the SAML authentication response (assertions) that are received from the IdP. You must generate the metadata document using the identity management software that is used as your organization's IdP.</p>
    #[doc(hidden)]
    pub saml_metadata_document: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the SAML provider to update.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[doc(hidden)]
    pub saml_provider_arn: ::std::option::Option<::std::string::String>,
}
impl UpdateSamlProviderInput {
    /// <p>An XML document generated by an identity provider (IdP) that supports SAML 2.0. The document includes the issuer's name, expiration information, and keys that can be used to validate the SAML authentication response (assertions) that are received from the IdP. You must generate the metadata document using the identity management software that is used as your organization's IdP.</p>
    pub fn saml_metadata_document(&self) -> ::std::option::Option<&str> {
        self.saml_metadata_document.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the SAML provider to update.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn saml_provider_arn(&self) -> ::std::option::Option<&str> {
        self.saml_provider_arn.as_deref()
    }
}
impl UpdateSamlProviderInput {
    /// Creates a new builder-style object to manufacture [`UpdateSamlProviderInput`](crate::operation::update_saml_provider::UpdateSamlProviderInput).
    pub fn builder(
    ) -> crate::operation::update_saml_provider::builders::UpdateSamlProviderInputBuilder {
        crate::operation::update_saml_provider::builders::UpdateSamlProviderInputBuilder::default()
    }
}

/// A builder for [`UpdateSamlProviderInput`](crate::operation::update_saml_provider::UpdateSamlProviderInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateSamlProviderInputBuilder {
    pub(crate) saml_metadata_document: ::std::option::Option<::std::string::String>,
    pub(crate) saml_provider_arn: ::std::option::Option<::std::string::String>,
}
impl UpdateSamlProviderInputBuilder {
    /// <p>An XML document generated by an identity provider (IdP) that supports SAML 2.0. The document includes the issuer's name, expiration information, and keys that can be used to validate the SAML authentication response (assertions) that are received from the IdP. You must generate the metadata document using the identity management software that is used as your organization's IdP.</p>
    pub fn saml_metadata_document(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.saml_metadata_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An XML document generated by an identity provider (IdP) that supports SAML 2.0. The document includes the issuer's name, expiration information, and keys that can be used to validate the SAML authentication response (assertions) that are received from the IdP. You must generate the metadata document using the identity management software that is used as your organization's IdP.</p>
    pub fn set_saml_metadata_document(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.saml_metadata_document = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the SAML provider to update.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn saml_provider_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.saml_provider_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the SAML provider to update.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn set_saml_provider_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.saml_provider_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateSamlProviderInput`](crate::operation::update_saml_provider::UpdateSamlProviderInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_saml_provider::UpdateSamlProviderInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_saml_provider::UpdateSamlProviderInput {
                saml_metadata_document: self.saml_metadata_document,
                saml_provider_arn: self.saml_provider_arn,
            },
        )
    }
}
