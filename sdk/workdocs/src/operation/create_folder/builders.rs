// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_folder::_create_folder_output::CreateFolderOutputBuilder;

pub use crate::operation::create_folder::_create_folder_input::CreateFolderInputBuilder;

/// Fluent builder constructing a request to `CreateFolder`.
///
/// <p>Creates a folder with the specified name and parent folder.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateFolderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_folder::builders::CreateFolderInputBuilder,
}
impl CreateFolderFluentBuilder {
    /// Creates a new `CreateFolder`.
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
            crate::operation::create_folder::CreateFolder,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_folder::CreateFolderError>,
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
        crate::operation::create_folder::CreateFolderOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_folder::CreateFolderError>,
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
        crate::operation::create_folder::CreateFolderOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_folder::CreateFolderError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_folder::CreateFolder,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_folder::CreateFolderError>,
    > {
        self.customize_middleware().await
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn authentication_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.authentication_token(input.into());
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn set_authentication_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_authentication_token(input);
        self
    }
    /// <p>The name of the new folder.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the new folder.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The ID of the parent folder.</p>
    pub fn parent_folder_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.parent_folder_id(input.into());
        self
    }
    /// <p>The ID of the parent folder.</p>
    pub fn set_parent_folder_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_parent_folder_id(input);
        self
    }
}
