// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetMobileSdkRelease`](crate::operation::get_mobile_sdk_release::builders::GetMobileSdkReleaseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`platform(Platform)`](crate::operation::get_mobile_sdk_release::builders::GetMobileSdkReleaseFluentBuilder::platform) / [`set_platform(Option<Platform>)`](crate::operation::get_mobile_sdk_release::builders::GetMobileSdkReleaseFluentBuilder::set_platform): <p>The device platform.</p>
    ///   - [`release_version(impl ::std::convert::Into<String>)`](crate::operation::get_mobile_sdk_release::builders::GetMobileSdkReleaseFluentBuilder::release_version) / [`set_release_version(Option<String>)`](crate::operation::get_mobile_sdk_release::builders::GetMobileSdkReleaseFluentBuilder::set_release_version): <p>The release version. For the latest available version, specify <code>LATEST</code>.</p>
    /// - On success, responds with [`GetMobileSdkReleaseOutput`](crate::operation::get_mobile_sdk_release::GetMobileSdkReleaseOutput) with field(s):
    ///   - [`mobile_sdk_release(Option<MobileSdkRelease>)`](crate::operation::get_mobile_sdk_release::GetMobileSdkReleaseOutput::mobile_sdk_release): <p>Information for a specified SDK release, including release notes and tags.</p>
    /// - On failure, responds with [`SdkError<GetMobileSdkReleaseError>`](crate::operation::get_mobile_sdk_release::GetMobileSdkReleaseError)
    pub fn get_mobile_sdk_release(
        &self,
    ) -> crate::operation::get_mobile_sdk_release::builders::GetMobileSdkReleaseFluentBuilder {
        crate::operation::get_mobile_sdk_release::builders::GetMobileSdkReleaseFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
