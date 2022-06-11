use std::{rc::Rc, cell::RefCell};

use backend::init_va;
use pipewire::{
    prelude::ListenerBuilderT,
    properties,
    stream::{Stream, StreamFlags},
    Context, MainLoop,
};
use portal_screencast::ScreenCast;

mod backend;

#[derive(Default)]
struct StreamData {
    stream: Option<Rc<RefCell<Stream<StreamData>>>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe { init_va(); }
    /*
    pipewire::init();

    let screencast = ScreenCast::new()?.start(None)?;
    let screencast_stream = screencast.streams().next().unwrap();
    let screencast_node = screencast_stream.pipewire_node();
    dbg!(screencast_node);

    let main_loop = MainLoop::new()?;
    let ctx = Context::new(&main_loop)?;
    let core = ctx.connect_fd(screencast.pipewire_fd(), None)?;
    let stream_props = properties! {
        *pipewire::keys::MEDIA_CLASS => "Stream/Input/Video",
    };
    let stream = Rc::new(RefCell::new(Stream::<StreamData>::new(&core, "vareplay", stream_props)?));
    let stream2 = Rc::clone(&stream);

    let pod = unsafe { backend::build_video_format() };
    stream.borrow_mut().connect(
        pipewire::spa::Direction::Input,
        Some(screencast_node),
        StreamFlags::AUTOCONNECT | StreamFlags::MAP_BUFFERS,
        &mut [pod],
    )?;
    unsafe { backend::free(pod as *const _) };
    let listener = stream
        .borrow_mut()
        .add_local_listener_with_user_data(StreamData { stream: Some(stream2) })
        .state_changed(|old, new| println!("{old:?} -> {new:?}"))
        .param_changed(|id, stream_ref, pod| {
            let mut params = [std::ptr::null(); 3];
            unsafe { backend::on_param_changed(id, pod, params.as_mut_ptr()) };
            stream_ref.stream.as_ref().unwrap().borrow_mut().update_params(&mut params).unwrap();
        })
        .process(|stream, _| {
            let mut buffer = stream.dequeue_buffer().unwrap();
            let data = &buffer.datas_mut()[0].0;
            dbg!(&data.fd);
        })
        .register()?;
    dbg!();
    main_loop.run();

    listener.unregister();

    unsafe { pipewire::deinit() };
    */

    Ok(())
}
