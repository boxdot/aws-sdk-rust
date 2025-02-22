// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// JobSettings contains all the transcode settings for a job.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JobSettings {
    /// When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time.
    #[doc(hidden)]
    pub ad_avail_offset: ::std::option::Option<i32>,
    /// Settings for ad avail blanking. Video can be blanked or overlaid with an image, and audio muted during SCTE-35 triggered ad avails.
    #[doc(hidden)]
    pub avail_blanking: ::std::option::Option<crate::types::AvailBlanking>,
    /// Settings for Event Signaling And Messaging (ESAM). If you don't do ad insertion, you can ignore these settings.
    #[doc(hidden)]
    pub esam: ::std::option::Option<crate::types::EsamSettings>,
    /// If your source content has EIA-608 Line 21 Data Services, enable this feature to specify what MediaConvert does with the Extended Data Services (XDS) packets. You can choose to pass through XDS packets, or remove them from the output. For more information about XDS, see EIA-608 Line Data Services, section 9.5.1.5 05h Content Advisory.
    #[doc(hidden)]
    pub extended_data_services: ::std::option::Option<crate::types::ExtendedDataServices>,
    /// Use Inputs (inputs) to define source file used in the transcode job. There can be multiple inputs add in a job. These inputs will be concantenated together to create the output.
    #[doc(hidden)]
    pub inputs: ::std::option::Option<::std::vec::Vec<crate::types::Input>>,
    /// Use these settings only when you use Kantar watermarking. Specify the values that MediaConvert uses to generate and place Kantar watermarks in your output audio. These settings apply to every output in your job. In addition to specifying these values, you also need to store your Kantar credentials in AWS Secrets Manager. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/kantar-watermarking.html.
    #[doc(hidden)]
    pub kantar_watermark: ::std::option::Option<crate::types::KantarWatermarkSettings>,
    /// Overlay motion graphics on top of your video. The motion graphics that you specify here appear on all outputs in all output groups. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/motion-graphic-overlay.html.
    #[doc(hidden)]
    pub motion_image_inserter: ::std::option::Option<crate::types::MotionImageInserter>,
    /// Settings for your Nielsen configuration. If you don't do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don't include any children of nielsenConfiguration, you still enable the setting.
    #[doc(hidden)]
    pub nielsen_configuration: ::std::option::Option<crate::types::NielsenConfiguration>,
    /// Ignore these settings unless you are using Nielsen non-linear watermarking. Specify the values that MediaConvert uses to generate and place Nielsen watermarks in your output audio. In addition to specifying these values, you also need to set up your cloud TIC server. These settings apply to every output in your job. The MediaConvert implementation is currently with the following Nielsen versions: Nielsen Watermark SDK Version 5.2.1 Nielsen NLM Watermark Engine Version 1.2.7 Nielsen Watermark Authenticator [SID_TIC] Version [5.0.0]
    #[doc(hidden)]
    pub nielsen_non_linear_watermark:
        ::std::option::Option<crate::types::NielsenNonLinearWatermarkSettings>,
    /// (OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE_GROUP_SETTINGS, FileGroupSettings * HLS_GROUP_SETTINGS, HlsGroupSettings * DASH_ISO_GROUP_SETTINGS, DashIsoGroupSettings * MS_SMOOTH_GROUP_SETTINGS, MsSmoothGroupSettings * CMAF_GROUP_SETTINGS, CmafGroupSettings
    #[doc(hidden)]
    pub output_groups: ::std::option::Option<::std::vec::Vec<crate::types::OutputGroup>>,
    /// These settings control how the service handles timecodes throughout the job. These settings don't affect input clipping.
    #[doc(hidden)]
    pub timecode_config: ::std::option::Option<crate::types::TimecodeConfig>,
    /// Insert user-defined custom ID3 metadata (id3) at timecodes (timecode) that you specify. In each output that you want to include this metadata, you must set ID3 metadata (timedMetadata) to Passthrough (PASSTHROUGH).
    #[doc(hidden)]
    pub timed_metadata_insertion: ::std::option::Option<crate::types::TimedMetadataInsertion>,
}
impl JobSettings {
    /// When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time.
    pub fn ad_avail_offset(&self) -> ::std::option::Option<i32> {
        self.ad_avail_offset
    }
    /// Settings for ad avail blanking. Video can be blanked or overlaid with an image, and audio muted during SCTE-35 triggered ad avails.
    pub fn avail_blanking(&self) -> ::std::option::Option<&crate::types::AvailBlanking> {
        self.avail_blanking.as_ref()
    }
    /// Settings for Event Signaling And Messaging (ESAM). If you don't do ad insertion, you can ignore these settings.
    pub fn esam(&self) -> ::std::option::Option<&crate::types::EsamSettings> {
        self.esam.as_ref()
    }
    /// If your source content has EIA-608 Line 21 Data Services, enable this feature to specify what MediaConvert does with the Extended Data Services (XDS) packets. You can choose to pass through XDS packets, or remove them from the output. For more information about XDS, see EIA-608 Line Data Services, section 9.5.1.5 05h Content Advisory.
    pub fn extended_data_services(
        &self,
    ) -> ::std::option::Option<&crate::types::ExtendedDataServices> {
        self.extended_data_services.as_ref()
    }
    /// Use Inputs (inputs) to define source file used in the transcode job. There can be multiple inputs add in a job. These inputs will be concantenated together to create the output.
    pub fn inputs(&self) -> ::std::option::Option<&[crate::types::Input]> {
        self.inputs.as_deref()
    }
    /// Use these settings only when you use Kantar watermarking. Specify the values that MediaConvert uses to generate and place Kantar watermarks in your output audio. These settings apply to every output in your job. In addition to specifying these values, you also need to store your Kantar credentials in AWS Secrets Manager. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/kantar-watermarking.html.
    pub fn kantar_watermark(
        &self,
    ) -> ::std::option::Option<&crate::types::KantarWatermarkSettings> {
        self.kantar_watermark.as_ref()
    }
    /// Overlay motion graphics on top of your video. The motion graphics that you specify here appear on all outputs in all output groups. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/motion-graphic-overlay.html.
    pub fn motion_image_inserter(
        &self,
    ) -> ::std::option::Option<&crate::types::MotionImageInserter> {
        self.motion_image_inserter.as_ref()
    }
    /// Settings for your Nielsen configuration. If you don't do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don't include any children of nielsenConfiguration, you still enable the setting.
    pub fn nielsen_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::NielsenConfiguration> {
        self.nielsen_configuration.as_ref()
    }
    /// Ignore these settings unless you are using Nielsen non-linear watermarking. Specify the values that MediaConvert uses to generate and place Nielsen watermarks in your output audio. In addition to specifying these values, you also need to set up your cloud TIC server. These settings apply to every output in your job. The MediaConvert implementation is currently with the following Nielsen versions: Nielsen Watermark SDK Version 5.2.1 Nielsen NLM Watermark Engine Version 1.2.7 Nielsen Watermark Authenticator [SID_TIC] Version [5.0.0]
    pub fn nielsen_non_linear_watermark(
        &self,
    ) -> ::std::option::Option<&crate::types::NielsenNonLinearWatermarkSettings> {
        self.nielsen_non_linear_watermark.as_ref()
    }
    /// (OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE_GROUP_SETTINGS, FileGroupSettings * HLS_GROUP_SETTINGS, HlsGroupSettings * DASH_ISO_GROUP_SETTINGS, DashIsoGroupSettings * MS_SMOOTH_GROUP_SETTINGS, MsSmoothGroupSettings * CMAF_GROUP_SETTINGS, CmafGroupSettings
    pub fn output_groups(&self) -> ::std::option::Option<&[crate::types::OutputGroup]> {
        self.output_groups.as_deref()
    }
    /// These settings control how the service handles timecodes throughout the job. These settings don't affect input clipping.
    pub fn timecode_config(&self) -> ::std::option::Option<&crate::types::TimecodeConfig> {
        self.timecode_config.as_ref()
    }
    /// Insert user-defined custom ID3 metadata (id3) at timecodes (timecode) that you specify. In each output that you want to include this metadata, you must set ID3 metadata (timedMetadata) to Passthrough (PASSTHROUGH).
    pub fn timed_metadata_insertion(
        &self,
    ) -> ::std::option::Option<&crate::types::TimedMetadataInsertion> {
        self.timed_metadata_insertion.as_ref()
    }
}
impl JobSettings {
    /// Creates a new builder-style object to manufacture [`JobSettings`](crate::types::JobSettings).
    pub fn builder() -> crate::types::builders::JobSettingsBuilder {
        crate::types::builders::JobSettingsBuilder::default()
    }
}

