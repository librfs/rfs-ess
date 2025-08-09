// ess/src/config.rs
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Canmi

pub fn generate_default_config() -> &'static str {
    r#"[common]
log_level = "info"

[rfsd]
unix_socket = "/opt/rfs/rfsd/rfsd.sock"
"#
}