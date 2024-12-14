use argh::FromArgs;

#[derive(FromArgs)]
/// Lox interpreter
pub struct Args {
    /// script file to run
    #[argh(positional)]
    pub script: Option<String>,
}

pub fn parse_args() -> Args {
    argh::from_env()
}
