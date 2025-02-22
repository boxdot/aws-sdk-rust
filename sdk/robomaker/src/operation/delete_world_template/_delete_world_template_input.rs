// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteWorldTemplateInput {
    /// <p>The Amazon Resource Name (arn) of the world template you want to delete.</p>
    #[doc(hidden)]
    pub template: ::std::option::Option<::std::string::String>,
}
impl DeleteWorldTemplateInput {
    /// <p>The Amazon Resource Name (arn) of the world template you want to delete.</p>
    pub fn template(&self) -> ::std::option::Option<&str> {
        self.template.as_deref()
    }
}
impl DeleteWorldTemplateInput {
    /// Creates a new builder-style object to manufacture [`DeleteWorldTemplateInput`](crate::operation::delete_world_template::DeleteWorldTemplateInput).
    pub fn builder(
    ) -> crate::operation::delete_world_template::builders::DeleteWorldTemplateInputBuilder {
        crate::operation::delete_world_template::builders::DeleteWorldTemplateInputBuilder::default(
        )
    }
}

/// A builder for [`DeleteWorldTemplateInput`](crate::operation::delete_world_template::DeleteWorldTemplateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteWorldTemplateInputBuilder {
    pub(crate) template: ::std::option::Option<::std::string::String>,
}
impl DeleteWorldTemplateInputBuilder {
    /// <p>The Amazon Resource Name (arn) of the world template you want to delete.</p>
    pub fn template(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.template = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (arn) of the world template you want to delete.</p>
    pub fn set_template(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.template = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteWorldTemplateInput`](crate::operation::delete_world_template::DeleteWorldTemplateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_world_template::DeleteWorldTemplateInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_world_template::DeleteWorldTemplateInput {
                template: self.template,
            },
        )
    }
}
