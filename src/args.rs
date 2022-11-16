use clap::ValueEnum;
use strum_macros::EnumVariantNames;

#[derive(Clone, Copy, ValueEnum, EnumVariantNames, strum_macros::Display)]
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
        format!("F{}", self)
    }
}

#[derive(Clone, ValueEnum, EnumVariantNames, strum_macros::Display)]
pub enum ColorScheme {
    Inferno,
    Viridis,
    Plasma,
    Magma,
    Turbo,
}

impl ColorScheme {
    pub fn subroutine_name(&self) -> String {
        format!("ColorMap{}", self)
    }
}
