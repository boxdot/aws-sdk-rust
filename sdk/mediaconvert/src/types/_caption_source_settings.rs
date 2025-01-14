// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// If your input captions are SCC, TTML, STL, SMI, SRT, or IMSC in an xml file, specify the URI of the input captions source file. If your input captions are IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CaptionSourceSettings {
    /// Settings for ancillary captions source.
    #[doc(hidden)]
    pub ancillary_source_settings: ::std::option::Option<crate::types::AncillarySourceSettings>,
    /// DVB Sub Source Settings
    #[doc(hidden)]
    pub dvb_sub_source_settings: ::std::option::Option<crate::types::DvbSubSourceSettings>,
    /// Settings for embedded captions Source
    #[doc(hidden)]
    pub embedded_source_settings: ::std::option::Option<crate::types::EmbeddedSourceSettings>,
    /// If your input captions are SCC, SMI, SRT, STL, TTML, WebVTT, or IMSC 1.1 in an xml file, specify the URI of the input caption source file. If your caption source is IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.
    #[doc(hidden)]
    pub file_source_settings: ::std::option::Option<crate::types::FileSourceSettings>,
    /// Use Source (SourceType) to identify the format of your input captions. The service cannot auto-detect caption format.
    #[doc(hidden)]
    pub source_type: ::std::option::Option<crate::types::CaptionSourceType>,
    /// Settings specific to Teletext caption sources, including Page number.
    #[doc(hidden)]
    pub teletext_source_settings: ::std::option::Option<crate::types::TeletextSourceSettings>,
    /// Settings specific to caption sources that are specified by track number. Currently, this is only IMSC captions in an IMF package. If your caption source is IMSC 1.1 in a separate xml file, use FileSourceSettings instead of TrackSourceSettings.
    #[doc(hidden)]
    pub track_source_settings: ::std::option::Option<crate::types::TrackSourceSettings>,
    /// Settings specific to WebVTT sources in HLS alternative rendition group. Specify the properties (renditionGroupId, renditionName or renditionLanguageCode) to identify the unique subtitle track among the alternative rendition groups present in the HLS manifest. If no unique track is found, or multiple tracks match the specified properties, the job fails. If there is only one subtitle track in the rendition group, the settings can be left empty and the default subtitle track will be chosen. If your caption source is a sidecar file, use FileSourceSettings instead of WebvttHlsSourceSettings.
    #[doc(hidden)]
    pub webvtt_hls_source_settings: ::std::option::Option<crate::types::WebvttHlsSourceSettings>,
}
impl CaptionSourceSettings {
    /// Settings for ancillary captions source.
    pub fn ancillary_source_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::AncillarySourceSettings> {
        self.ancillary_source_settings.as_ref()
    }
    /// DVB Sub Source Settings
    pub fn dvb_sub_source_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::DvbSubSourceSettings> {
        self.dvb_sub_source_settings.as_ref()
    }
    /// Settings for embedded captions Source
    pub fn embedded_source_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::EmbeddedSourceSettings> {
        self.embedded_source_settings.as_ref()
    }
    /// If your input captions are SCC, SMI, SRT, STL, TTML, WebVTT, or IMSC 1.1 in an xml file, specify the URI of the input caption source file. If your caption source is IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.
    pub fn file_source_settings(&self) -> ::std::option::Option<&crate::types::FileSourceSettings> {
        self.file_source_settings.as_ref()
    }
    /// Use Source (SourceType) to identify the format of your input captions. The service cannot auto-detect caption format.
    pub fn source_type(&self) -> ::std::option::Option<&crate::types::CaptionSourceType> {
        self.source_type.as_ref()
    }
    /// Settings specific to Teletext caption sources, including Page number.
    pub fn teletext_source_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::TeletextSourceSettings> {
        self.teletext_source_settings.as_ref()
    }
    /// Settings specific to caption sources that are specified by track number. Currently, this is only IMSC captions in an IMF package. If your caption source is IMSC 1.1 in a separate xml file, use FileSourceSettings instead of TrackSourceSettings.
    pub fn track_source_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::TrackSourceSettings> {
        self.track_source_settings.as_ref()
    }
    /// Settings specific to WebVTT sources in HLS alternative rendition group. Specify the properties (renditionGroupId, renditionName or renditionLanguageCode) to identify the unique subtitle track among the alternative rendition groups present in the HLS manifest. If no unique track is found, or multiple tracks match the specified properties, the job fails. If there is only one subtitle track in the rendition group, the settings can be left empty and the default subtitle track will be chosen. If your caption source is a sidecar file, use FileSourceSettings instead of WebvttHlsSourceSettings.
    pub fn webvtt_hls_source_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::WebvttHlsSourceSettings> {
        self.webvtt_hls_source_settings.as_ref()
    }
}
impl CaptionSourceSettings {
    /// Creates a new builder-style object to manufacture [`CaptionSourceSettings`](crate::types::CaptionSourceSettings).
    pub fn builder() -> crate::types::builders::CaptionSourceSettingsBuilder {
        crate::types::builders::CaptionSourceSettingsBuilder::default()
    }
}

