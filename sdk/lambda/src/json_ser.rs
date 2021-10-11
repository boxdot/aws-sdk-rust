// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_layer_version_permission_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddLayerVersionPermissionInput,
) {
    if let Some(var_1) = &input.action {
        object.key("Action").string(var_1);
    }
    if let Some(var_2) = &input.organization_id {
        object.key("OrganizationId").string(var_2);
    }
    if let Some(var_3) = &input.principal {
        object.key("Principal").string(var_3);
    }
    if let Some(var_4) = &input.statement_id {
        object.key("StatementId").string(var_4);
    }
}

pub fn serialize_structure_crate_input_add_permission_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddPermissionInput,
) {
    if let Some(var_5) = &input.action {
        object.key("Action").string(var_5);
    }
    if let Some(var_6) = &input.event_source_token {
        object.key("EventSourceToken").string(var_6);
    }
    if let Some(var_7) = &input.principal {
        object.key("Principal").string(var_7);
    }
    if let Some(var_8) = &input.revision_id {
        object.key("RevisionId").string(var_8);
    }
    if let Some(var_9) = &input.source_account {
        object.key("SourceAccount").string(var_9);
    }
    if let Some(var_10) = &input.source_arn {
        object.key("SourceArn").string(var_10);
    }
    if let Some(var_11) = &input.statement_id {
        object.key("StatementId").string(var_11);
    }
}

pub fn serialize_structure_crate_input_create_alias_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAliasInput,
) {
    if let Some(var_12) = &input.description {
        object.key("Description").string(var_12);
    }
    if let Some(var_13) = &input.function_version {
        object.key("FunctionVersion").string(var_13);
    }
    if let Some(var_14) = &input.name {
        object.key("Name").string(var_14);
    }
    if let Some(var_15) = &input.routing_config {
        let mut object_16 = object.key("RoutingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_alias_routing_configuration(
            &mut object_16,
            var_15,
        );
        object_16.finish();
    }
}

pub fn serialize_structure_crate_input_create_code_signing_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCodeSigningConfigInput,
) {
    if let Some(var_17) = &input.allowed_publishers {
        let mut object_18 = object.key("AllowedPublishers").start_object();
        crate::json_ser::serialize_structure_crate_model_allowed_publishers(&mut object_18, var_17);
        object_18.finish();
    }
    if let Some(var_19) = &input.code_signing_policies {
        let mut object_20 = object.key("CodeSigningPolicies").start_object();
        crate::json_ser::serialize_structure_crate_model_code_signing_policies(
            &mut object_20,
            var_19,
        );
        object_20.finish();
    }
    if let Some(var_21) = &input.description {
        object.key("Description").string(var_21);
    }
}

pub fn serialize_structure_crate_input_create_event_source_mapping_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEventSourceMappingInput,
) {
    if let Some(var_22) = &input.batch_size {
        object.key("BatchSize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    if let Some(var_23) = &input.bisect_batch_on_function_error {
        object.key("BisectBatchOnFunctionError").boolean(*var_23);
    }
    if let Some(var_24) = &input.destination_config {
        let mut object_25 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(&mut object_25, var_24);
        object_25.finish();
    }
    if let Some(var_26) = &input.enabled {
        object.key("Enabled").boolean(*var_26);
    }
    if let Some(var_27) = &input.event_source_arn {
        object.key("EventSourceArn").string(var_27);
    }
    if let Some(var_28) = &input.function_name {
        object.key("FunctionName").string(var_28);
    }
    if let Some(var_29) = &input.function_response_types {
        let mut array_30 = object.key("FunctionResponseTypes").start_array();
        for item_31 in var_29 {
            {
                array_30.value().string(item_31.as_str());
            }
        }
        array_30.finish();
    }
    if let Some(var_32) = &input.maximum_batching_window_in_seconds {
        object.key("MaximumBatchingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_32).into()),
        );
    }
    if let Some(var_33) = &input.maximum_record_age_in_seconds {
        object.key("MaximumRecordAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_33).into()),
        );
    }
    if let Some(var_34) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_34).into()),
        );
    }
    if let Some(var_35) = &input.parallelization_factor {
        object.key("ParallelizationFactor").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_35).into()),
        );
    }
    if let Some(var_36) = &input.queues {
        let mut array_37 = object.key("Queues").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38);
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.self_managed_event_source {
        let mut object_40 = object.key("SelfManagedEventSource").start_object();
        crate::json_ser::serialize_structure_crate_model_self_managed_event_source(
            &mut object_40,
            var_39,
        );
        object_40.finish();
    }
    if let Some(var_41) = &input.source_access_configurations {
        let mut array_42 = object.key("SourceAccessConfigurations").start_array();
        for item_43 in var_41 {
            {
                let mut object_44 = array_42.value().start_object();
                crate::json_ser::serialize_structure_crate_model_source_access_configuration(
                    &mut object_44,
                    item_43,
                );
                object_44.finish();
            }
        }
        array_42.finish();
    }
    if let Some(var_45) = &input.starting_position {
        object.key("StartingPosition").string(var_45.as_str());
    }
    if let Some(var_46) = &input.starting_position_timestamp {
        object
            .key("StartingPositionTimestamp")
            .instant(var_46, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_47) = &input.topics {
        let mut array_48 = object.key("Topics").start_array();
        for item_49 in var_47 {
            {
                array_48.value().string(item_49);
            }
        }
        array_48.finish();
    }
    if let Some(var_50) = &input.tumbling_window_in_seconds {
        object.key("TumblingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_50).into()),
        );
    }
}

