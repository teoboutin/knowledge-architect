//! A process of the test's own that is running, for a record to name as its owner.

use std::process::{Child, Command, Stdio};

/// A process that runs until this is dropped: it waits on a stdin this process holds, so it ends
/// with the test even where the test is killed.
pub struct Live(Child);

impl Live {
    /// Starts it.
    ///
    /// # Panics
    ///
    /// Where it cannot be started.
    pub fn start() -> Self {
        Live(
            Command::new("cat")
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .spawn()
                .expect("a process that waits"),
        )
    }

    /// Its pid.
    pub fn pid(&self) -> u32 {
        self.0.id()
    }
}

impl Drop for Live {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
