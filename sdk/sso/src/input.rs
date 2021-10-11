// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`GetRoleCredentialsInput`](crate::input::GetRoleCredentialsInput)
pub mod get_role_credentials_input {
    /// A builder for [`GetRoleCredentialsInput`](crate::input::GetRoleCredentialsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) role_name: std::option::Option<std::string::String>,
        pub(crate) account_id: std::option::Option<std::string::String>,
        pub(crate) access_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The friendly name of the role that is assigned to the user.</p>
        pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.role_name = Some(input.into());
            self
        }
        pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.role_name = input;
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.account_id = Some(input.into());
            self
        }
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.account_id = input;
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see
        /// <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_token = Some(input.into());
            self
        }
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.access_token = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRoleCredentialsInput`](crate::input::GetRoleCredentialsInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetRoleCredentialsInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetRoleCredentialsInput {
                role_name: self.role_name,
                account_id: self.account_id,
                access_token: self.access_token,
            })
        }
    }
}
#[doc(hidden)]
pub type GetRoleCredentialsInputOperationOutputAlias = crate::operation::GetRoleCredentials;
#[doc(hidden)]
pub type GetRoleCredentialsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetRoleCredentialsInput {
    /// Consumes the builder and constructs an Operation<[`GetRoleCredentials`](crate::operation::GetRoleCredentials)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::GetRoleCredentials,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::GetRoleCredentialsInput,
            output: &mut String,
        ) -> Result<(), smithy_http::operation::BuildError> {
            write!(output, "/federation/credentials").expect("formatting should succeed");
            Ok(())
        }
        fn add_headers(
            _input: &crate::input::GetRoleCredentialsInput,
            mut builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            if let Some(inner_1) = &_input.access_token {
                let formatted_2 = AsRef::<str>::as_ref(inner_1);
                if !formatted_2.is_empty() {
                    use std::convert::TryFrom;
                    let header_value = formatted_2;
                    let header_value = http::header::HeaderValue::try_from(&*header_value)
                        .map_err(|err| smithy_http::operation::BuildError::InvalidField {
                            field: "access_token",
                            details: format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***", err
                            ),
                        })?;
                    builder = builder.header("x-amz-sso_bearer_token", header_value);
                }
            }
            Ok(builder)
        }
        fn uri_query(_input: &crate::input::GetRoleCredentialsInput, mut output: &mut String) {
            let mut query = smithy_http::query::Writer::new(&mut output);
            if let Some(inner_3) = &_input.role_name {
                query.push_kv("role_name", &smithy_http::query::fmt_string(&inner_3));
            }
            if let Some(inner_4) = &_input.account_id {
                query.push_kv("account_id", &smithy_http::query::fmt_string(&inner_4));
            }
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::GetRoleCredentialsInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            uri_query(input, &mut uri);
            let builder = add_headers(input, builder)?;
            Ok(builder.method("GET").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::GetRoleCredentialsInput,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = smithy_http::body::SdkBody::from("");
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = smithy_http::operation::Request::from_parts(
            request.map(smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        signing_config.signing_requirements = aws_sig_auth::signer::SigningRequirements::Disabled;
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = smithy_http::operation::Operation::new(
            request,
            crate::operation::GetRoleCredentials::new(),
        )
        .with_metadata(smithy_http::operation::Metadata::new(
            "GetRoleCredentials",
            "sso",
        ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`GetRoleCredentialsInput`](crate::input::GetRoleCredentialsInput)
    pub fn builder() -> crate::input::get_role_credentials_input::Builder {
        crate::input::get_role_credentials_input::Builder::default()
    }
}

/// See [`ListAccountRolesInput`](crate::input::ListAccountRolesInput)
pub mod list_account_roles_input {
    /// A builder for [`ListAccountRolesInput`](crate::input::ListAccountRolesInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) access_token: std::option::Option<std::string::String>,
        pub(crate) account_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The page token from the previous response output when you request subsequent pages.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>The number of items that clients can request per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see
        /// <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_token = Some(input.into());
            self
        }
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.access_token = input;
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.account_id = Some(input.into());
            self
        }
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.account_id = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAccountRolesInput`](crate::input::ListAccountRolesInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::ListAccountRolesInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::ListAccountRolesInput {
                next_token: self.next_token,
                max_results: self.max_results,
                access_token: self.access_token,
                account_id: self.account_id,
            })
        }
    }
}
#[doc(hidden)]
pub type ListAccountRolesInputOperationOutputAlias = crate::operation::ListAccountRoles;
#[doc(hidden)]
pub type ListAccountRolesInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl ListAccountRolesInput {
    /// Consumes the builder and constructs an Operation<[`ListAccountRoles`](crate::operation::ListAccountRoles)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::ListAccountRoles,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::ListAccountRolesInput,
            output: &mut String,
        ) -> Result<(), smithy_http::operation::BuildError> {
            write!(output, "/assignment/roles").expect("formatting should succeed");
            Ok(())
        }
        fn add_headers(
            _input: &crate::input::ListAccountRolesInput,
            mut builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            if let Some(inner_5) = &_input.access_token {
                let formatted_6 = AsRef::<str>::as_ref(inner_5);
                if !formatted_6.is_empty() {
                    use std::convert::TryFrom;
                    let header_value = formatted_6;
                    let header_value = http::header::HeaderValue::try_from(&*header_value)
                        .map_err(|err| smithy_http::operation::BuildError::InvalidField {
                            field: "access_token",
                            details: format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***", err
                            ),
                        })?;
                    builder = builder.header("x-amz-sso_bearer_token", header_value);
                }
            }
            Ok(builder)
        }
        fn uri_query(_input: &crate::input::ListAccountRolesInput, mut output: &mut String) {
            let mut query = smithy_http::query::Writer::new(&mut output);
            if let Some(inner_7) = &_input.next_token {
                query.push_kv("next_token", &smithy_http::query::fmt_string(&inner_7));
            }
            if let Some(inner_8) = &_input.max_results {
                query.push_kv(
                    "max_result",
                    &smithy_types::primitive::Encoder::from(*inner_8).encode(),
                );
            }
            if let Some(inner_9) = &_input.account_id {
                query.push_kv("account_id", &smithy_http::query::fmt_string(&inner_9));
            }
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::ListAccountRolesInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            uri_query(input, &mut uri);
            let builder = add_headers(input, builder)?;
            Ok(builder.method("GET").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::ListAccountRolesInput,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = smithy_http::body::SdkBody::from("");
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = smithy_http::operation::Request::from_parts(
            request.map(smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        signing_config.signing_requirements = aws_sig_auth::signer::SigningRequirements::Disabled;
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = smithy_http::operation::Operation::new(
            request,
            crate::operation::ListAccountRoles::new(),
        )
        .with_metadata(smithy_http::operation::Metadata::new(
            "ListAccountRoles",
            "sso",
        ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`ListAccountRolesInput`](crate::input::ListAccountRolesInput)
    pub fn builder() -> crate::input::list_account_roles_input::Builder {
        crate::input::list_account_roles_input::Builder::default()
    }
}

/// See [`ListAccountsInput`](crate::input::ListAccountsInput)
pub mod list_accounts_input {
    /// A builder for [`ListAccountsInput`](crate::input::ListAccountsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) access_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>This is the number of items clients can request per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see
        /// <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_token = Some(input.into());
            self
        }
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.access_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAccountsInput`](crate::input::ListAccountsInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::ListAccountsInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::ListAccountsInput {
                next_token: self.next_token,
                max_results: self.max_results,
                access_token: self.access_token,
            })
        }
    }
}
#[doc(hidden)]
pub type ListAccountsInputOperationOutputAlias = crate::operation::ListAccounts;
#[doc(hidden)]
pub type ListAccountsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl ListAccountsInput {
    /// Consumes the builder and constructs an Operation<[`ListAccounts`](crate::operation::ListAccounts)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::ListAccounts,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::ListAccountsInput,
            output: &mut String,
        ) -> Result<(), smithy_http::operation::BuildError> {
            write!(output, "/assignment/accounts").expect("formatting should succeed");
            Ok(())
        }
        fn add_headers(
            _input: &crate::input::ListAccountsInput,
            mut builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            if let Some(inner_10) = &_input.access_token {
                let formatted_11 = AsRef::<str>::as_ref(inner_10);
                if !formatted_11.is_empty() {
                    use std::convert::TryFrom;
                    let header_value = formatted_11;
                    let header_value = http::header::HeaderValue::try_from(&*header_value)
                        .map_err(|err| smithy_http::operation::BuildError::InvalidField {
                            field: "access_token",
                            details: format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***", err
                            ),
                        })?;
                    builder = builder.header("x-amz-sso_bearer_token", header_value);
                }
            }
            Ok(builder)
        }
        fn uri_query(_input: &crate::input::ListAccountsInput, mut output: &mut String) {
            let mut query = smithy_http::query::Writer::new(&mut output);
            if let Some(inner_12) = &_input.next_token {
                query.push_kv("next_token", &smithy_http::query::fmt_string(&inner_12));
            }
            if let Some(inner_13) = &_input.max_results {
                query.push_kv(
                    "max_result",
                    &smithy_types::primitive::Encoder::from(*inner_13).encode(),
                );
            }
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::ListAccountsInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            uri_query(input, &mut uri);
            let builder = add_headers(input, builder)?;
            Ok(builder.method("GET").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::ListAccountsInput,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = smithy_http::body::SdkBody::from("");
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = smithy_http::operation::Request::from_parts(
            request.map(smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        signing_config.signing_requirements = aws_sig_auth::signer::SigningRequirements::Disabled;
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            smithy_http::operation::Operation::new(request, crate::operation::ListAccounts::new())
                .with_metadata(smithy_http::operation::Metadata::new("ListAccounts", "sso"));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`ListAccountsInput`](crate::input::ListAccountsInput)
    pub fn builder() -> crate::input::list_accounts_input::Builder {
        crate::input::list_accounts_input::Builder::default()
    }
}

/// See [`LogoutInput`](crate::input::LogoutInput)
pub mod logout_input {
    /// A builder for [`LogoutInput`](crate::input::LogoutInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) access_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see
        /// <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_token = Some(input.into());
            self
        }
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.access_token = input;
            self
        }
        /// Consumes the builder and constructs a [`LogoutInput`](crate::input::LogoutInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::LogoutInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::LogoutInput {
                access_token: self.access_token,
            })
        }
    }
}
#[doc(hidden)]
pub type LogoutInputOperationOutputAlias = crate::operation::Logout;
#[doc(hidden)]
pub type LogoutInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl LogoutInput {
    /// Consumes the builder and constructs an Operation<[`Logout`](crate::operation::Logout)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<crate::operation::Logout, aws_http::AwsErrorRetryPolicy>,
        smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::LogoutInput,
            output: &mut String,
        ) -> Result<(), smithy_http::operation::BuildError> {
            write!(output, "/logout").expect("formatting should succeed");
            Ok(())
        }
        fn add_headers(
            _input: &crate::input::LogoutInput,
            mut builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            if let Some(inner_14) = &_input.access_token {
                let formatted_15 = AsRef::<str>::as_ref(inner_14);
                if !formatted_15.is_empty() {
                    use std::convert::TryFrom;
                    let header_value = formatted_15;
                    let header_value = http::header::HeaderValue::try_from(&*header_value)
                        .map_err(|err| smithy_http::operation::BuildError::InvalidField {
                            field: "access_token",
                            details: format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***", err
                            ),
                        })?;
                    builder = builder.header("x-amz-sso_bearer_token", header_value);
                }
            }
            Ok(builder)
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::LogoutInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            let builder = add_headers(input, builder)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::LogoutInput,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = smithy_http::body::SdkBody::from("");
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = smithy_http::operation::Request::from_parts(
            request.map(smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        signing_config.signing_requirements = aws_sig_auth::signer::SigningRequirements::Disabled;
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = smithy_http::operation::Operation::new(request, crate::operation::Logout::new())
            .with_metadata(smithy_http::operation::Metadata::new("Logout", "sso"));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`LogoutInput`](crate::input::LogoutInput)
    pub fn builder() -> crate::input::logout_input::Builder {
        crate::input::logout_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct LogoutInput {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see
    /// <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    pub access_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for LogoutInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("LogoutInput");
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListAccountsInput {
    /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>This is the number of items clients can request per page.</p>
    pub max_results: std::option::Option<i32>,
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see
    /// <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    pub access_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListAccountsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListAccountsInput");
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListAccountRolesInput {
    /// <p>The page token from the previous response output when you request subsequent pages.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The number of items that clients can request per page.</p>
    pub max_results: std::option::Option<i32>,
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see
    /// <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    pub access_token: std::option::Option<std::string::String>,
    /// <p>The identifier for the AWS account that is assigned to the user.</p>
    pub account_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListAccountRolesInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListAccountRolesInput");
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.field("account_id", &self.account_id);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRoleCredentialsInput {
    /// <p>The friendly name of the role that is assigned to the user.</p>
    pub role_name: std::option::Option<std::string::String>,
    /// <p>The identifier for the AWS account that is assigned to the user.</p>
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see
    /// <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    pub access_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for GetRoleCredentialsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRoleCredentialsInput");
        formatter.field("role_name", &self.role_name);
        formatter.field("account_id", &self.account_id);
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