pub fn serialize_structure_crate_input_create_function_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFunctionInput,
) {
    if let Some(var_51) = &input.architectures {
        let mut array_52 = object.key("Architectures").start_array();
        for item_53 in var_51 {
            {
                array_52.value().string(item_53.as_str());
            }
        }
        array_52.finish();
    }
    if let Some(var_54) = &input.code {
        let mut object_55 = object.key("Code").start_object();
        crate::json_ser::serialize_structure_crate_model_function_code(&mut object_55, var_54);
        object_55.finish();
    }
    if let Some(var_56) = &input.code_signing_config_arn {
        object.key("CodeSigningConfigArn").string(var_56);
    }
    if let Some(var_57) = &input.dead_letter_config {
        let mut object_58 = object.key("DeadLetterConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_dead_letter_config(&mut object_58, var_57);
        object_58.finish();
    }
    if let Some(var_59) = &input.description {
        object.key("Description").string(var_59);
    }
    if let Some(var_60) = &input.environment {
        let mut object_61 = object.key("Environment").start_object();
        crate::json_ser::serialize_structure_crate_model_environment(&mut object_61, var_60);
        object_61.finish();
    }
    if let Some(var_62) = &input.file_system_configs {
        let mut array_63 = object.key("FileSystemConfigs").start_array();
        for item_64 in var_62 {
            {
                let mut object_65 = array_63.value().start_object();
                crate::json_ser::serialize_structure_crate_model_file_system_config(
                    &mut object_65,
                    item_64,
                );
                object_65.finish();
            }
        }
        array_63.finish();
    }
    if let Some(var_66) = &input.function_name {
        object.key("FunctionName").string(var_66);
    }
    if let Some(var_67) = &input.handler {
        object.key("Handler").string(var_67);
    }
    if let Some(var_68) = &input.image_config {
        let mut object_69 = object.key("ImageConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_image_config(&mut object_69, var_68);
        object_69.finish();
    }
    if let Some(var_70) = &input.kms_key_arn {
        object.key("KMSKeyArn").string(var_70);
    }
    if let Some(var_71) = &input.layers {
        let mut array_72 = object.key("Layers").start_array();
        for item_73 in var_71 {
            {
                array_72.value().string(item_73);
            }
        }
        array_72.finish();
    }
    if let Some(var_74) = &input.memory_size {
        object.key("MemorySize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_74).into()),
        );
    }
    if let Some(var_75) = &input.package_type {
        object.key("PackageType").string(var_75.as_str());
    }
    if input.publish {
        object.key("Publish").boolean(input.publish);
    }
    if let Some(var_76) = &input.role {
        object.key("Role").string(var_76);
    }
    if let Some(var_77) = &input.runtime {
        object.key("Runtime").string(var_77.as_str());
    }
    if let Some(var_78) = &input.tags {
        let mut object_79 = object.key("Tags").start_object();
        for (key_80, value_81) in var_78 {
            {
                object_79.key(key_80).string(value_81);
            }
        }
        object_79.finish();
    }
    if let Some(var_82) = &input.timeout {
        object.key("Timeout").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_82).into()),
        );
    }
    if let Some(var_83) = &input.tracing_config {
        let mut object_84 = object.key("TracingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_tracing_config(&mut object_84, var_83);
        object_84.finish();
    }
    if let Some(var_85) = &input.vpc_config {
        let mut object_86 = object.key("VpcConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_config(&mut object_86, var_85);
        object_86.finish();
    }
}

pub fn serialize_structure_crate_input_publish_layer_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PublishLayerVersionInput,
) {
    if let Some(var_87) = &input.compatible_architectures {
        let mut array_88 = object.key("CompatibleArchitectures").start_array();
        for item_89 in var_87 {
            {
                array_88.value().string(item_89.as_str());
            }
        }
        array_88.finish();
    }
    if let Some(var_90) = &input.compatible_runtimes {
        let mut array_91 = object.key("CompatibleRuntimes").start_array();
        for item_92 in var_90 {
            {
                array_91.value().string(item_92.as_str());
            }
        }
        array_91.finish();
    }
    if let Some(var_93) = &input.content {
        let mut object_94 = object.key("Content").start_object();
        crate::json_ser::serialize_structure_crate_model_layer_version_content_input(
            &mut object_94,
            var_93,
        );
        object_94.finish();
    }
    if let Some(var_95) = &input.description {
        object.key("Description").string(var_95);
    }
    if let Some(var_96) = &input.license_info {
        object.key("LicenseInfo").string(var_96);
    }
}

