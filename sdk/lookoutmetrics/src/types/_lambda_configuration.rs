// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a Lambda configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LambdaConfiguration {
    /// <p>The ARN of an IAM role that has permission to invoke the Lambda function.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the Lambda function.</p>
    #[doc(hidden)]
    pub lambda_arn: ::std::option::Option<::std::string::String>,
}
impl LambdaConfiguration {
    /// <p>The ARN of an IAM role that has permission to invoke the Lambda function.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The ARN of the Lambda function.</p>
    pub fn lambda_arn(&self) -> ::std::option::Option<&str> {
        self.lambda_arn.as_deref()
    }
}
impl LambdaConfiguration {
    /// Creates a new builder-style object to manufacture [`LambdaConfiguration`](crate::types::LambdaConfiguration).
    pub fn builder() -> crate::types::builders::LambdaConfigurationBuilder {
        crate::types::builders::LambdaConfigurationBuilder::default()
    }
}

/// A builder for [`LambdaConfiguration`](crate::types::LambdaConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LambdaConfigurationBuilder {
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) lambda_arn: ::std::option::Option<::std::string::String>,
}
impl LambdaConfigurationBuilder {
    /// <p>The ARN of an IAM role that has permission to invoke the Lambda function.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of an IAM role that has permission to invoke the Lambda function.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The ARN of the Lambda function.</p>
    pub fn lambda_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.lambda_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the Lambda function.</p>
    pub fn set_lambda_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.lambda_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`LambdaConfiguration`](crate::types::LambdaConfiguration).
    pub fn build(self) -> crate::types::LambdaConfiguration {
        crate::types::LambdaConfiguration {
            role_arn: self.role_arn,
            lambda_arn: self.lambda_arn,
        }
    }
}
