// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetFieldOutput {
    /// <p>A list of detailed field information. </p>
    #[doc(hidden)]
    pub fields: ::std::option::Option<::std::vec::Vec<crate::types::GetFieldResponse>>,
    /// <p>A list of field errors. </p>
    #[doc(hidden)]
    pub errors: ::std::option::Option<::std::vec::Vec<crate::types::FieldError>>,
    _request_id: Option<String>,
}
impl BatchGetFieldOutput {
    /// <p>A list of detailed field information. </p>
    pub fn fields(&self) -> ::std::option::Option<&[crate::types::GetFieldResponse]> {
        self.fields.as_deref()
    }
    /// <p>A list of field errors. </p>
    pub fn errors(&self) -> ::std::option::Option<&[crate::types::FieldError]> {
        self.errors.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchGetFieldOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchGetFieldOutput {
    /// Creates a new builder-style object to manufacture [`BatchGetFieldOutput`](crate::operation::batch_get_field::BatchGetFieldOutput).
    pub fn builder() -> crate::operation::batch_get_field::builders::BatchGetFieldOutputBuilder {
        crate::operation::batch_get_field::builders::BatchGetFieldOutputBuilder::default()
    }
}

/// A builder for [`BatchGetFieldOutput`](crate::operation::batch_get_field::BatchGetFieldOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetFieldOutputBuilder {
    pub(crate) fields: ::std::option::Option<::std::vec::Vec<crate::types::GetFieldResponse>>,
    pub(crate) errors: ::std::option::Option<::std::vec::Vec<crate::types::FieldError>>,
    _request_id: Option<String>,
}
impl BatchGetFieldOutputBuilder {
    /// Appends an item to `fields`.
    ///
    /// To override the contents of this collection use [`set_fields`](Self::set_fields).
    ///
    /// <p>A list of detailed field information. </p>
    pub fn fields(mut self, input: crate::types::GetFieldResponse) -> Self {
        let mut v = self.fields.unwrap_or_default();
        v.push(input);
        self.fields = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of detailed field information. </p>
    pub fn set_fields(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::GetFieldResponse>>,
    ) -> Self {
        self.fields = input;
        self
    }
    /// Appends an item to `errors`.
    ///
    /// To override the contents of this collection use [`set_errors`](Self::set_errors).
    ///
    /// <p>A list of field errors. </p>
    pub fn errors(mut self, input: crate::types::FieldError) -> Self {
        let mut v = self.errors.unwrap_or_default();
        v.push(input);
        self.errors = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of field errors. </p>
    pub fn set_errors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FieldError>>,
    ) -> Self {
        self.errors = input;
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
    /// Consumes the builder and constructs a [`BatchGetFieldOutput`](crate::operation::batch_get_field::BatchGetFieldOutput).
    pub fn build(self) -> crate::operation::batch_get_field::BatchGetFieldOutput {
        crate::operation::batch_get_field::BatchGetFieldOutput {
            fields: self.fields,
            errors: self.errors,
            _request_id: self._request_id,
        }
    }
}
