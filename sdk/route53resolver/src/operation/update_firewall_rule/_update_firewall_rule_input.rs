// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateFirewallRuleInput {
    /// <p>The unique identifier of the firewall rule group for the rule. </p>
    #[doc(hidden)]
    pub firewall_rule_group_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the domain list to use in the rule. </p>
    #[doc(hidden)]
    pub firewall_domain_list_id: ::std::option::Option<::std::string::String>,
    /// <p>The setting that determines the processing order of the rule in the rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.</p>
    /// <p>You must specify a unique priority for each rule in a rule group. To make it easier to insert rules later, leave space between the numbers, for example, use 100, 200, and so on. You can change the priority setting for the rules in a rule group at any time.</p>
    #[doc(hidden)]
    pub priority: ::std::option::Option<i32>,
    /// <p>The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list:</p>
    /// <ul>
    /// <li> <p> <code>ALLOW</code> - Permit the request to go through.</p> </li>
    /// <li> <p> <code>ALERT</code> - Permit the request to go through but send an alert to the logs.</p> </li>
    /// <li> <p> <code>BLOCK</code> - Disallow the request. This option requires additional details in the rule's <code>BlockResponse</code>. </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub action: ::std::option::Option<crate::types::Action>,
    /// <p>The way that you want DNS Firewall to block the request. Used for the rule action setting <code>BLOCK</code>.</p>
    /// <ul>
    /// <li> <p> <code>NODATA</code> - Respond indicating that the query was successful, but no response is available for it.</p> </li>
    /// <li> <p> <code>NXDOMAIN</code> - Respond indicating that the domain name that's in the query doesn't exist.</p> </li>
    /// <li> <p> <code>OVERRIDE</code> - Provide a custom override in the response. This option requires custom handling details in the rule's <code>BlockOverride*</code> settings. </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub block_response: ::std::option::Option<crate::types::BlockResponse>,
    /// <p>The custom DNS record to send back in response to the query. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    #[doc(hidden)]
    pub block_override_domain: ::std::option::Option<::std::string::String>,
    /// <p>The DNS record's type. This determines the format of the record value that you provided in <code>BlockOverrideDomain</code>. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    #[doc(hidden)]
    pub block_override_dns_type: ::std::option::Option<crate::types::BlockOverrideDnsType>,
    /// <p>The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    #[doc(hidden)]
    pub block_override_ttl: ::std::option::Option<i32>,
    /// <p>The name of the rule.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl UpdateFirewallRuleInput {
    /// <p>The unique identifier of the firewall rule group for the rule. </p>
    pub fn firewall_rule_group_id(&self) -> ::std::option::Option<&str> {
        self.firewall_rule_group_id.as_deref()
    }
    /// <p>The ID of the domain list to use in the rule. </p>
    pub fn firewall_domain_list_id(&self) -> ::std::option::Option<&str> {
        self.firewall_domain_list_id.as_deref()
    }
    /// <p>The setting that determines the processing order of the rule in the rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.</p>
    /// <p>You must specify a unique priority for each rule in a rule group. To make it easier to insert rules later, leave space between the numbers, for example, use 100, 200, and so on. You can change the priority setting for the rules in a rule group at any time.</p>
    pub fn priority(&self) -> ::std::option::Option<i32> {
        self.priority
    }
    /// <p>The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list:</p>
    /// <ul>
    /// <li> <p> <code>ALLOW</code> - Permit the request to go through.</p> </li>
    /// <li> <p> <code>ALERT</code> - Permit the request to go through but send an alert to the logs.</p> </li>
    /// <li> <p> <code>BLOCK</code> - Disallow the request. This option requires additional details in the rule's <code>BlockResponse</code>. </p> </li>
    /// </ul>
    pub fn action(&self) -> ::std::option::Option<&crate::types::Action> {
        self.action.as_ref()
    }
    /// <p>The way that you want DNS Firewall to block the request. Used for the rule action setting <code>BLOCK</code>.</p>
    /// <ul>
    /// <li> <p> <code>NODATA</code> - Respond indicating that the query was successful, but no response is available for it.</p> </li>
    /// <li> <p> <code>NXDOMAIN</code> - Respond indicating that the domain name that's in the query doesn't exist.</p> </li>
    /// <li> <p> <code>OVERRIDE</code> - Provide a custom override in the response. This option requires custom handling details in the rule's <code>BlockOverride*</code> settings. </p> </li>
    /// </ul>
    pub fn block_response(&self) -> ::std::option::Option<&crate::types::BlockResponse> {
        self.block_response.as_ref()
    }
    /// <p>The custom DNS record to send back in response to the query. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    pub fn block_override_domain(&self) -> ::std::option::Option<&str> {
        self.block_override_domain.as_deref()
    }
    /// <p>The DNS record's type. This determines the format of the record value that you provided in <code>BlockOverrideDomain</code>. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    pub fn block_override_dns_type(
        &self,
    ) -> ::std::option::Option<&crate::types::BlockOverrideDnsType> {
        self.block_override_dns_type.as_ref()
    }
    /// <p>The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    pub fn block_override_ttl(&self) -> ::std::option::Option<i32> {
        self.block_override_ttl
    }
    /// <p>The name of the rule.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl UpdateFirewallRuleInput {
    /// Creates a new builder-style object to manufacture [`UpdateFirewallRuleInput`](crate::operation::update_firewall_rule::UpdateFirewallRuleInput).
    pub fn builder(
    ) -> crate::operation::update_firewall_rule::builders::UpdateFirewallRuleInputBuilder {
        crate::operation::update_firewall_rule::builders::UpdateFirewallRuleInputBuilder::default()
    }
}

