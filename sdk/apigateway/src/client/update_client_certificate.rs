// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateClientCertificate`](crate::operation::update_client_certificate::builders::UpdateClientCertificateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_certificate_id(impl ::std::convert::Into<String>)`](crate::operation::update_client_certificate::builders::UpdateClientCertificateFluentBuilder::client_certificate_id) / [`set_client_certificate_id(Option<String>)`](crate::operation::update_client_certificate::builders::UpdateClientCertificateFluentBuilder::set_client_certificate_id): <p>The identifier of the ClientCertificate resource to be updated.</p>
    ///   - [`patch_operations(Vec<PatchOperation>)`](crate::operation::update_client_certificate::builders::UpdateClientCertificateFluentBuilder::patch_operations) / [`set_patch_operations(Option<Vec<PatchOperation>>)`](crate::operation::update_client_certificate::builders::UpdateClientCertificateFluentBuilder::set_patch_operations): <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
    /// - On success, responds with [`UpdateClientCertificateOutput`](crate::operation::update_client_certificate::UpdateClientCertificateOutput) with field(s):
    ///   - [`client_certificate_id(Option<String>)`](crate::operation::update_client_certificate::UpdateClientCertificateOutput::client_certificate_id): <p>The identifier of the client certificate.</p>
    ///   - [`description(Option<String>)`](crate::operation::update_client_certificate::UpdateClientCertificateOutput::description): <p>The description of the client certificate.</p>
    ///   - [`pem_encoded_certificate(Option<String>)`](crate::operation::update_client_certificate::UpdateClientCertificateOutput::pem_encoded_certificate): <p>The PEM-encoded public key of the client certificate, which can be used to configure certificate authentication in the integration endpoint .</p>
    ///   - [`created_date(Option<DateTime>)`](crate::operation::update_client_certificate::UpdateClientCertificateOutput::created_date): <p>The timestamp when the client certificate was created.</p>
    ///   - [`expiration_date(Option<DateTime>)`](crate::operation::update_client_certificate::UpdateClientCertificateOutput::expiration_date): <p>The timestamp when the client certificate will expire.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::update_client_certificate::UpdateClientCertificateOutput::tags): <p>The collection of tags. Each tag element is associated with a given resource.</p>
    /// - On failure, responds with [`SdkError<UpdateClientCertificateError>`](crate::operation::update_client_certificate::UpdateClientCertificateError)
    pub fn update_client_certificate(
        &self,
    ) -> crate::operation::update_client_certificate::builders::UpdateClientCertificateFluentBuilder
    {
        crate::operation::update_client_certificate::builders::UpdateClientCertificateFluentBuilder::new(self.handle.clone())
    }
}
