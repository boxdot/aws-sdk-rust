// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Inputs for the model explainability job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModelExplainabilityJobInput {
    /// <p>Input object for the endpoint</p>
    #[doc(hidden)]
    pub endpoint_input: ::std::option::Option<crate::types::EndpointInput>,
    /// <p>Input object for the batch transform job.</p>
    #[doc(hidden)]
    pub batch_transform_input: ::std::option::Option<crate::types::BatchTransformInput>,
}
impl ModelExplainabilityJobInput {
    /// <p>Input object for the endpoint</p>
    pub fn endpoint_input(&self) -> ::std::option::Option<&crate::types::EndpointInput> {
        self.endpoint_input.as_ref()
    }
    /// <p>Input object for the batch transform job.</p>
    pub fn batch_transform_input(
        &self,
    ) -> ::std::option::Option<&crate::types::BatchTransformInput> {
        self.batch_transform_input.as_ref()
    }
}
impl ModelExplainabilityJobInput {
    /// Creates a new builder-style object to manufacture [`ModelExplainabilityJobInput`](crate::types::ModelExplainabilityJobInput).
    pub fn builder() -> crate::types::builders::ModelExplainabilityJobInputBuilder {
        crate::types::builders::ModelExplainabilityJobInputBuilder::default()
    }
}

/// A builder for [`ModelExplainabilityJobInput`](crate::types::ModelExplainabilityJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ModelExplainabilityJobInputBuilder {
    pub(crate) endpoint_input: ::std::option::Option<crate::types::EndpointInput>,
    pub(crate) batch_transform_input: ::std::option::Option<crate::types::BatchTransformInput>,
}
impl ModelExplainabilityJobInputBuilder {
    /// <p>Input object for the endpoint</p>
    pub fn endpoint_input(mut self, input: crate::types::EndpointInput) -> Self {
        self.endpoint_input = ::std::option::Option::Some(input);
        self
    }
    /// <p>Input object for the endpoint</p>
    pub fn set_endpoint_input(
        mut self,
        input: ::std::option::Option<crate::types::EndpointInput>,
    ) -> Self {
        self.endpoint_input = input;
        self
    }
    /// <p>Input object for the batch transform job.</p>
    pub fn batch_transform_input(mut self, input: crate::types::BatchTransformInput) -> Self {
        self.batch_transform_input = ::std::option::Option::Some(input);
        self
    }
    /// <p>Input object for the batch transform job.</p>
    pub fn set_batch_transform_input(
        mut self,
        input: ::std::option::Option<crate::types::BatchTransformInput>,
    ) -> Self {
        self.batch_transform_input = input;
        self
    }
    /// Consumes the builder and constructs a [`ModelExplainabilityJobInput`](crate::types::ModelExplainabilityJobInput).
    pub fn build(self) -> crate::types::ModelExplainabilityJobInput {
        crate::types::ModelExplainabilityJobInput {
            endpoint_input: self.endpoint_input,
            batch_transform_input: self.batch_transform_input,
        }
    }
}
