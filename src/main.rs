use std::error::Error;

use glib::{EnumClass, clone};
use gstreamer::{Element, ElementFactory, MessageView, Pipeline, State, prelude::*};
use portal_screencast::ScreenCast;

fn main() -> Result<(), Box<dyn Error>> {
    gstreamer::init()?;

    let screencast = ScreenCast::new()?.start(None)?;
    let screencast_stream = screencast.streams().next().unwrap();

    let pw_src_factory = ElementFactory::find("pipewiresrc").unwrap();
    let videoconvert_factory = ElementFactory::find("videoconvert").unwrap();
    let va_factory = ElementFactory::find("vaapih265enc").unwrap();
    let queue_factory = ElementFactory::find("queue").unwrap();
    let output_selector_factory = ElementFactory::find("output-selector").unwrap();
    let fakesink_factory = ElementFactory::find("fakesink").unwrap();
    let h265parse_factory = ElementFactory::find("h265parse").unwrap();
    let matroskamux_factory = ElementFactory::find("matroskamux").unwrap();
    let filesink_factory = ElementFactory::find("filesink").unwrap();

    let pw_src = pw_src_factory.create(None)?;
    let videoconvert = videoconvert_factory.create(None)?;
    let va = va_factory.create(None)?;
    let videoqueue = queue_factory.create(None)?;
    let output_selector = output_selector_factory.create(None)?;
    let fakesink = fakesink_factory.create(None)?;
    let h265parse = h265parse_factory.create(None)?;
    let matroskamux = matroskamux_factory.create(None)?;
    let filesink = filesink_factory.create(None)?;

    pw_src.set_property("fd", &screencast.pipewire_fd())?;
    // pw api quirk? is always integer but we pass as string
    pw_src.set_property("path", &format!("{}", screencast_stream.pipewire_node()))?;
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
        &videoconvert,
        &va,
        &videoqueue,
        &output_selector,
        &fakesink,
        &h265parse,
        &matroskamux,
        &filesink,
    ])?;

    Element::link_many(&[&pw_src, &videoconvert, &va/*, &videoqueue, &output_selector*/, &fakesink])?;
//    Element::link_many(&[&va, &h265parse, &matroskamux, &filesink])?;
/*
    // the pads on the output_selector that lead to these things
    let fakesink_pad = fakesink.get_pads()[0].get_peer().unwrap();
    let realsink_pad = h265parse.get_sink_pads()[0].get_peer().unwrap();
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
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => break,
            _ => {},
        }
    }

    pipeline.set_state(State::Null)?;

    Ok(())
}
