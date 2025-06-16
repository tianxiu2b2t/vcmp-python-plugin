use pyo3::{
    Bound, Py, PyResult, Python, pyclass, pymethods,
    types::{
        PyAny, PyAnyMethods, PyByteArray, PyByteArrayMethods, PyBytes, PyBytesMethods, PyModule,
        PyModuleMethods,
    },
};
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read, Write};
use vcmp_bindings::encodes::{decode_gbk, encode_to_gbk};

use crate::py::bytes_repr;

#[pyclass]
struct WriteStream {
    buffer: Vec<u8>,
}

#[pymethods]
impl WriteStream {
    #[new]
    fn new() -> Self {
        WriteStream { buffer: Vec::new() }
    }

    fn __repr__(&self) -> String {
        format!("WriteStream({})", bytes_repr(self.get_raw_buffer()))
    }

    fn write_bytes(&mut self, py: Python<'_>, data: Py<PyAny>) -> PyResult<()> {
        let bound_data = data.bind(py);

        // 首先尝试处理 bytes 类型
        if let Ok(bytes) = bound_data.downcast::<PyBytes>() {
            self.buffer.extend_from_slice(bytes.as_bytes());
            return Ok(());
        }

        // 然后尝试处理 bytearray 类型
        if let Ok(bytearray) = bound_data.downcast::<PyByteArray>() {
            unsafe {
                self.buffer.extend_from_slice(bytearray.as_bytes());
            }
            return Ok(());
        }

        // 最后尝试处理 int 类型
        if let Ok(int_val) = bound_data.extract::<u8>() {
            self.buffer.push(int_val);
            return Ok(());
        }

        // 如果都不匹配，返回错误
        Err(pyo3::exceptions::PyTypeError::new_err(
            "data must be bytes, bytearray, or an integer (0-255)",
        ))
    }

    fn write_byte(&mut self, value: u8) -> PyResult<()> {
        self.buffer.write_all(&[value])?;
        Ok(())
    }

    fn write_int(&mut self, value: u32) -> PyResult<()> {
        let data = value.to_be_bytes();
        self.buffer.write_all(&data)?;
        Ok(())
    }

    fn write_long(&mut self, value: i64) -> PyResult<()> {
        let mut datum = (value << 1) ^ (value >> 63);
        loop {
            let mut byte = (datum & 0x7F) as u8;
            datum >>= 7;
            if datum != 0 {
                byte |= 0x80;
            }
            self.buffer.write_all(&[byte])?;
            if datum == 0 {
                break;
            }
        }
        Ok(())
    }

    fn write_sq_string(&mut self, value: &str) -> PyResult<()> {
        let binding = encode_to_gbk(value);
        let data = binding.as_ref();

        if data.len() > 4096 {
            self.buffer.write_all(&(4096i16).to_be_bytes()).unwrap();
            self.buffer.write_all(&data[0..4096])?;
            println!("String is too long, truncated to 4096 bytes");
        } else {
            self.buffer
                .write_all(&(data.len() as i16).to_be_bytes())
                .unwrap();
            self.buffer.write_all(data)?;
        }
        Ok(())
    }

    fn write_string(&mut self, value: &str) -> PyResult<()> {
        let binding = encode_to_gbk(value);
        let data = binding.as_ref();

        let length = data.len() as u64;
        self.write_long(length as i64)?;
        self.buffer.write_all(data)?;
        Ok(())
    }

    fn write_boolean(&mut self, value: bool) -> PyResult<()> {
        self.buffer.write_all(&[if value { 1 } else { 0 }])?;
        Ok(())
    }

    fn write_float(&mut self, value: f32) -> PyResult<()> {
        let data = value.to_be_bytes();
        self.buffer.write_all(&data)?;
        Ok(())
    }

    fn get_raw_buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }
}

#[pyclass]
struct ReadStream {
    buffer: Cursor<Vec<u8>>,
}

impl ReadStream {
    pub fn read(&mut self, length: usize) -> Vec<u8> {
        let mut buf = vec![0u8; length];
        let bytes_read = self.buffer.read(&mut buf).unwrap();
        buf.truncate(bytes_read);
        buf
    }
}

#[pymethods]
impl ReadStream {
    #[new]
    fn new(data: Vec<u8>) -> Self {
        ReadStream {
            buffer: Cursor::new(data),
        }
    }

    fn __repr__(&self) -> String {
        let binding = self.buffer.get_ref().to_vec();
        format!(
            "ReadStream(buffer={}, position={})",
            bytes_repr(binding),
            self.buffer.position()
        )
    }

    fn read_byte(&mut self) -> i8 {
        self.read(1)[0] as i8
    }

    fn read_bytes<'a>(&mut self, py: Python<'a>, length: usize) -> Bound<'a, PyBytes> {
        let buf = self.read(length);
        PyBytes::new(py, &buf)
    }
    // read_int
    fn read_i32(&mut self) -> i32 {
        let mut buf = [0u8; 4];
        self.buffer.read_exact(&mut buf).unwrap();
        i32::from_be_bytes(buf)
    }
    // var int avro encode
    fn read_i64(&mut self) -> i64 {
        let mut datum = 0i64;
        let mut shift = 0;
        loop {
            let mut buf = [0u8; 1];
            self.buffer.read_exact(&mut buf).unwrap();
            let byte = buf[0] as i64;
            datum |= (byte & 0x7F) << shift;
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        (datum >> 1) ^ -(datum & 1)
    }

    fn read_sq_string(&mut self) -> String {
        let mut buf = [0u8; 2];
        self.buffer.read_exact(&mut buf).unwrap();
        let length = i16::from_be_bytes(buf) as usize;
        let mut data = vec![0u8; length];
        self.buffer.read_exact(&mut data).unwrap();

        decode_gbk(&data)
    }

    fn read_string(&mut self) -> String {
        let length = self.read_i64() as usize;
        let mut data = vec![0u8; length];
        self.buffer.read_exact(&mut data).unwrap();

        decode_gbk(&data)
    }

    fn read_boolean(&mut self) -> bool {
        let mut buf = [0u8; 1];
        self.buffer.read_exact(&mut buf).unwrap();
        buf[0] != 0
    }

    fn read_float(&mut self) -> f32 {
        let mut buf = [0u8; 4];
        self.buffer.read_exact(&mut buf).unwrap();
        f32::from_be_bytes(buf)
    }

    fn get_raw_buffer(&self) -> Vec<u8> {
        self.buffer.get_ref().clone()
    }
}

impl Display for WriteStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WriteStream({})", bytes_repr(self.get_raw_buffer()))
    }
}

impl Display for ReadStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReadStream(buffer={}, position={})",
            bytes_repr(self.get_raw_buffer()),
            self.buffer.position()
        )
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WriteStream>()?;
    m.add_class::<ReadStream>()?;
    Ok(())
}
