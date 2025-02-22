// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchApplyUpdateActionOutput {
    /// <p>Update actions that have been processed successfully</p>
    #[doc(hidden)]
    pub processed_update_actions:
        ::std::option::Option<::std::vec::Vec<crate::types::ProcessedUpdateAction>>,
    /// <p>Update actions that haven't been processed successfully</p>
    #[doc(hidden)]
    pub unprocessed_update_actions:
        ::std::option::Option<::std::vec::Vec<crate::types::UnprocessedUpdateAction>>,
    _request_id: Option<String>,
}
impl BatchApplyUpdateActionOutput {
    /// <p>Update actions that have been processed successfully</p>
    pub fn processed_update_actions(
        &self,
    ) -> ::std::option::Option<&[crate::types::ProcessedUpdateAction]> {
        self.processed_update_actions.as_deref()
    }
    /// <p>Update actions that haven't been processed successfully</p>
    pub fn unprocessed_update_actions(
        &self,
    ) -> ::std::option::Option<&[crate::types::UnprocessedUpdateAction]> {
        self.unprocessed_update_actions.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchApplyUpdateActionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchApplyUpdateActionOutput {
    /// Creates a new builder-style object to manufacture [`BatchApplyUpdateActionOutput`](crate::operation::batch_apply_update_action::BatchApplyUpdateActionOutput).
    pub fn builder(
    ) -> crate::operation::batch_apply_update_action::builders::BatchApplyUpdateActionOutputBuilder
    {
        crate::operation::batch_apply_update_action::builders::BatchApplyUpdateActionOutputBuilder::default()
    }
}

/// A builder for [`BatchApplyUpdateActionOutput`](crate::operation::batch_apply_update_action::BatchApplyUpdateActionOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchApplyUpdateActionOutputBuilder {
    pub(crate) processed_update_actions:
        ::std::option::Option<::std::vec::Vec<crate::types::ProcessedUpdateAction>>,
    pub(crate) unprocessed_update_actions:
        ::std::option::Option<::std::vec::Vec<crate::types::UnprocessedUpdateAction>>,
    _request_id: Option<String>,
}
impl BatchApplyUpdateActionOutputBuilder {
    /// Appends an item to `processed_update_actions`.
    ///
    /// To override the contents of this collection use [`set_processed_update_actions`](Self::set_processed_update_actions).
    ///
    /// <p>Update actions that have been processed successfully</p>
    pub fn processed_update_actions(mut self, input: crate::types::ProcessedUpdateAction) -> Self {
        let mut v = self.processed_update_actions.unwrap_or_default();
        v.push(input);
        self.processed_update_actions = ::std::option::Option::Some(v);
        self
    }
    /// <p>Update actions that have been processed successfully</p>
    pub fn set_processed_update_actions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ProcessedUpdateAction>>,
    ) -> Self {
        self.processed_update_actions = input;
        self
    }
    /// Appends an item to `unprocessed_update_actions`.
    ///
    /// To override the contents of this collection use [`set_unprocessed_update_actions`](Self::set_unprocessed_update_actions).
    ///
    /// <p>Update actions that haven't been processed successfully</p>
    pub fn unprocessed_update_actions(
        mut self,
        input: crate::types::UnprocessedUpdateAction,
    ) -> Self {
        let mut v = self.unprocessed_update_actions.unwrap_or_default();
        v.push(input);
        self.unprocessed_update_actions = ::std::option::Option::Some(v);
        self
    }
    /// <p>Update actions that haven't been processed successfully</p>
    pub fn set_unprocessed_update_actions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::UnprocessedUpdateAction>>,
    ) -> Self {
        self.unprocessed_update_actions = input;
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
    /// Consumes the builder and constructs a [`BatchApplyUpdateActionOutput`](crate::operation::batch_apply_update_action::BatchApplyUpdateActionOutput).
    pub fn build(
        self,
    ) -> crate::operation::batch_apply_update_action::BatchApplyUpdateActionOutput {
        crate::operation::batch_apply_update_action::BatchApplyUpdateActionOutput {
            processed_update_actions: self.processed_update_actions,
            unprocessed_update_actions: self.unprocessed_update_actions,
            _request_id: self._request_id,
        }
    }
}
