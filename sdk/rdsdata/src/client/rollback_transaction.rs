// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RollbackTransaction`](crate::operation::rollback_transaction::builders::RollbackTransactionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::rollback_transaction::builders::RollbackTransactionFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::rollback_transaction::builders::RollbackTransactionFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    ///   - [`secret_arn(impl ::std::convert::Into<String>)`](crate::operation::rollback_transaction::builders::RollbackTransactionFluentBuilder::secret_arn) / [`set_secret_arn(Option<String>)`](crate::operation::rollback_transaction::builders::RollbackTransactionFluentBuilder::set_secret_arn): <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    ///   - [`transaction_id(impl ::std::convert::Into<String>)`](crate::operation::rollback_transaction::builders::RollbackTransactionFluentBuilder::transaction_id) / [`set_transaction_id(Option<String>)`](crate::operation::rollback_transaction::builders::RollbackTransactionFluentBuilder::set_transaction_id): <p>The identifier of the transaction to roll back.</p>
    /// - On success, responds with [`RollbackTransactionOutput`](crate::operation::rollback_transaction::RollbackTransactionOutput) with field(s):
    ///   - [`transaction_status(Option<String>)`](crate::operation::rollback_transaction::RollbackTransactionOutput::transaction_status): <p>The status of the rollback operation.</p>
    /// - On failure, responds with [`SdkError<RollbackTransactionError>`](crate::operation::rollback_transaction::RollbackTransactionError)
    pub fn rollback_transaction(
        &self,
    ) -> crate::operation::rollback_transaction::builders::RollbackTransactionFluentBuilder {
        crate::operation::rollback_transaction::builders::RollbackTransactionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
