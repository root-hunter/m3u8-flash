use std::process::Command;
use std::error::Error;

pub fn convert_ts_to_mp4(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new("ffmpeg")
        .args(&["-i", input, "-c:v", "copy", "-c:a", "copy", output])
        .status()?;

    if status.success() {
        println!("Conversione completata con successo: {} -> {}", input, output);
    } else {
        println!("Errore nella conversione.");
    }

    Ok(())
}