pub fn serialize_structure_crate_input_publish_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PublishVersionInput,
) {
    if let Some(var_97) = &input.code_sha256 {
        object.key("CodeSha256").string(var_97);
    }
    if let Some(var_98) = &input.description {
        object.key("Description").string(var_98);
    }
    if let Some(var_99) = &input.revision_id {
        object.key("RevisionId").string(var_99);
    }
}

pub fn serialize_structure_crate_input_put_function_code_signing_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutFunctionCodeSigningConfigInput,
) {
    if let Some(var_100) = &input.code_signing_config_arn {
        object.key("CodeSigningConfigArn").string(var_100);
    }
}

pub fn serialize_structure_crate_input_put_function_concurrency_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutFunctionConcurrencyInput,
) {
    if let Some(var_101) = &input.reserved_concurrent_executions {
        object.key("ReservedConcurrentExecutions").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_101).into()),
        );
    }
}

pub fn serialize_structure_crate_input_put_function_event_invoke_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutFunctionEventInvokeConfigInput,
) {
    if let Some(var_102) = &input.destination_config {
        let mut object_103 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(
            &mut object_103,
            var_102,
        );
        object_103.finish();
    }
    if let Some(var_104) = &input.maximum_event_age_in_seconds {
        object.key("MaximumEventAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_104).into()),
        );
    }
    if let Some(var_105) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_105).into()),
        );
    }
}

pub fn serialize_structure_crate_input_put_provisioned_concurrency_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutProvisionedConcurrencyConfigInput,
) {
    if let Some(var_106) = &input.provisioned_concurrent_executions {
        object.key("ProvisionedConcurrentExecutions").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_106).into()),
        );
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_107) = &input.tags {
        let mut object_108 = object.key("Tags").start_object();
        for (key_109, value_110) in var_107 {
            {
                object_108.key(key_109).string(value_110);
            }
        }
        object_108.finish();
    }
}

pub fn serialize_structure_crate_input_update_alias_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAliasInput,
) {
    if let Some(var_111) = &input.description {
        object.key("Description").string(var_111);
    }
    if let Some(var_112) = &input.function_version {
        object.key("FunctionVersion").string(var_112);
    }
    if let Some(var_113) = &input.revision_id {
        object.key("RevisionId").string(var_113);
    }
    if let Some(var_114) = &input.routing_config {
        let mut object_115 = object.key("RoutingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_alias_routing_configuration(
            &mut object_115,
            var_114,
        );
        object_115.finish();
    }
}

