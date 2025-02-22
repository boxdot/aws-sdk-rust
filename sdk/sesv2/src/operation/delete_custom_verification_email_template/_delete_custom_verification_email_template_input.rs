// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to delete an existing custom verification email template.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteCustomVerificationEmailTemplateInput {
    /// <p>The name of the custom verification email template that you want to delete.</p>
    #[doc(hidden)]
    pub template_name: ::std::option::Option<::std::string::String>,
}
impl DeleteCustomVerificationEmailTemplateInput {
    /// <p>The name of the custom verification email template that you want to delete.</p>
    pub fn template_name(&self) -> ::std::option::Option<&str> {
        self.template_name.as_deref()
    }
}
impl DeleteCustomVerificationEmailTemplateInput {
    /// Creates a new builder-style object to manufacture [`DeleteCustomVerificationEmailTemplateInput`](crate::operation::delete_custom_verification_email_template::DeleteCustomVerificationEmailTemplateInput).
    pub fn builder() -> crate::operation::delete_custom_verification_email_template::builders::DeleteCustomVerificationEmailTemplateInputBuilder{
        crate::operation::delete_custom_verification_email_template::builders::DeleteCustomVerificationEmailTemplateInputBuilder::default()
    }
}

/// A builder for [`DeleteCustomVerificationEmailTemplateInput`](crate::operation::delete_custom_verification_email_template::DeleteCustomVerificationEmailTemplateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteCustomVerificationEmailTemplateInputBuilder {
    pub(crate) template_name: ::std::option::Option<::std::string::String>,
}
impl DeleteCustomVerificationEmailTemplateInputBuilder {
    /// <p>The name of the custom verification email template that you want to delete.</p>
    pub fn template_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the custom verification email template that you want to delete.</p>
    pub fn set_template_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.template_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteCustomVerificationEmailTemplateInput`](crate::operation::delete_custom_verification_email_template::DeleteCustomVerificationEmailTemplateInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::delete_custom_verification_email_template::DeleteCustomVerificationEmailTemplateInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::delete_custom_verification_email_template::DeleteCustomVerificationEmailTemplateInput {
                template_name: self.template_name
                ,
            }
        )
    }
}
