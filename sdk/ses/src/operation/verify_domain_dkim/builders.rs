// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::verify_domain_dkim::_verify_domain_dkim_output::VerifyDomainDkimOutputBuilder;

pub use crate::operation::verify_domain_dkim::_verify_domain_dkim_input::VerifyDomainDkimInputBuilder;

/// Fluent builder constructing a request to `VerifyDomainDkim`.
///
/// <p>Returns a set of DKIM tokens for a domain identity.</p> <important>
/// <p>When you execute the <code>VerifyDomainDkim</code> operation, the domain that you specify is added to the list of identities that are associated with your account. This is true even if you haven't already associated the domain with your account by using the <code>VerifyDomainIdentity</code> operation. However, you can't send email from the domain until you either successfully <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-domains.html">verify it</a> or you successfully <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">set up DKIM for it</a>.</p>
/// </important>
/// <p>You use the tokens that are generated by this operation to create CNAME records. When Amazon SES detects that you've added these records to the DNS configuration for a domain, you can start sending email from that domain. You can start sending email even if you haven't added the TXT record provided by the VerifyDomainIdentity operation to the DNS configuration for your domain. All email that you send from the domain is authenticated using DKIM.</p>
/// <p>To create the CNAME records for DKIM authentication, use the following values:</p>
/// <ul>
/// <li> <p> <b>Name</b>: <i>token</i>._domainkey.<i>example.com</i> </p> </li>
/// <li> <p> <b>Type</b>: CNAME</p> </li>
/// <li> <p> <b>Value</b>: <i>token</i>.dkim.amazonses.com</p> </li>
/// </ul>
/// <p>In the preceding example, replace <i>token</i> with one of the tokens that are generated when you execute this operation. Replace <i>example.com</i> with your domain. Repeat this process for each token that's generated by this operation.</p>
/// <p>You can execute this operation no more than once per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct VerifyDomainDkimFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::verify_domain_dkim::builders::VerifyDomainDkimInputBuilder,
}
impl VerifyDomainDkimFluentBuilder {
    /// Creates a new `VerifyDomainDkim`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::verify_domain_dkim::VerifyDomainDkim,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::verify_domain_dkim::VerifyDomainDkimError,
        >,
    > {
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
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::verify_domain_dkim::VerifyDomainDkimOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::verify_domain_dkim::VerifyDomainDkimError,
        >,
    > {
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
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::verify_domain_dkim::VerifyDomainDkimOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::verify_domain_dkim::VerifyDomainDkimError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::verify_domain_dkim::VerifyDomainDkim,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::verify_domain_dkim::VerifyDomainDkimError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the domain to be verified for Easy DKIM signing.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>The name of the domain to be verified for Easy DKIM signing.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
}
