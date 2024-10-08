//! Read from and write to [Arrow IPC](https://arrow.apache.org/docs/format/Columnar.html#format-ipc) files.

mod reader;
mod writer;

pub use reader::{read_ipc, read_ipc_stream};
pub use writer::{write_ipc, write_ipc_stream};
