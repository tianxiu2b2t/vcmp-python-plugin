use std::sync::OnceLock;
use tracing::{Level, event};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{
    fmt::{self, writer::MakeWriterExt},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// 用于持有 WorkerGuard, 让它别 Drop 了
///
/// 非常好 生命周期, 使我烦死
static LOG_FILE_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

const MAX_LEVEL: Level = Level::INFO; // 硬编码最大日志级别

pub fn init() {

    // 创建按天轮换的文件 appender
    let file_appender = {
        RollingFileAppender::builder()
            .filename_suffix("vcmp-rs.log") // 要后缀!
            .rotation(Rotation::DAILY)
            .build("logs")
            .expect("Failed to build log file writer.")
    };
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

    let _ = LOG_FILE_GUARD.set(guard);

    let file = non_blocking_file.with_max_level(MAX_LEVEL);
    let stdio = std::io::stdout.with_max_level(MAX_LEVEL);

    let stdout_layer = fmt::layer()
        .with_writer(stdio)
        .with_line_number(true)
        .with_thread_ids(true);

    let file_layer = fmt::layer().with_writer(file).with_ansi(false);

    // 初始化订阅者
    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    event!(Level::INFO, "tracing 日志设置完成");
}
