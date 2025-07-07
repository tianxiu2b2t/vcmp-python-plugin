use std::io;
use tracing::{event, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt::{self, writer::MakeWriterExt}, prelude::*};

pub fn init() {
    let max_level = Level::DEBUG; // 硬编码最大日志级别

    // 创建按天轮换的文件 appender
    let file_appender = RollingFileAppender::new(Rotation::HOURLY, "logs", "app.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);

    let file = non_blocking_file.with_max_level(max_level);

    let stdio = std::io::stdout.with_max_level(max_level);

    // 控制台输出层 - 使用硬编码的最大级别
    let stdout_layer = fmt::layer()
        .with_writer(stdio)
        .with_line_number(true)
        .with_thread_ids(true);

    // // 文件输出层 - 使用硬编码的最大级别
    let file_layer = fmt::layer().with_writer(file).with_ansi(false);

    // 初始化订阅者
    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    event!(Level::WARN, "tracing 设置完成");
}
