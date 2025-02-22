// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetReservationPurchaseRecommendationOutput {
    /// <p>Information about this specific recommendation call, such as the time stamp for when Cost Explorer generated this recommendation.</p>
    #[doc(hidden)]
    pub metadata: ::std::option::Option<crate::types::ReservationPurchaseRecommendationMetadata>,
    /// <p>Recommendations for reservations to purchase.</p>
    #[doc(hidden)]
    pub recommendations:
        ::std::option::Option<::std::vec::Vec<crate::types::ReservationPurchaseRecommendation>>,
    /// <p>The pagination token for the next set of retrievable results.</p>
    #[doc(hidden)]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetReservationPurchaseRecommendationOutput {
    /// <p>Information about this specific recommendation call, such as the time stamp for when Cost Explorer generated this recommendation.</p>
    pub fn metadata(
        &self,
    ) -> ::std::option::Option<&crate::types::ReservationPurchaseRecommendationMetadata> {
        self.metadata.as_ref()
    }
    /// <p>Recommendations for reservations to purchase.</p>
    pub fn recommendations(
        &self,
    ) -> ::std::option::Option<&[crate::types::ReservationPurchaseRecommendation]> {
        self.recommendations.as_deref()
    }
    /// <p>The pagination token for the next set of retrievable results.</p>
    pub fn next_page_token(&self) -> ::std::option::Option<&str> {
        self.next_page_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetReservationPurchaseRecommendationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetReservationPurchaseRecommendationOutput {
    /// Creates a new builder-style object to manufacture [`GetReservationPurchaseRecommendationOutput`](crate::operation::get_reservation_purchase_recommendation::GetReservationPurchaseRecommendationOutput).
    pub fn builder() -> crate::operation::get_reservation_purchase_recommendation::builders::GetReservationPurchaseRecommendationOutputBuilder{
        crate::operation::get_reservation_purchase_recommendation::builders::GetReservationPurchaseRecommendationOutputBuilder::default()
    }
}

/// A builder for [`GetReservationPurchaseRecommendationOutput`](crate::operation::get_reservation_purchase_recommendation::GetReservationPurchaseRecommendationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetReservationPurchaseRecommendationOutputBuilder {
    pub(crate) metadata:
        ::std::option::Option<crate::types::ReservationPurchaseRecommendationMetadata>,
    pub(crate) recommendations:
        ::std::option::Option<::std::vec::Vec<crate::types::ReservationPurchaseRecommendation>>,
    pub(crate) next_page_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetReservationPurchaseRecommendationOutputBuilder {
    /// <p>Information about this specific recommendation call, such as the time stamp for when Cost Explorer generated this recommendation.</p>
    pub fn metadata(
        mut self,
        input: crate::types::ReservationPurchaseRecommendationMetadata,
    ) -> Self {
        self.metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about this specific recommendation call, such as the time stamp for when Cost Explorer generated this recommendation.</p>
    pub fn set_metadata(
        mut self,
        input: ::std::option::Option<crate::types::ReservationPurchaseRecommendationMetadata>,
    ) -> Self {
        self.metadata = input;
        self
    }
    /// Appends an item to `recommendations`.
    ///
    /// To override the contents of this collection use [`set_recommendations`](Self::set_recommendations).
    ///
    /// <p>Recommendations for reservations to purchase.</p>
    pub fn recommendations(
        mut self,
        input: crate::types::ReservationPurchaseRecommendation,
    ) -> Self {
        let mut v = self.recommendations.unwrap_or_default();
        v.push(input);
        self.recommendations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Recommendations for reservations to purchase.</p>
    pub fn set_recommendations(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::ReservationPurchaseRecommendation>,
        >,
    ) -> Self {
        self.recommendations = input;
        self
    }
    /// <p>The pagination token for the next set of retrievable results.</p>
    pub fn next_page_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.next_page_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token for the next set of retrievable results.</p>
    pub fn set_next_page_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.next_page_token = input;
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
    /// Consumes the builder and constructs a [`GetReservationPurchaseRecommendationOutput`](crate::operation::get_reservation_purchase_recommendation::GetReservationPurchaseRecommendationOutput).
    pub fn build(self) -> crate::operation::get_reservation_purchase_recommendation::GetReservationPurchaseRecommendationOutput{
        crate::operation::get_reservation_purchase_recommendation::GetReservationPurchaseRecommendationOutput {
            metadata: self.metadata
            ,
            recommendations: self.recommendations
            ,
            next_page_token: self.next_page_token
            ,
            _request_id: self._request_id,
        }
    }
}
