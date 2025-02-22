// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVolume`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::set_client_request_token): <p>(Optional) An idempotency token for resource creation, in a string of up to 63 ASCII characters. This token is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
    ///   - [`volume_id(impl ::std::convert::Into<String>)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::volume_id) / [`set_volume_id(Option<String>)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::set_volume_id): <p>The ID of the volume that you are deleting.</p>
    ///   - [`ontap_configuration(DeleteVolumeOntapConfiguration)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::ontap_configuration) / [`set_ontap_configuration(Option<DeleteVolumeOntapConfiguration>)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::set_ontap_configuration): <p>For Amazon FSx for ONTAP volumes, specify whether to take a final backup of the volume and apply tags to the backup. To apply tags to the backup, you must have the <code>fsx:TagResource</code> permission.</p>
    ///   - [`open_zfs_configuration(DeleteVolumeOpenZfsConfiguration)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::open_zfs_configuration) / [`set_open_zfs_configuration(Option<DeleteVolumeOpenZfsConfiguration>)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::set_open_zfs_configuration): <p>For Amazon FSx for OpenZFS volumes, specify whether to delete all child volumes and snapshots.</p>
    /// - On success, responds with [`DeleteVolumeOutput`](crate::operation::delete_volume::DeleteVolumeOutput) with field(s):
    ///   - [`volume_id(Option<String>)`](crate::operation::delete_volume::DeleteVolumeOutput::volume_id): <p>The ID of the volume that's being deleted.</p>
    ///   - [`lifecycle(Option<VolumeLifecycle>)`](crate::operation::delete_volume::DeleteVolumeOutput::lifecycle): <p>The lifecycle state of the volume being deleted. If the <code>DeleteVolume</code> operation is successful, this value is <code>DELETING</code>.</p>
    ///   - [`ontap_response(Option<DeleteVolumeOntapResponse>)`](crate::operation::delete_volume::DeleteVolumeOutput::ontap_response): <p>Returned after a <code>DeleteVolume</code> request, showing the status of the delete request.</p>
    /// - On failure, responds with [`SdkError<DeleteVolumeError>`](crate::operation::delete_volume::DeleteVolumeError)
    pub fn delete_volume(
        &self,
    ) -> crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder {
        crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
