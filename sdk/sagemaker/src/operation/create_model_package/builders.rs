// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_model_package::_create_model_package_output::CreateModelPackageOutputBuilder;

pub use crate::operation::create_model_package::_create_model_package_input::CreateModelPackageInputBuilder;

/// Fluent builder constructing a request to `CreateModelPackage`.
///
/// <p>Creates a model package that you can use to create SageMaker models or list on Amazon Web Services Marketplace, or a versioned model that is part of a model group. Buyers can subscribe to model packages listed on Amazon Web Services Marketplace to create models in SageMaker.</p>
/// <p>To create a model package by specifying a Docker container that contains your inference code and the Amazon S3 location of your model artifacts, provide values for <code>InferenceSpecification</code>. To create a model from an algorithm resource that you created or subscribed to in Amazon Web Services Marketplace, provide a value for <code>SourceAlgorithmSpecification</code>.</p> <note>
/// <p>There are two types of model packages:</p>
/// <ul>
/// <li> <p>Versioned - a model that is part of a model group in the model registry.</p> </li>
/// <li> <p>Unversioned - a model package that is not part of a model group.</p> </li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateModelPackageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_model_package::builders::CreateModelPackageInputBuilder,
}
impl CreateModelPackageFluentBuilder {
    /// Creates a new `CreateModelPackage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_model_package::CreateModelPackage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_model_package::CreateModelPackageError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_model_package::CreateModelPackageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_model_package::CreateModelPackageError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_model_package::CreateModelPackageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_model_package::CreateModelPackageError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_model_package::CreateModelPackage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_model_package::CreateModelPackageError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the model package. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    /// <p>This parameter is required for unversioned models. It is not applicable to versioned models.</p>
    pub fn model_package_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.model_package_name(input.into());
        self
    }
    /// <p>The name of the model package. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    /// <p>This parameter is required for unversioned models. It is not applicable to versioned models.</p>
    pub fn set_model_package_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_model_package_name(input);
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the model package group that this model version belongs to.</p>
    /// <p>This parameter is required for versioned models, and does not apply to unversioned models.</p>
    pub fn model_package_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.model_package_group_name(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the model package group that this model version belongs to.</p>
    /// <p>This parameter is required for versioned models, and does not apply to unversioned models.</p>
    pub fn set_model_package_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_model_package_group_name(input);
        self
    }
    /// <p>A description of the model package.</p>
    pub fn model_package_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.model_package_description(input.into());
        self
    }
    /// <p>A description of the model package.</p>
    pub fn set_model_package_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_model_package_description(input);
        self
    }
    /// <p>Specifies details about inference jobs that can be run with models based on this model package, including the following:</p>
    /// <ul>
    /// <li> <p>The Amazon ECR paths of containers that contain the inference code and model artifacts.</p> </li>
    /// <li> <p>The instance types that the model package supports for transform jobs and real-time endpoints used for inference.</p> </li>
    /// <li> <p>The input and output content formats that the model package supports for inference.</p> </li>
    /// </ul>
    pub fn inference_specification(mut self, input: crate::types::InferenceSpecification) -> Self {
        self.inner = self.inner.inference_specification(input);
        self
    }
    /// <p>Specifies details about inference jobs that can be run with models based on this model package, including the following:</p>
    /// <ul>
    /// <li> <p>The Amazon ECR paths of containers that contain the inference code and model artifacts.</p> </li>
    /// <li> <p>The instance types that the model package supports for transform jobs and real-time endpoints used for inference.</p> </li>
    /// <li> <p>The input and output content formats that the model package supports for inference.</p> </li>
    /// </ul>
    pub fn set_inference_specification(
        mut self,
        input: ::std::option::Option<crate::types::InferenceSpecification>,
    ) -> Self {
        self.inner = self.inner.set_inference_specification(input);
        self
    }
    /// <p>Specifies configurations for one or more transform jobs that SageMaker runs to test the model package.</p>
    pub fn validation_specification(
        mut self,
        input: crate::types::ModelPackageValidationSpecification,
    ) -> Self {
        self.inner = self.inner.validation_specification(input);
        self
    }
    /// <p>Specifies configurations for one or more transform jobs that SageMaker runs to test the model package.</p>
    pub fn set_validation_specification(
        mut self,
        input: ::std::option::Option<crate::types::ModelPackageValidationSpecification>,
    ) -> Self {
        self.inner = self.inner.set_validation_specification(input);
        self
    }
    /// <p>Details about the algorithm that was used to create the model package.</p>
    pub fn source_algorithm_specification(
        mut self,
        input: crate::types::SourceAlgorithmSpecification,
    ) -> Self {
        self.inner = self.inner.source_algorithm_specification(input);
        self
    }
    /// <p>Details about the algorithm that was used to create the model package.</p>
    pub fn set_source_algorithm_specification(
        mut self,
        input: ::std::option::Option<crate::types::SourceAlgorithmSpecification>,
    ) -> Self {
        self.inner = self.inner.set_source_algorithm_specification(input);
        self
    }
    /// <p>Whether to certify the model package for listing on Amazon Web Services Marketplace.</p>
    /// <p>This parameter is optional for unversioned models, and does not apply to versioned models.</p>
    pub fn certify_for_marketplace(mut self, input: bool) -> Self {
        self.inner = self.inner.certify_for_marketplace(input);
        self
    }
    /// <p>Whether to certify the model package for listing on Amazon Web Services Marketplace.</p>
    /// <p>This parameter is optional for unversioned models, and does not apply to versioned models.</p>
    pub fn set_certify_for_marketplace(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_certify_for_marketplace(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of key value pairs associated with the model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of key value pairs associated with the model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Whether the model is approved for deployment.</p>
    /// <p>This parameter is optional for versioned models, and does not apply to unversioned models.</p>
    /// <p>For versioned models, the value of this parameter must be set to <code>Approved</code> to deploy the model.</p>
    pub fn model_approval_status(mut self, input: crate::types::ModelApprovalStatus) -> Self {
        self.inner = self.inner.model_approval_status(input);
        self
    }
    /// <p>Whether the model is approved for deployment.</p>
    /// <p>This parameter is optional for versioned models, and does not apply to unversioned models.</p>
    /// <p>For versioned models, the value of this parameter must be set to <code>Approved</code> to deploy the model.</p>
    pub fn set_model_approval_status(
        mut self,
        input: ::std::option::Option<crate::types::ModelApprovalStatus>,
    ) -> Self {
        self.inner = self.inner.set_model_approval_status(input);
        self
    }
    /// <p>Metadata properties of the tracking entity, trial, or trial component.</p>
    pub fn metadata_properties(mut self, input: crate::types::MetadataProperties) -> Self {
        self.inner = self.inner.metadata_properties(input);
        self
    }
    /// <p>Metadata properties of the tracking entity, trial, or trial component.</p>
    pub fn set_metadata_properties(
        mut self,
        input: ::std::option::Option<crate::types::MetadataProperties>,
    ) -> Self {
        self.inner = self.inner.set_metadata_properties(input);
        self
    }
    /// <p>A structure that contains model metrics reports.</p>
    pub fn model_metrics(mut self, input: crate::types::ModelMetrics) -> Self {
        self.inner = self.inner.model_metrics(input);
        self
    }
    /// <p>A structure that contains model metrics reports.</p>
    pub fn set_model_metrics(
        mut self,
        input: ::std::option::Option<crate::types::ModelMetrics>,
    ) -> Self {
        self.inner = self.inner.set_model_metrics(input);
        self
    }
    /// <p>A unique token that guarantees that the call to this API is idempotent.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique token that guarantees that the call to this API is idempotent.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Adds a key-value pair to `CustomerMetadataProperties`.
    ///
    /// To override the contents of this collection use [`set_customer_metadata_properties`](Self::set_customer_metadata_properties).
    ///
    /// <p>The metadata properties associated with the model package versions.</p>
    pub fn customer_metadata_properties(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.customer_metadata_properties(k.into(), v.into());
        self
    }
    /// <p>The metadata properties associated with the model package versions.</p>
    pub fn set_customer_metadata_properties(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_customer_metadata_properties(input);
        self
    }
    /// <p>Represents the drift check baselines that can be used when the model monitor is set using the model package. For more information, see the topic on <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/pipelines-quality-clarify-baseline-lifecycle.html#pipelines-quality-clarify-baseline-drift-detection">Drift Detection against Previous Baselines in SageMaker Pipelines</a> in the <i>Amazon SageMaker Developer Guide</i>. </p>
    pub fn drift_check_baselines(mut self, input: crate::types::DriftCheckBaselines) -> Self {
        self.inner = self.inner.drift_check_baselines(input);
        self
    }
    /// <p>Represents the drift check baselines that can be used when the model monitor is set using the model package. For more information, see the topic on <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/pipelines-quality-clarify-baseline-lifecycle.html#pipelines-quality-clarify-baseline-drift-detection">Drift Detection against Previous Baselines in SageMaker Pipelines</a> in the <i>Amazon SageMaker Developer Guide</i>. </p>
    pub fn set_drift_check_baselines(
        mut self,
        input: ::std::option::Option<crate::types::DriftCheckBaselines>,
    ) -> Self {
        self.inner = self.inner.set_drift_check_baselines(input);
        self
    }
    /// <p>The machine learning domain of your model package and its components. Common machine learning domains include computer vision and natural language processing.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>The machine learning domain of your model package and its components. Common machine learning domains include computer vision and natural language processing.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p>The machine learning task your model package accomplishes. Common machine learning tasks include object detection and image classification. The following tasks are supported by Inference Recommender: <code>"IMAGE_CLASSIFICATION"</code> | <code>"OBJECT_DETECTION"</code> | <code>"TEXT_GENERATION"</code> |<code>"IMAGE_SEGMENTATION"</code> | <code>"FILL_MASK"</code> | <code>"CLASSIFICATION"</code> | <code>"REGRESSION"</code> | <code>"OTHER"</code>.</p>
    /// <p>Specify "OTHER" if none of the tasks listed fit your use case.</p>
    pub fn task(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.task(input.into());
        self
    }
    /// <p>The machine learning task your model package accomplishes. Common machine learning tasks include object detection and image classification. The following tasks are supported by Inference Recommender: <code>"IMAGE_CLASSIFICATION"</code> | <code>"OBJECT_DETECTION"</code> | <code>"TEXT_GENERATION"</code> |<code>"IMAGE_SEGMENTATION"</code> | <code>"FILL_MASK"</code> | <code>"CLASSIFICATION"</code> | <code>"REGRESSION"</code> | <code>"OTHER"</code>.</p>
    /// <p>Specify "OTHER" if none of the tasks listed fit your use case.</p>
    pub fn set_task(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_task(input);
        self
    }
    /// <p>The Amazon Simple Storage Service (Amazon S3) path where the sample payload is stored. This path must point to a single gzip compressed tar archive (.tar.gz suffix). This archive can hold multiple files that are all equally used in the load test. Each file in the archive must satisfy the size constraints of the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpoint.html#API_runtime_InvokeEndpoint_RequestSyntax">InvokeEndpoint</a> call.</p>
    pub fn sample_payload_url(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.sample_payload_url(input.into());
        self
    }
    /// <p>The Amazon Simple Storage Service (Amazon S3) path where the sample payload is stored. This path must point to a single gzip compressed tar archive (.tar.gz suffix). This archive can hold multiple files that are all equally used in the load test. Each file in the archive must satisfy the size constraints of the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpoint.html#API_runtime_InvokeEndpoint_RequestSyntax">InvokeEndpoint</a> call.</p>
    pub fn set_sample_payload_url(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sample_payload_url(input);
        self
    }
    /// Appends an item to `AdditionalInferenceSpecifications`.
    ///
    /// To override the contents of this collection use [`set_additional_inference_specifications`](Self::set_additional_inference_specifications).
    ///
    /// <p>An array of additional Inference Specification objects. Each additional Inference Specification specifies artifacts based on this model package that can be used on inference endpoints. Generally used with SageMaker Neo to store the compiled artifacts. </p>
    pub fn additional_inference_specifications(
        mut self,
        input: crate::types::AdditionalInferenceSpecificationDefinition,
    ) -> Self {
        self.inner = self.inner.additional_inference_specifications(input);
        self
    }
    /// <p>An array of additional Inference Specification objects. Each additional Inference Specification specifies artifacts based on this model package that can be used on inference endpoints. Generally used with SageMaker Neo to store the compiled artifacts. </p>
    pub fn set_additional_inference_specifications(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::AdditionalInferenceSpecificationDefinition>,
        >,
    ) -> Self {
        self.inner = self.inner.set_additional_inference_specifications(input);
        self
    }
}
