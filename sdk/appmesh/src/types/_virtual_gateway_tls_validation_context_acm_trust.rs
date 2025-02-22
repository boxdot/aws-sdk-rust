// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents a Transport Layer Security (TLS) validation context trust for an Certificate Manager certificate.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VirtualGatewayTlsValidationContextAcmTrust {
    /// <p>One or more ACM Amazon Resource Name (ARN)s.</p>
    #[doc(hidden)]
    pub certificate_authority_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl VirtualGatewayTlsValidationContextAcmTrust {
    /// <p>One or more ACM Amazon Resource Name (ARN)s.</p>
    pub fn certificate_authority_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.certificate_authority_arns.as_deref()
    }
}
impl VirtualGatewayTlsValidationContextAcmTrust {
    /// Creates a new builder-style object to manufacture [`VirtualGatewayTlsValidationContextAcmTrust`](crate::types::VirtualGatewayTlsValidationContextAcmTrust).
    pub fn builder() -> crate::types::builders::VirtualGatewayTlsValidationContextAcmTrustBuilder {
        crate::types::builders::VirtualGatewayTlsValidationContextAcmTrustBuilder::default()
    }
}

/// A builder for [`VirtualGatewayTlsValidationContextAcmTrust`](crate::types::VirtualGatewayTlsValidationContextAcmTrust).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VirtualGatewayTlsValidationContextAcmTrustBuilder {
    pub(crate) certificate_authority_arns:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl VirtualGatewayTlsValidationContextAcmTrustBuilder {
    /// Appends an item to `certificate_authority_arns`.
    ///
    /// To override the contents of this collection use [`set_certificate_authority_arns`](Self::set_certificate_authority_arns).
    ///
    /// <p>One or more ACM Amazon Resource Name (ARN)s.</p>
    pub fn certificate_authority_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.certificate_authority_arns.unwrap_or_default();
        v.push(input.into());
        self.certificate_authority_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more ACM Amazon Resource Name (ARN)s.</p>
    pub fn set_certificate_authority_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.certificate_authority_arns = input;
        self
    }
    /// Consumes the builder and constructs a [`VirtualGatewayTlsValidationContextAcmTrust`](crate::types::VirtualGatewayTlsValidationContextAcmTrust).
    pub fn build(self) -> crate::types::VirtualGatewayTlsValidationContextAcmTrust {
        crate::types::VirtualGatewayTlsValidationContextAcmTrust {
            certificate_authority_arns: self.certificate_authority_arns,
        }
    }
}
