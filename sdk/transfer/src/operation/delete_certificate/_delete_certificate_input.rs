// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteCertificateInput {
    /// <p>The identifier of the certificate object that you are deleting.</p>
    #[doc(hidden)]
    pub certificate_id: ::std::option::Option<::std::string::String>,
}
impl DeleteCertificateInput {
    /// <p>The identifier of the certificate object that you are deleting.</p>
    pub fn certificate_id(&self) -> ::std::option::Option<&str> {
        self.certificate_id.as_deref()
    }
}
impl DeleteCertificateInput {
    /// Creates a new builder-style object to manufacture [`DeleteCertificateInput`](crate::operation::delete_certificate::DeleteCertificateInput).
    pub fn builder() -> crate::operation::delete_certificate::builders::DeleteCertificateInputBuilder
    {
        crate::operation::delete_certificate::builders::DeleteCertificateInputBuilder::default()
    }
}

/// A builder for [`DeleteCertificateInput`](crate::operation::delete_certificate::DeleteCertificateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteCertificateInputBuilder {
    pub(crate) certificate_id: ::std::option::Option<::std::string::String>,
}
impl DeleteCertificateInputBuilder {
    /// <p>The identifier of the certificate object that you are deleting.</p>
    pub fn certificate_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.certificate_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the certificate object that you are deleting.</p>
    pub fn set_certificate_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.certificate_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteCertificateInput`](crate::operation::delete_certificate::DeleteCertificateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_certificate::DeleteCertificateInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_certificate::DeleteCertificateInput {
                certificate_id: self.certificate_id,
            },
        )
    }
}
