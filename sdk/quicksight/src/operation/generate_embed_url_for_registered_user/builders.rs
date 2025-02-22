// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::generate_embed_url_for_registered_user::_generate_embed_url_for_registered_user_output::GenerateEmbedUrlForRegisteredUserOutputBuilder;

pub use crate::operation::generate_embed_url_for_registered_user::_generate_embed_url_for_registered_user_input::GenerateEmbedUrlForRegisteredUserInputBuilder;

/// Fluent builder constructing a request to `GenerateEmbedUrlForRegisteredUser`.
///
/// <p>Generates an embed URL that you can use to embed an Amazon QuickSight experience in your website. This action can be used for any type of user registered in an Amazon QuickSight account. Before you use this action, make sure that you have configured the relevant Amazon QuickSight resource and permissions.</p>
/// <p>The following rules apply to the generated URL:</p>
/// <ul>
/// <li> <p>It contains a temporary bearer token. It is valid for 5 minutes after it is generated. Once redeemed within this period, it cannot be re-used again.</p> </li>
/// <li> <p>The URL validity period should not be confused with the actual session lifetime that can be customized using the <code> <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_GenerateEmbedUrlForRegisteredUser.html#QS-GenerateEmbedUrlForRegisteredUser-request-SessionLifetimeInMinutes">SessionLifetimeInMinutes</a> </code> parameter.</p> <p>The resulting user session is valid for 15 minutes (minimum) to 10 hours (maximum). The default session duration is 10 hours.</p> </li>
/// <li> <p>You are charged only when the URL is used or there is interaction with Amazon QuickSight.</p> </li>
/// </ul>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/embedded-analytics.html">Embedded Analytics</a> in the <i>Amazon QuickSight User Guide</i>.</p>
/// <p>For more information about the high-level steps for embedding and for an interactive demo of the ways you can customize embedding, visit the <a href="https://docs.aws.amazon.com/quicksight/latest/user/quicksight-dev-portal.html">Amazon QuickSight Developer Portal</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateEmbedUrlForRegisteredUserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::generate_embed_url_for_registered_user::builders::GenerateEmbedUrlForRegisteredUserInputBuilder,
}
impl GenerateEmbedUrlForRegisteredUserFluentBuilder {
    /// Creates a new `GenerateEmbedUrlForRegisteredUser`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::generate_embed_url_for_registered_user::GenerateEmbedUrlForRegisteredUser, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::generate_embed_url_for_registered_user::GenerateEmbedUrlForRegisteredUserError>
    >{
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::generate_embed_url_for_registered_user::GenerateEmbedUrlForRegisteredUserOutput, ::aws_smithy_http::result::SdkError<crate::operation::generate_embed_url_for_registered_user::GenerateEmbedUrlForRegisteredUserError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
                        pub async fn send(self) -> ::std::result::Result<crate::operation::generate_embed_url_for_registered_user::GenerateEmbedUrlForRegisteredUserOutput, ::aws_smithy_http::result::SdkError<crate::operation::generate_embed_url_for_registered_user::GenerateEmbedUrlForRegisteredUserError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::generate_embed_url_for_registered_user::GenerateEmbedUrlForRegisteredUser, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::generate_embed_url_for_registered_user::GenerateEmbedUrlForRegisteredUserError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ID for the Amazon Web Services account that contains the dashboard that you're embedding.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID for the Amazon Web Services account that contains the dashboard that you're embedding.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>How many minutes the session is valid. The session lifetime must be in [15-600] minutes range.</p>
    pub fn session_lifetime_in_minutes(mut self, input: i64) -> Self {
        self.inner = self.inner.session_lifetime_in_minutes(input);
        self
    }
    /// <p>How many minutes the session is valid. The session lifetime must be in [15-600] minutes range.</p>
    pub fn set_session_lifetime_in_minutes(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_session_lifetime_in_minutes(input);
        self
    }
    /// <p>The Amazon Resource Name for the registered user.</p>
    pub fn user_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name for the registered user.</p>
    pub fn set_user_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_arn(input);
        self
    }
    /// <p>The experience you are embedding. For registered users, you can embed Amazon QuickSight dashboards, Amazon QuickSight visuals, the Amazon QuickSight Q search bar, or the entire Amazon QuickSight console.</p>
    pub fn experience_configuration(
        mut self,
        input: crate::types::RegisteredUserEmbeddingExperienceConfiguration,
    ) -> Self {
        self.inner = self.inner.experience_configuration(input);
        self
    }
    /// <p>The experience you are embedding. For registered users, you can embed Amazon QuickSight dashboards, Amazon QuickSight visuals, the Amazon QuickSight Q search bar, or the entire Amazon QuickSight console.</p>
    pub fn set_experience_configuration(
        mut self,
        input: ::std::option::Option<crate::types::RegisteredUserEmbeddingExperienceConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_experience_configuration(input);
        self
    }
    /// Appends an item to `AllowedDomains`.
    ///
    /// To override the contents of this collection use [`set_allowed_domains`](Self::set_allowed_domains).
    ///
    /// <p>The domains that you want to add to the allow list for access to the generated URL that is then embedded. This optional parameter overrides the static domains that are configured in the Manage QuickSight menu in the Amazon QuickSight console. Instead, it allows only the domains that you include in this parameter. You can list up to three domains or subdomains in each API call.</p>
    /// <p>To include all subdomains under a specific domain to the allow list, use <code>*</code>. For example, <code>https://*.sapp.amazon.com</code> includes all subdomains under <code>https://sapp.amazon.com</code>.</p>
    pub fn allowed_domains(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.allowed_domains(input.into());
        self
    }
    /// <p>The domains that you want to add to the allow list for access to the generated URL that is then embedded. This optional parameter overrides the static domains that are configured in the Manage QuickSight menu in the Amazon QuickSight console. Instead, it allows only the domains that you include in this parameter. You can list up to three domains or subdomains in each API call.</p>
    /// <p>To include all subdomains under a specific domain to the allow list, use <code>*</code>. For example, <code>https://*.sapp.amazon.com</code> includes all subdomains under <code>https://sapp.amazon.com</code>.</p>
    pub fn set_allowed_domains(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_allowed_domains(input);
        self
    }
}