pub fn serialize_structure_crate_input_update_code_signing_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCodeSigningConfigInput,
) {
    if let Some(var_116) = &input.allowed_publishers {
        let mut object_117 = object.key("AllowedPublishers").start_object();
        crate::json_ser::serialize_structure_crate_model_allowed_publishers(
            &mut object_117,
            var_116,
        );
        object_117.finish();
    }
    if let Some(var_118) = &input.code_signing_policies {
        let mut object_119 = object.key("CodeSigningPolicies").start_object();
        crate::json_ser::serialize_structure_crate_model_code_signing_policies(
            &mut object_119,
            var_118,
        );
        object_119.finish();
    }
    if let Some(var_120) = &input.description {
        object.key("Description").string(var_120);
    }
}

pub fn serialize_structure_crate_input_update_event_source_mapping_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEventSourceMappingInput,
) {
    if let Some(var_121) = &input.batch_size {
        object.key("BatchSize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_121).into()),
        );
    }
    if let Some(var_122) = &input.bisect_batch_on_function_error {
        object.key("BisectBatchOnFunctionError").boolean(*var_122);
    }
    if let Some(var_123) = &input.destination_config {
        let mut object_124 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(
            &mut object_124,
            var_123,
        );
        object_124.finish();
    }
    if let Some(var_125) = &input.enabled {
        object.key("Enabled").boolean(*var_125);
    }
    if let Some(var_126) = &input.function_name {
        object.key("FunctionName").string(var_126);
    }
    if let Some(var_127) = &input.function_response_types {
        let mut array_128 = object.key("FunctionResponseTypes").start_array();
        for item_129 in var_127 {
            {
                array_128.value().string(item_129.as_str());
            }
        }
        array_128.finish();
    }
    if let Some(var_130) = &input.maximum_batching_window_in_seconds {
        object.key("MaximumBatchingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_130).into()),
        );
    }
    if let Some(var_131) = &input.maximum_record_age_in_seconds {
        object.key("MaximumRecordAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_131).into()),
        );
    }
    if let Some(var_132) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_132).into()),
        );
    }
    if let Some(var_133) = &input.parallelization_factor {
        object.key("ParallelizationFactor").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_133).into()),
        );
    }
    if let Some(var_134) = &input.source_access_configurations {
        let mut array_135 = object.key("SourceAccessConfigurations").start_array();
        for item_136 in var_134 {
            {
                let mut object_137 = array_135.value().start_object();
                crate::json_ser::serialize_structure_crate_model_source_access_configuration(
                    &mut object_137,
                    item_136,
                );
                object_137.finish();
            }
        }
        array_135.finish();
    }
    if let Some(var_138) = &input.tumbling_window_in_seconds {
        object.key("TumblingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_138).into()),
        );
    }
}

pub fn serialize_structure_crate_input_update_function_code_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFunctionCodeInput,
) {
    if let Some(var_139) = &input.architectures {
        let mut array_140 = object.key("Architectures").start_array();
        for item_141 in var_139 {
            {
                array_140.value().string(item_141.as_str());
            }
        }
        array_140.finish();
    }
    if input.dry_run {
        object.key("DryRun").boolean(input.dry_run);
    }
    if let Some(var_142) = &input.image_uri {
        object.key("ImageUri").string(var_142);
    }
    if input.publish {
        object.key("Publish").boolean(input.publish);
    }
    if let Some(var_143) = &input.revision_id {
        object.key("RevisionId").string(var_143);
    }
    if let Some(var_144) = &input.s3_bucket {
        object.key("S3Bucket").string(var_144);
    }
    if let Some(var_145) = &input.s3_key {
        object.key("S3Key").string(var_145);
    }
    if let Some(var_146) = &input.s3_object_version {
        object.key("S3ObjectVersion").string(var_146);
    }
    if let Some(var_147) = &input.zip_file {
        object
            .key("ZipFile")
            .string_unchecked(&smithy_types::base64::encode(var_147));
    }
}

