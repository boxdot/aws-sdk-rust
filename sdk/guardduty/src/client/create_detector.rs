// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDetector`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`enable(bool)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::enable) / [`set_enable(Option<bool>)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::set_enable): <p>A Boolean value that specifies whether the detector is to be enabled.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::set_client_token): <p>The idempotency token for the create request.</p>
    ///   - [`finding_publishing_frequency(FindingPublishingFrequency)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::finding_publishing_frequency) / [`set_finding_publishing_frequency(Option<FindingPublishingFrequency>)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::set_finding_publishing_frequency): <p>A value that specifies how frequently updated findings are exported.</p>
    ///   - [`data_sources(DataSourceConfigurations)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::data_sources) / [`set_data_sources(Option<DataSourceConfigurations>)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::set_data_sources): <p>Describes which data sources will be enabled for the detector.</p>  <p>There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. For more information, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_regions.html">Regions and endpoints</a>.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::set_tags): <p>The tags to be added to a new detector resource.</p>
    ///   - [`features(Vec<DetectorFeatureConfiguration>)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::features) / [`set_features(Option<Vec<DetectorFeatureConfiguration>>)`](crate::operation::create_detector::builders::CreateDetectorFluentBuilder::set_features): <p>A list of features that will be configured for the detector.</p>
    /// - On success, responds with [`CreateDetectorOutput`](crate::operation::create_detector::CreateDetectorOutput) with field(s):
    ///   - [`detector_id(Option<String>)`](crate::operation::create_detector::CreateDetectorOutput::detector_id): <p>The unique ID of the created detector.</p>
    ///   - [`unprocessed_data_sources(Option<UnprocessedDataSourcesResult>)`](crate::operation::create_detector::CreateDetectorOutput::unprocessed_data_sources): <p>Specifies the data sources that couldn't be enabled when GuardDuty was enabled for the first time.</p>
    /// - On failure, responds with [`SdkError<CreateDetectorError>`](crate::operation::create_detector::CreateDetectorError)
    pub fn create_detector(
        &self,
    ) -> crate::operation::create_detector::builders::CreateDetectorFluentBuilder {
        crate::operation::create_detector::builders::CreateDetectorFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
