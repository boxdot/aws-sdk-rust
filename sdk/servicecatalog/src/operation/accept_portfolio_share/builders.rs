// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::accept_portfolio_share::_accept_portfolio_share_output::AcceptPortfolioShareOutputBuilder;

pub use crate::operation::accept_portfolio_share::_accept_portfolio_share_input::AcceptPortfolioShareInputBuilder;

/// Fluent builder constructing a request to `AcceptPortfolioShare`.
///
/// <p>Accepts an offer to share the specified portfolio.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AcceptPortfolioShareFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareInputBuilder,
}
impl AcceptPortfolioShareFluentBuilder {
    /// Creates a new `AcceptPortfolioShare`.
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
            crate::operation::accept_portfolio_share::AcceptPortfolioShare,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_portfolio_share::AcceptPortfolioShareError,
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
        crate::operation::accept_portfolio_share::AcceptPortfolioShareOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_portfolio_share::AcceptPortfolioShareError,
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
        crate::operation::accept_portfolio_share::AcceptPortfolioShareOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_portfolio_share::AcceptPortfolioShareError,
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
            crate::operation::accept_portfolio_share::AcceptPortfolioShare,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_portfolio_share::AcceptPortfolioShareError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn accept_language(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.accept_language(input.into());
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn set_accept_language(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_accept_language(input);
        self
    }
    /// <p>The portfolio identifier.</p>
    pub fn portfolio_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.portfolio_id(input.into());
        self
    }
    /// <p>The portfolio identifier.</p>
    pub fn set_portfolio_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_portfolio_id(input);
        self
    }
    /// <p>The type of shared portfolios to accept. The default is to accept imported portfolios.</p>
    /// <ul>
    /// <li> <p> <code>AWS_ORGANIZATIONS</code> - Accept portfolios shared by the management account of your organization.</p> </li>
    /// <li> <p> <code>IMPORTED</code> - Accept imported portfolios.</p> </li>
    /// <li> <p> <code>AWS_SERVICECATALOG</code> - Not supported. (Throws ResourceNotFoundException.)</p> </li>
    /// </ul>
    /// <p>For example, <code>aws servicecatalog accept-portfolio-share --portfolio-id "port-2qwzkwxt3y5fk" --portfolio-share-type AWS_ORGANIZATIONS</code> </p>
    pub fn portfolio_share_type(mut self, input: crate::types::PortfolioShareType) -> Self {
        self.inner = self.inner.portfolio_share_type(input);
        self
    }
    /// <p>The type of shared portfolios to accept. The default is to accept imported portfolios.</p>
    /// <ul>
    /// <li> <p> <code>AWS_ORGANIZATIONS</code> - Accept portfolios shared by the management account of your organization.</p> </li>
    /// <li> <p> <code>IMPORTED</code> - Accept imported portfolios.</p> </li>
    /// <li> <p> <code>AWS_SERVICECATALOG</code> - Not supported. (Throws ResourceNotFoundException.)</p> </li>
    /// </ul>
    /// <p>For example, <code>aws servicecatalog accept-portfolio-share --portfolio-id "port-2qwzkwxt3y5fk" --portfolio-share-type AWS_ORGANIZATIONS</code> </p>
    pub fn set_portfolio_share_type(
        mut self,
        input: ::std::option::Option<crate::types::PortfolioShareType>,
    ) -> Self {
        self.inner = self.inner.set_portfolio_share_type(input);
        self
    }
}
