use wasm_bindgen::prelude::*;
use std::f64::consts::*;

#[wasm_bindgen]
pub struct SoundGen {
    sample_rate: usize,
    sine_wave: Vec<f64>,
}

#[wasm_bindgen]
impl SoundGen {
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: u32) -> SoundGen {
        let quarterwave = (0..(sample_rate / 4)).map(|i| ((i as f64) * 2.0 * PI / (sample_rate as f64)).sin());
        SoundGen {
            sample_rate: sample_rate as usize,
            sine_wave: quarterwave.clone()
                .chain(quarterwave.clone().rev())
                .chain(quarterwave.clone().map(|f| -f))
                .chain(quarterwave.clone().map(|f| -f).rev())
                .collect(),
        }
    }

    pub fn sound(&self, freq: u32) -> Box<[f32]> {
        self.note(
            freq,
            vec![
                interp(vec![0.75, 0.5, 0.75, 0.95, 0.95]),
                Box::new( |_| 0.1),
                Box::new(|_| 0.05),
                interp(vec![0.0, 0.0, 0.5, 0.2, 0.2, 0.1, 0.0, 0.0, 0.0])
            ],
        ).into_boxed_slice()
    }

    fn note(&self, freq: u32, obertones: Vec<Box<dyn Fn(f64) -> f64>>) -> Vec<f32> {
        let mut part = vec![0f32; self.sample_rate];
        for (i, obertone) in obertones.into_iter().enumerate() {
            let mut pos = 0;
            let step = freq as usize * (i + 1);
            for j in 0..self.sample_rate {
                part[j] += (self.sine_wave[pos] * obertone(j as f64 / self.sample_rate as f64)) as f32;
                pos = (pos + step) % self.sample_rate;
            }
        }
        part
    }
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
    use crate::sound::SoundGen;

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

    mod bench {
        extern crate test;
        use self::test::Bencher;

        fn array_iter(b: &mut Bencher, step: usize) {
            let arr: Vec<i32> = (0..48000).collect();
            b.iter(|| {
                let mut pos = 0;
                let mut res = Vec::with_capacity(48000);
                for _ in 0..48000 {
                    res.push(arr[pos]);
                    pos = (pos + step) % 4800;
                }
            });
        }

        #[bench]
        fn array_iter_100(b: &mut Bencher) {
            array_iter(b, 100);
        }

        #[bench]
        fn array_iter_200(b: &mut Bencher) {
            array_iter(b, 200);
        }

        #[bench]
        fn array_iter_300(b: &mut Bencher) {
            array_iter(b, 300);
        }

        #[bench]
        fn array_iter_400(b: &mut Bencher) {
            array_iter(b, 400);
        }

        #[bench]
        fn array_iter_500(b: &mut Bencher) {
            array_iter(b, 500);
        }
    }
}