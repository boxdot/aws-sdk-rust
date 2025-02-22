// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAnalysisPermissions`](crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the analysis whose permissions you're updating. You must be using the Amazon Web Services account that the analysis is in.</p>
    ///   - [`analysis_id(impl ::std::convert::Into<String>)`](crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder::analysis_id) / [`set_analysis_id(Option<String>)`](crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder::set_analysis_id): <p>The ID of the analysis whose permissions you're updating. The ID is part of the analysis URL.</p>
    ///   - [`grant_permissions(Vec<ResourcePermission>)`](crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder::grant_permissions) / [`set_grant_permissions(Option<Vec<ResourcePermission>>)`](crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder::set_grant_permissions): <p>A structure that describes the permissions to add and the principal to add them to.</p>
    ///   - [`revoke_permissions(Vec<ResourcePermission>)`](crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder::revoke_permissions) / [`set_revoke_permissions(Option<Vec<ResourcePermission>>)`](crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder::set_revoke_permissions): <p>A structure that describes the permissions to remove and the principal to remove them from.</p>
    /// - On success, responds with [`UpdateAnalysisPermissionsOutput`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsOutput) with field(s):
    ///   - [`analysis_arn(Option<String>)`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsOutput::analysis_arn): <p>The Amazon Resource Name (ARN) of the analysis that you updated.</p>
    ///   - [`analysis_id(Option<String>)`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsOutput::analysis_id): <p>The ID of the analysis that you updated permissions for.</p>
    ///   - [`permissions(Option<Vec<ResourcePermission>>)`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsOutput::permissions): <p>A structure that describes the principals and the resource-level permissions on an analysis.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<UpdateAnalysisPermissionsError>`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsError)
    pub fn update_analysis_permissions(&self) -> crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder{
        crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsFluentBuilder::new(self.handle.clone())
    }
}
