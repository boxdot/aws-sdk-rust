// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_resume_cluster_message(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ResumeClusterMessage,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClusterIdentifier");
    if let Some(var_2) = &input.cluster_identifier {
        scope_1.string(var_2);
    }
    Ok(())
}

pub fn de_resume_cluster_message(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ResumeClusterMessage, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ResumeClusterMessage::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ClusterIdentifier") /* ClusterIdentifier com.amazonaws.redshift#ResumeClusterMessage$ClusterIdentifier */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cluster_identifier(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
