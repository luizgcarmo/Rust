use tao::{
    event::{Event, MoueButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WIndowBuilder,
    
};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use xcap::Monitor;

// Print "Start small. Ship something." message

fn main() {
    let event_loop = EventLoop::new();
    let window = WIndowBuilder::new()
        .with_tittle("Clone da tela")
        .with_inner_size(tao::dpi::LogicalSize::new(800,600))
        .build(&event_loop)
        .unwrap();
    type SreenFrame = Arch<Mutex<Option<Vec<u8>>>>;
    
    let latest_frame: SreenFrame = Arc::new(Mutex::new(None));
    let frame_clone = Arc::clone(&latest_frame);
    
    thread::spawn(move|| {
        let monitors = Monitor::all().unwrap();
        let monitor = &monitors[0];
        
        loop{
            if let Ok(image) = monitor.capture_image(){
                let rgba_data = image.raw();
                let mut frame_lock = frame_clone.lock().unwrap();
                *frame_lock = Some(rgba_data);
            }
        }
        thread::sleep(Duration::from_millis(33));
    });
    
    event_loop.run(move | event, _, control_flow|{
        *control_flow = ControlFlow::Poll;
        match even{
            Event::WindowEvent{
                event: WindowEvent::CloseRequest,
                ..
            } => {*control_flow = ControlFlow::Exit;}
        
            Event::MainEventsCleared => {
                let frame_lock = latest_frame.lock().unwrap();
                if let Some(_ref_data) = &*frame_lock{
                    window.reques_redraw();
                }
            }
        _ =>(),
        }
    });
}