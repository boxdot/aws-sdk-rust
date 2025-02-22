// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateContactFlowModuleContentInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the flow module.</p>
    #[doc(hidden)]
    pub contact_flow_module_id: ::std::option::Option<::std::string::String>,
    /// <p>The content of the flow module.</p>
    #[doc(hidden)]
    pub content: ::std::option::Option<::std::string::String>,
}
impl UpdateContactFlowModuleContentInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The identifier of the flow module.</p>
    pub fn contact_flow_module_id(&self) -> ::std::option::Option<&str> {
        self.contact_flow_module_id.as_deref()
    }
    /// <p>The content of the flow module.</p>
    pub fn content(&self) -> ::std::option::Option<&str> {
        self.content.as_deref()
    }
}
impl UpdateContactFlowModuleContentInput {
    /// Creates a new builder-style object to manufacture [`UpdateContactFlowModuleContentInput`](crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentInput).
    pub fn builder() -> crate::operation::update_contact_flow_module_content::builders::UpdateContactFlowModuleContentInputBuilder{
        crate::operation::update_contact_flow_module_content::builders::UpdateContactFlowModuleContentInputBuilder::default()
    }
}

/// A builder for [`UpdateContactFlowModuleContentInput`](crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateContactFlowModuleContentInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) contact_flow_module_id: ::std::option::Option<::std::string::String>,
    pub(crate) content: ::std::option::Option<::std::string::String>,
}
impl UpdateContactFlowModuleContentInputBuilder {
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
    /// <p>The identifier of the flow module.</p>
    pub fn contact_flow_module_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.contact_flow_module_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the flow module.</p>
    pub fn set_contact_flow_module_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.contact_flow_module_id = input;
        self
    }
    /// <p>The content of the flow module.</p>
    pub fn content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The content of the flow module.</p>
    pub fn set_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateContactFlowModuleContentInput`](crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentInput {
                instance_id: self.instance_id
                ,
                contact_flow_module_id: self.contact_flow_module_id
                ,
                content: self.content
                ,
            }
        )
    }
}
