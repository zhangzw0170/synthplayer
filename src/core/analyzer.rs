use rodio::Source;
use rustfft::{num_complex::Complex, FftPlanner};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub type SampleBuffer = Arc<Mutex<Vec<f32>>>;

pub fn new_buffer() -> SampleBuffer {
    Arc::new(Mutex::new(Vec::with_capacity(8192)))
}

/// Wraps a rodio Source and feeds samples into a shared buffer.
pub struct AnalyzerSource<I> {
    inner: I,
    buffer: SampleBuffer,
}

impl<I: Iterator<Item = f32>> AnalyzerSource<I> {
    pub fn new(inner: I, buffer: SampleBuffer) -> Self {
        Self { inner, buffer }
    }
}

impl<I: Iterator<Item = f32>> Iterator for AnalyzerSource<I> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.inner.next().inspect(|&s| {
            if let Ok(mut buf) = self.buffer.lock() {
                // Keep a rolling window of up to 8192 samples
                if buf.len() >= 8192 {
                    let keep_start = buf.len() - 4096;
                    buf.drain(..keep_start);
                }
                buf.push(s);
            }
        })
    }
}

impl<I: Source<Item = f32>> Source for AnalyzerSource<I> {
    fn current_frame_len(&self) -> Option<usize> {
        self.inner.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }

    fn try_seek(&mut self, pos: Duration) -> Result<(), rodio::source::SeekError> {
        // Clear buffer on seek so old samples don't pollute the FFT
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
        self.inner.try_seek(pos)
    }
}

/// Compute FFT magnitudes from a sample buffer.
/// Returns `num_bars` frequency bins (log-spaced).
pub fn compute_spectrum(buffer: &[f32], sample_rate: u32, num_bars: usize) -> Vec<f32> {
    let len = buffer.len().min(4096);
    if len < 128 {
        return vec![0.0; num_bars];
    }

    // Find a power-of-two window size
    let fft_size = len.next_power_of_two().min(4096);
    let fft_size = fft_size.max(256);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);

    // Hann window + copy into complex buffer
    let mut input: Vec<Complex<f32>> = (0..fft_size)
        .map(|i| {
            let s = if i < buffer.len() { buffer[i] } else { 0.0 };
            let w = 0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / (fft_size - 1) as f32)
                .cos();
            Complex::new(s * w, 0.0)
        })
        .collect();

    fft.process(&mut input);

    // Magnitudes for positive frequencies (first half)
    let mags: Vec<f32> = input[..fft_size / 2]
        .iter()
        .map(|c| c.norm() / fft_size as f32)
        .collect();

    // Log-spaced binning into `num_bars`
    let max_hz = sample_rate as f32 / 2.0;
    let min_hz = 20.0f32;
    let mut bars = vec![0.0f32; num_bars];

    for (i, &mag) in mags.iter().enumerate() {
        let hz = i as f32 / fft_size as f32 * sample_rate as f32;
        if hz < min_hz {
            continue;
        }
        let log_idx =
            ((hz / min_hz).ln() / (max_hz / min_hz).ln()) * num_bars as f32;
        let idx = (log_idx as usize).min(num_bars - 1);
        bars[idx] = bars[idx].max(mag);
    }

    // Normalize to ~[0, 1]
    let max_mag = bars.iter().cloned().fold(0.0f32, f32::max).max(0.001);
    for b in &mut bars {
        *b = (*b / max_mag).clamp(0.0, 1.0);
    }

    bars
}
