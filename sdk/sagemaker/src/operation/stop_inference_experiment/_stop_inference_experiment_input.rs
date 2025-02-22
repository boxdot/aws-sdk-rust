// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopInferenceExperimentInput {
    /// <p>The name of the inference experiment to stop.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p> Array of key-value pairs, with names of variants mapped to actions. The possible actions are the following: </p>
    /// <ul>
    /// <li> <p> <code>Promote</code> - Promote the shadow variant to a production variant</p> </li>
    /// <li> <p> <code>Remove</code> - Delete the variant</p> </li>
    /// <li> <p> <code>Retain</code> - Keep the variant as it is</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub model_variant_actions: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::ModelVariantAction>,
    >,
    /// <p> An array of <code>ModelVariantConfig</code> objects. There is one for each variant that you want to deploy after the inference experiment stops. Each <code>ModelVariantConfig</code> describes the infrastructure configuration for deploying the corresponding variant. </p>
    #[doc(hidden)]
    pub desired_model_variants:
        ::std::option::Option<::std::vec::Vec<crate::types::ModelVariantConfig>>,
    /// <p> The desired state of the experiment after stopping. The possible states are the following: </p>
    /// <ul>
    /// <li> <p> <code>Completed</code>: The experiment completed successfully</p> </li>
    /// <li> <p> <code>Cancelled</code>: The experiment was canceled</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub desired_state: ::std::option::Option<crate::types::InferenceExperimentStopDesiredState>,
    /// <p>The reason for stopping the experiment.</p>
    #[doc(hidden)]
    pub reason: ::std::option::Option<::std::string::String>,
}
impl StopInferenceExperimentInput {
    /// <p>The name of the inference experiment to stop.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p> Array of key-value pairs, with names of variants mapped to actions. The possible actions are the following: </p>
    /// <ul>
    /// <li> <p> <code>Promote</code> - Promote the shadow variant to a production variant</p> </li>
    /// <li> <p> <code>Remove</code> - Delete the variant</p> </li>
    /// <li> <p> <code>Retain</code> - Keep the variant as it is</p> </li>
    /// </ul>
    pub fn model_variant_actions(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::ModelVariantAction>,
    > {
        self.model_variant_actions.as_ref()
    }
    /// <p> An array of <code>ModelVariantConfig</code> objects. There is one for each variant that you want to deploy after the inference experiment stops. Each <code>ModelVariantConfig</code> describes the infrastructure configuration for deploying the corresponding variant. </p>
    pub fn desired_model_variants(
        &self,
    ) -> ::std::option::Option<&[crate::types::ModelVariantConfig]> {
        self.desired_model_variants.as_deref()
    }
    /// <p> The desired state of the experiment after stopping. The possible states are the following: </p>
    /// <ul>
    /// <li> <p> <code>Completed</code>: The experiment completed successfully</p> </li>
    /// <li> <p> <code>Cancelled</code>: The experiment was canceled</p> </li>
    /// </ul>
    pub fn desired_state(
        &self,
    ) -> ::std::option::Option<&crate::types::InferenceExperimentStopDesiredState> {
        self.desired_state.as_ref()
    }
    /// <p>The reason for stopping the experiment.</p>
    pub fn reason(&self) -> ::std::option::Option<&str> {
        self.reason.as_deref()
    }
}
impl StopInferenceExperimentInput {
    /// Creates a new builder-style object to manufacture [`StopInferenceExperimentInput`](crate::operation::stop_inference_experiment::StopInferenceExperimentInput).
    pub fn builder(
    ) -> crate::operation::stop_inference_experiment::builders::StopInferenceExperimentInputBuilder
    {
        crate::operation::stop_inference_experiment::builders::StopInferenceExperimentInputBuilder::default()
    }
}

/// A builder for [`StopInferenceExperimentInput`](crate::operation::stop_inference_experiment::StopInferenceExperimentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StopInferenceExperimentInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) model_variant_actions: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::ModelVariantAction>,
    >,
    pub(crate) desired_model_variants:
        ::std::option::Option<::std::vec::Vec<crate::types::ModelVariantConfig>>,
    pub(crate) desired_state:
        ::std::option::Option<crate::types::InferenceExperimentStopDesiredState>,
    pub(crate) reason: ::std::option::Option<::std::string::String>,
}
impl StopInferenceExperimentInputBuilder {
    /// <p>The name of the inference experiment to stop.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the inference experiment to stop.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Adds a key-value pair to `model_variant_actions`.
    ///
    /// To override the contents of this collection use [`set_model_variant_actions`](Self::set_model_variant_actions).
    ///
    /// <p> Array of key-value pairs, with names of variants mapped to actions. The possible actions are the following: </p>
    /// <ul>
    /// <li> <p> <code>Promote</code> - Promote the shadow variant to a production variant</p> </li>
    /// <li> <p> <code>Remove</code> - Delete the variant</p> </li>
    /// <li> <p> <code>Retain</code> - Keep the variant as it is</p> </li>
    /// </ul>
    pub fn model_variant_actions(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::ModelVariantAction,
    ) -> Self {
        let mut hash_map = self.model_variant_actions.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.model_variant_actions = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p> Array of key-value pairs, with names of variants mapped to actions. The possible actions are the following: </p>
    /// <ul>
    /// <li> <p> <code>Promote</code> - Promote the shadow variant to a production variant</p> </li>
    /// <li> <p> <code>Remove</code> - Delete the variant</p> </li>
    /// <li> <p> <code>Retain</code> - Keep the variant as it is</p> </li>
    /// </ul>
    pub fn set_model_variant_actions(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::ModelVariantAction>,
        >,
    ) -> Self {
        self.model_variant_actions = input;
        self
    }
    /// Appends an item to `desired_model_variants`.
    ///
    /// To override the contents of this collection use [`set_desired_model_variants`](Self::set_desired_model_variants).
    ///
    /// <p> An array of <code>ModelVariantConfig</code> objects. There is one for each variant that you want to deploy after the inference experiment stops. Each <code>ModelVariantConfig</code> describes the infrastructure configuration for deploying the corresponding variant. </p>
    pub fn desired_model_variants(mut self, input: crate::types::ModelVariantConfig) -> Self {
        let mut v = self.desired_model_variants.unwrap_or_default();
        v.push(input);
        self.desired_model_variants = ::std::option::Option::Some(v);
        self
    }
    /// <p> An array of <code>ModelVariantConfig</code> objects. There is one for each variant that you want to deploy after the inference experiment stops. Each <code>ModelVariantConfig</code> describes the infrastructure configuration for deploying the corresponding variant. </p>
    pub fn set_desired_model_variants(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ModelVariantConfig>>,
    ) -> Self {
        self.desired_model_variants = input;
        self
    }
    /// <p> The desired state of the experiment after stopping. The possible states are the following: </p>
    /// <ul>
    /// <li> <p> <code>Completed</code>: The experiment completed successfully</p> </li>
    /// <li> <p> <code>Cancelled</code>: The experiment was canceled</p> </li>
    /// </ul>
    pub fn desired_state(
        mut self,
        input: crate::types::InferenceExperimentStopDesiredState,
    ) -> Self {
        self.desired_state = ::std::option::Option::Some(input);
        self
    }
    /// <p> The desired state of the experiment after stopping. The possible states are the following: </p>
    /// <ul>
    /// <li> <p> <code>Completed</code>: The experiment completed successfully</p> </li>
    /// <li> <p> <code>Cancelled</code>: The experiment was canceled</p> </li>
    /// </ul>
    pub fn set_desired_state(
        mut self,
        input: ::std::option::Option<crate::types::InferenceExperimentStopDesiredState>,
    ) -> Self {
        self.desired_state = input;
        self
    }
    /// <p>The reason for stopping the experiment.</p>
    pub fn reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason for stopping the experiment.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// Consumes the builder and constructs a [`StopInferenceExperimentInput`](crate::operation::stop_inference_experiment::StopInferenceExperimentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::stop_inference_experiment::StopInferenceExperimentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::stop_inference_experiment::StopInferenceExperimentInput {
                name: self.name,
                model_variant_actions: self.model_variant_actions,
                desired_model_variants: self.desired_model_variants,
                desired_state: self.desired_state,
                reason: self.reason,
            },
        )
    }
}
