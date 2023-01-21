use clap::Parser;
use miette::{IntoDiagnostic, Result, Diagnostic};
use thiserror::Error;
use totp_rs::{Secret, TOTP, SecretParseError};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// TOTP secret
    #[arg(short, long)]
    secret: String,

    /// Host of the machine running XLCore (default: localhost)
    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,

    /// Port of the machine running XLCore (default: 4646)
    #[arg(short, long, default_value = "4646")]
    port: u16,

    /// Show the OTP instead of sending it
    #[arg(short = 'S', long)]
    show: bool,
}

#[derive(Error, Diagnostic, Debug)]
enum AppError {
    #[error("Unable to parse secret: {:?}", .0)]
    SecretParse(SecretParseError),
    #[error("Time difference is too large: {:?}", .0)]
    TimeDiff(#[source] std::time::SystemTimeError),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();
    let totp = TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(args.secret).to_bytes().map_err(AppError::SecretParse)?,
    ).into_diagnostic()?;

    if !args.show {
        let url = format!("http://{}:{}/ffxivlauncher/{}", args.host, args.port, totp.generate_current().map_err(AppError::TimeDiff)?);
        reqwest::get(&url)
            .await
            .into_diagnostic()?
            .error_for_status()
            .into_diagnostic()?;
    } else {
        println!("{}", totp.generate_current().map_err(AppError::TimeDiff)?);
    }
    
    Ok(())
}