// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_location_fsx_ontap::_create_location_fsx_ontap_output::CreateLocationFsxOntapOutputBuilder;

pub use crate::operation::create_location_fsx_ontap::_create_location_fsx_ontap_input::CreateLocationFsxOntapInputBuilder;

/// Fluent builder constructing a request to `CreateLocationFsxOntap`.
///
/// <p>Creates an endpoint for an Amazon FSx for NetApp ONTAP file system that DataSync can access for a transfer. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-ontap-location.html">Creating a location for FSx for ONTAP</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLocationFsxOntapFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_location_fsx_ontap::builders::CreateLocationFsxOntapInputBuilder,
}
impl CreateLocationFsxOntapFluentBuilder {
    /// Creates a new `CreateLocationFsxOntap`.
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
            crate::operation::create_location_fsx_ontap::CreateLocationFsxOntap,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_location_fsx_ontap::CreateLocationFsxOntapError,
        >,
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
        crate::operation::create_location_fsx_ontap::CreateLocationFsxOntapOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_location_fsx_ontap::CreateLocationFsxOntapError,
        >,
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
        crate::operation::create_location_fsx_ontap::CreateLocationFsxOntapOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_location_fsx_ontap::CreateLocationFsxOntapError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_location_fsx_ontap::CreateLocationFsxOntap,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_location_fsx_ontap::CreateLocationFsxOntapError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies the data transfer protocol that DataSync uses to access your Amazon FSx file system.</p>
    pub fn protocol(mut self, input: crate::types::FsxProtocol) -> Self {
        self.inner = self.inner.protocol(input);
        self
    }
    /// <p>Specifies the data transfer protocol that DataSync uses to access your Amazon FSx file system.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<crate::types::FsxProtocol>) -> Self {
        self.inner = self.inner.set_protocol(input);
        self
    }
    /// Appends an item to `SecurityGroupArns`.
    ///
    /// To override the contents of this collection use [`set_security_group_arns`](Self::set_security_group_arns).
    ///
    /// <p>Specifies the Amazon EC2 security groups that provide access to your file system's preferred subnet.</p>
    /// <p>The security groups must allow outbound traffic on the following ports (depending on the protocol you use):</p>
    /// <ul>
    /// <li> <p> <b>Network File System (NFS)</b>: TCP ports 111, 635, and 2049</p> </li>
    /// <li> <p> <b>Server Message Block (SMB)</b>: TCP port 445</p> </li>
    /// </ul>
    /// <p>Your file system's security groups must also allow inbound traffic on the same ports.</p>
    pub fn security_group_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.security_group_arns(input.into());
        self
    }
    /// <p>Specifies the Amazon EC2 security groups that provide access to your file system's preferred subnet.</p>
    /// <p>The security groups must allow outbound traffic on the following ports (depending on the protocol you use):</p>
    /// <ul>
    /// <li> <p> <b>Network File System (NFS)</b>: TCP ports 111, 635, and 2049</p> </li>
    /// <li> <p> <b>Server Message Block (SMB)</b>: TCP port 445</p> </li>
    /// </ul>
    /// <p>Your file system's security groups must also allow inbound traffic on the same ports.</p>
    pub fn set_security_group_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_security_group_arns(input);
        self
    }
    /// <p>Specifies the ARN of the storage virtual machine (SVM) in your file system where you want to copy data to or from.</p>
    pub fn storage_virtual_machine_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.storage_virtual_machine_arn(input.into());
        self
    }
    /// <p>Specifies the ARN of the storage virtual machine (SVM) in your file system where you want to copy data to or from.</p>
    pub fn set_storage_virtual_machine_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_storage_virtual_machine_arn(input);
        self
    }
    /// <p>Specifies a path to the file share in the SVM where you'll copy your data.</p>
    /// <p>You can specify a junction path (also known as a mount point), qtree path (for NFS file shares), or share name (for SMB file shares). For example, your mount path might be <code>/vol1</code>, <code>/vol1/tree1</code>, or <code>/share1</code>.</p> <note>
    /// <p>Don't specify a junction path in the SVM's root volume. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-svms.html">Managing FSx for ONTAP storage virtual machines</a> in the <i>Amazon FSx for NetApp ONTAP User Guide</i>.</p>
    /// </note>
    pub fn subdirectory(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subdirectory(input.into());
        self
    }
    /// <p>Specifies a path to the file share in the SVM where you'll copy your data.</p>
    /// <p>You can specify a junction path (also known as a mount point), qtree path (for NFS file shares), or share name (for SMB file shares). For example, your mount path might be <code>/vol1</code>, <code>/vol1/tree1</code>, or <code>/share1</code>.</p> <note>
    /// <p>Don't specify a junction path in the SVM's root volume. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-svms.html">Managing FSx for ONTAP storage virtual machines</a> in the <i>Amazon FSx for NetApp ONTAP User Guide</i>.</p>
    /// </note>
    pub fn set_subdirectory(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subdirectory(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your location.</p>
    pub fn tags(mut self, input: crate::types::TagListEntry) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your location.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagListEntry>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
