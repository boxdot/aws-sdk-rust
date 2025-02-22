// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeregisterMailDomain`](crate::operation::deregister_mail_domain::builders::DeregisterMailDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_id(impl ::std::convert::Into<String>)`](crate::operation::deregister_mail_domain::builders::DeregisterMailDomainFluentBuilder::organization_id) / [`set_organization_id(Option<String>)`](crate::operation::deregister_mail_domain::builders::DeregisterMailDomainFluentBuilder::set_organization_id): <p>The WorkMail organization for which the domain will be deregistered.</p>
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::deregister_mail_domain::builders::DeregisterMailDomainFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::deregister_mail_domain::builders::DeregisterMailDomainFluentBuilder::set_domain_name): <p>The domain to deregister in WorkMail and SES.</p>
    /// - On success, responds with [`DeregisterMailDomainOutput`](crate::operation::deregister_mail_domain::DeregisterMailDomainOutput)
    /// - On failure, responds with [`SdkError<DeregisterMailDomainError>`](crate::operation::deregister_mail_domain::DeregisterMailDomainError)
    pub fn deregister_mail_domain(
        &self,
    ) -> crate::operation::deregister_mail_domain::builders::DeregisterMailDomainFluentBuilder {
        crate::operation::deregister_mail_domain::builders::DeregisterMailDomainFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
