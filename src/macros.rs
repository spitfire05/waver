#[macro_export]
macro_rules! wave {
    ($sample_rate:expr, $amplitude:expr, $function:expr) => {
        Wave {sample_rate: $sample_rate, amplitude: $amplitude, function: $function}
    };
    ($amplitude:expr, $function:expr) => {
        Wave {amplitude: $amplitude, function: $function, ..Default::default()}
    };
    ($function:expr) => {
        Wave {function: $function, ..Default::default()}
    };
}

#[macro_export]
macro_rules! sine {
    ($frequency:expr, $phase:expr) => {
        Rc::new(Sine::new($frequency, $phase))
    };
    ($frequency:expr) => {
        Rc::new(Sine::new($frequency, 0.0))
    };
}

#[macro_export]
macro_rules! rectangular {
    ($frequency:expr, $phase:expr, $duty_cycle:expr) => {
        Rc::new(Rectangular::new($frequency, $phase, $duty_cycle))
    };
}

#[macro_export]
macro_rules! triangle {
    ($frequency:expr) => {
        Rc::new(Triangle::new($frequency))
    };
}