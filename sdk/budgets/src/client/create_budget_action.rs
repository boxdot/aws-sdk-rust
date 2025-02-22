// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateBudgetAction`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::set_account_id): <p>The account ID of the user. It's a 12-digit number.</p>
    ///   - [`budget_name(impl ::std::convert::Into<String>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::budget_name) / [`set_budget_name(Option<String>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::set_budget_name): <p> A string that represents the budget name. The ":" and "\" characters aren't allowed.</p>
    ///   - [`notification_type(NotificationType)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::notification_type) / [`set_notification_type(Option<NotificationType>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::set_notification_type): <p> The type of a notification. It must be ACTUAL or FORECASTED.</p>
    ///   - [`action_type(ActionType)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::action_type) / [`set_action_type(Option<ActionType>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::set_action_type): <p> The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition. </p>
    ///   - [`action_threshold(ActionThreshold)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::action_threshold) / [`set_action_threshold(Option<ActionThreshold>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::set_action_threshold): <p>The trigger threshold of the action. </p>
    ///   - [`definition(Definition)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::definition) / [`set_definition(Option<Definition>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::set_definition): <p>Specifies all of the type-specific parameters. </p>
    ///   - [`execution_role_arn(impl ::std::convert::Into<String>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::execution_role_arn) / [`set_execution_role_arn(Option<String>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::set_execution_role_arn): <p> The role passed for action execution and reversion. Roles and actions must be in the same account. </p>
    ///   - [`approval_model(ApprovalModel)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::approval_model) / [`set_approval_model(Option<ApprovalModel>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::set_approval_model): <p> This specifies if the action needs manual or automatic approval. </p>
    ///   - [`subscribers(Vec<Subscriber>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::subscribers) / [`set_subscribers(Option<Vec<Subscriber>>)`](crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::set_subscribers): <p> A list of subscribers.</p>
    /// - On success, responds with [`CreateBudgetActionOutput`](crate::operation::create_budget_action::CreateBudgetActionOutput) with field(s):
    ///   - [`account_id(Option<String>)`](crate::operation::create_budget_action::CreateBudgetActionOutput::account_id): <p>The account ID of the user. It's a 12-digit number.</p>
    ///   - [`budget_name(Option<String>)`](crate::operation::create_budget_action::CreateBudgetActionOutput::budget_name): <p> A string that represents the budget name. The ":" and "\" characters aren't allowed.</p>
    ///   - [`action_id(Option<String>)`](crate::operation::create_budget_action::CreateBudgetActionOutput::action_id): <p> A system-generated universally unique identifier (UUID) for the action. </p>
    /// - On failure, responds with [`SdkError<CreateBudgetActionError>`](crate::operation::create_budget_action::CreateBudgetActionError)
    pub fn create_budget_action(
        &self,
    ) -> crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder {
        crate::operation::create_budget_action::builders::CreateBudgetActionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
