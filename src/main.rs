use std::{error::Error, time::{Duration, Instant}};

use glib::EnumClass;
use gstreamer::{ClockTime, Element, ElementFactory, MessageView, Pipeline, State, event::Eos, prelude::*};
use portal_screencast::ScreenCast;

fn main() -> Result<(), Box<dyn Error>> {
    gstreamer::init()?;

    let screencast = ScreenCast::new()?.start(None)?;
    let screencast_stream = screencast.streams().next().unwrap();

    let pw_src_factory = ElementFactory::find("pipewiresrc").unwrap();
    let vaapipostproc_factory = ElementFactory::find("vaapipostproc").unwrap();
    let va_factory = ElementFactory::find("vaapih264enc").unwrap();
    let queue_factory = ElementFactory::find("queue").unwrap();
    let output_selector_factory = ElementFactory::find("output-selector").unwrap();
    let fakesink_factory = ElementFactory::find("fakesink").unwrap();
    let h264parse_factory = ElementFactory::find("h264parse").unwrap();
    let matroskamux_factory = ElementFactory::find("matroskamux").unwrap();
    let filesink_factory = ElementFactory::find("filesink").unwrap();

    let pw_src = pw_src_factory.create(None)?;
    let vaapipostproc = vaapipostproc_factory.create(None)?;
    let va = va_factory.create(None)?;
    let videoqueue = queue_factory.create(None)?;
    let output_selector = output_selector_factory.create(None)?;
    let fakesink = fakesink_factory.create(None)?;
    let h264parse = h264parse_factory.create(None)?;
    let matroskamux = matroskamux_factory.create(None)?;
    let filesink = filesink_factory.create(None)?;

    pw_src.set_property("fd", &screencast.pipewire_fd())?;
    // pw api quirk? is always integer but we pass as string
    pw_src.set_property("path", &format!("{}", screencast_stream.pipewire_node()))?;
    pw_src.set_property("do-timestamp", &true)?;
    // 10 seconds
    videoqueue.set_property("max-size-time", &10_000_000_000u64)?;
    let ty = glib::Type::from_name("GstOutputSelectorPadNegotiationMode").unwrap();
    let class = EnumClass::new(ty).unwrap();
    let mode = class.value_by_nick("active").unwrap().to_value();
    output_selector.set_property("pad-negotiation-mode", &mode)?;
    filesink.set_property("location", &"coolvideo.mkv")?;

    let pipeline = Pipeline::new(None);
    pipeline.add_many(&[
        &pw_src,
        &vaapipostproc,
        &va,
        &h264parse,
        &matroskamux,
        &filesink,
    ])?;

    Element::link_many(&[&pw_src, &vaapipostproc, &va, &h264parse, &matroskamux, &filesink])?;
/*
    Element::link_many(&[&videotestsrc, &vaapipostproc, &va/*, &videoqueue, &output_selector, &fakesink*/])?;
    Element::link_many(&[&va, &h264parse, &matroskamux, &filesink])?;
    // the pads on the output_selector that lead to these things
    let fakesink_pad = fakesink.get_pads()[0].get_peer().unwrap();
    let realsink_pad = h264parse.get_sink_pads()[0].get_peer().unwrap();
    output_selector.set_property("active-pad", &fakesink_pad)?;

    videoqueue.connect(
        "overrun",
        true,
        clone!(@strong output_selector, @strong realsink_pad => move |x| {
            println!("hello!");
            output_selector.set_property("active-pad", &realsink_pad);
            None
        }),
    )?;
    videoqueue.connect(
        "underrun",
        true,
        clone!(@strong output_selector, @strong fakesink_pad => move |x| {
            println!("wowee");
            output_selector.set_property("active-pad", &fakesink_pad);
            None
        }),
    )?;
*/
    dbg!();
    pipeline.set_state(State::Paused)?;
    dbg!();
    pipeline.set_state(State::Playing)?;
    dbg!();

    let bus = pipeline.bus().unwrap();
    let start = Instant::now();
    loop {
        if Instant::now() - start > Duration::from_secs(10) {
            println!("end");
            pipeline.send_event(Eos::new());
            pipeline.set_state(State::Paused)?;
            println!("bye bye");
            break;
        }
        let msg = bus.timed_pop(ClockTime::from_seconds(1));
        println!("{:?}", msg);
        if msg.is_none() {
            continue;
        }
        match msg.unwrap().view() {
            MessageView::Eos(..) => break,
            _ => {},
        }
    }

    pipeline.set_state(State::Ready)?;

    Ok(())
}
