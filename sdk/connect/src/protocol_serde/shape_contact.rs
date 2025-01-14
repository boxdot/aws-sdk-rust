// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_contact<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::Contact>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
            let mut builder = crate::types::builders::ContactBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Arn" => {
                                builder = builder.set_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Id" => {
                                builder = builder.set_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "InitialContactId" => {
                                builder = builder.set_initial_contact_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "PreviousContactId" => {
                                builder = builder.set_previous_contact_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "InitiationMethod" => {
                                builder = builder.set_initiation_method(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::ContactInitiationMethod::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "Name" => {
                                builder = builder.set_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Channel" => {
                                builder = builder.set_channel(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::Channel::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "QueueInfo" => {
                                builder = builder.set_queue_info(
                                    crate::protocol_serde::shape_queue_info::de_queue_info(tokens)?,
                                );
                            }
                            "AgentInfo" => {
                                builder = builder.set_agent_info(
                                    crate::protocol_serde::shape_agent_info::de_agent_info(tokens)?,
                                );
                            }
                            "InitiationTimestamp" => {
                                builder = builder.set_initiation_timestamp(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "DisconnectTimestamp" => {
                                builder = builder.set_disconnect_timestamp(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "LastUpdateTimestamp" => {
                                builder = builder.set_last_update_timestamp(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "ScheduledTimestamp" => {
                                builder = builder.set_scheduled_timestamp(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "RelatedContactId" => {
                                builder = builder.set_related_contact_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "WisdomInfo" => {
                                builder = builder.set_wisdom_info(
                                    crate::protocol_serde::shape_wisdom_info::de_wisdom_info(
                                        tokens,
                                    )?,
                                );
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
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
