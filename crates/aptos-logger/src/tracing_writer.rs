// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::Writer;
use once_cell::sync::OnceCell;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

static GLOBAL_WRITER: OnceCell<Arc<(Mutex<NonBlocking>, WorkerGuard)>> = OnceCell::new();

#[derive(Clone)]
pub struct TracingWriter {
    writer_guard: Arc<(Mutex<NonBlocking>, WorkerGuard)>,
}

impl TracingWriter {
    pub fn new(log_file: PathBuf, max_log_size: u64, max_rotated_logs: u32) -> Self {
        let writer_guard = GLOBAL_WRITER.get_or_init(|| {
            let file_appender =
                SizeRollingWriter::new(log_file, max_log_size, max_rotated_logs).unwrap();
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
            Arc::new((Mutex::new(non_blocking), guard))
        });

        Self {
            writer_guard: writer_guard.clone(),
        }
    }
}

impl Writer for TracingWriter {
    /// Write to file
    fn write(&self, log: String) {
        let (writer_mutex, _guard) = &*self.writer_guard;
        if let Ok(mut writer) = writer_mutex.lock() {
            if let Err(err) = writeln!(writer, "{}", log) {
                eprintln!("Unable to write to log file: {}", err);
            }
        }
    }

    fn write_buferred(&mut self, log: String) {
        self.write(log);
    }
}

struct SizeRollingWriter {
    path: PathBuf,
    file: Option<File>,
    max_size: u64,
    max_files: u32,
    current_size: u64,
}

impl SizeRollingWriter {
    fn new(path: PathBuf, max_size: u64, max_files: u32) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        let current_size = file.metadata()?.len();

        Ok(Self {
            path,
            file: Some(file),
            max_size,
            max_files,
            current_size,
        })
    }

    fn open_file(&mut self) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        self.current_size = file.metadata()?.len();
        self.file = Some(file);
        Ok(())
    }

    fn rotate(&mut self) -> io::Result<()> {
        if let Some(file) = self.file.take() {
            drop(file);
        }

        if self.max_files == 0 {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.path)?;
            self.file = Some(file);
            self.current_size = 0;
            return Ok(());
        }

        let log_path_str = self.path.to_string_lossy();
        let oldest_log = format!("{}.{}", log_path_str, self.max_files);
        if Path::new(&oldest_log).exists() {
            fs::remove_file(oldest_log)?;
        }
        for i in (1..self.max_files).rev() {
            let from = format!("{}.{}", log_path_str, i);
            let to = format!("{}.{}", log_path_str, i + 1);
            if Path::new(&from).exists() {
                fs::rename(from, to)?;
            }
        }
        let to = format!("{}.1", log_path_str);
        if self.path.exists() {
            fs::rename(&self.path, to)?;
        }

        self.open_file()?;
        Ok(())
    }
}

impl Write for SizeRollingWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.file.is_none() {
            self.open_file()?;
        }
        if self.current_size + (buf.len() as u64) > self.max_size {
            self.rotate()?;
        }
        if let Some(file) = self.file.as_mut() {
            let written = file.write(buf)?;
            self.current_size += written as u64;
            Ok(written)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "File not open for writing",
            ))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(file) = self.file.as_mut() {
            file.flush()
        } else {
            Ok(())
        }
    }
}
