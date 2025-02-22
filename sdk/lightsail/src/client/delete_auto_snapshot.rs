// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAutoSnapshot`](crate::operation::delete_auto_snapshot::builders::DeleteAutoSnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_name(impl ::std::convert::Into<String>)`](crate::operation::delete_auto_snapshot::builders::DeleteAutoSnapshotFluentBuilder::resource_name) / [`set_resource_name(Option<String>)`](crate::operation::delete_auto_snapshot::builders::DeleteAutoSnapshotFluentBuilder::set_resource_name): <p>The name of the source instance or disk from which to delete the automatic snapshot.</p>
    ///   - [`date(impl ::std::convert::Into<String>)`](crate::operation::delete_auto_snapshot::builders::DeleteAutoSnapshotFluentBuilder::date) / [`set_date(Option<String>)`](crate::operation::delete_auto_snapshot::builders::DeleteAutoSnapshotFluentBuilder::set_date): <p>The date of the automatic snapshot to delete in <code>YYYY-MM-DD</code> format. Use the <code>get auto snapshots</code> operation to get the available automatic snapshots for a resource.</p>
    /// - On success, responds with [`DeleteAutoSnapshotOutput`](crate::operation::delete_auto_snapshot::DeleteAutoSnapshotOutput) with field(s):
    ///   - [`operations(Option<Vec<Operation>>)`](crate::operation::delete_auto_snapshot::DeleteAutoSnapshotOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
    /// - On failure, responds with [`SdkError<DeleteAutoSnapshotError>`](crate::operation::delete_auto_snapshot::DeleteAutoSnapshotError)
    pub fn delete_auto_snapshot(
        &self,
    ) -> crate::operation::delete_auto_snapshot::builders::DeleteAutoSnapshotFluentBuilder {
        crate::operation::delete_auto_snapshot::builders::DeleteAutoSnapshotFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
