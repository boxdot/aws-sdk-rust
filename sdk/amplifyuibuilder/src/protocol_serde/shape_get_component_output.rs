// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_component_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::Component>,
    crate::operation::get_component::GetComponentError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_component::de_component_payload(body)
                .map_err(crate::operation::get_component::GetComponentError::unhandled)
        })
        .transpose()
}
