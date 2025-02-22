// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_application_revision::_register_application_revision_output::RegisterApplicationRevisionOutputBuilder;

pub use crate::operation::register_application_revision::_register_application_revision_input::RegisterApplicationRevisionInputBuilder;

/// Fluent builder constructing a request to `RegisterApplicationRevision`.
///
/// <p>Registers with CodeDeploy a revision for the specified application.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterApplicationRevisionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::register_application_revision::builders::RegisterApplicationRevisionInputBuilder,
}
impl RegisterApplicationRevisionFluentBuilder {
    /// Creates a new `RegisterApplicationRevision`.
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
            crate::operation::register_application_revision::RegisterApplicationRevision,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::register_application_revision::RegisterApplicationRevisionError,
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
        crate::operation::register_application_revision::RegisterApplicationRevisionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::register_application_revision::RegisterApplicationRevisionError,
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
        crate::operation::register_application_revision::RegisterApplicationRevisionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::register_application_revision::RegisterApplicationRevisionError,
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
            crate::operation::register_application_revision::RegisterApplicationRevision,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::register_application_revision::RegisterApplicationRevisionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of an CodeDeploy application associated with the IAM user or Amazon Web Services account.</p>
    pub fn application_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of an CodeDeploy application associated with the IAM user or Amazon Web Services account.</p>
    pub fn set_application_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>A comment about the revision.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A comment about the revision.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Information about the application revision to register, including type and location.</p>
    pub fn revision(mut self, input: crate::types::RevisionLocation) -> Self {
        self.inner = self.inner.revision(input);
        self
    }
    /// <p>Information about the application revision to register, including type and location.</p>
    pub fn set_revision(
        mut self,
        input: ::std::option::Option<crate::types::RevisionLocation>,
    ) -> Self {
        self.inner = self.inner.set_revision(input);
        self
    }
}
