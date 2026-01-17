use std::borrow::{Borrow, BorrowMut};
use std::error::Error;

use midas::{Game, OptimalAi, Player};
use minifb::{Key, Window, WindowOptions};
use plotters::prelude::*;
use plotters_bitmap::BitMapBackend;
use plotters_bitmap::bitmap_pixel::BGRXPixel;

use crate::buffer_wrapper::BufferWrapper;

mod buffer_wrapper;

/// How many rounds to run before re-drawing chart.
/// Lower values will cause the chart to update more smoothly,
/// while higher values will allow more rounds to be run per second
const ROUNDS_PER_UPDATE: usize = 100;
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = BufferWrapper::new(vec![0u32; WINDOW_WIDTH * WINDOW_HEIGHT]);
    let mut window = Window::new(
        "Midas",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let mut highest_balance = 0.0;
    let mut data = vec![];

    let player = OptimalAi::new();
    let mut game = Game::new(player);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..ROUNDS_PER_UPDATE {
            if game.player.balance() > 0.0 {
                let _ = game.round();
                let new_balance = game.player.balance();
                data.push(new_balance);
                if new_balance > highest_balance {
                    highest_balance = new_balance;
                }
            } else {
                break;
            }
        }

        update_chart(buf.borrow_mut(), &data, highest_balance)?;
        window.update_with_buffer(buf.borrow(), WINDOW_WIDTH, WINDOW_HEIGHT)?;
    }
    Ok(())
}

fn update_chart(
    buf: &mut [u8],
    data: &Vec<f32>,
    highest_balance: f32,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
        buf.borrow_mut(),
        (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
    )
    .unwrap()
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Balance over time", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..data.len(), 0f32..highest_balance)?;

    chart
        .configure_mesh()
        .x_label_formatter(&|&x| format!("{x}"))
        .y_label_formatter(&|&y| format!("${y}"))
        .x_labels(15)
        .y_labels(10)
        .x_desc("Rounds")
        .y_desc("Balance")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..).zip(data.iter()).map(|(a, b)| (a, *b)),
            &Palette99::pick(0),
        ))?
        .label("Optimal")
        .legend(move |(x, y)| {
            Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(0))
        });

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
