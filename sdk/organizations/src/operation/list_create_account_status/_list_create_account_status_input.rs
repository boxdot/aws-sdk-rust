// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCreateAccountStatusInput {
    /// <p>A list of one or more states that you want included in the response. If this parameter isn't present, all requests are included in the response.</p>
    #[doc(hidden)]
    pub states: ::std::option::Option<::std::vec::Vec<crate::types::CreateAccountState>>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl ListCreateAccountStatusInput {
    /// <p>A list of one or more states that you want included in the response. If this parameter isn't present, all requests are included in the response.</p>
    pub fn states(&self) -> ::std::option::Option<&[crate::types::CreateAccountState]> {
        self.states.as_deref()
    }
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListCreateAccountStatusInput {
    /// Creates a new builder-style object to manufacture [`ListCreateAccountStatusInput`](crate::operation::list_create_account_status::ListCreateAccountStatusInput).
    pub fn builder(
    ) -> crate::operation::list_create_account_status::builders::ListCreateAccountStatusInputBuilder
    {
        crate::operation::list_create_account_status::builders::ListCreateAccountStatusInputBuilder::default()
    }
}

/// A builder for [`ListCreateAccountStatusInput`](crate::operation::list_create_account_status::ListCreateAccountStatusInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListCreateAccountStatusInputBuilder {
    pub(crate) states: ::std::option::Option<::std::vec::Vec<crate::types::CreateAccountState>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListCreateAccountStatusInputBuilder {
    /// Appends an item to `states`.
    ///
    /// To override the contents of this collection use [`set_states`](Self::set_states).
    ///
    /// <p>A list of one or more states that you want included in the response. If this parameter isn't present, all requests are included in the response.</p>
    pub fn states(mut self, input: crate::types::CreateAccountState) -> Self {
        let mut v = self.states.unwrap_or_default();
        v.push(input);
        self.states = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of one or more states that you want included in the response. If this parameter isn't present, all requests are included in the response.</p>
    pub fn set_states(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CreateAccountState>>,
    ) -> Self {
        self.states = input;
        self
    }
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListCreateAccountStatusInput`](crate::operation::list_create_account_status::ListCreateAccountStatusInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_create_account_status::ListCreateAccountStatusInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_create_account_status::ListCreateAccountStatusInput {
                states: self.states,
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}
