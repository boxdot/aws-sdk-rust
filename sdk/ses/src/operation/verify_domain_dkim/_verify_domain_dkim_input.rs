// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to generate the CNAME records needed to set up Easy DKIM with Amazon SES. For more information about setting up Easy DKIM, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VerifyDomainDkimInput {
    /// <p>The name of the domain to be verified for Easy DKIM signing.</p>
    #[doc(hidden)]
    pub domain: ::std::option::Option<::std::string::String>,
}
impl VerifyDomainDkimInput {
    /// <p>The name of the domain to be verified for Easy DKIM signing.</p>
    pub fn domain(&self) -> ::std::option::Option<&str> {
        self.domain.as_deref()
    }
}
impl VerifyDomainDkimInput {
    /// Creates a new builder-style object to manufacture [`VerifyDomainDkimInput`](crate::operation::verify_domain_dkim::VerifyDomainDkimInput).
    pub fn builder() -> crate::operation::verify_domain_dkim::builders::VerifyDomainDkimInputBuilder
    {
        crate::operation::verify_domain_dkim::builders::VerifyDomainDkimInputBuilder::default()
    }
}

/// A builder for [`VerifyDomainDkimInput`](crate::operation::verify_domain_dkim::VerifyDomainDkimInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VerifyDomainDkimInputBuilder {
    pub(crate) domain: ::std::option::Option<::std::string::String>,
}
impl VerifyDomainDkimInputBuilder {
    /// <p>The name of the domain to be verified for Easy DKIM signing.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the domain to be verified for Easy DKIM signing.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain = input;
        self
    }
    /// Consumes the builder and constructs a [`VerifyDomainDkimInput`](crate::operation::verify_domain_dkim::VerifyDomainDkimInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::verify_domain_dkim::VerifyDomainDkimInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::verify_domain_dkim::VerifyDomainDkimInput {
                domain: self.domain,
            },
        )
    }
}
