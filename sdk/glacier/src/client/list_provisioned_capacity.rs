// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListProvisionedCapacity`](crate::operation::list_provisioned_capacity::builders::ListProvisionedCapacityFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::list_provisioned_capacity::builders::ListProvisionedCapacityFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::list_provisioned_capacity::builders::ListProvisionedCapacityFluentBuilder::set_account_id): <p>The AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '-' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, don't include any hyphens ('-') in the ID. </p>
    /// - On success, responds with [`ListProvisionedCapacityOutput`](crate::operation::list_provisioned_capacity::ListProvisionedCapacityOutput) with field(s):
    ///   - [`provisioned_capacity_list(Option<Vec<ProvisionedCapacityDescription>>)`](crate::operation::list_provisioned_capacity::ListProvisionedCapacityOutput::provisioned_capacity_list): <p>The response body contains the following JSON fields.</p>
    /// - On failure, responds with [`SdkError<ListProvisionedCapacityError>`](crate::operation::list_provisioned_capacity::ListProvisionedCapacityError)
    pub fn list_provisioned_capacity(
        &self,
    ) -> crate::operation::list_provisioned_capacity::builders::ListProvisionedCapacityFluentBuilder
    {
        crate::operation::list_provisioned_capacity::builders::ListProvisionedCapacityFluentBuilder::new(self.handle.clone())
    }
}
