// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about an Amazon CloudWatch datasource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CloudWatchConfig {
    /// <p>An IAM role that gives Amazon Lookout for Metrics permission to access data in Amazon CloudWatch.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>Settings for backtest mode.</p>
    #[doc(hidden)]
    pub back_test_configuration: ::std::option::Option<crate::types::BackTestConfiguration>,
}
impl CloudWatchConfig {
    /// <p>An IAM role that gives Amazon Lookout for Metrics permission to access data in Amazon CloudWatch.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>Settings for backtest mode.</p>
    pub fn back_test_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::BackTestConfiguration> {
        self.back_test_configuration.as_ref()
    }
}
impl CloudWatchConfig {
    /// Creates a new builder-style object to manufacture [`CloudWatchConfig`](crate::types::CloudWatchConfig).
    pub fn builder() -> crate::types::builders::CloudWatchConfigBuilder {
        crate::types::builders::CloudWatchConfigBuilder::default()
    }
}

/// A builder for [`CloudWatchConfig`](crate::types::CloudWatchConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CloudWatchConfigBuilder {
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) back_test_configuration: ::std::option::Option<crate::types::BackTestConfiguration>,
}
impl CloudWatchConfigBuilder {
    /// <p>An IAM role that gives Amazon Lookout for Metrics permission to access data in Amazon CloudWatch.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An IAM role that gives Amazon Lookout for Metrics permission to access data in Amazon CloudWatch.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>Settings for backtest mode.</p>
    pub fn back_test_configuration(mut self, input: crate::types::BackTestConfiguration) -> Self {
        self.back_test_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Settings for backtest mode.</p>
    pub fn set_back_test_configuration(
        mut self,
        input: ::std::option::Option<crate::types::BackTestConfiguration>,
    ) -> Self {
        self.back_test_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`CloudWatchConfig`](crate::types::CloudWatchConfig).
    pub fn build(self) -> crate::types::CloudWatchConfig {
        crate::types::CloudWatchConfig {
            role_arn: self.role_arn,
            back_test_configuration: self.back_test_configuration,
        }
    }
}
