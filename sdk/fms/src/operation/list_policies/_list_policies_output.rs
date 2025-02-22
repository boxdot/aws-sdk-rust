// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListPoliciesOutput {
    /// <p>An array of <code>PolicySummary</code> objects.</p>
    #[doc(hidden)]
    pub policy_list: ::std::option::Option<::std::vec::Vec<crate::types::PolicySummary>>,
    /// <p>If you have more <code>PolicySummary</code> objects than the number that you specified for <code>MaxResults</code> in the request, the response includes a <code>NextToken</code> value. To list more <code>PolicySummary</code> objects, submit another <code>ListPolicies</code> request, and specify the <code>NextToken</code> value from the response in the <code>NextToken</code> value in the next request.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListPoliciesOutput {
    /// <p>An array of <code>PolicySummary</code> objects.</p>
    pub fn policy_list(&self) -> ::std::option::Option<&[crate::types::PolicySummary]> {
        self.policy_list.as_deref()
    }
    /// <p>If you have more <code>PolicySummary</code> objects than the number that you specified for <code>MaxResults</code> in the request, the response includes a <code>NextToken</code> value. To list more <code>PolicySummary</code> objects, submit another <code>ListPolicies</code> request, and specify the <code>NextToken</code> value from the response in the <code>NextToken</code> value in the next request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListPoliciesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListPoliciesOutput {
    /// Creates a new builder-style object to manufacture [`ListPoliciesOutput`](crate::operation::list_policies::ListPoliciesOutput).
    pub fn builder() -> crate::operation::list_policies::builders::ListPoliciesOutputBuilder {
        crate::operation::list_policies::builders::ListPoliciesOutputBuilder::default()
    }
}

/// A builder for [`ListPoliciesOutput`](crate::operation::list_policies::ListPoliciesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListPoliciesOutputBuilder {
    pub(crate) policy_list: ::std::option::Option<::std::vec::Vec<crate::types::PolicySummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListPoliciesOutputBuilder {
    /// Appends an item to `policy_list`.
    ///
    /// To override the contents of this collection use [`set_policy_list`](Self::set_policy_list).
    ///
    /// <p>An array of <code>PolicySummary</code> objects.</p>
    pub fn policy_list(mut self, input: crate::types::PolicySummary) -> Self {
        let mut v = self.policy_list.unwrap_or_default();
        v.push(input);
        self.policy_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of <code>PolicySummary</code> objects.</p>
    pub fn set_policy_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicySummary>>,
    ) -> Self {
        self.policy_list = input;
        self
    }
    /// <p>If you have more <code>PolicySummary</code> objects than the number that you specified for <code>MaxResults</code> in the request, the response includes a <code>NextToken</code> value. To list more <code>PolicySummary</code> objects, submit another <code>ListPolicies</code> request, and specify the <code>NextToken</code> value from the response in the <code>NextToken</code> value in the next request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If you have more <code>PolicySummary</code> objects than the number that you specified for <code>MaxResults</code> in the request, the response includes a <code>NextToken</code> value. To list more <code>PolicySummary</code> objects, submit another <code>ListPolicies</code> request, and specify the <code>NextToken</code> value from the response in the <code>NextToken</code> value in the next request.</p>
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
    /// Consumes the builder and constructs a [`ListPoliciesOutput`](crate::operation::list_policies::ListPoliciesOutput).
    pub fn build(self) -> crate::operation::list_policies::ListPoliciesOutput {
        crate::operation::list_policies::ListPoliciesOutput {
            policy_list: self.policy_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
