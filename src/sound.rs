use wasm_bindgen::prelude::*;

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
            ]
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
        values[step] * weight + values[step + 1] * (1.0 - weight)
    })
}
