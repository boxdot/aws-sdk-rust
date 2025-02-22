// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_web_acl_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_web_acl::CreateWebAclInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.scope {
        object.key("Scope").string(var_2.as_str());
    }
    if let Some(var_3) = &input.default_action {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DefaultAction").start_object();
        crate::protocol_serde::shape_default_action::ser_default_action(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.description {
        object.key("Description").string(var_5.as_str());
    }
    if let Some(var_6) = &input.rules {
        let mut array_7 = object.key("Rules").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_rule::ser_rule(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.visibility_config {
        #[allow(unused_mut)]
        let mut object_11 = object.key("VisibilityConfig").start_object();
        crate::protocol_serde::shape_visibility_config::ser_visibility_config(
            &mut object_11,
            var_10,
        )?;
        object_11.finish();
    }
    if let Some(var_12) = &input.tags {
        let mut array_13 = object.key("Tags").start_array();
        for item_14 in var_12 {
            {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.custom_response_bodies {
        #[allow(unused_mut)]
        let mut object_17 = object.key("CustomResponseBodies").start_object();
        for (key_18, value_19) in var_16 {
            {
                #[allow(unused_mut)]
                let mut object_20 = object_17.key(key_18.as_str()).start_object();
                crate::protocol_serde::shape_custom_response_body::ser_custom_response_body(
                    &mut object_20,
                    value_19,
                )?;
                object_20.finish();
            }
        }
        object_17.finish();
    }
    if let Some(var_21) = &input.captcha_config {
        #[allow(unused_mut)]
        let mut object_22 = object.key("CaptchaConfig").start_object();
        crate::protocol_serde::shape_captcha_config::ser_captcha_config(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.challenge_config {
        #[allow(unused_mut)]
        let mut object_24 = object.key("ChallengeConfig").start_object();
        crate::protocol_serde::shape_challenge_config::ser_challenge_config(
            &mut object_24,
            var_23,
        )?;
        object_24.finish();
    }
    if let Some(var_25) = &input.token_domains {
        let mut array_26 = object.key("TokenDomains").start_array();
        for item_27 in var_25 {
            {
                array_26.value().string(item_27.as_str());
            }
        }
        array_26.finish();
    }
    if let Some(var_28) = &input.association_config {
        #[allow(unused_mut)]
        let mut object_29 = object.key("AssociationConfig").start_object();
        crate::protocol_serde::shape_association_config::ser_association_config(
            &mut object_29,
            var_28,
        )?;
        object_29.finish();
    }
    Ok(())
}
