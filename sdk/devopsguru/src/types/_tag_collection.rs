// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A collection of Amazon Web Services tags.</p>
/// <p>Tags help you identify and organize your Amazon Web Services resources. Many Amazon Web Services services support tagging, so you can assign the same tag to resources from different services to indicate that the resources are related. For example, you can assign the same tag to an Amazon DynamoDB table resource that you assign to an Lambda function. For more information about using tags, see the <a href="https://docs.aws.amazon.com/whitepapers/latest/tagging-best-practices/tagging-best-practices.html">Tagging best practices</a> whitepaper. </p>
/// <p>Each Amazon Web Services tag has two parts. </p>
/// <ul>
/// <li> <p>A tag <i>key</i> (for example, <code>CostCenter</code>, <code>Environment</code>, <code>Project</code>, or <code>Secret</code>). Tag <i>keys</i> are case-sensitive.</p> </li>
/// <li> <p>An optional field known as a tag <i>value</i> (for example, <code>111122223333</code>, <code>Production</code>, or a team name). Omitting the tag <i>value</i> is the same as using an empty string. Like tag <i>keys</i>, tag <i>values</i> are case-sensitive.</p> </li>
/// </ul>
/// <p>Together these are known as <i>key</i>-<i>value</i> pairs.</p> <important>
/// <p>The string used for a <i>key</i> in a tag that you use to define your resource coverage must begin with the prefix <code>Devops-guru-</code>. The tag <i>key</i> might be <code>DevOps-Guru-deployment-application</code> or <code>devops-guru-rds-application</code>. When you create a <i>key</i>, the case of characters in the <i>key</i> can be whatever you choose. After you create a <i>key</i>, it is case-sensitive. For example, DevOps Guru works with a <i>key</i> named <code>devops-guru-rds</code> and a <i>key</i> named <code>DevOps-Guru-RDS</code>, and these act as two different <i>keys</i>. Possible <i>key</i>/<i>value</i> pairs in your application might be <code>Devops-Guru-production-application/RDS</code> or <code>Devops-Guru-production-application/containers</code>.</p>
/// </important>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TagCollection {
    /// <p>An Amazon Web Services tag <i>key</i> that is used to identify the Amazon Web Services resources that DevOps Guru analyzes. All Amazon Web Services resources in your account and Region tagged with this <i>key</i> make up your DevOps Guru application and analysis boundary.</p> <important>
    /// <p>The string used for a <i>key</i> in a tag that you use to define your resource coverage must begin with the prefix <code>Devops-guru-</code>. The tag <i>key</i> might be <code>DevOps-Guru-deployment-application</code> or <code>devops-guru-rds-application</code>. When you create a <i>key</i>, the case of characters in the <i>key</i> can be whatever you choose. After you create a <i>key</i>, it is case-sensitive. For example, DevOps Guru works with a <i>key</i> named <code>devops-guru-rds</code> and a <i>key</i> named <code>DevOps-Guru-RDS</code>, and these act as two different <i>keys</i>. Possible <i>key</i>/<i>value</i> pairs in your application might be <code>Devops-Guru-production-application/RDS</code> or <code>Devops-Guru-production-application/containers</code>.</p>
    /// </important>
    #[doc(hidden)]
    pub app_boundary_key: ::std::option::Option<::std::string::String>,
    /// <p>The values in an Amazon Web Services tag collection.</p>
    /// <p>The tag's <i>value</i> is an optional field used to associate a string with the tag <i>key</i> (for example, <code>111122223333</code>, <code>Production</code>, or a team name). The <i>key</i> and <i>value</i> are the tag's <i>key</i> pair. Omitting the tag <i>value</i> is the same as using an empty string. Like tag <i>keys</i>, tag <i>values</i> are case-sensitive. You can specify a maximum of 256 characters for a tag value.</p>
    #[doc(hidden)]
    pub tag_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl TagCollection {
    /// <p>An Amazon Web Services tag <i>key</i> that is used to identify the Amazon Web Services resources that DevOps Guru analyzes. All Amazon Web Services resources in your account and Region tagged with this <i>key</i> make up your DevOps Guru application and analysis boundary.</p> <important>
    /// <p>The string used for a <i>key</i> in a tag that you use to define your resource coverage must begin with the prefix <code>Devops-guru-</code>. The tag <i>key</i> might be <code>DevOps-Guru-deployment-application</code> or <code>devops-guru-rds-application</code>. When you create a <i>key</i>, the case of characters in the <i>key</i> can be whatever you choose. After you create a <i>key</i>, it is case-sensitive. For example, DevOps Guru works with a <i>key</i> named <code>devops-guru-rds</code> and a <i>key</i> named <code>DevOps-Guru-RDS</code>, and these act as two different <i>keys</i>. Possible <i>key</i>/<i>value</i> pairs in your application might be <code>Devops-Guru-production-application/RDS</code> or <code>Devops-Guru-production-application/containers</code>.</p>
    /// </important>
    pub fn app_boundary_key(&self) -> ::std::option::Option<&str> {
        self.app_boundary_key.as_deref()
    }
    /// <p>The values in an Amazon Web Services tag collection.</p>
    /// <p>The tag's <i>value</i> is an optional field used to associate a string with the tag <i>key</i> (for example, <code>111122223333</code>, <code>Production</code>, or a team name). The <i>key</i> and <i>value</i> are the tag's <i>key</i> pair. Omitting the tag <i>value</i> is the same as using an empty string. Like tag <i>keys</i>, tag <i>values</i> are case-sensitive. You can specify a maximum of 256 characters for a tag value.</p>
    pub fn tag_values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.tag_values.as_deref()
    }
}
impl TagCollection {
    /// Creates a new builder-style object to manufacture [`TagCollection`](crate::types::TagCollection).
    pub fn builder() -> crate::types::builders::TagCollectionBuilder {
        crate::types::builders::TagCollectionBuilder::default()
    }
}

