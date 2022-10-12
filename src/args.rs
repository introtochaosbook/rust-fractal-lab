use clap::{ArgGroup, Parser, ValueEnum};

#[derive(Clone, Copy, ValueEnum, strum_macros::Display)]
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

impl Default for JuliaFunction {
    fn default() -> Self {
        Self::Rabbit
    }
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
