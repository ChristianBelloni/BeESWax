use beeswax_types::{build_event_stream::{BuildEvent as BazelBuildEvent, BuildFinished, BuildMetrics, BuildStarted, BuildToolLogs, NamedSetOfFiles, TargetSummary, TestResult, TestSummary}, command_line::CommandLine};
use tokio::task::AbortHandle;

#[derive(Default)]
pub struct BuildHandler {
    identifier: String,
    started: Option<BuildStarted>,
    metrics: Option<BuildMetrics>,
    finished: Option<BuildFinished>,
    test_result: Option<TestResult>,
    command_line: Vec<CommandLine>,
    build_tool_logs: Option<BuildToolLogs>,
    test_summary: Option<TestSummary>,
    stdout: String,
    stderr: String,
    named_files: Option<NamedSetOfFiles>,
    target_summary: Option<TargetSummary>,

    abort_handle: Option<AbortHandle>
}

impl Drop for BuildHandler {
    fn drop(&mut self) {
        if let Some(ref handle) = self.abort_handle {
            tracing::info!("dropping handler {}", self.identifier);
            handle.abort()
        }
    }
}

impl BuildHandler {
    pub fn new(identifier: String) -> Self {
        let mut this = Self::default();
        this.identifier = identifier;
        this
    }

    pub fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
        self.abort_handle = Some(abort_handle);
    }

    pub fn handle_event(&mut self, event: BazelBuildEvent) {
        match event.payload.unwrap() {
            beeswax_types::build_event_stream::build_event::Payload::Started(started) => {
                self.started = Some(started);
            },
            beeswax_types::build_event_stream::build_event::Payload::Progress(progress) => {
                self.stdout.push_str(&progress.stdout);
                self.stderr.push_str(&progress.stderr);
            },
            beeswax_types::build_event_stream::build_event::Payload::StructuredCommandLine(cmdl) => {
                tracing::info!("storing command line");
                self.command_line.push(cmdl);
            },

            beeswax_types::build_event_stream::build_event::Payload::NamedSetOfFiles(files) => {
                tracing::info!("storing named set of files");
                self.named_files = Some(files);
            },
            beeswax_types::build_event_stream::build_event::Payload::TestResult(result) => {
                tracing::info!("storing test results");
                self.test_result = Some(result);
            },
            beeswax_types::build_event_stream::build_event::Payload::TestProgress(_progress) => {
                
            },
            beeswax_types::build_event_stream::build_event::Payload::TestSummary(summary) => {
                tracing::info!("storing test summary");
                self.test_summary = Some(summary);
            },
            beeswax_types::build_event_stream::build_event::Payload::Finished(finished) => {
                tracing::info!("storing finished data");
                self.finished = Some(finished);
            },
            beeswax_types::build_event_stream::build_event::Payload::BuildToolLogs(logs) => {
                tracing::info!("storing build tool logs");
                self.build_tool_logs = Some(logs);
            },
            beeswax_types::build_event_stream::build_event::Payload::BuildMetrics(metrics) => {
                tracing::info!("storing build metrics");
                self.metrics = Some(metrics);
            },
            beeswax_types::build_event_stream::build_event::Payload::TargetSummary(summary) => {
                tracing::info!("storing target summary");
                self.target_summary = Some(summary);
            },
            beeswax_types::build_event_stream::build_event::Payload::WorkspaceInfo(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::Aborted(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::UnstructuredCommandLine(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::OptionsParsed(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::WorkspaceStatus(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::Fetch(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::Configuration(_conf) => {},
            beeswax_types::build_event_stream::build_event::Payload::Expanded(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::Configured(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::Action(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::Completed(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::BuildMetadata(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::ConvenienceSymlinksIdentified(_) => {},
            beeswax_types::build_event_stream::build_event::Payload::ExecRequest(_) => {},
        }
    }
}
