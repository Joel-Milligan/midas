use std::borrow::{Borrow, BorrowMut};
use std::collections::vec_deque::VecDeque;
use std::time::SystemTime;

use minifb::{Key, Window, WindowOptions};
use plotters::prelude::*;
use plotters_bitmap::BitMapBackend;
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use systemstat::System;
use systemstat::platform::common::Platform;

const H: usize = 600;
const W: usize = 800;
const FRAME_RATE: f64 = 10.;

const FPS: u32 = 10;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;

struct BufferWrapper(Vec<u32>);

impl Borrow<[u8]> for BufferWrapper {
    fn borrow(&self) -> &[u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.0.len() * 4) }
    }
}

impl BorrowMut<[u8]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe { std::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut u8, self.0.len() * 4) }
    }
}

impl Borrow<[u32]> for BufferWrapper {
    fn borrow(&self) -> &[u32] {
        self.0.as_slice()
    }
}

impl BorrowMut<[u32]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u32] {
        self.0.as_mut_slice()
    }
}

fn main() {
    let mut buf = BufferWrapper(vec![0u32; W * H]);
    let mut window = Window::new("Real Time CPU Usage", W, H, WindowOptions::default()).unwrap();

    let sys = System::new();
    let mut load_measurement: Vec<_> = (0..FPS).map(|_| sys.cpu_load().unwrap()).collect();
    let mut epoch = 0;
    let mut data = vec![];

    let start_ts = SystemTime::now();
    let mut last_flushed = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let flush_epoch = SystemTime::now()
            .duration_since(start_ts)
            .unwrap()
            .as_secs_f64();

        if flush_epoch - last_flushed > 1.0 / FRAME_RATE {
            let cpu_loads = load_measurement[epoch % FPS as usize].done().unwrap();

            {
                let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
                    buf.borrow_mut(),
                    (W as u32, H as u32),
                )
                .unwrap()
                .into_drawing_area();

                root.fill(&WHITE).unwrap();

                if data.len() < cpu_loads.len() {
                    for _ in data.len()..cpu_loads.len() {
                        data.push(VecDeque::from(vec![0f32; N_DATA_POINTS + 1]));
                    }
                }

                for (core_load, target) in cpu_loads.into_iter().zip(data.iter_mut()) {
                    if target.len() == N_DATA_POINTS + 1 {
                        target.pop_front();
                    }
                    target.push_back(1.0 - core_load.idle);
                }

                let mut cc = ChartBuilder::on(&root)
                    .margin(10)
                    .caption("Real Time CPU Usage", ("sans-serif", 30))
                    .x_label_area_size(40)
                    .y_label_area_size(50)
                    .build_cartesian_2d(0..N_DATA_POINTS as u32, 0f32..1f32)
                    .unwrap();

                cc.configure_mesh()
                    .x_label_formatter(&|x| {
                        format!("{}", -(LENGTH as f32) + (*x as f32 / FPS as f32))
                    })
                    .y_label_formatter(&|y| format!("{}%", (*y * 100.0) as u32))
                    .x_labels(15)
                    .y_labels(5)
                    .x_desc("Seconds")
                    .y_desc("% Busy")
                    .axis_desc_style(("sans-serif", 15))
                    .draw()
                    .unwrap();

                for (idx, data) in (0..).zip(data.iter()) {
                    cc.draw_series(LineSeries::new(
                        (0..).zip(data.iter()).map(|(a, b)| (a, *b)),
                        &Palette99::pick(idx),
                    ))
                    .unwrap()
                    .label(format!("CPU {}", idx))
                    .legend(move |(x, y)| {
                        Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(idx))
                    });
                }

                cc.configure_series_labels()
                    .background_style(&WHITE.mix(0.8))
                    .border_style(&BLACK)
                    .draw()
                    .unwrap();
            };

            load_measurement[epoch % FPS as usize] = sys.cpu_load().unwrap();
            epoch += 1;

            window.update_with_buffer(buf.borrow(), W, H).unwrap();
            last_flushed = flush_epoch;
        }
    }
}
