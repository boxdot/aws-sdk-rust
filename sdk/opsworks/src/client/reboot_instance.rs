// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RebootInstance`](crate::operation::reboot_instance::builders::RebootInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::reboot_instance::builders::RebootInstanceFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::reboot_instance::builders::RebootInstanceFluentBuilder::set_instance_id): <p>The instance ID.</p>
    /// - On success, responds with [`RebootInstanceOutput`](crate::operation::reboot_instance::RebootInstanceOutput)
    /// - On failure, responds with [`SdkError<RebootInstanceError>`](crate::operation::reboot_instance::RebootInstanceError)
    pub fn reboot_instance(
        &self,
    ) -> crate::operation::reboot_instance::builders::RebootInstanceFluentBuilder {
        crate::operation::reboot_instance::builders::RebootInstanceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
