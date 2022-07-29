use std::{cell::RefCell, rc::Rc};

use fathom::{
    color::Color,
    event_loop::{
        ButtonState, Control, EventLoop, EventReply, MouseButton, WindowEventHandler, WindowHandle,
    },
    point::Point,
    renderer::{Renderer, SwapchainHandle, Vertex}, shapes::{Rect},
};

const TRIANGLE: [Vertex; 3] = [
    Vertex {
        point: Point { x: 0.0, y: -100.0 },
        color: Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    },
    Vertex {
        point: Point { x: 100.0, y: 100.0 },
        color: Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        },
    },
    Vertex {
        point: Point {
            x: -100.0,
            y: 100.0,
        },
        color: Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        },
    },
];

const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = Rc::new(RefCell::new(Renderer::new()?));

    let mut event_loop = EventLoop::new();
    event_loop.create_window(Box::new(Window::new(renderer)));
    event_loop.run();

    Ok(())
}

struct Window {
    swapchain: SwapchainHandle,
    renderer: Rc<RefCell<Renderer>>,
    width: u32,
    height: u32,
}

impl Window {
    fn new(renderer: Rc<RefCell<Renderer>>) -> Self {
        Self {
            swapchain: SwapchainHandle::default(),
            renderer,
            width: 0,
            height: 0,
        }
    }
}

impl WindowEventHandler for Window {
    fn on_create(
        &mut self,
        _control: &mut dyn Control,
        window_handle: WindowHandle,
    ) -> Result<EventReply, Box<dyn std::error::Error>> {
        let WindowHandle::Windows(hwnd) = window_handle;
        self.swapchain = self.renderer.borrow_mut().create_swapchain(hwnd).unwrap();
        Ok(EventReply::Continue)
    }

    fn on_close(
        &mut self,
        _control: &mut dyn Control,
    ) -> Result<EventReply, Box<dyn std::error::Error>> {
        Ok(EventReply::DestroyWindow)
    }

    fn on_redraw(
        &mut self,
        _control: &mut dyn Control,
        width: u32,
        height: u32,
    ) -> Result<EventReply, Box<dyn std::error::Error>> {

        if width != self.width || height != self.height {
            self.width = width;
            self.height = height;
            println!("Resizing to {}x{}", width, height);
        }

        if width > 0 && height > 0 {
            let mut renderer = self.renderer.borrow_mut();
            renderer.begin_frame(self.swapchain)?;

            let mut vertex_buffer = Vec::new();
            let mut index_buffer = Vec::new();

            let rect = Rect {
                top: 0.0,
                left: 100.0,
                bottom: 984.0,
                right: 200.0,
            };

            rect.draw(
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
                &mut vertex_buffer,
                &mut index_buffer,
            );

            renderer.end_frame(self.swapchain, &vertex_buffer, &index_buffer)?;
        }

        Ok(EventReply::Continue)
    }

    fn on_mouse_move(
        &mut self,
        _control: &mut dyn Control,
        _new_x: i32,
        _new_y: i32,
    ) -> Result<EventReply, Box<dyn std::error::Error>> {
        Ok(EventReply::Continue)
    }

    fn on_mouse_button(
        &mut self,
        control: &mut dyn Control,
        button: MouseButton,
        state: ButtonState,
    ) -> Result<EventReply, Box<dyn std::error::Error>> {
        match button {
            MouseButton::Left => match state {
                ButtonState::Pressed => {}
                ButtonState::Released => {
                    control.create_window(Box::new(Window::new(self.renderer.clone())));
                }
            },
            MouseButton::Right => {
                if state == ButtonState::Released {
                    return Ok(EventReply::DestroyWindow);
                }
            }
            MouseButton::Middle => {}
        }
        Ok(EventReply::Continue)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.renderer
            .borrow_mut()
            .destroy_swapchain(self.swapchain)
            .unwrap();
    }
}