/// A builder for [`CaptionSourceSettings`](crate::types::CaptionSourceSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CaptionSourceSettingsBuilder {
    pub(crate) ancillary_source_settings:
        ::std::option::Option<crate::types::AncillarySourceSettings>,
    pub(crate) dvb_sub_source_settings: ::std::option::Option<crate::types::DvbSubSourceSettings>,
    pub(crate) embedded_source_settings:
        ::std::option::Option<crate::types::EmbeddedSourceSettings>,
    pub(crate) file_source_settings: ::std::option::Option<crate::types::FileSourceSettings>,
    pub(crate) source_type: ::std::option::Option<crate::types::CaptionSourceType>,
    pub(crate) teletext_source_settings:
        ::std::option::Option<crate::types::TeletextSourceSettings>,
    pub(crate) track_source_settings: ::std::option::Option<crate::types::TrackSourceSettings>,
    pub(crate) webvtt_hls_source_settings:
        ::std::option::Option<crate::types::WebvttHlsSourceSettings>,
}
impl CaptionSourceSettingsBuilder {
    /// Settings for ancillary captions source.
    pub fn ancillary_source_settings(
        mut self,
        input: crate::types::AncillarySourceSettings,
    ) -> Self {
        self.ancillary_source_settings = ::std::option::Option::Some(input);
        self
    }
    /// Settings for ancillary captions source.
    pub fn set_ancillary_source_settings(
        mut self,
        input: ::std::option::Option<crate::types::AncillarySourceSettings>,
    ) -> Self {
        self.ancillary_source_settings = input;
        self
    }
    /// DVB Sub Source Settings
    pub fn dvb_sub_source_settings(mut self, input: crate::types::DvbSubSourceSettings) -> Self {
        self.dvb_sub_source_settings = ::std::option::Option::Some(input);
        self
    }
    /// DVB Sub Source Settings
    pub fn set_dvb_sub_source_settings(
        mut self,
        input: ::std::option::Option<crate::types::DvbSubSourceSettings>,
    ) -> Self {
        self.dvb_sub_source_settings = input;
        self
    }
    /// Settings for embedded captions Source
    pub fn embedded_source_settings(mut self, input: crate::types::EmbeddedSourceSettings) -> Self {
        self.embedded_source_settings = ::std::option::Option::Some(input);
        self
    }
    /// Settings for embedded captions Source
    pub fn set_embedded_source_settings(
        mut self,
        input: ::std::option::Option<crate::types::EmbeddedSourceSettings>,
    ) -> Self {
        self.embedded_source_settings = input;
        self
    }
    /// If your input captions are SCC, SMI, SRT, STL, TTML, WebVTT, or IMSC 1.1 in an xml file, specify the URI of the input caption source file. If your caption source is IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.
    pub fn file_source_settings(mut self, input: crate::types::FileSourceSettings) -> Self {
        self.file_source_settings = ::std::option::Option::Some(input);
        self
    }
    /// If your input captions are SCC, SMI, SRT, STL, TTML, WebVTT, or IMSC 1.1 in an xml file, specify the URI of the input caption source file. If your caption source is IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.
    pub fn set_file_source_settings(
        mut self,
        input: ::std::option::Option<crate::types::FileSourceSettings>,
    ) -> Self {
        self.file_source_settings = input;
        self
    }
    /// Use Source (SourceType) to identify the format of your input captions. The service cannot auto-detect caption format.
    pub fn source_type(mut self, input: crate::types::CaptionSourceType) -> Self {
        self.source_type = ::std::option::Option::Some(input);
        self
    }
    /// Use Source (SourceType) to identify the format of your input captions. The service cannot auto-detect caption format.
    pub fn set_source_type(
        mut self,
        input: ::std::option::Option<crate::types::CaptionSourceType>,
    ) -> Self {
        self.source_type = input;
        self
    }
    /// Settings specific to Teletext caption sources, including Page number.
    pub fn teletext_source_settings(mut self, input: crate::types::TeletextSourceSettings) -> Self {
        self.teletext_source_settings = ::std::option::Option::Some(input);
        self
    }
    /// Settings specific to Teletext caption sources, including Page number.
    pub fn set_teletext_source_settings(
        mut self,
        input: ::std::option::Option<crate::types::TeletextSourceSettings>,
    ) -> Self {
        self.teletext_source_settings = input;
        self
    }
    /// Settings specific to caption sources that are specified by track number. Currently, this is only IMSC captions in an IMF package. If your caption source is IMSC 1.1 in a separate xml file, use FileSourceSettings instead of TrackSourceSettings.
    pub fn track_source_settings(mut self, input: crate::types::TrackSourceSettings) -> Self {
        self.track_source_settings = ::std::option::Option::Some(input);
        self
    }
    /// Settings specific to caption sources that are specified by track number. Currently, this is only IMSC captions in an IMF package. If your caption source is IMSC 1.1 in a separate xml file, use FileSourceSettings instead of TrackSourceSettings.
    pub fn set_track_source_settings(
        mut self,
        input: ::std::option::Option<crate::types::TrackSourceSettings>,
    ) -> Self {
        self.track_source_settings = input;
        self
    }
    /// Settings specific to WebVTT sources in HLS alternative rendition group. Specify the properties (renditionGroupId, renditionName or renditionLanguageCode) to identify the unique subtitle track among the alternative rendition groups present in the HLS manifest. If no unique track is found, or multiple tracks match the specified properties, the job fails. If there is only one subtitle track in the rendition group, the settings can be left empty and the default subtitle track will be chosen. If your caption source is a sidecar file, use FileSourceSettings instead of WebvttHlsSourceSettings.
    pub fn webvtt_hls_source_settings(
        mut self,
        input: crate::types::WebvttHlsSourceSettings,
    ) -> Self {
        self.webvtt_hls_source_settings = ::std::option::Option::Some(input);
        self
    }
    /// Settings specific to WebVTT sources in HLS alternative rendition group. Specify the properties (renditionGroupId, renditionName or renditionLanguageCode) to identify the unique subtitle track among the alternative rendition groups present in the HLS manifest. If no unique track is found, or multiple tracks match the specified properties, the job fails. If there is only one subtitle track in the rendition group, the settings can be left empty and the default subtitle track will be chosen. If your caption source is a sidecar file, use FileSourceSettings instead of WebvttHlsSourceSettings.
    pub fn set_webvtt_hls_source_settings(
        mut self,
        input: ::std::option::Option<crate::types::WebvttHlsSourceSettings>,
    ) -> Self {
        self.webvtt_hls_source_settings = input;
        self
    }
    /// Consumes the builder and constructs a [`CaptionSourceSettings`](crate::types::CaptionSourceSettings).
    pub fn build(self) -> crate::types::CaptionSourceSettings {
        crate::types::CaptionSourceSettings {
            ancillary_source_settings: self.ancillary_source_settings,
            dvb_sub_source_settings: self.dvb_sub_source_settings,
            embedded_source_settings: self.embedded_source_settings,
            file_source_settings: self.file_source_settings,
            source_type: self.source_type,
            teletext_source_settings: self.teletext_source_settings,
            track_source_settings: self.track_source_settings,
            webvtt_hls_source_settings: self.webvtt_hls_source_settings,
        }
    }
}
