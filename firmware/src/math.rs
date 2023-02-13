use micromath::F32Ext;

pub type PhasePwmVector = [f32; 3];
pub type ParkVector = [f32; 2];
pub type ClarkeVector = [f32; 2];

trait PhasePwmVectorExt {
    fn new() -> Self;
    fn as_clarke(&self) -> ClarkeVector;
}

impl PhasePwmVectorExt for PhasePwmVector {
    fn new() -> Self {
        [0.0, 0.0, 0.0]
    }

    fn as_clarke(&self) -> ClarkeVector {
        let a = self[0];
        let b = self[1];
        let c = self[2];

        let alpha = a;
        let beta = (b - c) / (3.0f32).sqrt();

        [alpha, beta]
    }
}

trait ClarkeVectorExt {
    fn new() -> Self;
    fn as_phase_pwm(&self) -> PhasePwmVector;
}

impl ClarkeVectorExt for ClarkeVector {
    fn new() -> Self {
        [0.0, 0.0]
    }

    fn as_phase_pwm(&self) -> PhasePwmVector {
        let alpha = self[0];
        let beta = self[1];

        let a = alpha;
        let b = -0.5 * alpha + (3.0f32).sqrt() / 2.0 * beta;
        let c = -0.5 * alpha - (3.0f32).sqrt() / 2.0 * beta;

        [a, b, c]
    }
}
