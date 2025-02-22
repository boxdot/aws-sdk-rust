// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_project_source(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ProjectSource,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.r#type {
        object.key("type").string(var_1.as_str());
    }
    if let Some(var_2) = &input.location {
        object.key("location").string(var_2.as_str());
    }
    if let Some(var_3) = &input.git_clone_depth {
        object.key("gitCloneDepth").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.git_submodules_config {
        #[allow(unused_mut)]
        let mut object_5 = object.key("gitSubmodulesConfig").start_object();
        crate::protocol_serde::shape_git_submodules_config::ser_git_submodules_config(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    if let Some(var_6) = &input.buildspec {
        object.key("buildspec").string(var_6.as_str());
    }
    if let Some(var_7) = &input.auth {
        #[allow(unused_mut)]
        let mut object_8 = object.key("auth").start_object();
        crate::protocol_serde::shape_source_auth::ser_source_auth(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.report_build_status {
        object.key("reportBuildStatus").boolean(*var_9);
    }
    if let Some(var_10) = &input.build_status_config {
        #[allow(unused_mut)]
        let mut object_11 = object.key("buildStatusConfig").start_object();
        crate::protocol_serde::shape_build_status_config::ser_build_status_config(
            &mut object_11,
            var_10,
        )?;
        object_11.finish();
    }
    if let Some(var_12) = &input.insecure_ssl {
        object.key("insecureSsl").boolean(*var_12);
    }
    if let Some(var_13) = &input.source_identifier {
        object.key("sourceIdentifier").string(var_13.as_str());
    }
    Ok(())
}

pub(crate) fn de_project_source<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::ProjectSource>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ProjectSourceBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key
                        .to_unescaped()?
                        .as_ref()
                    {
                        "type" => {
                            builder = builder.set_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped()
                                        .map(|u| crate::types::SourceType::from(u.as_ref()))
                                })
                                .transpose()?,
                            );
                        }
                        "location" => {
                            builder = builder.set_location(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "gitCloneDepth" => {
                            builder = builder.set_git_clone_depth(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                    tokens.next(),
                                )?
                                .map(i32::try_from)
                                .transpose()?,
                            );
                        }
                        "gitSubmodulesConfig" => {
                            builder = builder.set_git_submodules_config(
                                    crate::protocol_serde::shape_git_submodules_config::de_git_submodules_config(tokens)?
                                );
                        }
                        "buildspec" => {
                            builder = builder.set_buildspec(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "auth" => {
                            builder = builder.set_auth(
                                crate::protocol_serde::shape_source_auth::de_source_auth(tokens)?,
                            );
                        }
                        "reportBuildStatus" => {
                            builder = builder.set_report_build_status(
                                ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                    tokens.next(),
                                )?,
                            );
                        }
                        "buildStatusConfig" => {
                            builder = builder.set_build_status_config(
                                    crate::protocol_serde::shape_build_status_config::de_build_status_config(tokens)?
                                );
                        }
                        "insecureSsl" => {
                            builder = builder.set_insecure_ssl(
                                ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                    tokens.next(),
                                )?,
                            );
                        }
                        "sourceIdentifier" => {
                            builder = builder.set_source_identifier(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(
                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                format!("expected object key or end object, found: {:?}", other),
                            ),
                        )
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(
            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}
