// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListMailDomains`](crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_id(impl ::std::convert::Into<String>)`](crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder::organization_id) / [`set_organization_id(Option<String>)`](crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder::set_organization_id): <p>The WorkMail organization for which to list domains.</p>
    ///   - [`max_results(i32)`](crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder::set_max_results): <p>The maximum number of results to return in a single call.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder::set_next_token): <p>The token to use to retrieve the next page of results. The first call does not require a token.</p>
    /// - On success, responds with [`ListMailDomainsOutput`](crate::operation::list_mail_domains::ListMailDomainsOutput) with field(s):
    ///   - [`mail_domains(Option<Vec<MailDomainSummary>>)`](crate::operation::list_mail_domains::ListMailDomainsOutput::mail_domains): <p>The list of mail domain summaries, specifying domains that exist in the specified WorkMail organization, along with the information about whether the domain is or isn't the default.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_mail_domains::ListMailDomainsOutput::next_token): <p>The token to use to retrieve the next page of results. The value becomes <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListMailDomainsError>`](crate::operation::list_mail_domains::ListMailDomainsError)
    pub fn list_mail_domains(
        &self,
    ) -> crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder {
        crate::operation::list_mail_domains::builders::ListMailDomainsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
