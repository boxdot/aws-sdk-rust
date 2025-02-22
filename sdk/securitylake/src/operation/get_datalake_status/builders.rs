// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_datalake_status::_get_datalake_status_output::GetDatalakeStatusOutputBuilder;

pub use crate::operation::get_datalake_status::_get_datalake_status_input::GetDatalakeStatusInputBuilder;

/// Fluent builder constructing a request to `GetDatalakeStatus`.
///
/// <p>Retrieves a snapshot of the current Region, including whether Amazon Security Lake is enabled for those accounts and which sources Security Lake is collecting data from. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDatalakeStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_datalake_status::builders::GetDatalakeStatusInputBuilder,
}
impl GetDatalakeStatusFluentBuilder {
    /// Creates a new `GetDatalakeStatus`.
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
            crate::operation::get_datalake_status::GetDatalakeStatus,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_datalake_status::GetDatalakeStatusError,
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
        crate::operation::get_datalake_status::GetDatalakeStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_datalake_status::GetDatalakeStatusError,
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
        crate::operation::get_datalake_status::GetDatalakeStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_datalake_status::GetDatalakeStatusError,
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
            crate::operation::get_datalake_status::GetDatalakeStatus,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_datalake_status::GetDatalakeStatusError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_datalake_status::paginator::GetDatalakeStatusPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::get_datalake_status::paginator::GetDatalakeStatusPaginator {
        crate::operation::get_datalake_status::paginator::GetDatalakeStatusPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `accountSet`.
    ///
    /// To override the contents of this collection use [`set_account_set`](Self::set_account_set).
    ///
    /// <p>The Amazon Web Services account ID for which a static snapshot of the current Amazon Web Services Region, including enabled accounts and log sources, is retrieved.</p>
    pub fn account_set(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_set(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for which a static snapshot of the current Amazon Web Services Region, including enabled accounts and log sources, is retrieved.</p>
    pub fn set_account_set(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_account_set(input);
        self
    }
    /// <p>The maximum limit of accounts for which the static snapshot of the current Region, including enabled accounts and log sources, is retrieved.</p>
    pub fn max_account_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_account_results(input);
        self
    }
    /// <p>The maximum limit of accounts for which the static snapshot of the current Region, including enabled accounts and log sources, is retrieved.</p>
    pub fn set_max_account_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_account_results(input);
        self
    }
    /// <p>Lists if there are more results available. The value of nextToken is a unique pagination token for each page. Repeat the call using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    /// <p>Each pagination token expires after 24 hours. Using an expired pagination token will return an HTTP 400 InvalidToken error.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Lists if there are more results available. The value of nextToken is a unique pagination token for each page. Repeat the call using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    /// <p>Each pagination token expires after 24 hours. Using an expired pagination token will return an HTTP 400 InvalidToken error.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
