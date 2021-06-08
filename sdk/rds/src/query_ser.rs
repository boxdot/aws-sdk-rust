// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_tag(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Tag,
) {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Key");
    if let Some(var_2) = &input.key {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_scaling_configuration(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::ScalingConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MinCapacity");
    if let Some(var_6) = &input.min_capacity {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("MaxCapacity");
    if let Some(var_8) = &input.max_capacity {
        scope_7.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("AutoPause");
    if let Some(var_10) = &input.auto_pause {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("SecondsUntilAutoPause");
    if let Some(var_12) = &input.seconds_until_auto_pause {
        scope_11.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("TimeoutAction");
    if let Some(var_14) = &input.timeout_action {
        scope_13.string(var_14);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_processor_feature(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::ProcessorFeature,
) {
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("Name");
    if let Some(var_16) = &input.name {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("Value");
    if let Some(var_18) = &input.value {
        scope_17.string(var_18);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_user_auth_config(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::UserAuthConfig,
) {
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("Description");
    if let Some(var_20) = &input.description {
        scope_19.string(var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("UserName");
    if let Some(var_22) = &input.user_name {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("AuthScheme");
    if let Some(var_24) = &input.auth_scheme {
        scope_23.string(var_24.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("SecretArn");
    if let Some(var_26) = &input.secret_arn {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("IAMAuth");
    if let Some(var_28) = &input.iam_auth {
        scope_27.string(var_28.as_str());
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_filter(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Filter,
) {
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("Name");
    if let Some(var_30) = &input.name {
        scope_29.string(var_30);
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("Values");
    if let Some(var_32) = &input.values {
        let mut list_34 = scope_31.start_list(false, Some("Value"));
        for item_33 in var_32 {
            #[allow(unused_mut)]
            let mut entry_35 = list_34.entry();
            entry_35.string(item_33);
        }
        list_34.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_cloudwatch_logs_export_configuration(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::CloudwatchLogsExportConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_36 = writer.prefix("EnableLogTypes");
    if let Some(var_37) = &input.enable_log_types {
        let mut list_39 = scope_36.start_list(false, None);
        for item_38 in var_37 {
            #[allow(unused_mut)]
            let mut entry_40 = list_39.entry();
            entry_40.string(item_38);
        }
        list_39.finish();
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("DisableLogTypes");
    if let Some(var_42) = &input.disable_log_types {
        let mut list_44 = scope_41.start_list(false, None);
        for item_43 in var_42 {
            #[allow(unused_mut)]
            let mut entry_45 = list_44.entry();
            entry_45.string(item_43);
        }
        list_44.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_parameter(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Parameter,
) {
    #[allow(unused_mut)]
    let mut scope_46 = writer.prefix("ParameterName");
    if let Some(var_47) = &input.parameter_name {
        scope_46.string(var_47);
    }
    #[allow(unused_mut)]
    let mut scope_48 = writer.prefix("ParameterValue");
    if let Some(var_49) = &input.parameter_value {
        scope_48.string(var_49);
    }
    #[allow(unused_mut)]
    let mut scope_50 = writer.prefix("Description");
    if let Some(var_51) = &input.description {
        scope_50.string(var_51);
    }
    #[allow(unused_mut)]
    let mut scope_52 = writer.prefix("Source");
    if let Some(var_53) = &input.source {
        scope_52.string(var_53);
    }
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("ApplyType");
    if let Some(var_55) = &input.apply_type {
        scope_54.string(var_55);
    }
    #[allow(unused_mut)]
    let mut scope_56 = writer.prefix("DataType");
    if let Some(var_57) = &input.data_type {
        scope_56.string(var_57);
    }
    #[allow(unused_mut)]
    let mut scope_58 = writer.prefix("AllowedValues");
    if let Some(var_59) = &input.allowed_values {
        scope_58.string(var_59);
    }
    #[allow(unused_mut)]
    let mut scope_60 = writer.prefix("IsModifiable");
    if input.is_modifiable {
        scope_60.boolean(input.is_modifiable);
    }
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("MinimumEngineVersion");
    if let Some(var_62) = &input.minimum_engine_version {
        scope_61.string(var_62);
    }
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("ApplyMethod");
    if let Some(var_64) = &input.apply_method {
        scope_63.string(var_64.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("SupportedEngineModes");
    if let Some(var_66) = &input.supported_engine_modes {
        let mut list_68 = scope_65.start_list(false, None);
        for item_67 in var_66 {
            #[allow(unused_mut)]
            let mut entry_69 = list_68.entry();
            entry_69.string(item_67);
        }
        list_68.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_connection_pool_configuration(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::ConnectionPoolConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_70 = writer.prefix("MaxConnectionsPercent");
    if let Some(var_71) = &input.max_connections_percent {
        scope_70.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_71).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_72 = writer.prefix("MaxIdleConnectionsPercent");
    if let Some(var_73) = &input.max_idle_connections_percent {
        scope_72.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_73).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_74 = writer.prefix("ConnectionBorrowTimeout");
    if let Some(var_75) = &input.connection_borrow_timeout {
        scope_74.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_75).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_76 = writer.prefix("SessionPinningFilters");
    if let Some(var_77) = &input.session_pinning_filters {
        let mut list_79 = scope_76.start_list(false, None);
        for item_78 in var_77 {
            #[allow(unused_mut)]
            let mut entry_80 = list_79.entry();
            entry_80.string(item_78);
        }
        list_79.finish();
    }
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("InitQuery");
    if let Some(var_82) = &input.init_query {
        scope_81.string(var_82);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_option_configuration(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::OptionConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("OptionName");
    if let Some(var_84) = &input.option_name {
        scope_83.string(var_84);
    }
    #[allow(unused_mut)]
    let mut scope_85 = writer.prefix("Port");
    if let Some(var_86) = &input.port {
        scope_85.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_86).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_87 = writer.prefix("OptionVersion");
    if let Some(var_88) = &input.option_version {
        scope_87.string(var_88);
    }
    #[allow(unused_mut)]
    let mut scope_89 = writer.prefix("DBSecurityGroupMemberships");
    if let Some(var_90) = &input.db_security_group_memberships {
        let mut list_92 = scope_89.start_list(false, Some("DBSecurityGroupName"));
        for item_91 in var_90 {
            #[allow(unused_mut)]
            let mut entry_93 = list_92.entry();
            entry_93.string(item_91);
        }
        list_92.finish();
    }
    #[allow(unused_mut)]
    let mut scope_94 = writer.prefix("VpcSecurityGroupMemberships");
    if let Some(var_95) = &input.vpc_security_group_memberships {
        let mut list_97 = scope_94.start_list(false, Some("VpcSecurityGroupId"));
        for item_96 in var_95 {
            #[allow(unused_mut)]
            let mut entry_98 = list_97.entry();
            entry_98.string(item_96);
        }
        list_97.finish();
    }
    #[allow(unused_mut)]
    let mut scope_99 = writer.prefix("OptionSettings");
    if let Some(var_100) = &input.option_settings {
        let mut list_102 = scope_99.start_list(false, Some("OptionSetting"));
        for item_101 in var_100 {
            #[allow(unused_mut)]
            let mut entry_103 = list_102.entry();
            crate::query_ser::serialize_structure_option_setting(entry_103, item_101);
        }
        list_102.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_option_setting(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::OptionSetting,
) {
    #[allow(unused_mut)]
    let mut scope_104 = writer.prefix("Name");
    if let Some(var_105) = &input.name {
        scope_104.string(var_105);
    }
    #[allow(unused_mut)]
    let mut scope_106 = writer.prefix("Value");
    if let Some(var_107) = &input.value {
        scope_106.string(var_107);
    }
    #[allow(unused_mut)]
    let mut scope_108 = writer.prefix("DefaultValue");
    if let Some(var_109) = &input.default_value {
        scope_108.string(var_109);
    }
    #[allow(unused_mut)]
    let mut scope_110 = writer.prefix("Description");
    if let Some(var_111) = &input.description {
        scope_110.string(var_111);
    }
    #[allow(unused_mut)]
    let mut scope_112 = writer.prefix("ApplyType");
    if let Some(var_113) = &input.apply_type {
        scope_112.string(var_113);
    }
    #[allow(unused_mut)]
    let mut scope_114 = writer.prefix("DataType");
    if let Some(var_115) = &input.data_type {
        scope_114.string(var_115);
    }
    #[allow(unused_mut)]
    let mut scope_116 = writer.prefix("AllowedValues");
    if let Some(var_117) = &input.allowed_values {
        scope_116.string(var_117);
    }
    #[allow(unused_mut)]
    let mut scope_118 = writer.prefix("IsModifiable");
    if input.is_modifiable {
        scope_118.boolean(input.is_modifiable);
    }
    #[allow(unused_mut)]
    let mut scope_119 = writer.prefix("IsCollection");
    if input.is_collection {
        scope_119.boolean(input.is_collection);
    }
}
