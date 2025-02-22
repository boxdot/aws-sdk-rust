// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::execute_transaction::_execute_transaction_output::ExecuteTransactionOutputBuilder;

pub use crate::operation::execute_transaction::_execute_transaction_input::ExecuteTransactionInputBuilder;

/// Fluent builder constructing a request to `ExecuteTransaction`.
///
/// <p>This operation allows you to perform transactional reads or writes on data stored in DynamoDB, using PartiQL.</p> <note>
/// <p>The entire transaction must consist of either read statements or write statements, you cannot mix both in one transaction. The EXISTS function is an exception and can be used to check the condition of specific attributes of the item in a similar manner to <code>ConditionCheck</code> in the <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/transaction-apis.html#transaction-apis-txwriteitems">TransactWriteItems</a> API.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ExecuteTransactionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::execute_transaction::builders::ExecuteTransactionInputBuilder,
}
impl ExecuteTransactionFluentBuilder {
    /// Creates a new `ExecuteTransaction`.
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
            crate::operation::execute_transaction::ExecuteTransaction,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::execute_transaction::ExecuteTransactionError,
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
        crate::operation::execute_transaction::ExecuteTransactionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::execute_transaction::ExecuteTransactionError,
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
        crate::operation::execute_transaction::ExecuteTransactionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::execute_transaction::ExecuteTransactionError,
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
            crate::operation::execute_transaction::ExecuteTransaction,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::execute_transaction::ExecuteTransactionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `TransactStatements`.
    ///
    /// To override the contents of this collection use [`set_transact_statements`](Self::set_transact_statements).
    ///
    /// <p>The list of PartiQL statements representing the transaction to run.</p>
    pub fn transact_statements(mut self, input: crate::types::ParameterizedStatement) -> Self {
        self.inner = self.inner.transact_statements(input);
        self
    }
    /// <p>The list of PartiQL statements representing the transaction to run.</p>
    pub fn set_transact_statements(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ParameterizedStatement>>,
    ) -> Self {
        self.inner = self.inner.set_transact_statements(input);
        self
    }
    /// <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_TransactGetItems.html">TransactGetItems</a> and <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_TransactWriteItems.html">TransactWriteItems</a>.</p>
    pub fn return_consumed_capacity(mut self, input: crate::types::ReturnConsumedCapacity) -> Self {
        self.inner = self.inner.return_consumed_capacity(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_TransactGetItems.html">TransactGetItems</a> and <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_TransactWriteItems.html">TransactWriteItems</a>.</p>
    pub fn set_return_consumed_capacity(
        mut self,
        input: ::std::option::Option<crate::types::ReturnConsumedCapacity>,
    ) -> Self {
        self.inner = self.inner.set_return_consumed_capacity(input);
        self
    }
}
