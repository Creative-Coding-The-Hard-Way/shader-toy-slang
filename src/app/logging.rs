use {
    anyhow::Result,
    flexi_logger::{
        Criterion, DeferredNow, Duplicate, FileSpec, Logger, LoggerHandle,
        Naming, Record, WriteMode,
    },
    regex::Regex,
    std::{
        fmt::Write as FmtWrite,
        path::{Path, PathBuf},
        sync::LazyLock,
    },
    textwrap::{termwidth, Options},
};

// Used to remove verbose base dir prefix from log messages with file
// information.
static RELATIVE_ROOT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::current_dir().unwrap());

//::new(r"(┃)(.*)$").unwrap();
static LAST_NEWLINE_DELIM_MACHER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(┃)(.*)$").unwrap());

/// A global handle to the initialized flexi_logger.
///
/// This gets setup on the first call to setup_logger().
static LOGGER_HANDLE: LazyLock<LoggerHandle> = LazyLock::new(|| {
    Logger::try_with_env_or_str("trace")
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs"))
        .rotate(
            Criterion::AgeOrSize(flexi_logger::Age::Hour, 1024 * 1024 * 8),
            Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(3),
        )
        .format(multiline_format)
        .duplicate_to_stdout(Duplicate::Info)
        .write_mode(WriteMode::Async)
        .start()
        .expect("Unable to start the logger!")
});

/// Initialize the multiline logger.
pub fn setup() {
    LOGGER_HANDLE.flush();
}

/// A multiline log format for flexi_logger.
///
/// Logs are automatically wrapped at terminal width and prefixed with unicode
/// so it's easy to tell where a big log statement begins and ends.
fn multiline_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let size = termwidth().min(74);
    let wrap_options = Options::new(size)
        .initial_indent("┏ ")
        .subsequent_indent("┃ ");

    let file = Path::new(record.file().unwrap());
    let file_message = if let Ok(relative_path) =
        file.strip_prefix(RELATIVE_ROOT_DIR.as_path())
    {
        relative_path.to_str().unwrap().to_owned()
    } else {
        file.to_str().unwrap().to_owned()
    };

    let mut full_line = String::new();
    writeln!(
        full_line,
        "{} [{}] [{}:{}]",
        record.level(),
        now.now().format("%H:%M:%S%.3f"),
        file_message,
        record.line().unwrap_or(0),
    )
    .expect("unable to format first log line");

    write!(&mut full_line, "{}", &record.args())
        .expect("unable to format log!");

    let wrapped = textwrap::fill(&full_line, wrap_options);
    let formatted = LAST_NEWLINE_DELIM_MACHER.replace(&wrapped, "┗$2");

    writeln!(w, "{formatted}")
}
