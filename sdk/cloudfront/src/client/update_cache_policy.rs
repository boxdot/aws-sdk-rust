// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateCachePolicy`](crate::operation::update_cache_policy::builders::UpdateCachePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cache_policy_config(CachePolicyConfig)`](crate::operation::update_cache_policy::builders::UpdateCachePolicyFluentBuilder::cache_policy_config) / [`set_cache_policy_config(Option<CachePolicyConfig>)`](crate::operation::update_cache_policy::builders::UpdateCachePolicyFluentBuilder::set_cache_policy_config): <p>A cache policy configuration.</p>
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::update_cache_policy::builders::UpdateCachePolicyFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_cache_policy::builders::UpdateCachePolicyFluentBuilder::set_id): <p>The unique identifier for the cache policy that you are updating. The identifier is returned in a cache behavior's <code>CachePolicyId</code> field in the response to <code>GetDistributionConfig</code>.</p>
    ///   - [`if_match(impl ::std::convert::Into<String>)`](crate::operation::update_cache_policy::builders::UpdateCachePolicyFluentBuilder::if_match) / [`set_if_match(Option<String>)`](crate::operation::update_cache_policy::builders::UpdateCachePolicyFluentBuilder::set_if_match): <p>The version of the cache policy that you are updating. The version is returned in the cache policy's <code>ETag</code> field in the response to <code>GetCachePolicyConfig</code>.</p>
    /// - On success, responds with [`UpdateCachePolicyOutput`](crate::operation::update_cache_policy::UpdateCachePolicyOutput) with field(s):
    ///   - [`cache_policy(Option<CachePolicy>)`](crate::operation::update_cache_policy::UpdateCachePolicyOutput::cache_policy): <p>A cache policy.</p>
    ///   - [`e_tag(Option<String>)`](crate::operation::update_cache_policy::UpdateCachePolicyOutput::e_tag): <p>The current version of the cache policy.</p>
    /// - On failure, responds with [`SdkError<UpdateCachePolicyError>`](crate::operation::update_cache_policy::UpdateCachePolicyError)
    pub fn update_cache_policy(
        &self,
    ) -> crate::operation::update_cache_policy::builders::UpdateCachePolicyFluentBuilder {
        crate::operation::update_cache_policy::builders::UpdateCachePolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
