// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteTaskTemplateInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for the task template.</p>
    #[doc(hidden)]
    pub task_template_id: ::std::option::Option<::std::string::String>,
}
impl DeleteTaskTemplateInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>A unique identifier for the task template.</p>
    pub fn task_template_id(&self) -> ::std::option::Option<&str> {
        self.task_template_id.as_deref()
    }
}
impl DeleteTaskTemplateInput {
    /// Creates a new builder-style object to manufacture [`DeleteTaskTemplateInput`](crate::operation::delete_task_template::DeleteTaskTemplateInput).
    pub fn builder(
    ) -> crate::operation::delete_task_template::builders::DeleteTaskTemplateInputBuilder {
        crate::operation::delete_task_template::builders::DeleteTaskTemplateInputBuilder::default()
    }
}

/// A builder for [`DeleteTaskTemplateInput`](crate::operation::delete_task_template::DeleteTaskTemplateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteTaskTemplateInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) task_template_id: ::std::option::Option<::std::string::String>,
}
impl DeleteTaskTemplateInputBuilder {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>A unique identifier for the task template.</p>
    pub fn task_template_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.task_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the task template.</p>
    pub fn set_task_template_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.task_template_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteTaskTemplateInput`](crate::operation::delete_task_template::DeleteTaskTemplateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_task_template::DeleteTaskTemplateInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_task_template::DeleteTaskTemplateInput {
                instance_id: self.instance_id,
                task_template_id: self.task_template_id,
            },
        )
    }
}
