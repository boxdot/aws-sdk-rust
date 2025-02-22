// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_table::_delete_table_output::DeleteTableOutputBuilder;

pub use crate::operation::delete_table::_delete_table_input::DeleteTableInputBuilder;

/// Fluent builder constructing a request to `DeleteTable`.
///
/// <p>The <code>DeleteTable</code> operation deletes a table and all of its data. After a <code>DeleteTable</code> request is received, the specified table is in the <code>DELETING</code> state until Amazon Keyspaces completes the deletion. If the table is in the <code>ACTIVE</code> state, you can delete it. If a table is either in the <code>CREATING</code> or <code>UPDATING</code> states, then Amazon Keyspaces returns a <code>ResourceInUseException</code>. If the specified table does not exist, Amazon Keyspaces returns a <code>ResourceNotFoundException</code>. If the table is already in the <code>DELETING</code> state, no error is returned.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_table::builders::DeleteTableInputBuilder,
}
impl DeleteTableFluentBuilder {
    /// Creates a new `DeleteTable`.
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
            crate::operation::delete_table::DeleteTable,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_table::DeleteTableError>,
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
        crate::operation::delete_table::DeleteTableOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_table::DeleteTableError>,
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
        crate::operation::delete_table::DeleteTableOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_table::DeleteTableError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::delete_table::DeleteTable,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_table::DeleteTableError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the keyspace of the to be deleted table.</p>
    pub fn keyspace_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.keyspace_name(input.into());
        self
    }
    /// <p>The name of the keyspace of the to be deleted table.</p>
    pub fn set_keyspace_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_keyspace_name(input);
        self
    }
    /// <p>The name of the table to be deleted.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table to be deleted.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
}
