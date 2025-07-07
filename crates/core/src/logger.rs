use std::io;
use tracing::{Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, prelude::*};

pub fn init() {
    let max_level = Level::TRACE; // 硬编码最大日志级别
    
    // 创建按天轮换的文件 appender
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "app.log"
    );
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
    
    // 控制台输出层 - 使用硬编码的最大级别
    let stdout_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_line_number(true)
        .with_thread_ids(true);
    
    // 文件输出层 - 使用硬编码的最大级别
    let file_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_ansi(false);
    
    // 初始化订阅者
    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();
}