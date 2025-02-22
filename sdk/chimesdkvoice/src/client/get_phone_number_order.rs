// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPhoneNumberOrder`](crate::operation::get_phone_number_order::builders::GetPhoneNumberOrderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`phone_number_order_id(impl ::std::convert::Into<String>)`](crate::operation::get_phone_number_order::builders::GetPhoneNumberOrderFluentBuilder::phone_number_order_id) / [`set_phone_number_order_id(Option<String>)`](crate::operation::get_phone_number_order::builders::GetPhoneNumberOrderFluentBuilder::set_phone_number_order_id): <p>The ID of the phone number order .</p>
    /// - On success, responds with [`GetPhoneNumberOrderOutput`](crate::operation::get_phone_number_order::GetPhoneNumberOrderOutput) with field(s):
    ///   - [`phone_number_order(Option<PhoneNumberOrder>)`](crate::operation::get_phone_number_order::GetPhoneNumberOrderOutput::phone_number_order): <p>The phone number order details.</p>
    /// - On failure, responds with [`SdkError<GetPhoneNumberOrderError>`](crate::operation::get_phone_number_order::GetPhoneNumberOrderError)
    pub fn get_phone_number_order(
        &self,
    ) -> crate::operation::get_phone_number_order::builders::GetPhoneNumberOrderFluentBuilder {
        crate::operation::get_phone_number_order::builders::GetPhoneNumberOrderFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
