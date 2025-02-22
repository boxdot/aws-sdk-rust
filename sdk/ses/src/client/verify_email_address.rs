// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`VerifyEmailAddress`](crate::operation::verify_email_address::builders::VerifyEmailAddressFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`email_address(impl ::std::convert::Into<String>)`](crate::operation::verify_email_address::builders::VerifyEmailAddressFluentBuilder::email_address) / [`set_email_address(Option<String>)`](crate::operation::verify_email_address::builders::VerifyEmailAddressFluentBuilder::set_email_address): <p>The email address to be verified.</p>
    /// - On success, responds with [`VerifyEmailAddressOutput`](crate::operation::verify_email_address::VerifyEmailAddressOutput)
    /// - On failure, responds with [`SdkError<VerifyEmailAddressError>`](crate::operation::verify_email_address::VerifyEmailAddressError)
    pub fn verify_email_address(
        &self,
    ) -> crate::operation::verify_email_address::builders::VerifyEmailAddressFluentBuilder {
        crate::operation::verify_email_address::builders::VerifyEmailAddressFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
