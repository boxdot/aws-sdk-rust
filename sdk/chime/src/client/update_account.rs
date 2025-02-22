// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAccount`](crate::operation::update_account::builders::UpdateAccountFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::update_account::builders::UpdateAccountFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::update_account::builders::UpdateAccountFluentBuilder::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_account::builders::UpdateAccountFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_account::builders::UpdateAccountFluentBuilder::set_name): <p>The new name for the specified Amazon Chime account.</p>
    ///   - [`default_license(License)`](crate::operation::update_account::builders::UpdateAccountFluentBuilder::default_license) / [`set_default_license(Option<License>)`](crate::operation::update_account::builders::UpdateAccountFluentBuilder::set_default_license): <p>The default license applied when you add users to an Amazon Chime account.</p>
    /// - On success, responds with [`UpdateAccountOutput`](crate::operation::update_account::UpdateAccountOutput) with field(s):
    ///   - [`account(Option<Account>)`](crate::operation::update_account::UpdateAccountOutput::account): <p>The updated Amazon Chime account details.</p>
    /// - On failure, responds with [`SdkError<UpdateAccountError>`](crate::operation::update_account::UpdateAccountError)
    pub fn update_account(
        &self,
    ) -> crate::operation::update_account::builders::UpdateAccountFluentBuilder {
        crate::operation::update_account::builders::UpdateAccountFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