/// A builder for [`JobSettings`](crate::types::JobSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct JobSettingsBuilder {
    pub(crate) ad_avail_offset: ::std::option::Option<i32>,
    pub(crate) avail_blanking: ::std::option::Option<crate::types::AvailBlanking>,
    pub(crate) esam: ::std::option::Option<crate::types::EsamSettings>,
    pub(crate) extended_data_services: ::std::option::Option<crate::types::ExtendedDataServices>,
    pub(crate) inputs: ::std::option::Option<::std::vec::Vec<crate::types::Input>>,
    pub(crate) kantar_watermark: ::std::option::Option<crate::types::KantarWatermarkSettings>,
    pub(crate) motion_image_inserter: ::std::option::Option<crate::types::MotionImageInserter>,
    pub(crate) nielsen_configuration: ::std::option::Option<crate::types::NielsenConfiguration>,
    pub(crate) nielsen_non_linear_watermark:
        ::std::option::Option<crate::types::NielsenNonLinearWatermarkSettings>,
    pub(crate) output_groups: ::std::option::Option<::std::vec::Vec<crate::types::OutputGroup>>,
    pub(crate) timecode_config: ::std::option::Option<crate::types::TimecodeConfig>,
    pub(crate) timed_metadata_insertion:
        ::std::option::Option<crate::types::TimedMetadataInsertion>,
}
impl JobSettingsBuilder {
    /// When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time.
    pub fn ad_avail_offset(mut self, input: i32) -> Self {
        self.ad_avail_offset = ::std::option::Option::Some(input);
        self
    }
    /// When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time.
    pub fn set_ad_avail_offset(mut self, input: ::std::option::Option<i32>) -> Self {
        self.ad_avail_offset = input;
        self
    }
    /// Settings for ad avail blanking. Video can be blanked or overlaid with an image, and audio muted during SCTE-35 triggered ad avails.
    pub fn avail_blanking(mut self, input: crate::types::AvailBlanking) -> Self {
        self.avail_blanking = ::std::option::Option::Some(input);
        self
    }
    /// Settings for ad avail blanking. Video can be blanked or overlaid with an image, and audio muted during SCTE-35 triggered ad avails.
    pub fn set_avail_blanking(
        mut self,
        input: ::std::option::Option<crate::types::AvailBlanking>,
    ) -> Self {
        self.avail_blanking = input;
        self
    }
    /// Settings for Event Signaling And Messaging (ESAM). If you don't do ad insertion, you can ignore these settings.
    pub fn esam(mut self, input: crate::types::EsamSettings) -> Self {
        self.esam = ::std::option::Option::Some(input);
        self
    }
    /// Settings for Event Signaling And Messaging (ESAM). If you don't do ad insertion, you can ignore these settings.
    pub fn set_esam(mut self, input: ::std::option::Option<crate::types::EsamSettings>) -> Self {
        self.esam = input;
        self
    }
    /// If your source content has EIA-608 Line 21 Data Services, enable this feature to specify what MediaConvert does with the Extended Data Services (XDS) packets. You can choose to pass through XDS packets, or remove them from the output. For more information about XDS, see EIA-608 Line Data Services, section 9.5.1.5 05h Content Advisory.
    pub fn extended_data_services(mut self, input: crate::types::ExtendedDataServices) -> Self {
        self.extended_data_services = ::std::option::Option::Some(input);
        self
    }
    /// If your source content has EIA-608 Line 21 Data Services, enable this feature to specify what MediaConvert does with the Extended Data Services (XDS) packets. You can choose to pass through XDS packets, or remove them from the output. For more information about XDS, see EIA-608 Line Data Services, section 9.5.1.5 05h Content Advisory.
    pub fn set_extended_data_services(
        mut self,
        input: ::std::option::Option<crate::types::ExtendedDataServices>,
    ) -> Self {
        self.extended_data_services = input;
        self
    }
    /// Appends an item to `inputs`.
    ///
    /// To override the contents of this collection use [`set_inputs`](Self::set_inputs).
    ///
    /// Use Inputs (inputs) to define source file used in the transcode job. There can be multiple inputs add in a job. These inputs will be concantenated together to create the output.
    pub fn inputs(mut self, input: crate::types::Input) -> Self {
        let mut v = self.inputs.unwrap_or_default();
        v.push(input);
        self.inputs = ::std::option::Option::Some(v);
        self
    }
    /// Use Inputs (inputs) to define source file used in the transcode job. There can be multiple inputs add in a job. These inputs will be concantenated together to create the output.
    pub fn set_inputs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Input>>,
    ) -> Self {
        self.inputs = input;
        self
    }
    /// Use these settings only when you use Kantar watermarking. Specify the values that MediaConvert uses to generate and place Kantar watermarks in your output audio. These settings apply to every output in your job. In addition to specifying these values, you also need to store your Kantar credentials in AWS Secrets Manager. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/kantar-watermarking.html.
    pub fn kantar_watermark(mut self, input: crate::types::KantarWatermarkSettings) -> Self {
        self.kantar_watermark = ::std::option::Option::Some(input);
        self
    }
    /// Use these settings only when you use Kantar watermarking. Specify the values that MediaConvert uses to generate and place Kantar watermarks in your output audio. These settings apply to every output in your job. In addition to specifying these values, you also need to store your Kantar credentials in AWS Secrets Manager. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/kantar-watermarking.html.
    pub fn set_kantar_watermark(
        mut self,
        input: ::std::option::Option<crate::types::KantarWatermarkSettings>,
    ) -> Self {
        self.kantar_watermark = input;
        self
    }
    /// Overlay motion graphics on top of your video. The motion graphics that you specify here appear on all outputs in all output groups. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/motion-graphic-overlay.html.
    pub fn motion_image_inserter(mut self, input: crate::types::MotionImageInserter) -> Self {
        self.motion_image_inserter = ::std::option::Option::Some(input);
        self
    }
    /// Overlay motion graphics on top of your video. The motion graphics that you specify here appear on all outputs in all output groups. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/motion-graphic-overlay.html.
    pub fn set_motion_image_inserter(
        mut self,
        input: ::std::option::Option<crate::types::MotionImageInserter>,
    ) -> Self {
        self.motion_image_inserter = input;
        self
    }
    /// Settings for your Nielsen configuration. If you don't do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don't include any children of nielsenConfiguration, you still enable the setting.
    pub fn nielsen_configuration(mut self, input: crate::types::NielsenConfiguration) -> Self {
        self.nielsen_configuration = ::std::option::Option::Some(input);
        self
    }
    /// Settings for your Nielsen configuration. If you don't do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don't include any children of nielsenConfiguration, you still enable the setting.
    pub fn set_nielsen_configuration(
        mut self,
        input: ::std::option::Option<crate::types::NielsenConfiguration>,
    ) -> Self {
        self.nielsen_configuration = input;
        self
    }
    /// Ignore these settings unless you are using Nielsen non-linear watermarking. Specify the values that MediaConvert uses to generate and place Nielsen watermarks in your output audio. In addition to specifying these values, you also need to set up your cloud TIC server. These settings apply to every output in your job. The MediaConvert implementation is currently with the following Nielsen versions: Nielsen Watermark SDK Version 5.2.1 Nielsen NLM Watermark Engine Version 1.2.7 Nielsen Watermark Authenticator [SID_TIC] Version [5.0.0]
    pub fn nielsen_non_linear_watermark(
        mut self,
        input: crate::types::NielsenNonLinearWatermarkSettings,
    ) -> Self {
        self.nielsen_non_linear_watermark = ::std::option::Option::Some(input);
        self
    }
    /// Ignore these settings unless you are using Nielsen non-linear watermarking. Specify the values that MediaConvert uses to generate and place Nielsen watermarks in your output audio. In addition to specifying these values, you also need to set up your cloud TIC server. These settings apply to every output in your job. The MediaConvert implementation is currently with the following Nielsen versions: Nielsen Watermark SDK Version 5.2.1 Nielsen NLM Watermark Engine Version 1.2.7 Nielsen Watermark Authenticator [SID_TIC] Version [5.0.0]
    pub fn set_nielsen_non_linear_watermark(
        mut self,
        input: ::std::option::Option<crate::types::NielsenNonLinearWatermarkSettings>,
    ) -> Self {
        self.nielsen_non_linear_watermark = input;
        self
    }
    /// Appends an item to `output_groups`.
    ///
    /// To override the contents of this collection use [`set_output_groups`](Self::set_output_groups).
    ///
    /// (OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE_GROUP_SETTINGS, FileGroupSettings * HLS_GROUP_SETTINGS, HlsGroupSettings * DASH_ISO_GROUP_SETTINGS, DashIsoGroupSettings * MS_SMOOTH_GROUP_SETTINGS, MsSmoothGroupSettings * CMAF_GROUP_SETTINGS, CmafGroupSettings
    pub fn output_groups(mut self, input: crate::types::OutputGroup) -> Self {
        let mut v = self.output_groups.unwrap_or_default();
        v.push(input);
        self.output_groups = ::std::option::Option::Some(v);
        self
    }
    /// (OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE_GROUP_SETTINGS, FileGroupSettings * HLS_GROUP_SETTINGS, HlsGroupSettings * DASH_ISO_GROUP_SETTINGS, DashIsoGroupSettings * MS_SMOOTH_GROUP_SETTINGS, MsSmoothGroupSettings * CMAF_GROUP_SETTINGS, CmafGroupSettings
    pub fn set_output_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OutputGroup>>,
    ) -> Self {
        self.output_groups = input;
        self
    }
    /// These settings control how the service handles timecodes throughout the job. These settings don't affect input clipping.
    pub fn timecode_config(mut self, input: crate::types::TimecodeConfig) -> Self {
        self.timecode_config = ::std::option::Option::Some(input);
        self
    }
    /// These settings control how the service handles timecodes throughout the job. These settings don't affect input clipping.
    pub fn set_timecode_config(
        mut self,
        input: ::std::option::Option<crate::types::TimecodeConfig>,
    ) -> Self {
        self.timecode_config = input;
        self
    }
    /// Insert user-defined custom ID3 metadata (id3) at timecodes (timecode) that you specify. In each output that you want to include this metadata, you must set ID3 metadata (timedMetadata) to Passthrough (PASSTHROUGH).
    pub fn timed_metadata_insertion(mut self, input: crate::types::TimedMetadataInsertion) -> Self {
        self.timed_metadata_insertion = ::std::option::Option::Some(input);
        self
    }
    /// Insert user-defined custom ID3 metadata (id3) at timecodes (timecode) that you specify. In each output that you want to include this metadata, you must set ID3 metadata (timedMetadata) to Passthrough (PASSTHROUGH).
    pub fn set_timed_metadata_insertion(
        mut self,
        input: ::std::option::Option<crate::types::TimedMetadataInsertion>,
    ) -> Self {
        self.timed_metadata_insertion = input;
        self
    }
    /// Consumes the builder and constructs a [`JobSettings`](crate::types::JobSettings).
    pub fn build(self) -> crate::types::JobSettings {
        crate::types::JobSettings {
            ad_avail_offset: self.ad_avail_offset,
            avail_blanking: self.avail_blanking,
            esam: self.esam,
            extended_data_services: self.extended_data_services,
            inputs: self.inputs,
            kantar_watermark: self.kantar_watermark,
            motion_image_inserter: self.motion_image_inserter,
            nielsen_configuration: self.nielsen_configuration,
            nielsen_non_linear_watermark: self.nielsen_non_linear_watermark,
            output_groups: self.output_groups,
            timecode_config: self.timecode_config,
            timed_metadata_insertion: self.timed_metadata_insertion,
        }
    }
}
