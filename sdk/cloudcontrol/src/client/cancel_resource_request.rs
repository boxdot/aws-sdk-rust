// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelResourceRequest`](crate::operation::cancel_resource_request::builders::CancelResourceRequestFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`request_token(impl ::std::convert::Into<String>)`](crate::operation::cancel_resource_request::builders::CancelResourceRequestFluentBuilder::request_token) / [`set_request_token(Option<String>)`](crate::operation::cancel_resource_request::builders::CancelResourceRequestFluentBuilder::set_request_token): <p>The <code>RequestToken</code> of the <code>ProgressEvent</code> object returned by the resource operation request.</p>
    /// - On success, responds with [`CancelResourceRequestOutput`](crate::operation::cancel_resource_request::CancelResourceRequestOutput) with field(s):
    ///   - [`progress_event(Option<ProgressEvent>)`](crate::operation::cancel_resource_request::CancelResourceRequestOutput::progress_event): <p>Represents the current status of a resource operation request. For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html">Managing resource operation requests</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    /// - On failure, responds with [`SdkError<CancelResourceRequestError>`](crate::operation::cancel_resource_request::CancelResourceRequestError)
    pub fn cancel_resource_request(
        &self,
    ) -> crate::operation::cancel_resource_request::builders::CancelResourceRequestFluentBuilder
    {
        crate::operation::cancel_resource_request::builders::CancelResourceRequestFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
