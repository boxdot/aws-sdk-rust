// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBillingGroup`](crate::operation::delete_billing_group::builders::DeleteBillingGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`billing_group_name(impl ::std::convert::Into<String>)`](crate::operation::delete_billing_group::builders::DeleteBillingGroupFluentBuilder::billing_group_name) / [`set_billing_group_name(Option<String>)`](crate::operation::delete_billing_group::builders::DeleteBillingGroupFluentBuilder::set_billing_group_name): <p>The name of the billing group.</p>
    ///   - [`expected_version(i64)`](crate::operation::delete_billing_group::builders::DeleteBillingGroupFluentBuilder::expected_version) / [`set_expected_version(Option<i64>)`](crate::operation::delete_billing_group::builders::DeleteBillingGroupFluentBuilder::set_expected_version): <p>The expected version of the billing group. If the version of the billing group does not match the expected version specified in the request, the <code>DeleteBillingGroup</code> request is rejected with a <code>VersionConflictException</code>.</p>
    /// - On success, responds with [`DeleteBillingGroupOutput`](crate::operation::delete_billing_group::DeleteBillingGroupOutput)
    /// - On failure, responds with [`SdkError<DeleteBillingGroupError>`](crate::operation::delete_billing_group::DeleteBillingGroupError)
    pub fn delete_billing_group(
        &self,
    ) -> crate::operation::delete_billing_group::builders::DeleteBillingGroupFluentBuilder {
        crate::operation::delete_billing_group::builders::DeleteBillingGroupFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
