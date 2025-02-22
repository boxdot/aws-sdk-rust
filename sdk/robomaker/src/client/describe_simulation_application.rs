// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeSimulationApplication`](crate::operation::describe_simulation_application::builders::DescribeSimulationApplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application(impl ::std::convert::Into<String>)`](crate::operation::describe_simulation_application::builders::DescribeSimulationApplicationFluentBuilder::application) / [`set_application(Option<String>)`](crate::operation::describe_simulation_application::builders::DescribeSimulationApplicationFluentBuilder::set_application): <p>The application information for the simulation application.</p>
    ///   - [`application_version(impl ::std::convert::Into<String>)`](crate::operation::describe_simulation_application::builders::DescribeSimulationApplicationFluentBuilder::application_version) / [`set_application_version(Option<String>)`](crate::operation::describe_simulation_application::builders::DescribeSimulationApplicationFluentBuilder::set_application_version): <p>The version of the simulation application to describe.</p>
    /// - On success, responds with [`DescribeSimulationApplicationOutput`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::arn): <p>The Amazon Resource Name (ARN) of the robot simulation application.</p>
    ///   - [`name(Option<String>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::name): <p>The name of the simulation application.</p>
    ///   - [`version(Option<String>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::version): <p>The version of the simulation application.</p>
    ///   - [`sources(Option<Vec<Source>>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::sources): <p>The sources of the simulation application.</p>
    ///   - [`simulation_software_suite(Option<SimulationSoftwareSuite>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::simulation_software_suite): <p>The simulation software suite used by the simulation application.</p>
    ///   - [`robot_software_suite(Option<RobotSoftwareSuite>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::robot_software_suite): <p>Information about the robot software suite (ROS distribution).</p>
    ///   - [`rendering_engine(Option<RenderingEngine>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::rendering_engine): <p>The rendering engine for the simulation application.</p>
    ///   - [`revision_id(Option<String>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::revision_id): <p>The revision id of the simulation application.</p>
    ///   - [`last_updated_at(Option<DateTime>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::last_updated_at): <p>The time, in milliseconds since the epoch, when the simulation application was last updated.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::tags): <p>The list of all tags added to the specified simulation application.</p>
    ///   - [`environment(Option<Environment>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::environment): <p>The object that contains the Docker image URI used to create the simulation application.</p>
    ///   - [`image_digest(Option<String>)`](crate::operation::describe_simulation_application::DescribeSimulationApplicationOutput::image_digest): <p>A SHA256 identifier for the Docker image that you use for your simulation application.</p>
    /// - On failure, responds with [`SdkError<DescribeSimulationApplicationError>`](crate::operation::describe_simulation_application::DescribeSimulationApplicationError)
    pub fn describe_simulation_application(&self) -> crate::operation::describe_simulation_application::builders::DescribeSimulationApplicationFluentBuilder{
        crate::operation::describe_simulation_application::builders::DescribeSimulationApplicationFluentBuilder::new(self.handle.clone())
    }
}
