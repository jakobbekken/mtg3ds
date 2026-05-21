use ctru::linear::LinearAllocator;
use ctru::services::ndsp::wave::Wave;
use ctru::services::ndsp::{AudioFormat, AudioMix, Channel, Ndsp, OutputMode};

const SAMPLE_RATE: f32 = 22050.0;
const FREQ: f32 = 880.0;
const DURATION_SAMPLES: usize = 2205;

pub struct Audio {
    pub ndsp: Ndsp,
}

impl Audio {
    pub fn new() -> Self {
        let mut ndsp = Ndsp::new().expect("Failed to init ndsp");
        ndsp.set_output_mode(OutputMode::Stereo);
        Self { ndsp }
    }

    pub fn setup_channel(&self) -> Channel<'_> {
        let mut channel = self.ndsp.channel(0).expect("Failed to get channel");
        channel.set_format(AudioFormat::PCM16Mono);
        channel.set_sample_rate(SAMPLE_RATE);
        let mut mix = AudioMix::zeroed();
        mix.set_front(0.5, 0.5);
        channel.set_mix(&mix);
        channel
    }

    pub fn generate_tone() -> Wave<Vec<u8, LinearAllocator>> {
        let mut buf = Vec::with_capacity_in(DURATION_SAMPLES * 2, LinearAllocator);
        for i in 0..DURATION_SAMPLES {
            let t = i as f32 / SAMPLE_RATE;
            let sample = (f32::sin(2.0 * core::f32::consts::PI * FREQ * t) * 0.5 * 32767.0) as i16;
            let bytes = sample.to_le_bytes();
            buf.push(bytes[0]);
            buf.push(bytes[1]);
        }
        Wave::new(buf, AudioFormat::PCM16Mono, false)
    }
}
