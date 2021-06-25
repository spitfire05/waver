use core::{f32::consts::PI, fmt::Display};

use libm::{fabsf, floorf, sinf};
use num_traits::float::FloatCore;

pub trait Function: Display {
    fn sample(&self, t: f32) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct Sine {
    /// The frequency of this sine wave.
    frequency: f32,

    /// The phase of this sine wave.
    phase: f32,
}

impl Sine {
    pub fn new(frequency: f32, phase: f32) -> Self {
        Self { frequency, phase }
    }

    /// Get a reference to the sine function's frequency.
    pub fn frequency(&self) -> &f32 {
        &self.frequency
    }

    /// Get a reference to the sine function's phase.
    pub fn phase(&self) -> &f32 {
        &self.phase
    }
}

impl Function for Sine {
    fn sample(&self, t: f32) -> f32 {
        sinf(2.0 * PI * t * self.frequency + self.phase)
    }
}

impl Display for Sine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[Sine] Freq: {}, Phase: {}", self.frequency, self.phase)
    }
}

impl Default for Sine {
    fn default() -> Self {
        Sine {
            frequency: 0.0,
            phase: 0.0,
        }
    }
}

pub struct Rectangular {
    frequency: f32,
    phase: f32,
    duty_cycle: f32
}

impl Rectangular {
    pub fn new(frequency: f32, phase: f32, duty_cycle: f32) -> Self { Self { frequency, phase, duty_cycle } }
}

impl Function for Rectangular {
    fn sample(&self, t: f32) -> f32 {
        (sinf(2.0 * PI * t * self.frequency + self.phase) + self.duty_cycle - 0.5 ).signum()
    }
}

impl Display for Rectangular {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[Rectangular] Freq: {}, Phase: {}, DutyCycle: {}", self.frequency, self.phase, self.duty_cycle)
    }
}

pub struct Triangle {
    frequency: f32,
    // TODO: add phase suport
}

impl Triangle {
    pub fn new(frequency: f32) -> Self { Self { frequency } }
}

impl Function for Triangle {
    fn sample(&self, t: f32) -> f32 {
        2.0 * fabsf( (t * self.frequency - 0.5) - 2.0 * floorf( (t * self.frequency - 0.5) / 2.0) - 1.0 ) - 1.0
        // 1f-4f*(float)Math.Abs( Math.Round(t-0.25f)-(t-0.25f) );
        // 1.0 - 4.0 * fabsf(floorf((t * self.frequency) - 0.25) - ((t * self.frequency) - 0.25))
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[Triangle] Freq: {}", self.frequency)
    }
}
