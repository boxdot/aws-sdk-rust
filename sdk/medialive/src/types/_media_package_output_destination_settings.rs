// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// MediaPackage Output Destination Settings
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MediaPackageOutputDestinationSettings {
    /// ID of the channel in MediaPackage that is the destination for this output group. You do not need to specify the individual inputs in MediaPackage; MediaLive will handle the connection of the two MediaLive pipelines to the two MediaPackage inputs. The MediaPackage channel and MediaLive channel must be in the same region.
    #[doc(hidden)]
    pub channel_id: ::std::option::Option<::std::string::String>,
}
impl MediaPackageOutputDestinationSettings {
    /// ID of the channel in MediaPackage that is the destination for this output group. You do not need to specify the individual inputs in MediaPackage; MediaLive will handle the connection of the two MediaLive pipelines to the two MediaPackage inputs. The MediaPackage channel and MediaLive channel must be in the same region.
    pub fn channel_id(&self) -> ::std::option::Option<&str> {
        self.channel_id.as_deref()
    }
}
impl MediaPackageOutputDestinationSettings {
    /// Creates a new builder-style object to manufacture [`MediaPackageOutputDestinationSettings`](crate::types::MediaPackageOutputDestinationSettings).
    pub fn builder() -> crate::types::builders::MediaPackageOutputDestinationSettingsBuilder {
        crate::types::builders::MediaPackageOutputDestinationSettingsBuilder::default()
    }
}

/// A builder for [`MediaPackageOutputDestinationSettings`](crate::types::MediaPackageOutputDestinationSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MediaPackageOutputDestinationSettingsBuilder {
    pub(crate) channel_id: ::std::option::Option<::std::string::String>,
}
impl MediaPackageOutputDestinationSettingsBuilder {
    /// ID of the channel in MediaPackage that is the destination for this output group. You do not need to specify the individual inputs in MediaPackage; MediaLive will handle the connection of the two MediaLive pipelines to the two MediaPackage inputs. The MediaPackage channel and MediaLive channel must be in the same region.
    pub fn channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_id = ::std::option::Option::Some(input.into());
        self
    }
    /// ID of the channel in MediaPackage that is the destination for this output group. You do not need to specify the individual inputs in MediaPackage; MediaLive will handle the connection of the two MediaLive pipelines to the two MediaPackage inputs. The MediaPackage channel and MediaLive channel must be in the same region.
    pub fn set_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_id = input;
        self
    }
    /// Consumes the builder and constructs a [`MediaPackageOutputDestinationSettings`](crate::types::MediaPackageOutputDestinationSettings).
    pub fn build(self) -> crate::types::MediaPackageOutputDestinationSettings {
        crate::types::MediaPackageOutputDestinationSettings {
            channel_id: self.channel_id,
        }
    }
}
