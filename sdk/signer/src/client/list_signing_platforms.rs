// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSigningPlatforms`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`category(impl ::std::convert::Into<String>)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::category) / [`set_category(Option<String>)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::set_category): <p>The category type of a signing platform.</p>
    ///   - [`partner(impl ::std::convert::Into<String>)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::partner) / [`set_partner(Option<String>)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::set_partner): <p>Any partner entities connected to a signing platform.</p>
    ///   - [`target(impl ::std::convert::Into<String>)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::target) / [`set_target(Option<String>)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::set_target): <p>The validation template that is used by the target signing platform.</p>
    ///   - [`max_results(i32)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::set_max_results): <p>The maximum number of results to be returned by this operation.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::set_next_token): <p>Value for specifying the next set of paginated results to return. After you receive a response with truncated results, use this parameter in a subsequent request. Set it to the value of <code>nextToken</code> from the response that you just received.</p>
    /// - On success, responds with [`ListSigningPlatformsOutput`](crate::operation::list_signing_platforms::ListSigningPlatformsOutput) with field(s):
    ///   - [`platforms(Option<Vec<SigningPlatform>>)`](crate::operation::list_signing_platforms::ListSigningPlatformsOutput::platforms): <p>A list of all platforms that match the request parameters.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_signing_platforms::ListSigningPlatformsOutput::next_token): <p>Value for specifying the next set of paginated results to return.</p>
    /// - On failure, responds with [`SdkError<ListSigningPlatformsError>`](crate::operation::list_signing_platforms::ListSigningPlatformsError)
    pub fn list_signing_platforms(
        &self,
    ) -> crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder {
        crate::operation::list_signing_platforms::builders::ListSigningPlatformsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
