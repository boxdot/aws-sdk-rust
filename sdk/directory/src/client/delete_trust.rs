// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTrust`](crate::operation::delete_trust::builders::DeleteTrustFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`trust_id(impl ::std::convert::Into<String>)`](crate::operation::delete_trust::builders::DeleteTrustFluentBuilder::trust_id) / [`set_trust_id(Option<String>)`](crate::operation::delete_trust::builders::DeleteTrustFluentBuilder::set_trust_id): <p>The Trust ID of the trust relationship to be deleted.</p>
    ///   - [`delete_associated_conditional_forwarder(bool)`](crate::operation::delete_trust::builders::DeleteTrustFluentBuilder::delete_associated_conditional_forwarder) / [`set_delete_associated_conditional_forwarder(Option<bool>)`](crate::operation::delete_trust::builders::DeleteTrustFluentBuilder::set_delete_associated_conditional_forwarder): <p>Delete a conditional forwarder as part of a DeleteTrustRequest.</p>
    /// - On success, responds with [`DeleteTrustOutput`](crate::operation::delete_trust::DeleteTrustOutput) with field(s):
    ///   - [`trust_id(Option<String>)`](crate::operation::delete_trust::DeleteTrustOutput::trust_id): <p>The Trust ID of the trust relationship that was deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteTrustError>`](crate::operation::delete_trust::DeleteTrustError)
    pub fn delete_trust(
        &self,
    ) -> crate::operation::delete_trust::builders::DeleteTrustFluentBuilder {
        crate::operation::delete_trust::builders::DeleteTrustFluentBuilder::new(self.handle.clone())
    }
}
