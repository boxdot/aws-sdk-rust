// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_campaign_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_campaign::CreateCampaignInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.signal_catalog_arn {
        object.key("signalCatalogArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.target_arn {
        object.key("targetArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.start_time {
        object
            .key("startTime")
            .date_time(var_5, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_6) = &input.expiry_time {
        object
            .key("expiryTime")
            .date_time(var_6, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_7) = &input.post_trigger_collection_duration {
        object.key("postTriggerCollectionDuration").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.diagnostics_mode {
        object.key("diagnosticsMode").string(var_8.as_str());
    }
    if let Some(var_9) = &input.spooling_mode {
        object.key("spoolingMode").string(var_9.as_str());
    }
    if let Some(var_10) = &input.compression {
        object.key("compression").string(var_10.as_str());
    }
    if let Some(var_11) = &input.priority {
        object.key("priority").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.signals_to_collect {
        let mut array_13 = object.key("signalsToCollect").start_array();
        for item_14 in var_12 {
            {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_signal_information::ser_signal_information(
                    &mut object_15,
                    item_14,
                )?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.collection_scheme {
        #[allow(unused_mut)]
        let mut object_17 = object.key("collectionScheme").start_object();
        crate::protocol_serde::shape_collection_scheme::ser_collection_scheme(
            &mut object_17,
            var_16,
        )?;
        object_17.finish();
    }
    if let Some(var_18) = &input.data_extra_dimensions {
        let mut array_19 = object.key("dataExtraDimensions").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("tags").start_array();
        for item_23 in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    Ok(())
}