pub fn serialize_structure_crate_input_update_function_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFunctionConfigurationInput,
) {
    if let Some(var_148) = &input.dead_letter_config {
        let mut object_149 = object.key("DeadLetterConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_dead_letter_config(
            &mut object_149,
            var_148,
        );
        object_149.finish();
    }
    if let Some(var_150) = &input.description {
        object.key("Description").string(var_150);
    }
    if let Some(var_151) = &input.environment {
        let mut object_152 = object.key("Environment").start_object();
        crate::json_ser::serialize_structure_crate_model_environment(&mut object_152, var_151);
        object_152.finish();
    }
    if let Some(var_153) = &input.file_system_configs {
        let mut array_154 = object.key("FileSystemConfigs").start_array();
        for item_155 in var_153 {
            {
                let mut object_156 = array_154.value().start_object();
                crate::json_ser::serialize_structure_crate_model_file_system_config(
                    &mut object_156,
                    item_155,
                );
                object_156.finish();
            }
        }
        array_154.finish();
    }
    if let Some(var_157) = &input.handler {
        object.key("Handler").string(var_157);
    }
    if let Some(var_158) = &input.image_config {
        let mut object_159 = object.key("ImageConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_image_config(&mut object_159, var_158);
        object_159.finish();
    }
    if let Some(var_160) = &input.kms_key_arn {
        object.key("KMSKeyArn").string(var_160);
    }
    if let Some(var_161) = &input.layers {
        let mut array_162 = object.key("Layers").start_array();
        for item_163 in var_161 {
            {
                array_162.value().string(item_163);
            }
        }
        array_162.finish();
    }
    if let Some(var_164) = &input.memory_size {
        object.key("MemorySize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_164).into()),
        );
    }
    if let Some(var_165) = &input.revision_id {
        object.key("RevisionId").string(var_165);
    }
    if let Some(var_166) = &input.role {
        object.key("Role").string(var_166);
    }
    if let Some(var_167) = &input.runtime {
        object.key("Runtime").string(var_167.as_str());
    }
    if let Some(var_168) = &input.timeout {
        object.key("Timeout").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_168).into()),
        );
    }
    if let Some(var_169) = &input.tracing_config {
        let mut object_170 = object.key("TracingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_tracing_config(&mut object_170, var_169);
        object_170.finish();
    }
    if let Some(var_171) = &input.vpc_config {
        let mut object_172 = object.key("VpcConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_config(&mut object_172, var_171);
        object_172.finish();
    }
}

pub fn serialize_structure_crate_input_update_function_event_invoke_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFunctionEventInvokeConfigInput,
) {
    if let Some(var_173) = &input.destination_config {
        let mut object_174 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(
            &mut object_174,
            var_173,
        );
        object_174.finish();
    }
    if let Some(var_175) = &input.maximum_event_age_in_seconds {
        object.key("MaximumEventAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_175).into()),
        );
    }
    if let Some(var_176) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_176).into()),
        );
    }
}

pub fn serialize_structure_crate_model_alias_routing_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AliasRoutingConfiguration,
) {
    if let Some(var_177) = &input.additional_version_weights {
        let mut object_178 = object.key("AdditionalVersionWeights").start_object();
        for (key_179, value_180) in var_177 {
            {
                object_178.key(key_179).number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::Float((*value_180).into()),
                );
            }
        }
        object_178.finish();
    }
}

pub fn serialize_structure_crate_model_allowed_publishers(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AllowedPublishers,
) {
    if let Some(var_181) = &input.signing_profile_version_arns {
        let mut array_182 = object.key("SigningProfileVersionArns").start_array();
        for item_183 in var_181 {
            {
                array_182.value().string(item_183);
            }
        }
        array_182.finish();
    }
}

pub fn serialize_structure_crate_model_code_signing_policies(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CodeSigningPolicies,
) {
    if let Some(var_184) = &input.untrusted_artifact_on_deployment {
        object
            .key("UntrustedArtifactOnDeployment")
            .string(var_184.as_str());
    }
}

pub fn serialize_structure_crate_model_destination_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DestinationConfig,
) {
    if let Some(var_185) = &input.on_success {
        let mut object_186 = object.key("OnSuccess").start_object();
        crate::json_ser::serialize_structure_crate_model_on_success(&mut object_186, var_185);
        object_186.finish();
    }
    if let Some(var_187) = &input.on_failure {
        let mut object_188 = object.key("OnFailure").start_object();
        crate::json_ser::serialize_structure_crate_model_on_failure(&mut object_188, var_187);
        object_188.finish();
    }
}

pub fn serialize_structure_crate_model_self_managed_event_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SelfManagedEventSource,
) {
    if let Some(var_189) = &input.endpoints {
        let mut object_190 = object.key("Endpoints").start_object();
        for (key_191, value_192) in var_189 {
            {
                let mut array_193 = object_190.key(key_191.as_str()).start_array();
                for item_194 in value_192 {
                    {
                        array_193.value().string(item_194);
                    }
                }
                array_193.finish();
            }
        }
        object_190.finish();
    }
}

