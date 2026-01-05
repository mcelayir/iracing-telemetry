use crate::provider::TelemetryFrame; // Import our shared data type
use colored::*;
use std::io::{self, Write};

pub fn render_gauge(frame: &TelemetryFrame) {
    // 1. Color Logic (Redline at 8000 RPM)
    let rpm_color = if frame.rpm > 8000.0 {
        format!("{:<5.0}", frame.rpm).red().bold()
    } else if frame.rpm > 7000.0 {
        format!("{:<5.0}", frame.rpm).yellow()
    } else {
        format!("{:<5.0}", frame.rpm).white()
    };

    // 2. Build the output string
    // \r moves cursor to start of line, print! keeps us on one line
    print!(
        "\r{} Gear: [{}] | RPM: {} | Speed: {:<5.1} km/h   ",
        "POC >>".bright_cyan(),
        frame.gear.to_string().green().bold(),
        rpm_color,
        frame.speed
    );

    // 3. Force the terminal to show the update immediately
    io::stdout().flush().unwrap();
}