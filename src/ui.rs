use crate::player::Player;
use ctru::prelude::*;

pub fn draw_player(console: &Console, player: &Player, active: bool) {
    console.select();
    print!("\x1b[2J");
    println!("\n\n  === {} ===\n", player.name);
    println!("  Life: {}", player.life);
    if active {
        println!("\n   >> ACTIVE <<");
    } else {
        println!("\n");
    }
    println!("\n\n  UP/DOWN: life\n  L/R: switch\n  SELECT: reset\n  START: quit");
}
