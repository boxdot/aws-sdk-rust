// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an SSH public key.</p>
/// <p>This data type is used as a response element in the <code>GetSSHPublicKey</code> and <code>UploadSSHPublicKey</code> operations. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SshPublicKey {
    /// <p>The name of the IAM user associated with the SSH public key.</p>
    #[doc(hidden)]
    pub user_name: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the SSH public key.</p>
    #[doc(hidden)]
    pub ssh_public_key_id: ::std::option::Option<::std::string::String>,
    /// <p>The MD5 message digest of the SSH public key.</p>
    #[doc(hidden)]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    /// <p>The SSH public key.</p>
    #[doc(hidden)]
    pub ssh_public_key_body: ::std::option::Option<::std::string::String>,
    /// <p>The status of the SSH public key. <code>Active</code> means that the key can be used for authentication with an CodeCommit repository. <code>Inactive</code> means that the key cannot be used.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::StatusType>,
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the SSH public key was uploaded.</p>
    #[doc(hidden)]
    pub upload_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl SshPublicKey {
    /// <p>The name of the IAM user associated with the SSH public key.</p>
    pub fn user_name(&self) -> ::std::option::Option<&str> {
        self.user_name.as_deref()
    }
    /// <p>The unique identifier for the SSH public key.</p>
    pub fn ssh_public_key_id(&self) -> ::std::option::Option<&str> {
        self.ssh_public_key_id.as_deref()
    }
    /// <p>The MD5 message digest of the SSH public key.</p>
    pub fn fingerprint(&self) -> ::std::option::Option<&str> {
        self.fingerprint.as_deref()
    }
    /// <p>The SSH public key.</p>
    pub fn ssh_public_key_body(&self) -> ::std::option::Option<&str> {
        self.ssh_public_key_body.as_deref()
    }
    /// <p>The status of the SSH public key. <code>Active</code> means that the key can be used for authentication with an CodeCommit repository. <code>Inactive</code> means that the key cannot be used.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::StatusType> {
        self.status.as_ref()
    }
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the SSH public key was uploaded.</p>
    pub fn upload_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.upload_date.as_ref()
    }
}
impl SshPublicKey {
    /// Creates a new builder-style object to manufacture [`SshPublicKey`](crate::types::SshPublicKey).
    pub fn builder() -> crate::types::builders::SshPublicKeyBuilder {
        crate::types::builders::SshPublicKeyBuilder::default()
    }
}

/// A builder for [`SshPublicKey`](crate::types::SshPublicKey).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SshPublicKeyBuilder {
    pub(crate) user_name: ::std::option::Option<::std::string::String>,
    pub(crate) ssh_public_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) fingerprint: ::std::option::Option<::std::string::String>,
    pub(crate) ssh_public_key_body: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::StatusType>,
    pub(crate) upload_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl SshPublicKeyBuilder {
    /// <p>The name of the IAM user associated with the SSH public key.</p>
    pub fn user_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the IAM user associated with the SSH public key.</p>
    pub fn set_user_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_name = input;
        self
    }
    /// <p>The unique identifier for the SSH public key.</p>
    pub fn ssh_public_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.ssh_public_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the SSH public key.</p>
    pub fn set_ssh_public_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.ssh_public_key_id = input;
        self
    }
    /// <p>The MD5 message digest of the SSH public key.</p>
    pub fn fingerprint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fingerprint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The MD5 message digest of the SSH public key.</p>
    pub fn set_fingerprint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fingerprint = input;
        self
    }
    /// <p>The SSH public key.</p>
    pub fn ssh_public_key_body(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.ssh_public_key_body = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The SSH public key.</p>
    pub fn set_ssh_public_key_body(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.ssh_public_key_body = input;
        self
    }
    /// <p>The status of the SSH public key. <code>Active</code> means that the key can be used for authentication with an CodeCommit repository. <code>Inactive</code> means that the key cannot be used.</p>
    pub fn status(mut self, input: crate::types::StatusType) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the SSH public key. <code>Active</code> means that the key can be used for authentication with an CodeCommit repository. <code>Inactive</code> means that the key cannot be used.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::StatusType>) -> Self {
        self.status = input;
        self
    }
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the SSH public key was uploaded.</p>
    pub fn upload_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.upload_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the SSH public key was uploaded.</p>
    pub fn set_upload_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.upload_date = input;
        self
    }
    /// Consumes the builder and constructs a [`SshPublicKey`](crate::types::SshPublicKey).
    pub fn build(self) -> crate::types::SshPublicKey {
        crate::types::SshPublicKey {
            user_name: self.user_name,
            ssh_public_key_id: self.ssh_public_key_id,
            fingerprint: self.fingerprint,
            ssh_public_key_body: self.ssh_public_key_body,
            status: self.status,
            upload_date: self.upload_date,
        }
    }
}
