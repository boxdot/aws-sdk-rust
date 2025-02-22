// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ResetSnapshotAttribute`](crate::operation::reset_snapshot_attribute::builders::ResetSnapshotAttributeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`attribute(SnapshotAttributeName)`](crate::operation::reset_snapshot_attribute::builders::ResetSnapshotAttributeFluentBuilder::attribute) / [`set_attribute(Option<SnapshotAttributeName>)`](crate::operation::reset_snapshot_attribute::builders::ResetSnapshotAttributeFluentBuilder::set_attribute): <p>The attribute to reset. Currently, only the attribute for permission to create volumes can be reset.</p>
    ///   - [`snapshot_id(impl ::std::convert::Into<String>)`](crate::operation::reset_snapshot_attribute::builders::ResetSnapshotAttributeFluentBuilder::snapshot_id) / [`set_snapshot_id(Option<String>)`](crate::operation::reset_snapshot_attribute::builders::ResetSnapshotAttributeFluentBuilder::set_snapshot_id): <p>The ID of the snapshot.</p>
    ///   - [`dry_run(bool)`](crate::operation::reset_snapshot_attribute::builders::ResetSnapshotAttributeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::reset_snapshot_attribute::builders::ResetSnapshotAttributeFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ResetSnapshotAttributeOutput`](crate::operation::reset_snapshot_attribute::ResetSnapshotAttributeOutput)
    /// - On failure, responds with [`SdkError<ResetSnapshotAttributeError>`](crate::operation::reset_snapshot_attribute::ResetSnapshotAttributeError)
    pub fn reset_snapshot_attribute(
        &self,
    ) -> crate::operation::reset_snapshot_attribute::builders::ResetSnapshotAttributeFluentBuilder
    {
        crate::operation::reset_snapshot_attribute::builders::ResetSnapshotAttributeFluentBuilder::new(self.handle.clone())
    }
}
