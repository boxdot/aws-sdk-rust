// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDataQualityRuleRecommendationRunInput {
    /// <p>The unique run identifier associated with this run.</p>
    #[doc(hidden)]
    pub run_id: ::std::option::Option<::std::string::String>,
}
impl GetDataQualityRuleRecommendationRunInput {
    /// <p>The unique run identifier associated with this run.</p>
    pub fn run_id(&self) -> ::std::option::Option<&str> {
        self.run_id.as_deref()
    }
}
impl GetDataQualityRuleRecommendationRunInput {
    /// Creates a new builder-style object to manufacture [`GetDataQualityRuleRecommendationRunInput`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunInput).
    pub fn builder() -> crate::operation::get_data_quality_rule_recommendation_run::builders::GetDataQualityRuleRecommendationRunInputBuilder{
        crate::operation::get_data_quality_rule_recommendation_run::builders::GetDataQualityRuleRecommendationRunInputBuilder::default()
    }
}

/// A builder for [`GetDataQualityRuleRecommendationRunInput`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetDataQualityRuleRecommendationRunInputBuilder {
    pub(crate) run_id: ::std::option::Option<::std::string::String>,
}
impl GetDataQualityRuleRecommendationRunInputBuilder {
    /// <p>The unique run identifier associated with this run.</p>
    pub fn run_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.run_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique run identifier associated with this run.</p>
    pub fn set_run_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.run_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetDataQualityRuleRecommendationRunInput`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunInput {
                run_id: self.run_id
                ,
            }
        )
    }
}
