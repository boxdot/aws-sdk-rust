// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies code that runs when a job is run.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JobCommand {
    /// <p>The name of the job command. For an Apache Spark ETL job, this must be <code>glueetl</code>. For a Python shell job, it must be <code>pythonshell</code>. For an Apache Spark streaming ETL job, this must be <code>gluestreaming</code>.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the Amazon Simple Storage Service (Amazon S3) path to a script that runs a job.</p>
    #[doc(hidden)]
    pub script_location: ::std::option::Option<::std::string::String>,
    /// <p>The Python version being used to run a Python shell job. Allowed values are 2 or 3.</p>
    #[doc(hidden)]
    pub python_version: ::std::option::Option<::std::string::String>,
}
impl JobCommand {
    /// <p>The name of the job command. For an Apache Spark ETL job, this must be <code>glueetl</code>. For a Python shell job, it must be <code>pythonshell</code>. For an Apache Spark streaming ETL job, this must be <code>gluestreaming</code>.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Specifies the Amazon Simple Storage Service (Amazon S3) path to a script that runs a job.</p>
    pub fn script_location(&self) -> ::std::option::Option<&str> {
        self.script_location.as_deref()
    }
    /// <p>The Python version being used to run a Python shell job. Allowed values are 2 or 3.</p>
    pub fn python_version(&self) -> ::std::option::Option<&str> {
        self.python_version.as_deref()
    }
}
impl JobCommand {
    /// Creates a new builder-style object to manufacture [`JobCommand`](crate::types::JobCommand).
    pub fn builder() -> crate::types::builders::JobCommandBuilder {
        crate::types::builders::JobCommandBuilder::default()
    }
}

/// A builder for [`JobCommand`](crate::types::JobCommand).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct JobCommandBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) script_location: ::std::option::Option<::std::string::String>,
    pub(crate) python_version: ::std::option::Option<::std::string::String>,
}
impl JobCommandBuilder {
    /// <p>The name of the job command. For an Apache Spark ETL job, this must be <code>glueetl</code>. For a Python shell job, it must be <code>pythonshell</code>. For an Apache Spark streaming ETL job, this must be <code>gluestreaming</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the job command. For an Apache Spark ETL job, this must be <code>glueetl</code>. For a Python shell job, it must be <code>pythonshell</code>. For an Apache Spark streaming ETL job, this must be <code>gluestreaming</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Specifies the Amazon Simple Storage Service (Amazon S3) path to a script that runs a job.</p>
    pub fn script_location(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.script_location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the Amazon Simple Storage Service (Amazon S3) path to a script that runs a job.</p>
    pub fn set_script_location(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.script_location = input;
        self
    }
    /// <p>The Python version being used to run a Python shell job. Allowed values are 2 or 3.</p>
    pub fn python_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.python_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Python version being used to run a Python shell job. Allowed values are 2 or 3.</p>
    pub fn set_python_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.python_version = input;
        self
    }
    /// Consumes the builder and constructs a [`JobCommand`](crate::types::JobCommand).
    pub fn build(self) -> crate::types::JobCommand {
        crate::types::JobCommand {
            name: self.name,
            script_location: self.script_location,
            python_version: self.python_version,
        }
    }
}
