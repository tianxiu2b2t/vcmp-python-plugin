pub struct ServerInitialiseEvent;

impl From<()> for ServerInitialiseEvent {
    fn from(_: ()) -> Self {
        Self {}
    }
}

pub struct ServerShutdownEvent;

impl From<()> for ServerShutdownEvent {
    fn from(_: ()) -> Self {
        Self {}
    }
}

pub struct ServerFrameEvent {
    pub elapsed_time: f32,
}

impl From<f32> for ServerFrameEvent {
    fn from(value: f32) -> Self {
        Self {
            elapsed_time: value,
        }
    }
}

pub struct ServerPerformanceReportEvent {
    pub entry_count: usize,
    pub descriptions: Vec<String>,
    pub times: Vec<u64>,
}

impl From<(usize, *mut *const ::std::os::raw::c_char, *mut u64)> for ServerPerformanceReportEvent {
    fn from(value: (usize, *mut *const ::std::os::raw::c_char, *mut u64)) -> Self {
        let entry_count = value.0;
        let descriptions_ptr = value.1;
        let times_ptr = value.2;

        let descriptions: Vec<String> = unsafe {
            std::slice::from_raw_parts(descriptions_ptr, entry_count)
                .iter()
                .filter_map(|&desc_ptr| {
                    if desc_ptr.is_null() {
                        None
                    } else {
                        Some(
                            std::ffi::CStr::from_ptr(desc_ptr)
                                .to_string_lossy()
                                .to_string(),
                        )
                    }
                })
                .collect()
        };

        // 将时间数据指针数组转换为 Vec<u64>
        let times: Vec<u64> = unsafe {
            std::slice::from_raw_parts(times_ptr, entry_count)
                .iter()
                .cloned()
                .collect()
        };

        Self {
            entry_count,
            descriptions,
            times,
        }
    }
}
