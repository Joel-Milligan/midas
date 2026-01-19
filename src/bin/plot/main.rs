use core::f32;
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, VecDeque};
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

/// Determines if the chart infinitely expands, or scrolls horizontally once it hits MAX_ROUNDS.
/// Expanding shows all historical data, but slows down the longer the simulation runs
const EXPANDING: bool = true;

/// Maximum number of rounds shown when chart is scrolling / not expanding
const MAX_ROUNDS: usize = 100_000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = BufferWrapper::new(vec![0u32; WIDTH * HEIGHT]);
    let mut window = Window::new("Midas", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut highest_balance = f32::MIN;
    let mut n_rounds = 0;

    let mut players = Vec::new();
    players.push(Player::new(
        0,
        10_000.0,
        Box::new(SimpleActionStrategy),
        Box::new(FlatBettingStrategy),
    ));
    players.push(Player::new(
        1,
        10_000.0,
        Box::new(SimpleActionStrategy),
        Box::new(HiLoCountingStrategy::new()),
    ));
    players.push(Player::new(
        2,
        10_000.0,
        Box::new(OptimalActionStrategy),
        Box::new(FlatBettingStrategy),
    ));
    players.push(Player::new(
        3,
        10_000.0,
        Box::new(OptimalActionStrategy),
        Box::new(HiLoCountingStrategy::new()),
    ));
    let mut game = Game::new(players);

    let mut data = HashMap::new();
    for id in game.players.iter().map(|p| p.id) {
        data.insert(id, VecDeque::new());
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..ROUNDS_PER_UPDATE {
            if game.players.iter().any(|p| p.balance > 0.0) {
                let _ = game.round();
                n_rounds += 1;
                for player in &mut game.players {
                    let balance = player.balance;
                    if balance > 0.0 {
                        data.get_mut(&player.id).unwrap().push_back(balance);
                        if balance > highest_balance {
                            highest_balance = balance;
                        }
                    }
                }
                if !EXPANDING && n_rounds > MAX_ROUNDS {
                    data.values_mut().for_each(|data| {
                        data.pop_front();
                    });
                }
            }
        }

        let min_x = if EXPANDING || n_rounds < MAX_ROUNDS {
            0
        } else {
            n_rounds - MAX_ROUNDS
        };

        let max_x = if EXPANDING {
            n_rounds
        } else {
            n_rounds.max(MAX_ROUNDS)
        };

        update_chart(
            buf.borrow_mut(),
            &data,
            &game.players,
            min_x,
            max_x,
            highest_balance,
        )?;
        window.update_with_buffer(buf.borrow(), WIDTH, HEIGHT)?;
    }
    Ok(())
}

fn update_chart(
    buf: &mut [u8],
    data: &HashMap<u8, VecDeque<f32>>,
    players: &Vec<Player>,
    min_x: usize,
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
        .build_cartesian_2d(min_x..max_x, 0.0..max_y)?;

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

    for id in players.iter().map(|p| p.id) {
        let player_name = match id {
            0 => "Simple/Flat",
            1 => "Simple/HiLo",
            2 => "Optimal/Flat",
            3 => "Optimal/HiLo",
            _ => unreachable!(),
        };

        chart
            .draw_series(LineSeries::new(
                (min_x..).zip(data[&id].iter()).map(|(a, b)| (a, *b)),
                Palette99::pick(id as usize),
            ))?
            .label(player_name)
            .legend(move |(x, y)| {
                Rectangle::new(
                    [(x - 5, y - 5), (x + 5, y + 5)],
                    Palette99::pick(id as usize),
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
