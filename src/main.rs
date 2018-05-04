
extern crate rand;

pub mod clock;
pub mod consts;
pub mod synth;
pub mod events;
pub mod device;
pub mod effects;
pub mod conversions;

use clock::*;
use synth::*;
use events::*;
use device::*;
use effects::*;
use conversions::*;

use portaudio as pa;

extern crate portaudio;

fn main() {
    match run() {
        Ok(_) => {},
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

// device

fn run() -> Result<(), pa::Error> {
    let c = Clock::new();
    let m = TimeCalculator::new(120.0);

    let mut events = Vec::<(u64, NoteEvent)>::new();
    events.push((m.add_bars(0.0).add_sixteenths(0.0).time(), NoteEvent::NoteOn(220.0)));
    events.push((m.add_bars(0.0).add_sixteenths(1.0).time(), NoteEvent::NoteOff));
    events.push((m.add_bars(0.0).add_sixteenths(2.0).time(), NoteEvent::NoteOn(440.0)));
    events.push((m.add_bars(0.0).add_sixteenths(3.0).time(), NoteEvent::NoteOff));
    events.push((m.add_bars(0.0).add_sixteenths(4.0).time(), NoteEvent::NoteOn(220.0)));
    events.push((m.add_bars(0.0).add_sixteenths(5.0).time(), NoteEvent::NoteOff));
    events.push((m.add_bars(0.0).add_sixteenths(6.0).time(), NoteEvent::NoteOn(440.0)));
    events.push((m.add_bars(0.0).add_sixteenths(7.0).time(), NoteEvent::NoteOff));
    events.push((m.add_bars(0.0).add_sixteenths(8.0).time(), NoteEvent::NoteOn(220.0)));
    events.push((m.add_bars(0.0).add_sixteenths(16.0).time(), NoteEvent::NoteOff));
    events.push((m.add_bars(1.0).add_sixteenths(0.0).time(), NoteEvent::NoteOn(440.0)));
    events.push((m.add_bars(1.0).add_sixteenths(1.0).time(), NoteEvent::NoteOff));
    events.push((m.add_bars(1.0).add_sixteenths(2.0).time(), NoteEvent::NoteOn(220.0)));
    events.push((m.add_bars(1.0).add_sixteenths(3.0).time(), NoteEvent::NoteOff));
    events.push((m.add_bars(1.0).add_sixteenths(4.0).time(), NoteEvent::NoteOn(440.0)));
    events.push((m.add_bars(1.0).add_sixteenths(5.0).time(), NoteEvent::NoteOff));
    events.push((m.add_bars(1.0).add_sixteenths(6.0).time(), NoteEvent::NoteOn(220.0)));
    events.push((m.add_bars(1.0).add_sixteenths(7.0).time(), NoteEvent::NoteOff));
    events.push((m.add_bars(1.0).add_sixteenths(8.0).time(), NoteEvent::NoteOn(440.0)));
    events.push((m.add_bars(1.0).add_sixteenths(16.0).time(), NoteEvent::NoteOff));

    let mut eventsfifth = Vec::<(u64, NoteEvent)>::new();
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(0.0).time(), NoteEvent::NoteOn(2.5*220.0)));
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(1.0).time(), NoteEvent::NoteOff));
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(2.0).time(), NoteEvent::NoteOn(1.5*440.0)));
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(3.0).time(), NoteEvent::NoteOff));
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(4.0).time(), NoteEvent::NoteOn(1.5*220.0)));
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(5.0).time(), NoteEvent::NoteOff));
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(6.0).time(), NoteEvent::NoteOn(1.5*440.0)));
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(7.0).time(), NoteEvent::NoteOff));
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(8.0).time(), NoteEvent::NoteOn(1.5*220.0)));
    eventsfifth.push((m.add_bars(0.0).add_sixteenths(16.0).time(), NoteEvent::NoteOff));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(0.0).time(), NoteEvent::NoteOn(1.5*440.0)));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(1.0).time(), NoteEvent::NoteOff));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(2.0).time(), NoteEvent::NoteOn(1.5*220.0)));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(3.0).time(), NoteEvent::NoteOff));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(4.0).time(), NoteEvent::NoteOn(1.5*440.0)));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(5.0).time(), NoteEvent::NoteOff));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(6.0).time(), NoteEvent::NoteOn(1.5*220.0)));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(7.0).time(), NoteEvent::NoteOff));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(8.0).time(), NoteEvent::NoteOn(1.5*440.0)));
    eventsfifth.push((m.add_bars(1.0).add_sixteenths(16.0).time(), NoteEvent::NoteOff));

    let es5 = EventSource::new(eventsfifth, c.clone());
    let osc5 = Oscillator::new(c.clone(), es5.clone(), ConstSignal::new(c.clone(), 0.5));
    let wtp5 = ConstSignal::new(c.clone(), 0.0);
    let wave5 = Wave::triangle();
    let wavetable5 = WaveTable::new(vec![wave5]);
    let env5 = Envelope::new(c.clone(), es5.clone());
    let n5 = MonoSynth::new(c.clone(), wavetable5.clone(), osc5.clone(), wtp5.clone(), env5.clone());
    let gn5 = Gain::new(c.clone(), MonoToStereo::new(c.clone(), n5.clone()), ConstSignal::new(c.clone(), decibels(0.0)));

    let es = EventSource::new(events, c.clone());
    //let n = (StupidOsc::new(c.clone(), es.clone()));
    let wave = Wave::saw();
    let wavetable = WaveTable::new(vec![wave]);
    let detune_multiplier_1 = ConstSignal::new(c.clone(), 1.01);
    let detune_multiplier_2 = ConstSignal::new(c.clone(), 0.98);
    let wavetable_position = ConstSignal::new(c.clone(), 0.0);
    let oscillator1 = Oscillator::new(c.clone(), es.clone(), detune_multiplier_1.clone());
    let oscillator2 = Oscillator::new(c.clone(), es.clone(), detune_multiplier_2.clone());
    let envelope = Envelope::new(c.clone(), es.clone());
    let n1 = MonoSynth::new(c.clone(), wavetable.clone(), oscillator1.clone(), wavetable_position.clone(), envelope.clone());
    let n2 = MonoSynth::new(c.clone(), wavetable.clone(), oscillator2.clone(), wavetable_position.clone(), envelope.clone());

    let n1s = MonoToStereo::new(c.clone(), n1.clone());
    let n2s = MonoToStereo::new(c.clone(), n2.clone());

    let left = ConstSignal::new(c.clone(), -0.5);
    let right = ConstSignal::new(c.clone(), 0.5);

    let nn1 = Pan::new(c.clone(), n1s, left);
    let nn2 = Pan::new(c.clone(), n2s, right);

    let noise = WhiteNoise::new(c.clone());
    let noise_gained = Gain::new(c.clone(), noise.clone(), envelope.clone());

    let nn1shaped = WaveShaperEffect::new(
            c.clone(),
            Gain::new(c.clone(), nn1.clone(), ConstSignal::new(c.clone(), decibels(0.0))),
            HardClipper::new()
        );

    let nn2shaped = WaveShaperEffect::new(
            c.clone(),
            Gain::new(c.clone(), nn2.clone(), ConstSignal::new(c.clone(), decibels(0.0))),
            HardClipper::new()
        );

    let mix = Mixer::new(c.clone(), vec![nn1shaped, nn2shaped, gn5, noise_gained]);
    let mastergain = ConstSignal::new(c.clone(), decibels(-20.0));
    let master = Gain::new(c.clone(), mix.clone(), mastergain.clone());

    let pa = try!(pa::PortAudio::new());
    let mut settings = try!(pa.default_output_stream_settings(
            2, // num channels
            consts::SAMPLE_RATE as f64,
            consts::CHUNK_SIZE as u32
        ));
    settings.flags = pa::stream_flags::CLIP_OFF;


    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let left = master.output().0;
        let right = master.output().1;
        assert_eq!(frames, consts::CHUNK_SIZE);
        for f in 0..frames {
            if left[f].abs() > 1.0 || right[f].abs() > 1.0 {
                println!("WARNING: The signal is clipping!");
            }
            buffer[2*f] = left[f];
            buffer[2*f+1] = right[f];
        }
        c.increment();
        pa::Continue
    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    println!("Playing for 5 seconds.");
    pa.sleep(5 * 1_000);

    try!(stream.stop());
    try!(stream.close());

    println!("Finished!");

    Ok(())
}
