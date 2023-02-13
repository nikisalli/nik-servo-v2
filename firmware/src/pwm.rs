use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;
use micromath::F32Ext;
use rp_pico::hal::gpio::bank0::BankPinId;
use rp_pico::hal::pwm::{Channel, FreeRunning, ValidPwmOutputPin};
use rp_pico::hal::{
    gpio::{Output, Pin, PinId, PushPull},
    pwm::{ChannelId, SliceId},
};

use crate::math::PhasePwmVector;

pub struct PwmManagerPhase<S, C, P, I>
where
    S: SliceId,
    C: ChannelId,
    P: PinId,
    I: PinId + ValidPwmOutputPin<S, C>,
    Channel<S, FreeRunning, C>: PwmPin,
{
    pwm: Channel<S, FreeRunning, C>,
    pwm_pin: Pin<I, Output<PushPull>>,
    dir: Pin<P, Output<PushPull>>,
}

// with 3 phases
pub struct PwmManager<S0, S1, S2, C0, C1, C2, P0, P1, P2, I0, I1, I2>
where
    S0: SliceId,
    S1: SliceId,
    S2: SliceId,
    C0: ChannelId,
    C1: ChannelId,
    C2: ChannelId,
    P0: PinId,
    P1: PinId,
    P2: PinId,
    I0: PinId + ValidPwmOutputPin<S0, C0> + BankPinId,
    I1: PinId + ValidPwmOutputPin<S1, C1> + BankPinId,
    I2: PinId + ValidPwmOutputPin<S2, C2> + BankPinId,
    Channel<S0, FreeRunning, C0>: PwmPin,
    Channel<S1, FreeRunning, C1>: PwmPin,
    Channel<S2, FreeRunning, C2>: PwmPin,
{
    pwm0: PwmManagerPhase<S0, C0, P0, I0>,
    pwm1: PwmManagerPhase<S1, C1, P1, I1>,
    pwm2: PwmManagerPhase<S2, C2, P2, I2>,
}

impl<
        S0: SliceId,
        S1: SliceId,
        S2: SliceId,
        C0: ChannelId,
        C1: ChannelId,
        C2: ChannelId,
        P0: PinId,
        P1: PinId,
        P2: PinId,
        I0: PinId + ValidPwmOutputPin<S0, C0> + BankPinId,
        I1: PinId + ValidPwmOutputPin<S1, C1> + BankPinId,
        I2: PinId + ValidPwmOutputPin<S2, C2> + BankPinId,
    > PwmManager<S0, S1, S2, C0, C1, C2, P0, P1, P2, I0, I1, I2>
where
    Channel<S0, FreeRunning, C0>: PwmPin,
    Channel<S1, FreeRunning, C1>: PwmPin,
    Channel<S2, FreeRunning, C2>: PwmPin,
{
    pub fn new(
        pwm0: Channel<S0, FreeRunning, C0>,
        pwm1: Channel<S1, FreeRunning, C1>,
        pwm2: Channel<S2, FreeRunning, C2>,
        dir0: Pin<P0, Output<PushPull>>,
        dir1: Pin<P1, Output<PushPull>>,
        dir2: Pin<P2, Output<PushPull>>,
        pwm_pin0: Pin<I0, Output<PushPull>>,
        pwm_pin1: Pin<I1, Output<PushPull>>,
        pwm_pin2: Pin<I2, Output<PushPull>>,
    ) -> Self {
        Self {
            pwm0: PwmManagerPhase {
                pwm: pwm0,
                dir: dir0,
                pwm_pin: pwm_pin0,
            },
            pwm1: PwmManagerPhase {
                pwm: pwm1,
                dir: dir1,
                pwm_pin: pwm_pin1,
            },
            pwm2: PwmManagerPhase {
                pwm: pwm2,
                dir: dir2,
                pwm_pin: pwm_pin2,
            },
        }
    }

    pub fn set_duty(&mut self, value: PhasePwmVector) {
        if value[0] >= 0.0 {
            self.pwm0.dir.set_high().unwrap();
        } else {
            self.pwm0.dir.set_low().unwrap();
        }

        if value[1] >= 0.0 {
            self.pwm1.dir.set_high().unwrap();
        } else {
            self.pwm1.dir.set_low().unwrap();
        }

        if value[2] >= 0.0 {
            self.pwm2.dir.set_high().unwrap();
        } else {
            self.pwm2.dir.set_low().unwrap();
        }

        self.pwm0
            .pwm
            .set_duty((value[0].abs() * 65535.0) as Channel<S0, FreeRunning, C0>::Duty);
    }
}
