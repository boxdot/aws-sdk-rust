// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies mandatory fields for running an Inference Recommender job directly in the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateInferenceRecommendationsJob.html">CreateInferenceRecommendationsJob</a> API. The fields specified in <code>ContainerConfig</code> override the corresponding fields in the model package. Use <code>ContainerConfig</code> if you want to specify these fields for the recommendation job but don't want to edit them in your model package.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RecommendationJobContainerConfig {
    /// <p>The machine learning domain of the model and its components.</p>
    /// <p>Valid Values: <code>COMPUTER_VISION | NATURAL_LANGUAGE_PROCESSING | MACHINE_LEARNING</code> </p>
    #[doc(hidden)]
    pub domain: ::std::option::Option<::std::string::String>,
    /// <p>The machine learning task that the model accomplishes.</p>
    /// <p>Valid Values: <code>IMAGE_CLASSIFICATION | OBJECT_DETECTION | TEXT_GENERATION | IMAGE_SEGMENTATION | FILL_MASK | CLASSIFICATION | REGRESSION | OTHER</code> </p>
    #[doc(hidden)]
    pub task: ::std::option::Option<::std::string::String>,
    /// <p>The machine learning framework of the container image.</p>
    /// <p>Valid Values: <code>TENSORFLOW | PYTORCH | XGBOOST | SAGEMAKER-SCIKIT-LEARN</code> </p>
    #[doc(hidden)]
    pub framework: ::std::option::Option<::std::string::String>,
    /// <p>The framework version of the container image.</p>
    #[doc(hidden)]
    pub framework_version: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the <code>SamplePayloadUrl</code> and all other sample payload-related fields.</p>
    #[doc(hidden)]
    pub payload_config: ::std::option::Option<crate::types::RecommendationJobPayloadConfig>,
    /// <p>The name of a pre-trained machine learning model benchmarked by Amazon SageMaker Inference Recommender that matches your model.</p>
    /// <p>Valid Values: <code>efficientnetb7 | unet | xgboost | faster-rcnn-resnet101 | nasnetlarge | vgg16 | inception-v3 | mask-rcnn | sagemaker-scikit-learn | densenet201-gluon | resnet18v2-gluon | xception | densenet201 | yolov4 | resnet152 | bert-base-cased | xceptionV1-keras | resnet50 | retinanet</code> </p>
    #[doc(hidden)]
    pub nearest_model_name: ::std::option::Option<::std::string::String>,
    /// <p>A list of the instance types that are used to generate inferences in real-time.</p>
    #[doc(hidden)]
    pub supported_instance_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specifies the name and shape of the expected data inputs for your trained model with a JSON dictionary form. This field is used for optimizing your model using SageMaker Neo. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_InputConfig.html#sagemaker-Type-InputConfig-DataInputConfig">DataInputConfig</a>.</p>
    #[doc(hidden)]
    pub data_input_config: ::std::option::Option<::std::string::String>,
}
impl RecommendationJobContainerConfig {
    /// <p>The machine learning domain of the model and its components.</p>
    /// <p>Valid Values: <code>COMPUTER_VISION | NATURAL_LANGUAGE_PROCESSING | MACHINE_LEARNING</code> </p>
    pub fn domain(&self) -> ::std::option::Option<&str> {
        self.domain.as_deref()
    }
    /// <p>The machine learning task that the model accomplishes.</p>
    /// <p>Valid Values: <code>IMAGE_CLASSIFICATION | OBJECT_DETECTION | TEXT_GENERATION | IMAGE_SEGMENTATION | FILL_MASK | CLASSIFICATION | REGRESSION | OTHER</code> </p>
    pub fn task(&self) -> ::std::option::Option<&str> {
        self.task.as_deref()
    }
    /// <p>The machine learning framework of the container image.</p>
    /// <p>Valid Values: <code>TENSORFLOW | PYTORCH | XGBOOST | SAGEMAKER-SCIKIT-LEARN</code> </p>
    pub fn framework(&self) -> ::std::option::Option<&str> {
        self.framework.as_deref()
    }
    /// <p>The framework version of the container image.</p>
    pub fn framework_version(&self) -> ::std::option::Option<&str> {
        self.framework_version.as_deref()
    }
    /// <p>Specifies the <code>SamplePayloadUrl</code> and all other sample payload-related fields.</p>
    pub fn payload_config(
        &self,
    ) -> ::std::option::Option<&crate::types::RecommendationJobPayloadConfig> {
        self.payload_config.as_ref()
    }
    /// <p>The name of a pre-trained machine learning model benchmarked by Amazon SageMaker Inference Recommender that matches your model.</p>
    /// <p>Valid Values: <code>efficientnetb7 | unet | xgboost | faster-rcnn-resnet101 | nasnetlarge | vgg16 | inception-v3 | mask-rcnn | sagemaker-scikit-learn | densenet201-gluon | resnet18v2-gluon | xception | densenet201 | yolov4 | resnet152 | bert-base-cased | xceptionV1-keras | resnet50 | retinanet</code> </p>
    pub fn nearest_model_name(&self) -> ::std::option::Option<&str> {
        self.nearest_model_name.as_deref()
    }
    /// <p>A list of the instance types that are used to generate inferences in real-time.</p>
    pub fn supported_instance_types(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.supported_instance_types.as_deref()
    }
    /// <p>Specifies the name and shape of the expected data inputs for your trained model with a JSON dictionary form. This field is used for optimizing your model using SageMaker Neo. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_InputConfig.html#sagemaker-Type-InputConfig-DataInputConfig">DataInputConfig</a>.</p>
    pub fn data_input_config(&self) -> ::std::option::Option<&str> {
        self.data_input_config.as_deref()
    }
}
impl RecommendationJobContainerConfig {
    /// Creates a new builder-style object to manufacture [`RecommendationJobContainerConfig`](crate::types::RecommendationJobContainerConfig).
    pub fn builder() -> crate::types::builders::RecommendationJobContainerConfigBuilder {
        crate::types::builders::RecommendationJobContainerConfigBuilder::default()
    }
}

