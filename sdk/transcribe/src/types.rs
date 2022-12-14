// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use aws_smithy_http::result::SdkError;
pub use aws_smithy_types::error::display::DisplayErrorContext;
pub use aws_smithy_types::DateTime;

/// Opaque struct used as inner data for the `Unknown` variant defined in enums in
/// the crate
///
/// While this is not intended to be used directly, it is marked as `pub` because it is
/// part of the enums that are public interface.
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub struct UnknownVariantValue(pub(crate) String);
impl UnknownVariantValue {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}
