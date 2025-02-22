// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAccessToken`](crate::operation::get_access_token::builders::GetAccessTokenFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`token(impl ::std::convert::Into<String>)`](crate::operation::get_access_token::builders::GetAccessTokenFluentBuilder::token) / [`set_token(Option<String>)`](crate::operation::get_access_token::builders::GetAccessTokenFluentBuilder::set_token): <p>Refresh token, encoded as a JWT token.</p>
    ///   - [`token_properties(Vec<String>)`](crate::operation::get_access_token::builders::GetAccessTokenFluentBuilder::token_properties) / [`set_token_properties(Option<Vec<String>>)`](crate::operation::get_access_token::builders::GetAccessTokenFluentBuilder::set_token_properties): <p>Token properties to validate against those present in the JWT token.</p>
    /// - On success, responds with [`GetAccessTokenOutput`](crate::operation::get_access_token::GetAccessTokenOutput) with field(s):
    ///   - [`access_token(Option<String>)`](crate::operation::get_access_token::GetAccessTokenOutput::access_token): <p>Temporary access token.</p>
    /// - On failure, responds with [`SdkError<GetAccessTokenError>`](crate::operation::get_access_token::GetAccessTokenError)
    pub fn get_access_token(
        &self,
    ) -> crate::operation::get_access_token::builders::GetAccessTokenFluentBuilder {
        crate::operation::get_access_token::builders::GetAccessTokenFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
