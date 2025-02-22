// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AcceptInvitation`](crate::operation::accept_invitation::builders::AcceptInvitationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`graph_arn(impl ::std::convert::Into<String>)`](crate::operation::accept_invitation::builders::AcceptInvitationFluentBuilder::graph_arn) / [`set_graph_arn(Option<String>)`](crate::operation::accept_invitation::builders::AcceptInvitationFluentBuilder::set_graph_arn): <p>The ARN of the behavior graph that the member account is accepting the invitation for.</p>  <p>The member account status in the behavior graph must be <code>INVITED</code>.</p>
    /// - On success, responds with [`AcceptInvitationOutput`](crate::operation::accept_invitation::AcceptInvitationOutput)
    /// - On failure, responds with [`SdkError<AcceptInvitationError>`](crate::operation::accept_invitation::AcceptInvitationError)
    pub fn accept_invitation(
        &self,
    ) -> crate::operation::accept_invitation::builders::AcceptInvitationFluentBuilder {
        crate::operation::accept_invitation::builders::AcceptInvitationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