/// A builder for [`RecommendationJobContainerConfig`](crate::types::RecommendationJobContainerConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RecommendationJobContainerConfigBuilder {
    pub(crate) domain: ::std::option::Option<::std::string::String>,
    pub(crate) task: ::std::option::Option<::std::string::String>,
    pub(crate) framework: ::std::option::Option<::std::string::String>,
    pub(crate) framework_version: ::std::option::Option<::std::string::String>,
    pub(crate) payload_config: ::std::option::Option<crate::types::RecommendationJobPayloadConfig>,
    pub(crate) nearest_model_name: ::std::option::Option<::std::string::String>,
    pub(crate) supported_instance_types:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) data_input_config: ::std::option::Option<::std::string::String>,
}
impl RecommendationJobContainerConfigBuilder {
    /// <p>The machine learning domain of the model and its components.</p>
    /// <p>Valid Values: <code>COMPUTER_VISION | NATURAL_LANGUAGE_PROCESSING | MACHINE_LEARNING</code> </p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The machine learning domain of the model and its components.</p>
    /// <p>Valid Values: <code>COMPUTER_VISION | NATURAL_LANGUAGE_PROCESSING | MACHINE_LEARNING</code> </p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain = input;
        self
    }
    /// <p>The machine learning task that the model accomplishes.</p>
    /// <p>Valid Values: <code>IMAGE_CLASSIFICATION | OBJECT_DETECTION | TEXT_GENERATION | IMAGE_SEGMENTATION | FILL_MASK | CLASSIFICATION | REGRESSION | OTHER</code> </p>
    pub fn task(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.task = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The machine learning task that the model accomplishes.</p>
    /// <p>Valid Values: <code>IMAGE_CLASSIFICATION | OBJECT_DETECTION | TEXT_GENERATION | IMAGE_SEGMENTATION | FILL_MASK | CLASSIFICATION | REGRESSION | OTHER</code> </p>
    pub fn set_task(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.task = input;
        self
    }
    /// <p>The machine learning framework of the container image.</p>
    /// <p>Valid Values: <code>TENSORFLOW | PYTORCH | XGBOOST | SAGEMAKER-SCIKIT-LEARN</code> </p>
    pub fn framework(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.framework = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The machine learning framework of the container image.</p>
    /// <p>Valid Values: <code>TENSORFLOW | PYTORCH | XGBOOST | SAGEMAKER-SCIKIT-LEARN</code> </p>
    pub fn set_framework(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.framework = input;
        self
    }
    /// <p>The framework version of the container image.</p>
    pub fn framework_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.framework_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The framework version of the container image.</p>
    pub fn set_framework_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.framework_version = input;
        self
    }
    /// <p>Specifies the <code>SamplePayloadUrl</code> and all other sample payload-related fields.</p>
    pub fn payload_config(mut self, input: crate::types::RecommendationJobPayloadConfig) -> Self {
        self.payload_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the <code>SamplePayloadUrl</code> and all other sample payload-related fields.</p>
    pub fn set_payload_config(
        mut self,
        input: ::std::option::Option<crate::types::RecommendationJobPayloadConfig>,
    ) -> Self {
        self.payload_config = input;
        self
    }
    /// <p>The name of a pre-trained machine learning model benchmarked by Amazon SageMaker Inference Recommender that matches your model.</p>
    /// <p>Valid Values: <code>efficientnetb7 | unet | xgboost | faster-rcnn-resnet101 | nasnetlarge | vgg16 | inception-v3 | mask-rcnn | sagemaker-scikit-learn | densenet201-gluon | resnet18v2-gluon | xception | densenet201 | yolov4 | resnet152 | bert-base-cased | xceptionV1-keras | resnet50 | retinanet</code> </p>
    pub fn nearest_model_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.nearest_model_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a pre-trained machine learning model benchmarked by Amazon SageMaker Inference Recommender that matches your model.</p>
    /// <p>Valid Values: <code>efficientnetb7 | unet | xgboost | faster-rcnn-resnet101 | nasnetlarge | vgg16 | inception-v3 | mask-rcnn | sagemaker-scikit-learn | densenet201-gluon | resnet18v2-gluon | xception | densenet201 | yolov4 | resnet152 | bert-base-cased | xceptionV1-keras | resnet50 | retinanet</code> </p>
    pub fn set_nearest_model_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.nearest_model_name = input;
        self
    }
    /// Appends an item to `supported_instance_types`.
    ///
    /// To override the contents of this collection use [`set_supported_instance_types`](Self::set_supported_instance_types).
    ///
    /// <p>A list of the instance types that are used to generate inferences in real-time.</p>
    pub fn supported_instance_types(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.supported_instance_types.unwrap_or_default();
        v.push(input.into());
        self.supported_instance_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the instance types that are used to generate inferences in real-time.</p>
    pub fn set_supported_instance_types(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.supported_instance_types = input;
        self
    }
    /// <p>Specifies the name and shape of the expected data inputs for your trained model with a JSON dictionary form. This field is used for optimizing your model using SageMaker Neo. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_InputConfig.html#sagemaker-Type-InputConfig-DataInputConfig">DataInputConfig</a>.</p>
    pub fn data_input_config(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.data_input_config = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the name and shape of the expected data inputs for your trained model with a JSON dictionary form. This field is used for optimizing your model using SageMaker Neo. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_InputConfig.html#sagemaker-Type-InputConfig-DataInputConfig">DataInputConfig</a>.</p>
    pub fn set_data_input_config(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.data_input_config = input;
        self
    }
    /// Consumes the builder and constructs a [`RecommendationJobContainerConfig`](crate::types::RecommendationJobContainerConfig).
    pub fn build(self) -> crate::types::RecommendationJobContainerConfig {
        crate::types::RecommendationJobContainerConfig {
            domain: self.domain,
            task: self.task,
            framework: self.framework,
            framework_version: self.framework_version,
            payload_config: self.payload_config,
            nearest_model_name: self.nearest_model_name,
            supported_instance_types: self.supported_instance_types,
            data_input_config: self.data_input_config,
        }
    }
}
