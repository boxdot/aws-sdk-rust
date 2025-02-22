// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_get_datalake_status_output_next_token(
    input: &crate::operation::get_datalake_status::GetDatalakeStatusOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_datalake_exceptions_output_next_token(
    input: &crate::operation::list_datalake_exceptions::ListDatalakeExceptionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_log_sources_output_next_token(
    input: &crate::operation::list_log_sources::ListLogSourcesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_subscribers_output_next_token(
    input: &crate::operation::list_subscribers::ListSubscribersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_datalake_status_output_account_sources_list(
    input: crate::operation::get_datalake_status::GetDatalakeStatusOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AccountSources>> {
    let input = match input.account_sources_list {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_datalake_exceptions_output_non_retryable_failures(
    input: crate::operation::list_datalake_exceptions::ListDatalakeExceptionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::FailuresResponse>> {
    let input = match input.non_retryable_failures {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_log_sources_output_region_source_types_accounts_list(
    input: crate::operation::list_log_sources::ListLogSourcesOutput,
) -> ::std::option::Option<
    ::std::vec::Vec<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<::std::string::String>,
            >,
        >,
    >,
> {
    let input = match input.region_source_types_accounts_list {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_subscribers_output_subscribers(
    input: crate::operation::list_subscribers::ListSubscribersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::SubscriberResource>> {
    let input = match input.subscribers {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}
