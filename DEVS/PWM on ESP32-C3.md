# PWM on ESP32-C3

From ["ESP32 Embedded Rust at the HAL: PWM Buzzer"](https://dev.to/theembeddedrustacean/esp32-embedded-rust-at-the-hal-pwm-buzzer-5b2i) (tutorial, May'23):

>Implementing hardware-based PWM in the ESP32C3 is a bit non-conventional. Meaning that I expected the timer peripheral to have a PWM function. ESP32s rather seem to have three types of application-driven peripherals that enable PWM implementation; the LED controller (LEDC) peripheral, the motor control (MCPWM) peripheral, and the Remote Control Peripheral (RMT).
>The ESP32C3 in particular does not have an MCPWM peripheral, so the choices come down to two.

