// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListModelPackageGroupsOutput {
    /// <p>A list of summaries of the model groups in your Amazon Web Services account.</p>
    #[doc(hidden)]
    pub model_package_group_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::ModelPackageGroupSummary>>,
    /// <p>If the response is truncated, SageMaker returns this token. To retrieve the next set of model groups, use it in the subsequent request.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListModelPackageGroupsOutput {
    /// <p>A list of summaries of the model groups in your Amazon Web Services account.</p>
    pub fn model_package_group_summary_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::ModelPackageGroupSummary]> {
        self.model_package_group_summary_list.as_deref()
    }
    /// <p>If the response is truncated, SageMaker returns this token. To retrieve the next set of model groups, use it in the subsequent request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListModelPackageGroupsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListModelPackageGroupsOutput {
    /// Creates a new builder-style object to manufacture [`ListModelPackageGroupsOutput`](crate::operation::list_model_package_groups::ListModelPackageGroupsOutput).
    pub fn builder(
    ) -> crate::operation::list_model_package_groups::builders::ListModelPackageGroupsOutputBuilder
    {
        crate::operation::list_model_package_groups::builders::ListModelPackageGroupsOutputBuilder::default()
    }
}

/// A builder for [`ListModelPackageGroupsOutput`](crate::operation::list_model_package_groups::ListModelPackageGroupsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListModelPackageGroupsOutputBuilder {
    pub(crate) model_package_group_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::ModelPackageGroupSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListModelPackageGroupsOutputBuilder {
    /// Appends an item to `model_package_group_summary_list`.
    ///
    /// To override the contents of this collection use [`set_model_package_group_summary_list`](Self::set_model_package_group_summary_list).
    ///
    /// <p>A list of summaries of the model groups in your Amazon Web Services account.</p>
    pub fn model_package_group_summary_list(
        mut self,
        input: crate::types::ModelPackageGroupSummary,
    ) -> Self {
        let mut v = self.model_package_group_summary_list.unwrap_or_default();
        v.push(input);
        self.model_package_group_summary_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of summaries of the model groups in your Amazon Web Services account.</p>
    pub fn set_model_package_group_summary_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ModelPackageGroupSummary>>,
    ) -> Self {
        self.model_package_group_summary_list = input;
        self
    }
    /// <p>If the response is truncated, SageMaker returns this token. To retrieve the next set of model groups, use it in the subsequent request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the response is truncated, SageMaker returns this token. To retrieve the next set of model groups, use it in the subsequent request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListModelPackageGroupsOutput`](crate::operation::list_model_package_groups::ListModelPackageGroupsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_model_package_groups::ListModelPackageGroupsOutput {
        crate::operation::list_model_package_groups::ListModelPackageGroupsOutput {
            model_package_group_summary_list: self.model_package_group_summary_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