pub fn serialize_structure_crate_model_source_access_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SourceAccessConfiguration,
) {
    if let Some(var_195) = &input.r#type {
        object.key("Type").string(var_195.as_str());
    }
    if let Some(var_196) = &input.uri {
        object.key("URI").string(var_196);
    }
}

pub fn serialize_structure_crate_model_function_code(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FunctionCode,
) {
    if let Some(var_197) = &input.zip_file {
        object
            .key("ZipFile")
            .string_unchecked(&smithy_types::base64::encode(var_197));
    }
    if let Some(var_198) = &input.s3_bucket {
        object.key("S3Bucket").string(var_198);
    }
    if let Some(var_199) = &input.s3_key {
        object.key("S3Key").string(var_199);
    }
    if let Some(var_200) = &input.s3_object_version {
        object.key("S3ObjectVersion").string(var_200);
    }
    if let Some(var_201) = &input.image_uri {
        object.key("ImageUri").string(var_201);
    }
}

pub fn serialize_structure_crate_model_dead_letter_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeadLetterConfig,
) {
    if let Some(var_202) = &input.target_arn {
        object.key("TargetArn").string(var_202);
    }
}

pub fn serialize_structure_crate_model_environment(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Environment,
) {
    if let Some(var_203) = &input.variables {
        let mut object_204 = object.key("Variables").start_object();
        for (key_205, value_206) in var_203 {
            {
                object_204.key(key_205).string(value_206);
            }
        }
        object_204.finish();
    }
}

pub fn serialize_structure_crate_model_file_system_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FileSystemConfig,
) {
    if let Some(var_207) = &input.arn {
        object.key("Arn").string(var_207);
    }
    if let Some(var_208) = &input.local_mount_path {
        object.key("LocalMountPath").string(var_208);
    }
}

pub fn serialize_structure_crate_model_image_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImageConfig,
) {
    if let Some(var_209) = &input.entry_point {
        let mut array_210 = object.key("EntryPoint").start_array();
        for item_211 in var_209 {
            {
                array_210.value().string(item_211);
            }
        }
        array_210.finish();
    }
    if let Some(var_212) = &input.command {
        let mut array_213 = object.key("Command").start_array();
        for item_214 in var_212 {
            {
                array_213.value().string(item_214);
            }
        }
        array_213.finish();
    }
    if let Some(var_215) = &input.working_directory {
        object.key("WorkingDirectory").string(var_215);
    }
}

pub fn serialize_structure_crate_model_tracing_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TracingConfig,
) {
    if let Some(var_216) = &input.mode {
        object.key("Mode").string(var_216.as_str());
    }
}

pub fn serialize_structure_crate_model_vpc_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcConfig,
) {
    if let Some(var_217) = &input.subnet_ids {
        let mut array_218 = object.key("SubnetIds").start_array();
        for item_219 in var_217 {
            {
                array_218.value().string(item_219);
            }
        }
        array_218.finish();
    }
    if let Some(var_220) = &input.security_group_ids {
        let mut array_221 = object.key("SecurityGroupIds").start_array();
        for item_222 in var_220 {
            {
                array_221.value().string(item_222);
            }
        }
        array_221.finish();
    }
}

pub fn serialize_structure_crate_model_layer_version_content_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LayerVersionContentInput,
) {
    if let Some(var_223) = &input.s3_bucket {
        object.key("S3Bucket").string(var_223);
    }
    if let Some(var_224) = &input.s3_key {
        object.key("S3Key").string(var_224);
    }
    if let Some(var_225) = &input.s3_object_version {
        object.key("S3ObjectVersion").string(var_225);
    }
    if let Some(var_226) = &input.zip_file {
        object
            .key("ZipFile")
            .string_unchecked(&smithy_types::base64::encode(var_226));
    }
}

pub fn serialize_structure_crate_model_on_success(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OnSuccess,
) {
    if let Some(var_227) = &input.destination {
        object.key("Destination").string(var_227);
    }
}

pub fn serialize_structure_crate_model_on_failure(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OnFailure,
) {
    if let Some(var_228) = &input.destination {
        object.key("Destination").string(var_228);
    }
}
