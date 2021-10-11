// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates and persists a DataIntegration resource.</p>
/// <note>
/// <p>You cannot create a DataIntegration association for a DataIntegration that has been previously associated.  
/// Use a different DataIntegration, or recreate the DataIntegration using the
/// <code>CreateDataIntegration</code> API.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateDataIntegration {
    _private: (),
}
impl CreateDataIntegration {
    /// Creates a new builder-style object to manufacture [`CreateDataIntegrationInput`](crate::input::CreateDataIntegrationInput)
    pub fn builder() -> crate::input::create_data_integration_input::Builder {
        crate::input::create_data_integration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateDataIntegration {
    type Output = std::result::Result<
        crate::output::CreateDataIntegrationOutput,
        crate::error::CreateDataIntegrationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_data_integration_error(response)
        } else {
            crate::operation_deser::parse_create_data_integration_response(response)
        }
    }
}

/// <p>Creates an EventIntegration, given a specified name, description, and a reference to an
/// Amazon EventBridge bus in your account and a partner event source that pushes events to that bus. No
/// objects are created in the your account, only metadata that is persisted on the
/// EventIntegration control plane.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateEventIntegration {
    _private: (),
}
impl CreateEventIntegration {
    /// Creates a new builder-style object to manufacture [`CreateEventIntegrationInput`](crate::input::CreateEventIntegrationInput)
    pub fn builder() -> crate::input::create_event_integration_input::Builder {
        crate::input::create_event_integration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateEventIntegration {
    type Output = std::result::Result<
        crate::output::CreateEventIntegrationOutput,
        crate::error::CreateEventIntegrationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_event_integration_error(response)
        } else {
            crate::operation_deser::parse_create_event_integration_response(response)
        }
    }
}

/// <p>Deletes the DataIntegration. Only DataIntegrations that don't have any
/// DataIntegrationAssociations can be deleted. Deleting a DataIntegration also deletes the
/// underlying Amazon AppFlow flow and service linked role. </p>
/// <note>
/// <p>You cannot create a DataIntegration association for a DataIntegration that has been previously associated.  
/// Use a different DataIntegration, or recreate the DataIntegration using the
/// <a href="https://docs.aws.amazon.com/appintegrations/latest/APIReference/API_CreateDataIntegration.html">CreateDataIntegration</a> API.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDataIntegration {
    _private: (),
}
impl DeleteDataIntegration {
    /// Creates a new builder-style object to manufacture [`DeleteDataIntegrationInput`](crate::input::DeleteDataIntegrationInput)
    pub fn builder() -> crate::input::delete_data_integration_input::Builder {
        crate::input::delete_data_integration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteDataIntegration {
    type Output = std::result::Result<
        crate::output::DeleteDataIntegrationOutput,
        crate::error::DeleteDataIntegrationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_data_integration_error(response)
        } else {
            crate::operation_deser::parse_delete_data_integration_response(response)
        }
    }
}

/// <p>Deletes the specified existing event integration. If the event integration is associated
/// with clients, the request is rejected.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEventIntegration {
    _private: (),
}
impl DeleteEventIntegration {
    /// Creates a new builder-style object to manufacture [`DeleteEventIntegrationInput`](crate::input::DeleteEventIntegrationInput)
    pub fn builder() -> crate::input::delete_event_integration_input::Builder {
        crate::input::delete_event_integration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteEventIntegration {
    type Output = std::result::Result<
        crate::output::DeleteEventIntegrationOutput,
        crate::error::DeleteEventIntegrationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_event_integration_error(response)
        } else {
            crate::operation_deser::parse_delete_event_integration_response(response)
        }
    }
}

/// <p>Returns information about the DataIntegration.</p>
/// <note>
/// <p>You cannot create a DataIntegration association for a DataIntegration that has been previously associated.  
/// Use a different DataIntegration, or recreate the DataIntegration using the
/// <a href="https://docs.aws.amazon.com/appintegrations/latest/APIReference/API_CreateDataIntegration.html">CreateDataIntegration</a> API.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDataIntegration {
    _private: (),
}
impl GetDataIntegration {
    /// Creates a new builder-style object to manufacture [`GetDataIntegrationInput`](crate::input::GetDataIntegrationInput)
    pub fn builder() -> crate::input::get_data_integration_input::Builder {
        crate::input::get_data_integration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetDataIntegration {
    type Output = std::result::Result<
        crate::output::GetDataIntegrationOutput,
        crate::error::GetDataIntegrationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_data_integration_error(response)
        } else {
            crate::operation_deser::parse_get_data_integration_response(response)
        }
    }
}

/// <p>Returns information about the event integration.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetEventIntegration {
    _private: (),
}
impl GetEventIntegration {
    /// Creates a new builder-style object to manufacture [`GetEventIntegrationInput`](crate::input::GetEventIntegrationInput)
    pub fn builder() -> crate::input::get_event_integration_input::Builder {
        crate::input::get_event_integration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetEventIntegration {
    type Output = std::result::Result<
        crate::output::GetEventIntegrationOutput,
        crate::error::GetEventIntegrationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_event_integration_error(response)
        } else {
            crate::operation_deser::parse_get_event_integration_response(response)
        }
    }
}

