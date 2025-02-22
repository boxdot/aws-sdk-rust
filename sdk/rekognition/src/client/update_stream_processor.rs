// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateStreamProcessor`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::set_name): <p> Name of the stream processor that you want to update. </p>
    ///   - [`settings_for_update(StreamProcessorSettingsForUpdate)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::settings_for_update) / [`set_settings_for_update(Option<StreamProcessorSettingsForUpdate>)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::set_settings_for_update): <p> The stream processor settings that you want to update. Label detection settings can be updated to detect different labels with a different minimum confidence. </p>
    ///   - [`regions_of_interest_for_update(Vec<RegionOfInterest>)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::regions_of_interest_for_update) / [`set_regions_of_interest_for_update(Option<Vec<RegionOfInterest>>)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::set_regions_of_interest_for_update): <p> Specifies locations in the frames where Amazon Rekognition checks for objects or people. This is an optional parameter for label detection stream processors. </p>
    ///   - [`data_sharing_preference_for_update(StreamProcessorDataSharingPreference)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::data_sharing_preference_for_update) / [`set_data_sharing_preference_for_update(Option<StreamProcessorDataSharingPreference>)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::set_data_sharing_preference_for_update): <p> Shows whether you are sharing data with Rekognition to improve model performance. You can choose this option at the account level or on a per-stream basis. Note that if you opt out at the account level this setting is ignored on individual streams. </p>
    ///   - [`parameters_to_delete(Vec<StreamProcessorParameterToDelete>)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::parameters_to_delete) / [`set_parameters_to_delete(Option<Vec<StreamProcessorParameterToDelete>>)`](crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::set_parameters_to_delete): <p> A list of parameters you want to delete from the stream processor. </p>
    /// - On success, responds with [`UpdateStreamProcessorOutput`](crate::operation::update_stream_processor::UpdateStreamProcessorOutput)
    /// - On failure, responds with [`SdkError<UpdateStreamProcessorError>`](crate::operation::update_stream_processor::UpdateStreamProcessorError)
    pub fn update_stream_processor(
        &self,
    ) -> crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder
    {
        crate::operation::update_stream_processor::builders::UpdateStreamProcessorFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
