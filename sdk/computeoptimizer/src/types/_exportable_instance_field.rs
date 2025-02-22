// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ExportableInstanceField`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let exportableinstancefield = unimplemented!();
/// match exportableinstancefield {
///     ExportableInstanceField::AccountId => { /* ... */ },
///     ExportableInstanceField::CurrentInstanceType => { /* ... */ },
///     ExportableInstanceField::CurrentMemory => { /* ... */ },
///     ExportableInstanceField::CurrentNetwork => { /* ... */ },
///     ExportableInstanceField::CurrentOnDemandPrice => { /* ... */ },
///     ExportableInstanceField::CurrentPerformanceRisk => { /* ... */ },
///     ExportableInstanceField::CurrentStandardOneYearNoUpfrontReservedPrice => { /* ... */ },
///     ExportableInstanceField::CurrentStandardThreeYearNoUpfrontReservedPrice => { /* ... */ },
///     ExportableInstanceField::CurrentStorage => { /* ... */ },
///     ExportableInstanceField::CurrentVcpus => { /* ... */ },
///     ExportableInstanceField::EffectiveRecommendationPreferencesCpuVendorArchitectures => { /* ... */ },
///     ExportableInstanceField::EffectiveRecommendationPreferencesEnhancedInfrastructureMetrics => { /* ... */ },
///     ExportableInstanceField::EffectiveRecommendationPreferencesExternalMetricsSource => { /* ... */ },
///     ExportableInstanceField::EffectiveRecommendationPreferencesInferredWorkloadTypes => { /* ... */ },
///     ExportableInstanceField::ExternalMetricStatusCode => { /* ... */ },
///     ExportableInstanceField::ExternalMetricStatusReason => { /* ... */ },
///     ExportableInstanceField::Finding => { /* ... */ },
///     ExportableInstanceField::FindingReasonCodes => { /* ... */ },
///     ExportableInstanceField::InferredWorkloadTypes => { /* ... */ },
///     ExportableInstanceField::InstanceArn => { /* ... */ },
///     ExportableInstanceField::InstanceName => { /* ... */ },
///     ExportableInstanceField::InstanceState => { /* ... */ },
///     ExportableInstanceField::LastRefreshTimestamp => { /* ... */ },
///     ExportableInstanceField::LookbackPeriodInDays => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsEstimatedMonthlySavingsCurrency => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsEstimatedMonthlySavingsValue => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsInstanceType => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsMemory => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsMigrationEffort => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsNetwork => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsOnDemandPrice => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsPerformanceRisk => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsPlatformDifferences => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsSavingsOpportunityPercentage => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsStorage => { /* ... */ },
///     ExportableInstanceField::RecommendationOptionsVcpus => { /* ... */ },
///     ExportableInstanceField::RecommendationsSourcesRecommendationSourceArn => { /* ... */ },
///     ExportableInstanceField::RecommendationsSourcesRecommendationSourceType => { /* ... */ },
///     ExportableInstanceField::Tags => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsCpuMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsDiskReadBytesPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsDiskReadOpsPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsDiskWriteBytesPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsDiskWriteOpsPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsEbsReadBytesPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsEbsReadOpsPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsEbsWriteBytesPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsEbsWriteOpsPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsMemoryMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsNetworkInBytesPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsNetworkOutBytesPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsNetworkPacketsInPerSecondMaximum => { /* ... */ },
///     ExportableInstanceField::UtilizationMetricsNetworkPacketsOutPerSecondMaximum => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `exportableinstancefield` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ExportableInstanceField::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ExportableInstanceField::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ExportableInstanceField::NewFeature` is defined.
/// Specifically, when `exportableinstancefield` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ExportableInstanceField::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone,
    ::std::cmp::Eq,
    ::std::cmp::Ord,
    ::std::cmp::PartialEq,
    ::std::cmp::PartialOrd,
    ::std::fmt::Debug,
    ::std::hash::Hash,
)]
pub enum ExportableInstanceField {
    #[allow(missing_docs)] // documentation missing in model
    AccountId,
    #[allow(missing_docs)] // documentation missing in model
    CurrentInstanceType,
    #[allow(missing_docs)] // documentation missing in model
    CurrentMemory,
    #[allow(missing_docs)] // documentation missing in model
    CurrentNetwork,
    #[allow(missing_docs)] // documentation missing in model
    CurrentOnDemandPrice,
    #[allow(missing_docs)] // documentation missing in model
    CurrentPerformanceRisk,
    #[allow(missing_docs)] // documentation missing in model
    CurrentStandardOneYearNoUpfrontReservedPrice,
    #[allow(missing_docs)] // documentation missing in model
    CurrentStandardThreeYearNoUpfrontReservedPrice,
    #[allow(missing_docs)] // documentation missing in model
    CurrentStorage,
    #[allow(missing_docs)] // documentation missing in model
    CurrentVcpus,
    #[allow(missing_docs)] // documentation missing in model
    EffectiveRecommendationPreferencesCpuVendorArchitectures,
    #[allow(missing_docs)] // documentation missing in model
    EffectiveRecommendationPreferencesEnhancedInfrastructureMetrics,
    #[allow(missing_docs)] // documentation missing in model
    EffectiveRecommendationPreferencesExternalMetricsSource,
    #[allow(missing_docs)] // documentation missing in model
    EffectiveRecommendationPreferencesInferredWorkloadTypes,
    #[allow(missing_docs)] // documentation missing in model
    ExternalMetricStatusCode,
    #[allow(missing_docs)] // documentation missing in model
    ExternalMetricStatusReason,
    #[allow(missing_docs)] // documentation missing in model
    Finding,
    #[allow(missing_docs)] // documentation missing in model
    FindingReasonCodes,
    #[allow(missing_docs)] // documentation missing in model
    InferredWorkloadTypes,
    #[allow(missing_docs)] // documentation missing in model
    InstanceArn,
    #[allow(missing_docs)] // documentation missing in model
    InstanceName,
    #[allow(missing_docs)] // documentation missing in model
    InstanceState,
    #[allow(missing_docs)] // documentation missing in model
    LastRefreshTimestamp,
    #[allow(missing_docs)] // documentation missing in model
    LookbackPeriodInDays,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsEstimatedMonthlySavingsCurrency,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsEstimatedMonthlySavingsValue,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsInstanceType,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsMemory,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsMigrationEffort,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsNetwork,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsOnDemandPrice,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsPerformanceRisk,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsPlatformDifferences,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsProjectedUtilizationMetricsCpuMaximum,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsSavingsOpportunityPercentage,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsStandardOneYearNoUpfrontReservedPrice,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsStorage,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationOptionsVcpus,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationsSourcesRecommendationSourceArn,
    #[allow(missing_docs)] // documentation missing in model
    RecommendationsSourcesRecommendationSourceType,
    #[allow(missing_docs)] // documentation missing in model
    Tags,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsCpuMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsDiskReadBytesPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsDiskReadOpsPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsDiskWriteBytesPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsDiskWriteOpsPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsEbsReadBytesPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsEbsReadOpsPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsEbsWriteBytesPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsEbsWriteOpsPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsMemoryMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsNetworkInBytesPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsNetworkOutBytesPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsNetworkPacketsInPerSecondMaximum,
    #[allow(missing_docs)] // documentation missing in model
    UtilizationMetricsNetworkPacketsOutPerSecondMaximum,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for ExportableInstanceField {
    fn from(s: &str) -> Self {
        match s {
                        "AccountId" => ExportableInstanceField::AccountId,
"CurrentInstanceType" => ExportableInstanceField::CurrentInstanceType,
"CurrentMemory" => ExportableInstanceField::CurrentMemory,
"CurrentNetwork" => ExportableInstanceField::CurrentNetwork,
"CurrentOnDemandPrice" => ExportableInstanceField::CurrentOnDemandPrice,
"CurrentPerformanceRisk" => ExportableInstanceField::CurrentPerformanceRisk,
"CurrentStandardOneYearNoUpfrontReservedPrice" => ExportableInstanceField::CurrentStandardOneYearNoUpfrontReservedPrice,
"CurrentStandardThreeYearNoUpfrontReservedPrice" => ExportableInstanceField::CurrentStandardThreeYearNoUpfrontReservedPrice,
"CurrentStorage" => ExportableInstanceField::CurrentStorage,
"CurrentVCpus" => ExportableInstanceField::CurrentVcpus,
"EffectiveRecommendationPreferencesCpuVendorArchitectures" => ExportableInstanceField::EffectiveRecommendationPreferencesCpuVendorArchitectures,
"EffectiveRecommendationPreferencesEnhancedInfrastructureMetrics" => ExportableInstanceField::EffectiveRecommendationPreferencesEnhancedInfrastructureMetrics,
"EffectiveRecommendationPreferencesExternalMetricsSource" => ExportableInstanceField::EffectiveRecommendationPreferencesExternalMetricsSource,
"EffectiveRecommendationPreferencesInferredWorkloadTypes" => ExportableInstanceField::EffectiveRecommendationPreferencesInferredWorkloadTypes,
"ExternalMetricStatusCode" => ExportableInstanceField::ExternalMetricStatusCode,
"ExternalMetricStatusReason" => ExportableInstanceField::ExternalMetricStatusReason,
"Finding" => ExportableInstanceField::Finding,
"FindingReasonCodes" => ExportableInstanceField::FindingReasonCodes,
"InferredWorkloadTypes" => ExportableInstanceField::InferredWorkloadTypes,
"InstanceArn" => ExportableInstanceField::InstanceArn,
"InstanceName" => ExportableInstanceField::InstanceName,
"InstanceState" => ExportableInstanceField::InstanceState,
"LastRefreshTimestamp" => ExportableInstanceField::LastRefreshTimestamp,
"LookbackPeriodInDays" => ExportableInstanceField::LookbackPeriodInDays,
"RecommendationOptionsEstimatedMonthlySavingsCurrency" => ExportableInstanceField::RecommendationOptionsEstimatedMonthlySavingsCurrency,
"RecommendationOptionsEstimatedMonthlySavingsValue" => ExportableInstanceField::RecommendationOptionsEstimatedMonthlySavingsValue,
"RecommendationOptionsInstanceType" => ExportableInstanceField::RecommendationOptionsInstanceType,
"RecommendationOptionsMemory" => ExportableInstanceField::RecommendationOptionsMemory,
"RecommendationOptionsMigrationEffort" => ExportableInstanceField::RecommendationOptionsMigrationEffort,
"RecommendationOptionsNetwork" => ExportableInstanceField::RecommendationOptionsNetwork,
"RecommendationOptionsOnDemandPrice" => ExportableInstanceField::RecommendationOptionsOnDemandPrice,
"RecommendationOptionsPerformanceRisk" => ExportableInstanceField::RecommendationOptionsPerformanceRisk,
"RecommendationOptionsPlatformDifferences" => ExportableInstanceField::RecommendationOptionsPlatformDifferences,
"RecommendationOptionsProjectedUtilizationMetricsCpuMaximum" => ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum,
"RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum" => ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum,
"RecommendationOptionsSavingsOpportunityPercentage" => ExportableInstanceField::RecommendationOptionsSavingsOpportunityPercentage,
"RecommendationOptionsStandardOneYearNoUpfrontReservedPrice" => ExportableInstanceField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice,
"RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice" => ExportableInstanceField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice,
"RecommendationOptionsStorage" => ExportableInstanceField::RecommendationOptionsStorage,
"RecommendationOptionsVcpus" => ExportableInstanceField::RecommendationOptionsVcpus,
"RecommendationsSourcesRecommendationSourceArn" => ExportableInstanceField::RecommendationsSourcesRecommendationSourceArn,
"RecommendationsSourcesRecommendationSourceType" => ExportableInstanceField::RecommendationsSourcesRecommendationSourceType,
"Tags" => ExportableInstanceField::Tags,
"UtilizationMetricsCpuMaximum" => ExportableInstanceField::UtilizationMetricsCpuMaximum,
"UtilizationMetricsDiskReadBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsDiskReadBytesPerSecondMaximum,
"UtilizationMetricsDiskReadOpsPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsDiskReadOpsPerSecondMaximum,
"UtilizationMetricsDiskWriteBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsDiskWriteBytesPerSecondMaximum,
"UtilizationMetricsDiskWriteOpsPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsDiskWriteOpsPerSecondMaximum,
"UtilizationMetricsEbsReadBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsReadBytesPerSecondMaximum,
"UtilizationMetricsEbsReadOpsPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsReadOpsPerSecondMaximum,
"UtilizationMetricsEbsWriteBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsWriteBytesPerSecondMaximum,
"UtilizationMetricsEbsWriteOpsPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsWriteOpsPerSecondMaximum,
"UtilizationMetricsMemoryMaximum" => ExportableInstanceField::UtilizationMetricsMemoryMaximum,
"UtilizationMetricsNetworkInBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsNetworkInBytesPerSecondMaximum,
"UtilizationMetricsNetworkOutBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsNetworkOutBytesPerSecondMaximum,
"UtilizationMetricsNetworkPacketsInPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsNetworkPacketsInPerSecondMaximum,
"UtilizationMetricsNetworkPacketsOutPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsNetworkPacketsOutPerSecondMaximum,
other => ExportableInstanceField::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
                    }
    }
}
impl ::std::str::FromStr for ExportableInstanceField {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(ExportableInstanceField::from(s))
    }
}
impl ExportableInstanceField {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
    ExportableInstanceField::AccountId => "AccountId",
    ExportableInstanceField::CurrentInstanceType => "CurrentInstanceType",
    ExportableInstanceField::CurrentMemory => "CurrentMemory",
    ExportableInstanceField::CurrentNetwork => "CurrentNetwork",
    ExportableInstanceField::CurrentOnDemandPrice => "CurrentOnDemandPrice",
    ExportableInstanceField::CurrentPerformanceRisk => "CurrentPerformanceRisk",
    ExportableInstanceField::CurrentStandardOneYearNoUpfrontReservedPrice => "CurrentStandardOneYearNoUpfrontReservedPrice",
    ExportableInstanceField::CurrentStandardThreeYearNoUpfrontReservedPrice => "CurrentStandardThreeYearNoUpfrontReservedPrice",
    ExportableInstanceField::CurrentStorage => "CurrentStorage",
    ExportableInstanceField::CurrentVcpus => "CurrentVCpus",
    ExportableInstanceField::EffectiveRecommendationPreferencesCpuVendorArchitectures => "EffectiveRecommendationPreferencesCpuVendorArchitectures",
    ExportableInstanceField::EffectiveRecommendationPreferencesEnhancedInfrastructureMetrics => "EffectiveRecommendationPreferencesEnhancedInfrastructureMetrics",
    ExportableInstanceField::EffectiveRecommendationPreferencesExternalMetricsSource => "EffectiveRecommendationPreferencesExternalMetricsSource",
    ExportableInstanceField::EffectiveRecommendationPreferencesInferredWorkloadTypes => "EffectiveRecommendationPreferencesInferredWorkloadTypes",
    ExportableInstanceField::ExternalMetricStatusCode => "ExternalMetricStatusCode",
    ExportableInstanceField::ExternalMetricStatusReason => "ExternalMetricStatusReason",
    ExportableInstanceField::Finding => "Finding",
    ExportableInstanceField::FindingReasonCodes => "FindingReasonCodes",
    ExportableInstanceField::InferredWorkloadTypes => "InferredWorkloadTypes",
    ExportableInstanceField::InstanceArn => "InstanceArn",
    ExportableInstanceField::InstanceName => "InstanceName",
    ExportableInstanceField::InstanceState => "InstanceState",
    ExportableInstanceField::LastRefreshTimestamp => "LastRefreshTimestamp",
    ExportableInstanceField::LookbackPeriodInDays => "LookbackPeriodInDays",
    ExportableInstanceField::RecommendationOptionsEstimatedMonthlySavingsCurrency => "RecommendationOptionsEstimatedMonthlySavingsCurrency",
    ExportableInstanceField::RecommendationOptionsEstimatedMonthlySavingsValue => "RecommendationOptionsEstimatedMonthlySavingsValue",
    ExportableInstanceField::RecommendationOptionsInstanceType => "RecommendationOptionsInstanceType",
    ExportableInstanceField::RecommendationOptionsMemory => "RecommendationOptionsMemory",
    ExportableInstanceField::RecommendationOptionsMigrationEffort => "RecommendationOptionsMigrationEffort",
    ExportableInstanceField::RecommendationOptionsNetwork => "RecommendationOptionsNetwork",
    ExportableInstanceField::RecommendationOptionsOnDemandPrice => "RecommendationOptionsOnDemandPrice",
    ExportableInstanceField::RecommendationOptionsPerformanceRisk => "RecommendationOptionsPerformanceRisk",
    ExportableInstanceField::RecommendationOptionsPlatformDifferences => "RecommendationOptionsPlatformDifferences",
    ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum => "RecommendationOptionsProjectedUtilizationMetricsCpuMaximum",
    ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum => "RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum",
    ExportableInstanceField::RecommendationOptionsSavingsOpportunityPercentage => "RecommendationOptionsSavingsOpportunityPercentage",
    ExportableInstanceField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice => "RecommendationOptionsStandardOneYearNoUpfrontReservedPrice",
    ExportableInstanceField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice => "RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice",
    ExportableInstanceField::RecommendationOptionsStorage => "RecommendationOptionsStorage",
    ExportableInstanceField::RecommendationOptionsVcpus => "RecommendationOptionsVcpus",
    ExportableInstanceField::RecommendationsSourcesRecommendationSourceArn => "RecommendationsSourcesRecommendationSourceArn",
    ExportableInstanceField::RecommendationsSourcesRecommendationSourceType => "RecommendationsSourcesRecommendationSourceType",
    ExportableInstanceField::Tags => "Tags",
    ExportableInstanceField::UtilizationMetricsCpuMaximum => "UtilizationMetricsCpuMaximum",
    ExportableInstanceField::UtilizationMetricsDiskReadBytesPerSecondMaximum => "UtilizationMetricsDiskReadBytesPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsDiskReadOpsPerSecondMaximum => "UtilizationMetricsDiskReadOpsPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsDiskWriteBytesPerSecondMaximum => "UtilizationMetricsDiskWriteBytesPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsDiskWriteOpsPerSecondMaximum => "UtilizationMetricsDiskWriteOpsPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsEbsReadBytesPerSecondMaximum => "UtilizationMetricsEbsReadBytesPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsEbsReadOpsPerSecondMaximum => "UtilizationMetricsEbsReadOpsPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsEbsWriteBytesPerSecondMaximum => "UtilizationMetricsEbsWriteBytesPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsEbsWriteOpsPerSecondMaximum => "UtilizationMetricsEbsWriteOpsPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsMemoryMaximum => "UtilizationMetricsMemoryMaximum",
    ExportableInstanceField::UtilizationMetricsNetworkInBytesPerSecondMaximum => "UtilizationMetricsNetworkInBytesPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsNetworkOutBytesPerSecondMaximum => "UtilizationMetricsNetworkOutBytesPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsNetworkPacketsInPerSecondMaximum => "UtilizationMetricsNetworkPacketsInPerSecondMaximum",
    ExportableInstanceField::UtilizationMetricsNetworkPacketsOutPerSecondMaximum => "UtilizationMetricsNetworkPacketsOutPerSecondMaximum",
    ExportableInstanceField::Unknown(value) => value.as_str()
}
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "AccountId",
            "CurrentInstanceType",
            "CurrentMemory",
            "CurrentNetwork",
            "CurrentOnDemandPrice",
            "CurrentPerformanceRisk",
            "CurrentStandardOneYearNoUpfrontReservedPrice",
            "CurrentStandardThreeYearNoUpfrontReservedPrice",
            "CurrentStorage",
            "CurrentVCpus",
            "EffectiveRecommendationPreferencesCpuVendorArchitectures",
            "EffectiveRecommendationPreferencesEnhancedInfrastructureMetrics",
            "EffectiveRecommendationPreferencesExternalMetricsSource",
            "EffectiveRecommendationPreferencesInferredWorkloadTypes",
            "ExternalMetricStatusCode",
            "ExternalMetricStatusReason",
            "Finding",
            "FindingReasonCodes",
            "InferredWorkloadTypes",
            "InstanceArn",
            "InstanceName",
            "InstanceState",
            "LastRefreshTimestamp",
            "LookbackPeriodInDays",
            "RecommendationOptionsEstimatedMonthlySavingsCurrency",
            "RecommendationOptionsEstimatedMonthlySavingsValue",
            "RecommendationOptionsInstanceType",
            "RecommendationOptionsMemory",
            "RecommendationOptionsMigrationEffort",
            "RecommendationOptionsNetwork",
            "RecommendationOptionsOnDemandPrice",
            "RecommendationOptionsPerformanceRisk",
            "RecommendationOptionsPlatformDifferences",
            "RecommendationOptionsProjectedUtilizationMetricsCpuMaximum",
            "RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum",
            "RecommendationOptionsSavingsOpportunityPercentage",
            "RecommendationOptionsStandardOneYearNoUpfrontReservedPrice",
            "RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice",
            "RecommendationOptionsStorage",
            "RecommendationOptionsVcpus",
            "RecommendationsSourcesRecommendationSourceArn",
            "RecommendationsSourcesRecommendationSourceType",
            "Tags",
            "UtilizationMetricsCpuMaximum",
            "UtilizationMetricsDiskReadBytesPerSecondMaximum",
            "UtilizationMetricsDiskReadOpsPerSecondMaximum",
            "UtilizationMetricsDiskWriteBytesPerSecondMaximum",
            "UtilizationMetricsDiskWriteOpsPerSecondMaximum",
            "UtilizationMetricsEbsReadBytesPerSecondMaximum",
            "UtilizationMetricsEbsReadOpsPerSecondMaximum",
            "UtilizationMetricsEbsWriteBytesPerSecondMaximum",
            "UtilizationMetricsEbsWriteOpsPerSecondMaximum",
            "UtilizationMetricsMemoryMaximum",
            "UtilizationMetricsNetworkInBytesPerSecondMaximum",
            "UtilizationMetricsNetworkOutBytesPerSecondMaximum",
            "UtilizationMetricsNetworkPacketsInPerSecondMaximum",
            "UtilizationMetricsNetworkPacketsOutPerSecondMaximum",
        ]
    }
}
impl ::std::convert::AsRef<str> for ExportableInstanceField {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