/// <p>Returns a paginated list of DataIntegration associations in the account.</p>
/// <note>
/// <p>You cannot create a DataIntegration association for a DataIntegration that has been previously associated.  
/// Use a different DataIntegration, or recreate the DataIntegration using the
/// <a href="https://docs.aws.amazon.com/appintegrations/latest/APIReference/API_CreateDataIntegration.html">CreateDataIntegration</a> API.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDataIntegrationAssociations {
    _private: (),
}
impl ListDataIntegrationAssociations {
    /// Creates a new builder-style object to manufacture [`ListDataIntegrationAssociationsInput`](crate::input::ListDataIntegrationAssociationsInput)
    pub fn builder() -> crate::input::list_data_integration_associations_input::Builder {
        crate::input::list_data_integration_associations_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListDataIntegrationAssociations {
    type Output = std::result::Result<
        crate::output::ListDataIntegrationAssociationsOutput,
        crate::error::ListDataIntegrationAssociationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_data_integration_associations_error(response)
        } else {
            crate::operation_deser::parse_list_data_integration_associations_response(response)
        }
    }
}

/// <p>Returns a paginated list of DataIntegrations in the account.</p>
/// <note>
/// <p>You cannot create a DataIntegration association for a DataIntegration that has been previously associated.  
/// Use a different DataIntegration, or recreate the DataIntegration using the
/// <a href="https://docs.aws.amazon.com/appintegrations/latest/APIReference/API_CreateDataIntegration.html">CreateDataIntegration</a> API.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDataIntegrations {
    _private: (),
}
impl ListDataIntegrations {
    /// Creates a new builder-style object to manufacture [`ListDataIntegrationsInput`](crate::input::ListDataIntegrationsInput)
    pub fn builder() -> crate::input::list_data_integrations_input::Builder {
        crate::input::list_data_integrations_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListDataIntegrations {
    type Output = std::result::Result<
        crate::output::ListDataIntegrationsOutput,
        crate::error::ListDataIntegrationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_data_integrations_error(response)
        } else {
            crate::operation_deser::parse_list_data_integrations_response(response)
        }
    }
}

/// <p>Returns a paginated list of event integration associations in the account. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEventIntegrationAssociations {
    _private: (),
}
impl ListEventIntegrationAssociations {
    /// Creates a new builder-style object to manufacture [`ListEventIntegrationAssociationsInput`](crate::input::ListEventIntegrationAssociationsInput)
    pub fn builder() -> crate::input::list_event_integration_associations_input::Builder {
        crate::input::list_event_integration_associations_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListEventIntegrationAssociations {
    type Output = std::result::Result<
        crate::output::ListEventIntegrationAssociationsOutput,
        crate::error::ListEventIntegrationAssociationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_event_integration_associations_error(response)
        } else {
            crate::operation_deser::parse_list_event_integration_associations_response(response)
        }
    }
}

/// <p>Returns a paginated list of event integrations in the account.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEventIntegrations {
    _private: (),
}
impl ListEventIntegrations {
    /// Creates a new builder-style object to manufacture [`ListEventIntegrationsInput`](crate::input::ListEventIntegrationsInput)
    pub fn builder() -> crate::input::list_event_integrations_input::Builder {
        crate::input::list_event_integrations_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListEventIntegrations {
    type Output = std::result::Result<
        crate::output::ListEventIntegrationsOutput,
        crate::error::ListEventIntegrationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_event_integrations_error(response)
        } else {
            crate::operation_deser::parse_list_event_integrations_response(response)
        }
    }
}

/// <p>Lists the tags for the specified resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// <p>Adds the specified tags to the specified resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>Removes the specified tags from the specified resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// <p>Updates the description of a DataIntegration.</p>
/// <note>
/// <p>You cannot create a DataIntegration association for a DataIntegration that has been previously associated.  
/// Use a different DataIntegration, or recreate the DataIntegration using the
/// <a href="https://docs.aws.amazon.com/appintegrations/latest/APIReference/API_CreateDataIntegration.html">CreateDataIntegration</a> API.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateDataIntegration {
    _private: (),
}
impl UpdateDataIntegration {
    /// Creates a new builder-style object to manufacture [`UpdateDataIntegrationInput`](crate::input::UpdateDataIntegrationInput)
    pub fn builder() -> crate::input::update_data_integration_input::Builder {
        crate::input::update_data_integration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateDataIntegration {
    type Output = std::result::Result<
        crate::output::UpdateDataIntegrationOutput,
        crate::error::UpdateDataIntegrationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_data_integration_error(response)
        } else {
            crate::operation_deser::parse_update_data_integration_response(response)
        }
    }
}

/// <p>Updates the description of an event integration.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateEventIntegration {
    _private: (),
}
impl UpdateEventIntegration {
    /// Creates a new builder-style object to manufacture [`UpdateEventIntegrationInput`](crate::input::UpdateEventIntegrationInput)
    pub fn builder() -> crate::input::update_event_integration_input::Builder {
        crate::input::update_event_integration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateEventIntegration {
    type Output = std::result::Result<
        crate::output::UpdateEventIntegrationOutput,
        crate::error::UpdateEventIntegrationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_event_integration_error(response)
        } else {
            crate::operation_deser::parse_update_event_integration_response(response)
        }
    }
}
