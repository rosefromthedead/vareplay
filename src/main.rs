use std::{error::Error, time::{Duration, Instant}};

use glib::{EnumClass, clone, Type};
use gstreamer::{ClockTime, Element, ElementFactory, MessageView, Pipeline, State, event::Eos, prelude::*};
use portal_screencast::ScreenCast;

fn main() -> Result<(), Box<dyn Error>> {
    gstreamer::init()?;

    let screencast = ScreenCast::new()?.start(None)?;
    let screencast_stream = screencast.streams().next().unwrap();

    let pipeline = gstreamer::parse_launch(&format!(
        "pipewiresrc fd={pw_fd} path={pw_path} do-timestamp=true
        ! videoparse framerate=60/1 format=bgra width=2560 height=1440
        ! vaapipostproc
        ! vaapih264enc
        ! h264parse
        ! queue
        ! matroskamux
        ! filesink location=coolvideo.mkv
        ", pw_fd = screencast.pipewire_fd(), pw_path = screencast_stream.pipewire_node()
    ))?.downcast::<Pipeline>().unwrap();

    let queue = pipeline.by_name("queue0").unwrap();
    let valve_factory = ElementFactory::find("valve").unwrap();
    let valve = valve_factory.create(None)?;
    let matroskamux = pipeline.by_name("matroskamux0").unwrap();
    queue.connect(
        "underrun",
        true,
        clone!(@strong queue, @strong matroskamux => move |x| {
            println!("wowee");
            //queue.unlink(&matroskamux);
            //matroskamux.send_event(Eos::new());
            None
        }),
    )?;

    pipeline.set_state(State::Paused)?;
    pipeline.set_state(State::Playing)?;

    let bus = pipeline.bus().unwrap();
    loop {
        let msg = bus.timed_pop(ClockTime::from_seconds(1));
        println!("{:?}", msg);
        println!("{:?}", queue.property("current-level-time")?);
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
