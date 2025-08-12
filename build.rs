// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
// SPDX-License-Identifier: BSD-3-Clause
use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["describe", "--tags", "--dirty", "--always"])
        .output()
        .expect("Failed to execute git");

    let version = String::from_utf8(output.stdout).unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_VERSION={}", version.trim());
}
