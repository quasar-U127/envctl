use clap::Parser;
use

#[derive(ArgEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum Driver{
    Mamba
}

#[derive(Parser,Default,Debug)]
struct Arguments {
    driver:D
}
