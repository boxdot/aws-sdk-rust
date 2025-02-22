// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_faces::_search_faces_output::SearchFacesOutputBuilder;

pub use crate::operation::search_faces::_search_faces_input::SearchFacesInputBuilder;

/// Fluent builder constructing a request to `SearchFaces`.
///
/// <p>For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the <code>IndexFaces</code> operation. The operation compares the features of the input face with faces in the specified collection. </p> <note>
/// <p>You can also search faces without indexing faces by using the <code>SearchFacesByImage</code> operation.</p>
/// </note>
/// <p> The operation response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match that is found. Along with the metadata, the response also includes a <code>confidence</code> value for each face match, indicating the confidence that the specific face matches the input face. </p>
/// <p>For an example, see Searching for a face using its face ID in the Amazon Rekognition Developer Guide.</p>
/// <p>This operation requires permissions to perform the <code>rekognition:SearchFaces</code> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchFacesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_faces::builders::SearchFacesInputBuilder,
}
impl SearchFacesFluentBuilder {
    /// Creates a new `SearchFaces`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::search_faces::SearchFaces,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::search_faces::SearchFacesError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::search_faces::SearchFacesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::search_faces::SearchFacesError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::search_faces::SearchFacesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::search_faces::SearchFacesError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::search_faces::SearchFaces,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::search_faces::SearchFacesError>,
    > {
        self.customize_middleware().await
    }
    /// <p>ID of the collection the face belongs to.</p>
    pub fn collection_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.collection_id(input.into());
        self
    }
    /// <p>ID of the collection the face belongs to.</p>
    pub fn set_collection_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_collection_id(input);
        self
    }
    /// <p>ID of a face to find matches for in the collection.</p>
    pub fn face_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.face_id(input.into());
        self
    }
    /// <p>ID of a face to find matches for in the collection.</p>
    pub fn set_face_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_face_id(input);
        self
    }
    /// <p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>
    pub fn max_faces(mut self, input: i32) -> Self {
        self.inner = self.inner.max_faces(input);
        self
    }
    /// <p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>
    pub fn set_max_faces(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_faces(input);
        self
    }
    /// <p>Optional value specifying the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%. The default value is 80%. </p>
    pub fn face_match_threshold(mut self, input: f32) -> Self {
        self.inner = self.inner.face_match_threshold(input);
        self
    }
    /// <p>Optional value specifying the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%. The default value is 80%. </p>
    pub fn set_face_match_threshold(mut self, input: ::std::option::Option<f32>) -> Self {
        self.inner = self.inner.set_face_match_threshold(input);
        self
    }
}