/// A builder for [`UpdateFirewallRuleInput`](crate::operation::update_firewall_rule::UpdateFirewallRuleInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateFirewallRuleInputBuilder {
    pub(crate) firewall_rule_group_id: ::std::option::Option<::std::string::String>,
    pub(crate) firewall_domain_list_id: ::std::option::Option<::std::string::String>,
    pub(crate) priority: ::std::option::Option<i32>,
    pub(crate) action: ::std::option::Option<crate::types::Action>,
    pub(crate) block_response: ::std::option::Option<crate::types::BlockResponse>,
    pub(crate) block_override_domain: ::std::option::Option<::std::string::String>,
    pub(crate) block_override_dns_type: ::std::option::Option<crate::types::BlockOverrideDnsType>,
    pub(crate) block_override_ttl: ::std::option::Option<i32>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl UpdateFirewallRuleInputBuilder {
    /// <p>The unique identifier of the firewall rule group for the rule. </p>
    pub fn firewall_rule_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.firewall_rule_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the firewall rule group for the rule. </p>
    pub fn set_firewall_rule_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.firewall_rule_group_id = input;
        self
    }
    /// <p>The ID of the domain list to use in the rule. </p>
    pub fn firewall_domain_list_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.firewall_domain_list_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the domain list to use in the rule. </p>
    pub fn set_firewall_domain_list_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.firewall_domain_list_id = input;
        self
    }
    /// <p>The setting that determines the processing order of the rule in the rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.</p>
    /// <p>You must specify a unique priority for each rule in a rule group. To make it easier to insert rules later, leave space between the numbers, for example, use 100, 200, and so on. You can change the priority setting for the rules in a rule group at any time.</p>
    pub fn priority(mut self, input: i32) -> Self {
        self.priority = ::std::option::Option::Some(input);
        self
    }
    /// <p>The setting that determines the processing order of the rule in the rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.</p>
    /// <p>You must specify a unique priority for each rule in a rule group. To make it easier to insert rules later, leave space between the numbers, for example, use 100, 200, and so on. You can change the priority setting for the rules in a rule group at any time.</p>
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.priority = input;
        self
    }
    /// <p>The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list:</p>
    /// <ul>
    /// <li> <p> <code>ALLOW</code> - Permit the request to go through.</p> </li>
    /// <li> <p> <code>ALERT</code> - Permit the request to go through but send an alert to the logs.</p> </li>
    /// <li> <p> <code>BLOCK</code> - Disallow the request. This option requires additional details in the rule's <code>BlockResponse</code>. </p> </li>
    /// </ul>
    pub fn action(mut self, input: crate::types::Action) -> Self {
        self.action = ::std::option::Option::Some(input);
        self
    }
    /// <p>The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list:</p>
    /// <ul>
    /// <li> <p> <code>ALLOW</code> - Permit the request to go through.</p> </li>
    /// <li> <p> <code>ALERT</code> - Permit the request to go through but send an alert to the logs.</p> </li>
    /// <li> <p> <code>BLOCK</code> - Disallow the request. This option requires additional details in the rule's <code>BlockResponse</code>. </p> </li>
    /// </ul>
    pub fn set_action(mut self, input: ::std::option::Option<crate::types::Action>) -> Self {
        self.action = input;
        self
    }
    /// <p>The way that you want DNS Firewall to block the request. Used for the rule action setting <code>BLOCK</code>.</p>
    /// <ul>
    /// <li> <p> <code>NODATA</code> - Respond indicating that the query was successful, but no response is available for it.</p> </li>
    /// <li> <p> <code>NXDOMAIN</code> - Respond indicating that the domain name that's in the query doesn't exist.</p> </li>
    /// <li> <p> <code>OVERRIDE</code> - Provide a custom override in the response. This option requires custom handling details in the rule's <code>BlockOverride*</code> settings. </p> </li>
    /// </ul>
    pub fn block_response(mut self, input: crate::types::BlockResponse) -> Self {
        self.block_response = ::std::option::Option::Some(input);
        self
    }
    /// <p>The way that you want DNS Firewall to block the request. Used for the rule action setting <code>BLOCK</code>.</p>
    /// <ul>
    /// <li> <p> <code>NODATA</code> - Respond indicating that the query was successful, but no response is available for it.</p> </li>
    /// <li> <p> <code>NXDOMAIN</code> - Respond indicating that the domain name that's in the query doesn't exist.</p> </li>
    /// <li> <p> <code>OVERRIDE</code> - Provide a custom override in the response. This option requires custom handling details in the rule's <code>BlockOverride*</code> settings. </p> </li>
    /// </ul>
    pub fn set_block_response(
        mut self,
        input: ::std::option::Option<crate::types::BlockResponse>,
    ) -> Self {
        self.block_response = input;
        self
    }
    /// <p>The custom DNS record to send back in response to the query. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    pub fn block_override_domain(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.block_override_domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom DNS record to send back in response to the query. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    pub fn set_block_override_domain(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.block_override_domain = input;
        self
    }
    /// <p>The DNS record's type. This determines the format of the record value that you provided in <code>BlockOverrideDomain</code>. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    pub fn block_override_dns_type(mut self, input: crate::types::BlockOverrideDnsType) -> Self {
        self.block_override_dns_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The DNS record's type. This determines the format of the record value that you provided in <code>BlockOverrideDomain</code>. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    pub fn set_block_override_dns_type(
        mut self,
        input: ::std::option::Option<crate::types::BlockOverrideDnsType>,
    ) -> Self {
        self.block_override_dns_type = input;
        self
    }
    /// <p>The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    pub fn block_override_ttl(mut self, input: i32) -> Self {
        self.block_override_ttl = ::std::option::Option::Some(input);
        self
    }
    /// <p>The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
    pub fn set_block_override_ttl(mut self, input: ::std::option::Option<i32>) -> Self {
        self.block_override_ttl = input;
        self
    }
    /// <p>The name of the rule.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the rule.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateFirewallRuleInput`](crate::operation::update_firewall_rule::UpdateFirewallRuleInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_firewall_rule::UpdateFirewallRuleInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_firewall_rule::UpdateFirewallRuleInput {
                firewall_rule_group_id: self.firewall_rule_group_id,
                firewall_domain_list_id: self.firewall_domain_list_id,
                priority: self.priority,
                action: self.action,
                block_response: self.block_response,
                block_override_domain: self.block_override_domain,
                block_override_dns_type: self.block_override_dns_type,
                block_override_ttl: self.block_override_ttl,
                name: self.name,
            },
        )
    }
}
