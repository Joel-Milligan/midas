use core::f32;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::error::Error;

use midas::{
    FlatBettingStrategy, Game, HiLoCountingStrategy, OptimalActionStrategy, Player,
    SimpleActionStrategy,
};
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
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = BufferWrapper::new(vec![0u32; WIDTH * HEIGHT]);
    let mut window = Window::new("Midas", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut highest_balance = f32::MIN;
    let mut n_rounds = 0;

    let mut players = HashMap::new();
    players.insert(
        0,
        Player::new(
            0,
            10_000.0,
            Box::new(FlatBettingStrategy),
            Box::new(OptimalActionStrategy),
        ),
    );
    players.insert(
        1,
        Player::new(
            0,
            10_000.0,
            Box::new(FlatBettingStrategy),
            Box::new(SimpleActionStrategy),
        ),
    );
    players.insert(
        2,
        Player::new(
            0,
            10_000.0,
            Box::new(HiLoCountingStrategy::new()),
            Box::new(SimpleActionStrategy),
        ),
    );
    let mut game = Game::new(players);

    let mut data = HashMap::new();
    data.insert(0, vec![]);
    data.insert(1, vec![]);
    data.insert(2, vec![]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..ROUNDS_PER_UPDATE {
            if game.players.values().any(|p| p.balance > 0.0) {
                let _ = game.round();
                n_rounds += 1;
                for (id, player) in &game.players {
                    let balance = player.balance;
                    if balance > 0.0 {
                        data.get_mut(id).unwrap().push(balance);
                        if balance > highest_balance {
                            highest_balance = balance;
                        }
                    }
                }
            }
        }
        update_chart(
            buf.borrow_mut(),
            &data,
            &game.players,
            n_rounds,
            highest_balance,
        )?;
        window.update_with_buffer(buf.borrow(), WIDTH, HEIGHT)?;
    }
    Ok(())
}

fn update_chart(
    buf: &mut [u8],
    data: &HashMap<u8, Vec<f32>>,
    players: &HashMap<u8, Player>,
    max_x: usize,
    max_y: f32,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
        buf.borrow_mut(),
        (WIDTH as u32, HEIGHT as u32),
    )
    .unwrap()
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Balance over time", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..max_x, 0.0..max_y)?;

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

    for id in players.keys() {
        let player_name = match id {
            0 => "Optimal AI",
            1 => "Simple AI",
            2 => "HiLo AI",
            _ => unreachable!(),
        };

        chart
            .draw_series(LineSeries::new(
                (0..).zip(data[id].iter()).map(|(a, b)| (a, *b)),
                Palette99::pick(*id as usize),
            ))?
            .label(player_name)
            .legend(move |(x, y)| {
                Rectangle::new(
                    [(x - 5, y - 5), (x + 5, y + 5)],
                    Palette99::pick(*id as usize),
                )
            });
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}
