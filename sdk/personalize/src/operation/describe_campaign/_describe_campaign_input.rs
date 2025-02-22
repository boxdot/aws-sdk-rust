// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeCampaignInput {
    /// <p>The Amazon Resource Name (ARN) of the campaign.</p>
    #[doc(hidden)]
    pub campaign_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeCampaignInput {
    /// <p>The Amazon Resource Name (ARN) of the campaign.</p>
    pub fn campaign_arn(&self) -> ::std::option::Option<&str> {
        self.campaign_arn.as_deref()
    }
}
impl DescribeCampaignInput {
    /// Creates a new builder-style object to manufacture [`DescribeCampaignInput`](crate::operation::describe_campaign::DescribeCampaignInput).
    pub fn builder() -> crate::operation::describe_campaign::builders::DescribeCampaignInputBuilder
    {
        crate::operation::describe_campaign::builders::DescribeCampaignInputBuilder::default()
    }
}

/// A builder for [`DescribeCampaignInput`](crate::operation::describe_campaign::DescribeCampaignInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeCampaignInputBuilder {
    pub(crate) campaign_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeCampaignInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the campaign.</p>
    pub fn campaign_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.campaign_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the campaign.</p>
    pub fn set_campaign_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.campaign_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeCampaignInput`](crate::operation::describe_campaign::DescribeCampaignInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_campaign::DescribeCampaignInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_campaign::DescribeCampaignInput {
            campaign_arn: self.campaign_arn,
        })
    }
}
