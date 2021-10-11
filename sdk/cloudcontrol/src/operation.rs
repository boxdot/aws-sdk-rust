// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Cancels the specified resource operation request. For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html#resource-operations-manage-requests-cancel">Canceling resource operation requests</a> in the
/// <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
/// <p>Only resource operations requests with a status of <code>PENDING</code> or
/// <code>IN_PROGRESS</code> can be cancelled.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelResourceRequest {
    _private: (),
}
impl CancelResourceRequest {
    /// Creates a new builder-style object to manufacture [`CancelResourceRequestInput`](crate::input::CancelResourceRequestInput)
    pub fn builder() -> crate::input::cancel_resource_request_input::Builder {
        crate::input::cancel_resource_request_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CancelResourceRequest {
    type Output = std::result::Result<
        crate::output::CancelResourceRequestOutput,
        crate::error::CancelResourceRequestError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_resource_request_error(response)
        } else {
            crate::operation_deser::parse_cancel_resource_request_response(response)
        }
    }
}

/// <p>Creates the specified resource. For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-create.html">Creating a
/// resource</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
/// <p>After you have initiated a resource creation request, you can monitor the progress of your
/// request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the
/// <code>ProgressEvent</code> type returned by <code>CreateResource</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateResource {
    _private: (),
}
impl CreateResource {
    /// Creates a new builder-style object to manufacture [`CreateResourceInput`](crate::input::CreateResourceInput)
    pub fn builder() -> crate::input::create_resource_input::Builder {
        crate::input::create_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateResource {
    type Output =
        std::result::Result<crate::output::CreateResourceOutput, crate::error::CreateResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_resource_error(response)
        } else {
            crate::operation_deser::parse_create_resource_response(response)
        }
    }
}

/// <p>Deletes the specified resource. For details, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-delete.html">Deleting a
/// resource</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
/// <p>After you have initiated a resource deletion request, you can monitor the progress of your
/// request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the
/// <code>ProgressEvent</code> returned by <code>DeleteResource</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteResource {
    _private: (),
}
impl DeleteResource {
    /// Creates a new builder-style object to manufacture [`DeleteResourceInput`](crate::input::DeleteResourceInput)
    pub fn builder() -> crate::input::delete_resource_input::Builder {
        crate::input::delete_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteResource {
    type Output =
        std::result::Result<crate::output::DeleteResourceOutput, crate::error::DeleteResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_resource_error(response)
        } else {
            crate::operation_deser::parse_delete_resource_response(response)
        }
    }
}

/// <p>Returns information about the current state of the specified resource. For details, see
/// <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-read.html">Reading a resource's current state</a>.</p>
/// <p>You can use this action to return information about an existing resource in your account
/// and Amazon Web Services Region, whether or not those resources were provisioned using Cloud Control API.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResource {
    _private: (),
}
impl GetResource {
    /// Creates a new builder-style object to manufacture [`GetResourceInput`](crate::input::GetResourceInput)
    pub fn builder() -> crate::input::get_resource_input::Builder {
        crate::input::get_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetResource {
    type Output =
        std::result::Result<crate::output::GetResourceOutput, crate::error::GetResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_error(response)
        } else {
            crate::operation_deser::parse_get_resource_response(response)
        }
    }
}

/// <p>Returns the current status of a resource operation request. For more information, see
/// <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html#resource-operations-manage-requests-track">Tracking the progress of resource operation requests</a> in the
/// <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceRequestStatus {
    _private: (),
}
impl GetResourceRequestStatus {
    /// Creates a new builder-style object to manufacture [`GetResourceRequestStatusInput`](crate::input::GetResourceRequestStatusInput)
    pub fn builder() -> crate::input::get_resource_request_status_input::Builder {
        crate::input::get_resource_request_status_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetResourceRequestStatus {
    type Output = std::result::Result<
        crate::output::GetResourceRequestStatusOutput,
        crate::error::GetResourceRequestStatusError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_request_status_error(response)
        } else {
            crate::operation_deser::parse_get_resource_request_status_response(response)
        }
    }
}

/// <p>Returns existing resource operation requests. This includes requests of all status types.
/// For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html#resource-operations-manage-requests-list">Listing active resource operation requests</a> in the
/// <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
/// <note>
/// <p>Resource operation requests expire after seven days.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResourceRequests {
    _private: (),
}
impl ListResourceRequests {
    /// Creates a new builder-style object to manufacture [`ListResourceRequestsInput`](crate::input::ListResourceRequestsInput)
    pub fn builder() -> crate::input::list_resource_requests_input::Builder {
        crate::input::list_resource_requests_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListResourceRequests {
    type Output = std::result::Result<
        crate::output::ListResourceRequestsOutput,
        crate::error::ListResourceRequestsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resource_requests_error(response)
        } else {
            crate::operation_deser::parse_list_resource_requests_response(response)
        }
    }
}

/// <p>Returns information about the specified resources. For more information, see <a href="cloudcontrolapi/latest/userguide/resource-operations-list.html">Discovering
/// resources</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
/// <p>You can use this action to return information about existing resources in your account and
/// Amazon Web Services Region, whether or not those resources were provisioned using Cloud Control API.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResources {
    _private: (),
}
impl ListResources {
    /// Creates a new builder-style object to manufacture [`ListResourcesInput`](crate::input::ListResourcesInput)
    pub fn builder() -> crate::input::list_resources_input::Builder {
        crate::input::list_resources_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListResources {
    type Output =
        std::result::Result<crate::output::ListResourcesOutput, crate::error::ListResourcesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resources_error(response)
        } else {
            crate::operation_deser::parse_list_resources_response(response)
        }
    }
}

/// <p>Updates the specified property values in the resource.</p>
/// <p>You specify your resource property updates as a list of patch operations contained in a
/// JSON patch document that adheres to the <a href="https://datatracker.ietf.org/doc/html/rfc6902">
/// <i>RFC 6902 - JavaScript Object
/// Notation (JSON) Patch</i>
/// </a> standard.</p>
/// <p>For details on how Cloud Control API performs resource update operations, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-update.html">Updating a resource</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
/// <p>After you have initiated a resource update request, you can monitor the progress of your
/// request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the
/// <code>ProgressEvent</code> returned by <code>UpdateResource</code>.</p>
/// <p>For more information about the properties of a specific resource, refer to the related
/// topic for the resource in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Resource and property types reference</a> in the <i>Amazon Web Services
/// CloudFormation Users Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateResource {
    _private: (),
}
impl UpdateResource {
    /// Creates a new builder-style object to manufacture [`UpdateResourceInput`](crate::input::UpdateResourceInput)
    pub fn builder() -> crate::input::update_resource_input::Builder {
        crate::input::update_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateResource {
    type Output =
        std::result::Result<crate::output::UpdateResourceOutput, crate::error::UpdateResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_resource_error(response)
        } else {
            crate::operation_deser::parse_update_resource_response(response)
        }
    }
}
