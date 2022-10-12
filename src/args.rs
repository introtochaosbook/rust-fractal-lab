use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct JuliaArgs {
    #[arg(value_enum)]
    pub function: JuliaFunction,

    #[arg(value_enum, default_value_t = ColorScheme::Turbo, short, long)]
    pub color_scheme: ColorScheme,
}

#[derive(Clone, ValueEnum, strum_macros::Display)]
pub enum JuliaFunction {
    Cos,
    Sin,
    Rabbit,
    Siegel,
    Dragon,
    Amoeba,
    Flower1,
    Flower2,
    Cloud,
    Snowflakes,
    Dendrite,
    Ekg,
}

impl JuliaFunction {
    pub fn subroutine_name(&self) -> String {
        format!("F{}", self.to_string())
    }
}

#[derive(Clone, ValueEnum, strum_macros::Display)]
pub enum ColorScheme {
    Inferno,
    Viridis,
    Plasma,
    Magma,
    Turbo,
}

impl ColorScheme {
    pub fn subroutine_name(&self) -> String {
        format!("ColorMap{}", self.to_string())
    }
}