// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribePartnersOutput {
    /// <p>A list of partner integrations.</p>
    #[doc(hidden)]
    pub partner_integration_info_list:
        ::std::option::Option<::std::vec::Vec<crate::types::PartnerIntegrationInfo>>,
    _request_id: Option<String>,
}
impl DescribePartnersOutput {
    /// <p>A list of partner integrations.</p>
    pub fn partner_integration_info_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::PartnerIntegrationInfo]> {
        self.partner_integration_info_list.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribePartnersOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribePartnersOutput {
    /// Creates a new builder-style object to manufacture [`DescribePartnersOutput`](crate::operation::describe_partners::DescribePartnersOutput).
    pub fn builder() -> crate::operation::describe_partners::builders::DescribePartnersOutputBuilder
    {
        crate::operation::describe_partners::builders::DescribePartnersOutputBuilder::default()
    }
}

/// A builder for [`DescribePartnersOutput`](crate::operation::describe_partners::DescribePartnersOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribePartnersOutputBuilder {
    pub(crate) partner_integration_info_list:
        ::std::option::Option<::std::vec::Vec<crate::types::PartnerIntegrationInfo>>,
    _request_id: Option<String>,
}
impl DescribePartnersOutputBuilder {
    /// Appends an item to `partner_integration_info_list`.
    ///
    /// To override the contents of this collection use [`set_partner_integration_info_list`](Self::set_partner_integration_info_list).
    ///
    /// <p>A list of partner integrations.</p>
    pub fn partner_integration_info_list(
        mut self,
        input: crate::types::PartnerIntegrationInfo,
    ) -> Self {
        let mut v = self.partner_integration_info_list.unwrap_or_default();
        v.push(input);
        self.partner_integration_info_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of partner integrations.</p>
    pub fn set_partner_integration_info_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PartnerIntegrationInfo>>,
    ) -> Self {
        self.partner_integration_info_list = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribePartnersOutput`](crate::operation::describe_partners::DescribePartnersOutput).
    pub fn build(self) -> crate::operation::describe_partners::DescribePartnersOutput {
        crate::operation::describe_partners::DescribePartnersOutput {
            partner_integration_info_list: self.partner_integration_info_list,
            _request_id: self._request_id,
        }
    }
}
