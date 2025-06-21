use tracing::Level;
use tracing_subscriber::{
    fmt::{
        format::{FmtContext, Writer},
        FmtSpan, FormatEvent, FormatFields,
    },
    registry::LookupSpan,
};
use tracing_subscriber::field::Visit;
use std::fmt::Write;
use chrono::Local;
use colored::Colorize;

pub struct CustomFormat;

impl<S, N, T> FormatEvent<S, N, T> for CustomFormat
where
    S: LookupSpan<N, T>,
    N: for<'lookup> LookupSpan<'lookup>,
    T: FormatFields<N>,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N, T>,
        mut writer: Writer<'_, N, T>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        // ✅ 获取时间
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        write!(writer, "[{}] ", timestamp.green())?;

        // ✅ 获取日志级别并根据级别设置颜色
        let level = event.metadata().level();
        let level_str = match *level {
            Level::TRACE => level.to_string().cyan(),
            Level::DEBUG => level.to_string().blue(),
            Level::INFO => level.to_string().green(),
            Level::WARN => level.to_string().yellow(),
            Level::ERROR => level.to_string().red(),
        };
        write!(writer, "[{}] ", level_str)?;

        // ✅ 获取模块名、函数名、行号
        let target = event.metadata().target();
        let file = event.metadata().file().unwrap_or("unknown");
        let line = event.metadata().line().unwrap_or(0);

        // ✅ 获取函数名（需要显式记录）
        let mut function = None;
        struct FunctionVisitor<'a>(&'a mut Option<String>);
        impl Visit for FunctionVisitor<'_> {
            fn record_debug(&mut self, field: &tracing::Field, value: &dyn std::fmt::Debug) {
                if field.name() == "function" {
                    if let Some(s) = value.downcast_ref::<&str>() {
                        self.0.replace((*s).to_string());
                    } else {
                        let s = format!("{:?}", value);
                        self.0.replace(s);
                    }
                }
            }
        }
        event.record(&mut FunctionVisitor(&mut function));
        let function = function.unwrap_or_else(|| "unknown".to_string());

        // ✅ 构建位置信息
        let loc = format!("{}:{}:{}", target, function, line);
        write!(writer, "[{}] ", loc.yellow())?;

        // ✅ 获取消息内容
        let mut visitor = FormatVisitor::default();
        event.record(&mut visitor);
        write!(writer, "{}", visitor.message.yellow())?;

        writeln!(writer)
    }
}

#[derive(Default)]
struct FormatVisitor {
    message: String,
}

impl Visit for FormatVisitor {
    fn record_debug(&mut self, field: &tracing::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message.push_str(&format!("{:?}", value));
        }
    }
}