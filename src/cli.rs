use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The mode for conversion
    #[arg(value_enum)]
    mode: Mode,
    /// Input temperature to convert
    #[arg(short, long, value_name = "TEMPERATURE")]
    temperature: f64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    K2C,
    C2K,
    F2C,
    C2F,
    F2K,
    K2F
}

fn convert(temperature: f64, mode: Mode) -> f64 {
    match mode {
        Mode::K2C => temperature - 273.15,
        Mode::C2K => temperature + 273.15,
        Mode::F2C => (temperature - 32.0) * 5.0/9.0,
        Mode::C2F => (temperature * 9.0/5.0) + 32.0,
        Mode::F2K => (temperature - 32.0) * 5.0/9.0 + 273.15,
        Mode::K2F => (temperature - 273.15) * 9.0/5.0 + 32.0,
    }
}

pub fn execute() {
    let cli = Cli::parse();

    let result = convert(cli.temperature, cli.mode);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}