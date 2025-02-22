// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetWorkflowOutput {
    /// <p>The workflow's ARN.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The workflow's ID.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The workflow's status.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::WorkflowStatus>,
    /// <p>The workflow's type.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::WorkflowType>,
    /// <p>The workflow's name.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The workflow's description.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The workflow's engine.</p>
    #[doc(hidden)]
    pub engine: ::std::option::Option<crate::types::WorkflowEngine>,
    /// <p>The workflow's definition.</p>
    #[doc(hidden)]
    pub definition: ::std::option::Option<::std::string::String>,
    /// <p>The path of the main definition file for the workflow.</p>
    #[doc(hidden)]
    pub main: ::std::option::Option<::std::string::String>,
    /// <p>The workflow's digest.</p>
    #[doc(hidden)]
    pub digest: ::std::option::Option<::std::string::String>,
    /// <p>The workflow's parameter template.</p>
    #[doc(hidden)]
    pub parameter_template: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::WorkflowParameter>,
    >,
    /// <p>The workflow's storage capacity in gigabytes.</p>
    #[doc(hidden)]
    pub storage_capacity: ::std::option::Option<i32>,
    /// <p>When the workflow was created.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The workflow's status message.</p>
    #[doc(hidden)]
    pub status_message: ::std::option::Option<::std::string::String>,
    /// <p>The workflow's tags.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p> Gets metadata for workflow. </p>
    #[doc(hidden)]
    pub metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p> The computational accelerator specified to run the workflow. </p>
    #[doc(hidden)]
    pub accelerators: ::std::option::Option<crate::types::Accelerators>,
    _request_id: Option<String>,
}
impl GetWorkflowOutput {
    /// <p>The workflow's ARN.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The workflow's ID.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The workflow's status.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::WorkflowStatus> {
        self.status.as_ref()
    }
    /// <p>The workflow's type.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::WorkflowType> {
        self.r#type.as_ref()
    }
    /// <p>The workflow's name.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The workflow's description.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The workflow's engine.</p>
    pub fn engine(&self) -> ::std::option::Option<&crate::types::WorkflowEngine> {
        self.engine.as_ref()
    }
    /// <p>The workflow's definition.</p>
    pub fn definition(&self) -> ::std::option::Option<&str> {
        self.definition.as_deref()
    }
    /// <p>The path of the main definition file for the workflow.</p>
    pub fn main(&self) -> ::std::option::Option<&str> {
        self.main.as_deref()
    }
    /// <p>The workflow's digest.</p>
    pub fn digest(&self) -> ::std::option::Option<&str> {
        self.digest.as_deref()
    }
    /// <p>The workflow's parameter template.</p>
    pub fn parameter_template(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::WorkflowParameter>,
    > {
        self.parameter_template.as_ref()
    }
    /// <p>The workflow's storage capacity in gigabytes.</p>
    pub fn storage_capacity(&self) -> ::std::option::Option<i32> {
        self.storage_capacity
    }
    /// <p>When the workflow was created.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The workflow's status message.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>The workflow's tags.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// <p> Gets metadata for workflow. </p>
    pub fn metadata(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.metadata.as_ref()
    }
    /// <p> The computational accelerator specified to run the workflow. </p>
    pub fn accelerators(&self) -> ::std::option::Option<&crate::types::Accelerators> {
        self.accelerators.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetWorkflowOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetWorkflowOutput {
    /// Creates a new builder-style object to manufacture [`GetWorkflowOutput`](crate::operation::get_workflow::GetWorkflowOutput).
    pub fn builder() -> crate::operation::get_workflow::builders::GetWorkflowOutputBuilder {
        crate::operation::get_workflow::builders::GetWorkflowOutputBuilder::default()
    }
}

/// A builder for [`GetWorkflowOutput`](crate::operation::get_workflow::GetWorkflowOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetWorkflowOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::WorkflowStatus>,
    pub(crate) r#type: ::std::option::Option<crate::types::WorkflowType>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) engine: ::std::option::Option<crate::types::WorkflowEngine>,
    pub(crate) definition: ::std::option::Option<::std::string::String>,
    pub(crate) main: ::std::option::Option<::std::string::String>,
    pub(crate) digest: ::std::option::Option<::std::string::String>,
    pub(crate) parameter_template: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::WorkflowParameter>,
    >,
    pub(crate) storage_capacity: ::std::option::Option<i32>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) accelerators: ::std::option::Option<crate::types::Accelerators>,
    _request_id: Option<String>,
}
impl GetWorkflowOutputBuilder {
    /// <p>The workflow's ARN.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The workflow's ARN.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The workflow's ID.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The workflow's ID.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The workflow's status.</p>
    pub fn status(mut self, input: crate::types::WorkflowStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The workflow's status.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::WorkflowStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The workflow's type.</p>
    pub fn r#type(mut self, input: crate::types::WorkflowType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The workflow's type.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::WorkflowType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The workflow's name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The workflow's name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The workflow's description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The workflow's description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The workflow's engine.</p>
    pub fn engine(mut self, input: crate::types::WorkflowEngine) -> Self {
        self.engine = ::std::option::Option::Some(input);
        self
    }
    /// <p>The workflow's engine.</p>
    pub fn set_engine(
        mut self,
        input: ::std::option::Option<crate::types::WorkflowEngine>,
    ) -> Self {
        self.engine = input;
        self
    }
    /// <p>The workflow's definition.</p>
    pub fn definition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.definition = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The workflow's definition.</p>
    pub fn set_definition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.definition = input;
        self
    }
    /// <p>The path of the main definition file for the workflow.</p>
    pub fn main(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.main = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The path of the main definition file for the workflow.</p>
    pub fn set_main(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.main = input;
        self
    }
    /// <p>The workflow's digest.</p>
    pub fn digest(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.digest = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The workflow's digest.</p>
    pub fn set_digest(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.digest = input;
        self
    }
    /// Adds a key-value pair to `parameter_template`.
    ///
    /// To override the contents of this collection use [`set_parameter_template`](Self::set_parameter_template).
    ///
    /// <p>The workflow's parameter template.</p>
    pub fn parameter_template(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::WorkflowParameter,
    ) -> Self {
        let mut hash_map = self.parameter_template.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.parameter_template = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The workflow's parameter template.</p>
    pub fn set_parameter_template(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::WorkflowParameter>,
        >,
    ) -> Self {
        self.parameter_template = input;
        self
    }
    /// <p>The workflow's storage capacity in gigabytes.</p>
    pub fn storage_capacity(mut self, input: i32) -> Self {
        self.storage_capacity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The workflow's storage capacity in gigabytes.</p>
    pub fn set_storage_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.storage_capacity = input;
        self
    }
    /// <p>When the workflow was created.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the workflow was created.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The workflow's status message.</p>
    pub fn status_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The workflow's status message.</p>
    pub fn set_status_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.status_message = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The workflow's tags.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The workflow's tags.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Adds a key-value pair to `metadata`.
    ///
    /// To override the contents of this collection use [`set_metadata`](Self::set_metadata).
    ///
    /// <p> Gets metadata for workflow. </p>
    pub fn metadata(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.metadata.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.metadata = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p> Gets metadata for workflow. </p>
    pub fn set_metadata(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.metadata = input;
        self
    }
    /// <p> The computational accelerator specified to run the workflow. </p>
    pub fn accelerators(mut self, input: crate::types::Accelerators) -> Self {
        self.accelerators = ::std::option::Option::Some(input);
        self
    }
    /// <p> The computational accelerator specified to run the workflow. </p>
    pub fn set_accelerators(
        mut self,
        input: ::std::option::Option<crate::types::Accelerators>,
    ) -> Self {
        self.accelerators = input;
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
    /// Consumes the builder and constructs a [`GetWorkflowOutput`](crate::operation::get_workflow::GetWorkflowOutput).
    pub fn build(self) -> crate::operation::get_workflow::GetWorkflowOutput {
        crate::operation::get_workflow::GetWorkflowOutput {
            arn: self.arn,
            id: self.id,
            status: self.status,
            r#type: self.r#type,
            name: self.name,
            description: self.description,
            engine: self.engine,
            definition: self.definition,
            main: self.main,
            digest: self.digest,
            parameter_template: self.parameter_template,
            storage_capacity: self.storage_capacity,
            creation_time: self.creation_time,
            status_message: self.status_message,
            tags: self.tags,
            metadata: self.metadata,
            accelerators: self.accelerators,
            _request_id: self._request_id,
        }
    }
}