/// A builder for [`TagCollection`](crate::types::TagCollection).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagCollectionBuilder {
    pub(crate) app_boundary_key: ::std::option::Option<::std::string::String>,
    pub(crate) tag_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl TagCollectionBuilder {
    /// <p>An Amazon Web Services tag <i>key</i> that is used to identify the Amazon Web Services resources that DevOps Guru analyzes. All Amazon Web Services resources in your account and Region tagged with this <i>key</i> make up your DevOps Guru application and analysis boundary.</p> <important>
    /// <p>The string used for a <i>key</i> in a tag that you use to define your resource coverage must begin with the prefix <code>Devops-guru-</code>. The tag <i>key</i> might be <code>DevOps-Guru-deployment-application</code> or <code>devops-guru-rds-application</code>. When you create a <i>key</i>, the case of characters in the <i>key</i> can be whatever you choose. After you create a <i>key</i>, it is case-sensitive. For example, DevOps Guru works with a <i>key</i> named <code>devops-guru-rds</code> and a <i>key</i> named <code>DevOps-Guru-RDS</code>, and these act as two different <i>keys</i>. Possible <i>key</i>/<i>value</i> pairs in your application might be <code>Devops-Guru-production-application/RDS</code> or <code>Devops-Guru-production-application/containers</code>.</p>
    /// </important>
    pub fn app_boundary_key(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_boundary_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An Amazon Web Services tag <i>key</i> that is used to identify the Amazon Web Services resources that DevOps Guru analyzes. All Amazon Web Services resources in your account and Region tagged with this <i>key</i> make up your DevOps Guru application and analysis boundary.</p> <important>
    /// <p>The string used for a <i>key</i> in a tag that you use to define your resource coverage must begin with the prefix <code>Devops-guru-</code>. The tag <i>key</i> might be <code>DevOps-Guru-deployment-application</code> or <code>devops-guru-rds-application</code>. When you create a <i>key</i>, the case of characters in the <i>key</i> can be whatever you choose. After you create a <i>key</i>, it is case-sensitive. For example, DevOps Guru works with a <i>key</i> named <code>devops-guru-rds</code> and a <i>key</i> named <code>DevOps-Guru-RDS</code>, and these act as two different <i>keys</i>. Possible <i>key</i>/<i>value</i> pairs in your application might be <code>Devops-Guru-production-application/RDS</code> or <code>Devops-Guru-production-application/containers</code>.</p>
    /// </important>
    pub fn set_app_boundary_key(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_boundary_key = input;
        self
    }
    /// Appends an item to `tag_values`.
    ///
    /// To override the contents of this collection use [`set_tag_values`](Self::set_tag_values).
    ///
    /// <p>The values in an Amazon Web Services tag collection.</p>
    /// <p>The tag's <i>value</i> is an optional field used to associate a string with the tag <i>key</i> (for example, <code>111122223333</code>, <code>Production</code>, or a team name). The <i>key</i> and <i>value</i> are the tag's <i>key</i> pair. Omitting the tag <i>value</i> is the same as using an empty string. Like tag <i>keys</i>, tag <i>values</i> are case-sensitive. You can specify a maximum of 256 characters for a tag value.</p>
    pub fn tag_values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.tag_values.unwrap_or_default();
        v.push(input.into());
        self.tag_values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The values in an Amazon Web Services tag collection.</p>
    /// <p>The tag's <i>value</i> is an optional field used to associate a string with the tag <i>key</i> (for example, <code>111122223333</code>, <code>Production</code>, or a team name). The <i>key</i> and <i>value</i> are the tag's <i>key</i> pair. Omitting the tag <i>value</i> is the same as using an empty string. Like tag <i>keys</i>, tag <i>values</i> are case-sensitive. You can specify a maximum of 256 characters for a tag value.</p>
    pub fn set_tag_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.tag_values = input;
        self
    }
    /// Consumes the builder and constructs a [`TagCollection`](crate::types::TagCollection).
    pub fn build(self) -> crate::types::TagCollection {
        crate::types::TagCollection {
            app_boundary_key: self.app_boundary_key,
            tag_values: self.tag_values,
        }
    }
}
