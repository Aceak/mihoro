use std::process::{Command, ExitStatus};

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemdScope {
    User,
    System,
}

pub struct Systemctl {
    systemctl: Command,
    scope: SystemdScope,
}

impl Systemctl {
    pub fn with_scope(scope: SystemdScope) -> Self {
        Self {
            systemctl: Command::new("systemctl"),
            scope,
        }
    }

    fn add_scope_arg(&mut self) {
        if matches!(self.scope, SystemdScope::User) {
            self.systemctl.arg("--user");
        }
    }

    pub fn enable(&mut self, service: &str) -> &mut Self {
        self.add_scope_arg();
        self.systemctl.arg("enable").arg(service);
        self
    }

    pub fn start(&mut self, service: &str) -> &mut Self {
        self.add_scope_arg();
        self.systemctl.arg("start").arg(service);
        self
    }

    pub fn stop(&mut self, service: &str) -> &mut Self {
        self.add_scope_arg();
        self.systemctl.arg("stop").arg(service);
        self
    }

    pub fn restart(&mut self, service: &str) -> &mut Self {
        self.add_scope_arg();
        self.systemctl.arg("restart").arg(service);
        self
    }

    pub fn status(&mut self, service: &str) -> &mut Self {
        self.add_scope_arg();
        self.systemctl.arg("status").arg(service);
        self
    }

    pub fn disable(&mut self, service: &str) -> &mut Self {
        self.add_scope_arg();
        self.systemctl.arg("disable").arg(service);
        self
    }

    pub fn daemon_reload(&mut self) -> &mut Self {
        self.add_scope_arg();
        self.systemctl.arg("daemon-reload");
        self
    }

    pub fn reset_failed(&mut self) -> &mut Self {
        self.add_scope_arg();
        self.systemctl.arg("reset-failed");
        self
    }

    pub fn execute(&mut self) -> Result<ExitStatus> {
        self.systemctl
            .spawn()?
            .wait()
            .with_context(|| "failed to execute systemctl")
    }

    /// Returns `true` if the given service is currently active.
    pub fn is_active(&self, service: &str) -> bool {
        let mut cmd = Command::new("systemctl");
        if matches!(self.scope, SystemdScope::User) {
            cmd.arg("--user");
        }
        cmd.arg("is-active")
            .arg("--quiet")
            .arg(service)
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Returns `true` if the given service is enabled for autostart.
    pub fn is_enabled(&self, service: &str) -> bool {
        let mut cmd = Command::new("systemctl");
        if matches!(self.scope, SystemdScope::User) {
            cmd.arg("--user");
        }
        cmd.arg("is-enabled")
            .arg("--quiet")
            .arg(service)
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}
