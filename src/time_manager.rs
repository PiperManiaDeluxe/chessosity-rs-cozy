use crate::eval_count_material::eval_count_material;
use cozy_chess::{Board, Color};

pub fn manage_time(game: &Board, wtime: u64, btime: u64, winc: u64, binc: u64) -> u64 {
    let move_side = game.side_to_move();
    let move_side_material =
        -eval_count_material(game) * if move_side == Color::White { 1 } else { -1 };

    let time_left_naive = if move_side == Color::White {
        wtime / 20 + winc / 2
    } else {
        btime / 20 + binc / 2
    };

    // Use more time the worse the position is
    let material_worst = -900;
    let material_good = 200;
    // 0 when material_worst, 1 when material_good
    let material_lerp = (move_side_material as f32 - material_worst as f32)
        / (material_good as f32 - material_worst as f32);
    let material_lerp = material_lerp.clamp(0.0, 1.0);
    let material_loss_bonus = if move_side_material > material_good {
        1.0
    } else {
        lerp(
            1.0,
            2.5,
            material_lerp,
        )
    };

    // Use less time in the opening
    let game_moves = game.fullmove_number();
    let opening_progress = (game_moves as f32 / 7.0).clamp(0.0, 1.0);
    let opening_bonus = lerp(0.25, 1.0, opening_progress);

    (time_left_naive as f32 * material_loss_bonus * opening_bonus) as u64
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}
