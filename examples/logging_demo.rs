use chrono::{DateTime, Utc};
use env_logger::Builder;
use log::Level;
use logging_timer::{executing, finish, stimer, timer};
use std::io::Write;

/// Demonstrates the various timer macros.
///
/// To run in Linux, do:
///     RUST_LOG=debug cargo run --example logging_demo
///
/// To run in PowerShell, do:
///     $env:RUST_LOG="debug"
///     cargo run --example logging_demo

fn main() {
    configure_logging();

    timer_with_name_only();

    println!("");
    stimer_with_name_only();

    println!("");
    stimer_with_intermediate_messages_and_final_message();

    println!("");
    stimer_with_intermediate_messages_and_no_automatic_final_message();

    println!("");
    timer_with_custom_log_level();

    println!("");
    stimer_with_args();

    println!("");
    executing_with_args();

    println!("");
    finish_with_args();
}

// Section 1. Basic operation of all macros.
fn timer_with_name_only() {
    let _tmr = timer!("NAMED_TIMER");
}

fn stimer_with_name_only() {
    let _tmr = stimer!("NAMED_S_TIMER");
}

fn stimer_with_intermediate_messages_and_final_message() {
    let tmr = stimer!("S_TIMER_INTER_FINAL");
    executing!(tmr, "Stuff is happening");
    executing!(tmr, "More stuff is happening");
}

fn stimer_with_intermediate_messages_and_no_automatic_final_message() {
    let tmr = stimer!("S_TIMER_INTER_NOFINAL");
    executing!(tmr, "Stuff is happening");
    executing!(tmr, "More stuff is happening");
    finish!(tmr, "All done. Frobbed 5 wuidgets.");
}

// Section 2. Changing the log level.
fn timer_with_custom_log_level() {
    let _tmr = timer!("CUSTOM_LOG_LEVEL").level(Level::Warn);
}

// Section 3. Using format args.
fn stimer_with_args() {
    let _tmr = stimer!("FORMATTED_S_TIMER", "extra info");
    let _tmr2 = stimer!("FORMATTED_S_TIMER", "extra info: {} widgets", 5);
}

fn executing_with_args() {
    let tmr = stimer!("EXEC_WITH_ARGS", "Expecting to process {} widgets", 20);
    executing!(tmr, "More info: Processed {} widgets", 5);
    executing!(tmr, "More info: Processed {} widgets", 10);
}

fn finish_with_args() {
    let tmr = stimer!("FINISH_WITH_ARGS", "Expecting to process {} widgets", 20);
    executing!(tmr, "More info: Processed {} widgets", 10);
    executing!(tmr, "More info: Processed {} widgets", 20);
    finish!(tmr, "Done. Processed {} widgets", 20);
}

// Just configures logging in such a way that we can see everything.
fn configure_logging() {
    let mut builder = Builder::from_default_env();
    builder.format(|buf, record| {
        let utc: DateTime<Utc> = Utc::now();

        write!(
            buf,
            "{:?} {} [{}] ",
            //utc.format("%Y-%m-%dT%H:%M:%S.%fZ"),
            utc, // same, probably faster?
            record.level(),
            record.target()
        )?;

        match (record.file(), record.line()) {
            (Some(file), Some(line)) => write!(buf, "[{}/{}] ", file, line),
            (Some(file), None) => write!(buf, "[{}] ", file),
            (None, Some(_line)) => write!(buf, " "),
            (None, None) => write!(buf, " "),
        }?;

        writeln!(buf, "{}", record.args())
    });

    builder.init();
}
