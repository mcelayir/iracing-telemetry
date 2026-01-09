use crate::provider::SimState; // Now importing the full composite state
use colored::*;
use std::io::{self, Write};

pub fn render_gauge(state: &SimState) {
    // 1. Extract Sub-frames for readability
    let dash = &state.dashboard;
    let tel = &state.telemetry;
    let race = &state.race;

    // 2. Formatting Dashboard Data
    let gear_display = match dash.gear {
        0 => "N".yellow().bold(),
        -1 => "R".red().bold(),
        g => g.to_string().green().bold(),
    };

    let rpm_color = if dash.rpm > 8000.0 {
        format!("{:<5.0}", dash.rpm).on_red().white().bold()
    } else {
        format!("{:<5.0}", dash.rpm).white()
    };

    // 3. Formatting Telemetry (Inputs) & Race Context
    // We use a small visual bar for Throttle/Brake to make it easy to verify
    let throttle_pct = (tel.throttle * 10.0) as usize;
    let brake_pct = (tel.brake * 10.0) as usize;

    // 4. Final Composite Output
    // \r ensures we overwrite the same line for a "live" dashboard feel
    print!(
        "\r{prefix} {gear} | {rpm} | {speed}km/h | T:{thr} B:{brk} | Pos:{pos} Temp:{temp}°C SoF:{sof}          ",
        prefix = "DEBUG >>".bright_magenta(),
        gear = format!("G:[{}]", gear_display),
        rpm = format!("RPM:{}", rpm_color),
        speed = format!("{:<5.1}", dash.speed),
        thr = format!("{:_<10}", "I".repeat(throttle_pct)).green(),
        brk = format!("{:_<10}", "I".repeat(brake_pct)).red(),
        pos = race.position.to_string().cyan(),
        temp = format!("{:.1}", race.track_temp).yellow(),
        sof = race.sof.to_string().blue(),
    );

    io::stdout().flush().unwrap();
}