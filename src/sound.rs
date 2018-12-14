use wasm_bindgen::prelude::*;
use std::f64::consts::*;

#[wasm_bindgen]
pub struct SoundGen {
    sample_rate: u32,
    sine_wave: Vec<f64>,
}

#[wasm_bindgen]
impl SoundGen {
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: u32) -> SoundGen {
        let quarterwave = (0..(sample_rate / 4)).map(|i| ((i as f64) * 2.0 * PI).sin());
        SoundGen {
            sample_rate,
            sine_wave: quarterwave.clone()
                .chain(quarterwave.clone().rev())
                .chain(quarterwave.clone().map(|f| -f))
                .chain(quarterwave.clone().map(|f| -f).rev())
                .collect(),
        }
    }
}

#[wasm_bindgen]
pub fn sound(sample_rate: u32) -> Box<[f32]> {
    let mut out: Vec<f32> = vec![];
    for note in 0..=5i16 {
        out.append(&mut note_sound(
            sample_rate,
            220f64 * (2.0f64).powf(note as f64 / 4.0),
            1.0,
            vec![
                interp(vec![0.25, 0.45, 0.5, 0.5, 0.5]),
                interp(vec![0.05, 0.1, 0.15, 0.2, 0.25]),
                Box::new(|_| 0.0),
                interp(vec![0.5, 0.45, 0.15, 0.1, 0.05])
            ],
        ));
    }
    out.into_boxed_slice()
}

fn note_sound(sample_rate: u32, freq: f64, duration: f64, obertones: Vec<Box<dyn Fn(f64) -> f64>>) -> Vec<f32> {
    let mut part = vec![0f32; (sample_rate as f64 * duration) as usize];
    for (i, obertone) in obertones.into_iter().enumerate() {
        let wave = sine(freq * i as f64, sample_rate, duration, obertone);
        for (j, f) in wave.into_iter().enumerate() {
            part[j] += f;
        }
    }
    part
}

fn sine(freq: f64, sample_rate: u32, duration: f64, loudness: Box<dyn Fn(f64) -> f64>) -> Vec<f32> {
    (0..(sample_rate as f64 * duration).floor() as u64).map(|i| ((i as f64 * freq).sin() * loudness(i as f64 / sample_rate as f64 / duration)) as f32).collect()
}

fn interp(values: Vec<f64>) -> Box<dyn Fn(f64) -> f64> {
    let steps = values.len() as u32 as f64 - 1.0;
    Box::new(move |f| {
        let pos = f * steps;
        let step = pos.floor() as usize;
        let weight = pos - step as f64;
        values[step] * (1.0 - weight) + values[step + 1] * weight
    })
}


#[cfg(test)]
mod test {
    extern crate assert_approx_eq;

    use crate::sound::interp;
    use self::assert_approx_eq::assert_approx_eq;

    #[test]
    fn interp_test_simple() {
        assert_eq!(interp(vec![2.0, 0.0])(0.5), 1.0);
    }

    macro_rules! interp_tests_median {
        ($v1:expr, $($v:expr),+) => {
            interp_tests_param!(0.5, [$v1, $($v),+]);
        }
    }

    macro_rules! interp_tests_param {
        ($param:expr, [$v1:expr, $($v:expr),+]) => {
            assert!($param > 0.0 && $param < 1.0);
            let v = vec![$v1, $($v),+];
            let count = interp_tests_param!(@count $($v)*);
            assert!(count > 0);
            for i in 0..count {
                let f = i as f64;
                assert_approx_eq!(interp(v.clone())((f + $param) / count as f64), (1.0 - $param) * v[i] + $param * v[i + 1]);
            }
        };
        (@count) => (0usize);
        (@count $x:tt $($xs:tt)* ) => (1usize + interp_tests_param!(@count $($xs)*));
    }

    #[test]
    fn interp_test_complex() {
        interp_tests_median!(0.1, 0.2, 0.1, 0.4, 0.5);
        interp_tests_param!(0.1, [0.1, 0.2, 0.1, 0.4, 0.5]);
        interp_tests_param!(0.25, [0.01, 1.0, 0.1]);
        interp_tests_param!(0.75, [1.0, 0.0, 1.0, 0.5]);
        interp_tests_median!(0.0, 0.5, 0.75, 0.85, 0.9, 0.925, 0.93, 1.0);
    }

    #[test]
    fn interp_tests_manual() {
        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.5))(0.4), 0.8);
        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.5))(0.6), 0.9);
        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.5))(0.9), 0.6);
        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.5))(0.1), 0.2);

        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.0, 0.0, 0.1))(0.75), 0.0);
        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.0, 0.0, 0.1))(0.125), 0.5);
        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.0, 0.0, 0.1))(0.0625), 0.25);
        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.0, 0.0, 0.1))(0.375), 0.5);
        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.0, 0.0, 0.1))(0.625), 0.0);
        assert_approx_eq!(interp(vec!(0.0, 1.0, 0.0, 0.0, 0.1))(0.875), 0.05);
    }
}