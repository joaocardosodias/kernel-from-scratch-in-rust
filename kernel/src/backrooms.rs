#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
use crate::drivers::vga::{get_height, get_pitch, get_width};

const PI: f32 = core::f32::consts::PI;
const FOV: f32 = core::f32::consts::FRAC_PI_3;

const MAP_WIDTH: usize = 32;
const MAP_HEIGHT: usize = 32;
const MAP: [u8; 1024] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 3, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1,
    1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1,
    1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1,
    1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 3, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1,
    1, 1, 1, 3, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

const LIGHT_MAP: [u8; 1024] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn get_static_light_wall(x: f32, y: f32) -> f32 {
    let cx = x as i32;
    let cy = y as i32;
    let mut max_light = 0.0f32;

    for dy in -2..=2 {
        let ny = cy + dy;
        if ny < 0 || ny >= 32 {
            continue;
        }
        for dx in -2..=2 {
            let nx = cx + dx;
            if nx < 0 || nx >= 32 {
                continue;
            }
            if LIGHT_MAP[(ny as usize) * 32 + (nx as usize)] == 1 {
                let lx = x - (nx as f32 + 0.5);
                let ly = y - (ny as f32 + 0.5);
                let dist_sq = lx * lx + ly * ly;
                let dist = float_sqrt(dist_sq);
                let intensity = (1.0 - dist / 2.5).max(0.0);
                let light = intensity * intensity;
                if light > max_light {
                    max_light = light;
                }
            }
        }
    }
    max_light
}

static mut BACKBUFFER: [u32; 480 * 270] = [0; 480 * 270];
pub static mut GAME_KEYS: [bool; 256] = [false; 256];
pub static mut GAME_MOUSE_BUF: [u8; 256] = [0; 256];
pub static mut GAME_MOUSE_HEAD: usize = 0;
pub static mut GAME_MOUSE_TAIL: usize = 0;
pub static mut IN_GAME: bool = false;

fn float_sin(mut x: f32) -> f32 {
    while x > PI {
        x -= 2.0 * PI;
    }
    while x < -PI {
        x += 2.0 * PI;
    }
    if x < 0.0 {
        -float_sin(-x)
    } else {
        16.0 * x * (PI - x) / (5.0 * PI * PI - 4.0 * x * (PI - x))
    }
}

fn float_cos(x: f32) -> f32 { float_sin(x + PI / 2.0) }

fn float_sqrt(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut y = x;
    for _ in 0..10 {
        y = 0.5 * (y + x / y);
    }
    y
}

fn float_atan2(y: f32, x: f32) -> f32 {
    let half_pi = core::f32::consts::FRAC_PI_2;
    if x == 0.0 {
        if y > 0.0 {
            return half_pi;
        }
        if y < 0.0 {
            return -half_pi;
        }
        return 0.0;
    }
    let z = y / x;
    let mut abs_z = z;
    if abs_z < 0.0 {
        abs_z = -abs_z;
    }
    let atan = if abs_z <= 1.0 {
        z / (1.0 + 0.28 * z * z)
    } else if z > 0.0 {
        half_pi - z / (z * z + 0.28)
    } else {
        -half_pi - z / (z * z + 0.28)
    };
    if x < 0.0 {
        if y >= 0.0 {
            return atan + PI;
        }
        return atan - PI;
    }
    atan
}

fn draw_pixel(x: u32, y: u32, color: u32) {
    let width = get_width();
    let height = get_height();
    if x >= width || y >= height {
        return;
    }
    let pitch = get_pitch();
    let offset = (y * pitch + x * 4) as usize;
    unsafe {
        *((0xA00000 + offset) as *mut u32) = color;
    }
}

fn can_move_to(x: f32, y: f32) -> bool {
    let cx = x as usize;
    let cy = y as usize;
    if cx >= MAP_WIDTH || cy >= MAP_HEIGHT {
        return false;
    }
    MAP[cy * MAP_WIDTH + cx] == 0 || MAP[cy * MAP_WIDTH + cx] == 2
}

fn can_move_to_radius(x: f32, y: f32, r: f32) -> bool {
    can_move_to(x - r, y - r)
        && can_move_to(x + r, y - r)
        && can_move_to(x - r, y + r)
        && can_move_to(x + r, y + r)
}

fn show_title_screen() -> bool {
    for y in 0..1080 {
        for x in 0..1920 {
            draw_pixel(x, y, 0x887733);
        }
    }

    for y in 50..1030 {
        draw_pixel(50, y, 0x554422);
        draw_pixel(1870, y, 0x554422);
    }
    for x in 50..1870 {
        draw_pixel(x, 50, 0x554422);
        draw_pixel(x, 1030, 0x554422);
    }

    let title = b"THE BACKROOMS";
    let subtitle = b"CARTEIRA DE TRABALHO EDITION";
    let play_msg = b"Pressione ENTER para Jogar";
    let quit_msg = b"Pressione Q para Sair";

    let x_title = (1920 - title.len() as u32 * 16) / 2;
    let x_sub = (1920 - subtitle.len() as u32 * 16) / 2;
    let x_play = (1920 - play_msg.len() as u32 * 16) / 2;
    let x_quit = (1920 - quit_msg.len() as u32 * 16) / 2;

    for (i, &byte) in title.iter().enumerate() {
        draw_str_byte(byte, x_title + (i as u32 * 16), 300, 0xFFFFFF, 0x887733);
    }
    for (i, &byte) in subtitle.iter().enumerate() {
        draw_str_byte(byte, x_sub + (i as u32 * 16), 380, 0xFFCC33, 0x887733);
    }
    for (i, &byte) in play_msg.iter().enumerate() {
        draw_str_byte(byte, x_play + (i as u32 * 16), 600, 0xFFFFFF, 0x887733);
    }
    for (i, &byte) in quit_msg.iter().enumerate() {
        draw_str_byte(byte, x_quit + (i as u32 * 16), 680, 0xCCCCCC, 0x887733);
    }

    loop {
        if let Some(ascii) = poll_key() {
            if ascii == 10 {

                return true;
            }
            if ascii == b'q' || ascii == b'Q' {
                return false;
            }
        }
        for _ in 0..10_000 {
            core::hint::spin_loop();
        }
    }
}

pub fn play_game() {
    unsafe {
        GAME_KEYS = [false; 256];
        GAME_MOUSE_HEAD = 0;
        GAME_MOUSE_TAIL = 0;
        IN_GAME = true;
        core::arch::asm!("sti");
    }
    crate::drivers::mouse::enable();

    if show_title_screen() {
        play_game_inner();
    }

    crate::drivers::mouse::disable();
    unsafe {
        core::arch::asm!("cli");
        GAME_KEYS = [false; 256];
        IN_GAME = false;
    }
}

fn play_game_inner() {
    let mut px = 1.5f32;
    let mut py = 1.5f32;
    let mut pa = 0.0f32;
    let mut mx = 29.5f32;
    let mut my = 29.5f32;
    let mut z_buffer = [999.0f32; 480];
    let mut mouse_cycle = 0;
    let mut mouse_packet = [0u8; 3];

    let mut ceiling_table = [0.0f32; 135];
    for y in 0..135 {
        ceiling_table[y] = 135.0 / (135.0 - y as f32);
    }
    let mut floor_table = [0.0f32; 135];
    for y in 135..270 {
        floor_table[y - 135] = 135.0 / (y as f32 - 135.0);
    }

    let mut last_px = -999.0f32;
    let mut last_py = -999.0f32;
    let mut last_pa = -999.0f32;
    let mut last_mx = -999.0f32;
    let mut last_my = -999.0f32;
    let mut first_frame = true;
    let mut frame_count = 0u32;

    loop {
        loop {
            let maybe_byte = unsafe {
                let head = core::ptr::read_volatile(core::ptr::addr_of!(GAME_MOUSE_HEAD));
                let tail = core::ptr::read_volatile(core::ptr::addr_of!(GAME_MOUSE_TAIL));
                if head != tail {
                    let b = GAME_MOUSE_BUF[tail];
                    let next_tail = (tail + 1) % 256;
                    core::ptr::write_volatile(core::ptr::addr_of_mut!(GAME_MOUSE_TAIL), next_tail);
                    Some(b)
                } else {
                    None
                }
            };
            if let Some(byte) = maybe_byte {
                match mouse_cycle {
                    0 =>
                        if (byte & 0x08) != 0 {
                            mouse_packet[0] = byte;
                            mouse_cycle = 1;
                        },
                    1 => {
                        mouse_packet[1] = byte;
                        mouse_cycle = 2;
                    },
                    2 => {
                        mouse_packet[2] = byte;
                        mouse_cycle = 0;
                        let flags = mouse_packet[0];
                        let mut dx = mouse_packet[1] as i32;
                        if (flags & 0x10) != 0 {
                            dx -= 256;
                        }
                        let mouse_sensitivity = 0.0008f32;
                        pa += (dx as f32) * mouse_sensitivity;
                    },
                    _ => mouse_cycle = 0,
                }
            } else {
                break;
            }
        }
        let key_active = |k: u8| -> bool {
            unsafe {
                let ptr = &GAME_KEYS[k as usize] as *const bool;
                core::ptr::read_volatile(ptr)
            }
        };
        if key_active(b'q') || key_active(b'Q') {
            crate::drivers::vga::WRITER.lock().clear_screen();
            return;
        }
        let move_speed = 0.09f32;
        if key_active(b'w') || key_active(b'W') {
            let nx = px + float_cos(pa) * move_speed;
            let ny = py + float_sin(pa) * move_speed;
            if can_move_to(nx, py) {
                px = nx;
            }
            if can_move_to(px, ny) {
                py = ny;
            }
        }
        if key_active(b's') || key_active(b'S') {
            let nx = px - float_cos(pa) * move_speed;
            let ny = py - float_sin(pa) * move_speed;
            if can_move_to(nx, py) {
                px = nx;
            }
            if can_move_to(px, ny) {
                py = ny;
            }
        }
        if key_active(b'a') || key_active(b'A') {
            let nx = px - float_cos(pa + PI / 2.0) * move_speed;
            let ny = py - float_sin(pa + PI / 2.0) * move_speed;
            if can_move_to(nx, py) {
                px = nx;
            }
            if can_move_to(px, ny) {
                py = ny;
            }
        }
        if key_active(b'd') || key_active(b'D') {
            let nx = px + float_cos(pa + PI / 2.0) * move_speed;
            let ny = py + float_sin(pa + PI / 2.0) * move_speed;
            if can_move_to(nx, py) {
                px = nx;
            }
            if can_move_to(px, ny) {
                py = ny;
            }
        }

        let moved = (px - last_px).abs() > 0.001
            || (py - last_py).abs() > 0.001
            || (pa - last_pa).abs() > 0.001
            || (mx - last_mx).abs() > 0.001
            || (my - last_my).abs() > 0.001
            || first_frame;

        if moved {
            first_frame = false;
            last_px = px;
            last_py = py;
            last_pa = pa;
            last_mx = mx;
            last_my = my;

            const WALL_TEX: &[u8; 128 * 128 * 3] = include_bytes!("assets/tex_wall.bin");
            const CEIL_TEX: &[u8; 128 * 128 * 3] = include_bytes!("assets/tex_ceil.bin");
            const FLOOR_TEX: &[u8; 128 * 128 * 3] = include_bytes!("assets/tex_floor.bin");
            const MONSTER_TEX: &[u8; 256 * 256 * 3] = include_bytes!("assets/tex_monster.bin");

            for sx in 0..480u32 {
                let ra = pa - FOV / 2.0 + (sx as f32 / 480.0) * FOV;
                let cos_ra = float_cos(ra);
                let sin_ra = float_sin(ra);

                let r_dx = if cos_ra == 0.0 {
                    1e30
                } else {
                    (1.0 / cos_ra).abs()
                };
                let r_dy = if sin_ra == 0.0 {
                    1e30
                } else {
                    (1.0 / sin_ra).abs()
                };

                let mut map_x = px as i32;
                let mut map_y = py as i32;

                let mut side_dist_x = 0.0;
                let mut side_dist_y = 0.0;

                let step_x: i32;
                let step_y: i32;

                if cos_ra < 0.0 {
                    step_x = -1;
                    side_dist_x = (px - map_x as f32) * r_dx;
                } else {
                    step_x = 1;
                    side_dist_x = (map_x as f32 + 1.0 - px) * r_dx;
                }

                if sin_ra < 0.0 {
                    step_y = -1;
                    side_dist_y = (py - map_y as f32) * r_dy;
                } else {
                    step_y = 1;
                    side_dist_y = (map_y as f32 + 1.0 - py) * r_dy;
                }

                let mut wall_type = 0;
                let mut side = 0;
                let mut steps = 0;
                while steps < 50 {
                    if side_dist_x < side_dist_y {
                        side_dist_x += r_dx;
                        map_x += step_x;
                        side = 0;
                    } else {
                        side_dist_y += r_dy;
                        map_y += step_y;
                        side = 1;
                    }
                    if map_x < 0
                        || map_x >= MAP_WIDTH as i32
                        || map_y < 0
                        || map_y >= MAP_HEIGHT as i32
                    {
                        break;
                    }
                    let cell = MAP[(map_y as usize) * MAP_WIDTH + (map_x as usize)];
                    if cell > 0 {
                        wall_type = cell;
                        break;
                    }
                    steps += 1;
                }

                let mut d = if side == 0 {
                    side_dist_x - r_dx
                } else {
                    side_dist_y - r_dy
                };
                if d < 0.01 {
                    d = 0.01;
                }

                let mut diff = ra - pa;
                while diff > PI {
                    diff -= 2.0 * PI;
                }
                while diff < -PI {
                    diff += 2.0 * PI;
                }
                let cos_diff = float_cos(diff);
                d *= cos_diff;
                z_buffer[sx as usize] = d;

                let h = (270.0 / d) as i32;
                let y1 = (270 - h) / 2;
                let y2 = y1 + h;

                let f = 0.39 / (1.0 + d * d * 0.25);
                let mut shadow = f;
                if side == 1 {
                    shadow *= 0.75;
                }

                let wall_x = if side == 0 {
                    py + d * sin_ra
                } else {
                    px + d * cos_ra
                };
                let wall_u = wall_x - (wall_x as i32) as f32;

                let mut edge_shadow = 1.0f32;
                let edge_dist = wall_u.min(1.0 - wall_u);
                if edge_dist < 0.12 {
                    let has_corner = if side == 0 {
                        if wall_u < 0.12 {

                            map_y > 0 && MAP[(map_y as usize - 1) * MAP_WIDTH + map_x as usize] == 0
                        } else {

                            map_y < (MAP_HEIGHT as i32 - 1)
                                && MAP[(map_y as usize + 1) * MAP_WIDTH + map_x as usize] == 0
                        }
                    } else {
                        if wall_u < 0.12 {

                            map_x > 0 && MAP[map_y as usize * MAP_WIDTH + (map_x as usize - 1)] == 0
                        } else {

                            map_x < (MAP_WIDTH as i32 - 1)
                                && MAP[map_y as usize * MAP_WIDTH + (map_x as usize + 1)] == 0
                        }
                    };
                    if has_corner {
                        edge_shadow = 0.2 + 0.8 * (edge_dist / 0.12);
                    }
                }

                let rx = px + d * cos_ra;
                let ry = py + d * sin_ra;
                let static_light = get_static_light_wall(rx, ry);
                let wall_shadow = (shadow + 0.68 * static_light) * edge_shadow;

                let inv_cos = 1.0 / cos_diff;

                let limit_y1 = y1.max(0);
                for y in 0..limit_y1 {
                    let dist = ceiling_table[y as usize] * inv_cos;
                    let cx_pos = px + cos_ra * dist;
                    let cy_pos = py + sin_ra * dist;
                    let tx = ((cx_pos * 128.0) as usize) % 128;
                    let ty = ((cy_pos * 128.0) as usize) % 128;
                    let off = (ty * 128 + tx) * 3;

                    let final_f = 0.39 / (1.0 + dist * dist * 0.25);

                    let r = ((CEIL_TEX[off] as f32) * final_f) as u32;
                    let g = ((CEIL_TEX[off + 1] as f32) * final_f) as u32;
                    let b = ((CEIL_TEX[off + 2] as f32) * final_f) as u32;
                    let color = (r << 16) | (g << 8) | b;
                    unsafe {
                        BACKBUFFER[y as usize * 480 + sx as usize] = color;
                    }
                }

                let tex_u = (wall_u * 128.0) as usize;
                let tex_u = if tex_u >= 128 { 127 } else { tex_u };
                let inv_h = 1.0 / h as f32;
                let draw_y1 = y1.max(0);
                let draw_y2 = y2.min(270);
                for y in draw_y1..draw_y2 {
                    let wall_v = (y - y1) as f32 * inv_h;
                    let tex_v = (wall_v * 128.0) as usize;
                    let tex_v = if tex_v >= 128 { 127 } else { tex_v };
                    let off = (tex_v * 128 + tex_u) * 3;
                    let final_f = wall_shadow;

                    let (r, g, b) = if wall_type == 2 {
                        let gr = ((WALL_TEX[off] as f32 * 0.2 + 180.0) * final_f) as u32;
                        let gg = ((WALL_TEX[off + 1] as f32 * 0.2 + 180.0) * final_f) as u32;
                        let gb = ((WALL_TEX[off + 2] as f32 * 0.2) * final_f) as u32;
                        (gr.min(255), gg.min(255), gb.min(255))
                    } else if wall_type == 3 {
                        if tex_u >= 25 && tex_u <= 103 && tex_v >= 15 {
                            let brightness = (WALL_TEX[off] as f32
                                + WALL_TEX[off + 1] as f32
                                + WALL_TEX[off + 2] as f32)
                                / 3.0
                                / 255.0;
                            let mut dr = (100.0 * brightness * final_f) as u32;
                            let mut dg = (60.0 * brightness * final_f) as u32;
                            let mut db = (30.0 * brightness * final_f) as u32;

                            if tex_u >= 85 && tex_u <= 90 && tex_v >= 70 && tex_v <= 75 {
                                dr = (220.0 * final_f) as u32;
                                dg = (180.0 * final_f) as u32;
                                db = (50.0 * final_f) as u32;
                            }

                            if tex_u == 25 || tex_u == 103 || tex_v == 15 {
                                dr = (50.0 * final_f) as u32;
                                dg = (30.0 * final_f) as u32;
                                db = (15.0 * final_f) as u32;
                            }
                            (dr, dg, db)
                        } else {
                            let r = ((WALL_TEX[off] as f32) * final_f) as u32;
                            let g = ((WALL_TEX[off + 1] as f32) * final_f) as u32;
                            let b = ((WALL_TEX[off + 2] as f32) * final_f) as u32;
                            (r, g, b)
                        }
                    } else {
                        let r = ((WALL_TEX[off] as f32) * final_f) as u32;
                        let g = ((WALL_TEX[off + 1] as f32) * final_f) as u32;
                        let b = ((WALL_TEX[off + 2] as f32) * final_f) as u32;
                        (r, g, b)
                    };
                    let color = (r << 16) | (g << 8) | b;
                    unsafe {
                        BACKBUFFER[y as usize * 480 + sx as usize] = color;
                    }
                }

                let start_y = y2.min(270);
                for y in start_y..270 {
                    let dist = floor_table[y as usize - 135] * inv_cos;
                    let fx_pos = px + cos_ra * dist;
                    let fy_pos = py + sin_ra * dist;
                    let tx = ((fx_pos * 128.0) as usize) % 128;
                    let ty = ((fy_pos * 128.0) as usize) % 128;
                    let off = (ty * 128 + tx) * 3;

                    let final_f = 0.39 / (1.0 + dist * dist * 0.25);

                    let r = ((FLOOR_TEX[off] as f32) * final_f) as u32;
                    let g = ((FLOOR_TEX[off + 1] as f32) * final_f) as u32;
                    let b = ((FLOOR_TEX[off + 2] as f32) * final_f) as u32;
                    let color = (r << 16) | (g << 8) | b;
                    unsafe {
                        BACKBUFFER[y as usize * 480 + sx as usize] = color;
                    }
                }
            }

            let mx_rel = mx - px;
            let my_rel = my - py;
            let m_dist = float_sqrt(mx_rel * mx_rel + my_rel * my_rel);
            let m_angle = float_atan2(my_rel, mx_rel);
            let mut diff = m_angle - pa;
            while diff > PI {
                diff -= 2.0 * PI;
            }
            while diff < -PI {
                diff += 2.0 * PI;
            }
            if diff >= -FOV / 2.0 && diff <= FOV / 2.0 {
                let m_screen_x = (240.0 + (diff / (FOV / 2.0)) * 240.0) as i32;
                let m_size = (360.0 / m_dist) as i32;
                let h = if m_size > 270 { 270 } else { m_size };
                let sprite_width = (h / 3).max(1);
                let start_x = m_screen_x - sprite_width;
                let end_x = m_screen_x + sprite_width;
                for sx in start_x..=end_x {
                    if sx >= 0 && sx < 480 && m_dist < z_buffer[sx as usize] {
                        let y1 = (270 - h) / 2;
                        let y2 = y1 + h;
                        let f = 1.0 / (1.0 + m_dist * m_dist * 0.05);

                        let dx_center = (sx as f32 - m_screen_x as f32) / (sprite_width as f32);

                        for y in y1..y2 {
                            let dy_center = (y as f32 - (y1 as f32 + y2 as f32) / 2.0)
                                / (h as f32 / 2.0).max(1.0);

                            let tx = (((dx_center + 1.0) / 2.0) * 256.0) as i32;
                            let ty = (((dy_center + 1.0) / 2.0) * 256.0) as i32;
                            let tx = tx.clamp(0, 255) as usize;
                            let ty = ty.clamp(0, 255) as usize;
                            let off = (ty * 256 + tx) * 3;

                            let r_tex = MONSTER_TEX[off] as u32;
                            let g_tex = MONSTER_TEX[off + 1] as u32;
                            let b_tex = MONSTER_TEX[off + 2] as u32;

                            let light_f = (f + 0.15).min(1.0);

                            let dx_c = (sx as f32 - 240.0) / 240.0;
                            let v_x = 1.0 - dx_c * dx_c * 0.25;
                            let dy_c = (y as f32 - 135.0) / 135.0;
                            let v_y = 1.0 - dy_c * dy_c * 0.25;
                            let vig = v_x * v_y;

                            let r = (((r_tex as f32) * light_f) * vig) as u32;
                            let g = (((g_tex as f32) * light_f) * vig) as u32;
                            let b = (((b_tex as f32) * light_f) * vig) as u32;
                            let final_color = (r << 16) | (g << 8) | b;

                            unsafe {
                                BACKBUFFER[y as usize * 480 + sx as usize] = final_color;
                            }
                        }
                    }
                }
            }
            blit_backbuffer();
        }

        let mut target_x = px;
        let mut target_y = py;

        let mx_idx = mx as usize;
        let my_idx = my as usize;
        let px_idx = px as usize;
        let py_idx = py as usize;

        if mx_idx < 32 && my_idx < 32 && px_idx < 32 && py_idx < 32 {
            if mx_idx != px_idx || my_idx != py_idx {
                let mut dist = [65535u16; 1024];
                let mut queue = [(0u8, 0u8); 1024];
                let mut head = 0;
                let mut tail = 0;

                dist[py_idx * 32 + px_idx] = 0;
                queue[head] = (px_idx as u8, py_idx as u8);
                head += 1;

                while tail < head {
                    let (cx, cy) = queue[tail];
                    tail += 1;

                    let current_dist = dist[cy as usize * 32 + cx as usize];

                    let neighbors = [
                        (cx as i32 - 1, cy as i32),
                        (cx as i32 + 1, cy as i32),
                        (cx as i32, cy as i32 - 1),
                        (cx as i32, cy as i32 + 1),
                    ];

                    for &(nx, ny) in &neighbors {
                        if nx >= 0 && nx < 32 && ny >= 0 && ny < 32 {
                            let idx = ny as usize * 32 + nx as usize;
                            if MAP[idx] == 0 || MAP[idx] == 2 {
                                if dist[idx] == 65535 {
                                    dist[idx] = current_dist + 1;
                                    queue[head] = (nx as u8, ny as u8);
                                    head += 1;
                                }
                            }
                        }
                    }
                }

                let mut min_dist = 65535u16;
                let mut best_nx = mx_idx;
                let mut best_ny = my_idx;

                let neighbors = [
                    (mx_idx as i32 - 1, my_idx as i32),
                    (mx_idx as i32 + 1, my_idx as i32),
                    (mx_idx as i32, my_idx as i32 - 1),
                    (mx_idx as i32, my_idx as i32 + 1),
                ];

                for &(nx, ny) in &neighbors {
                    if nx >= 0 && nx < 32 && ny >= 0 && ny < 32 {
                        let idx = ny as usize * 32 + nx as usize;
                        if dist[idx] < min_dist {
                            min_dist = dist[idx];
                            best_nx = nx as usize;
                            best_ny = ny as usize;
                        }
                    }
                }

                if min_dist < 65535 {
                    target_x = best_nx as f32 + 0.5;
                    target_y = best_ny as f32 + 0.5;
                }
            }
        }

        let m_speed = 0.075f32;
        let dx = target_x - mx;
        let dy = target_y - my;
        let m_dist = float_sqrt(dx * dx + dy * dy);
        if m_dist > 0.05 {
            let step_x = (dx / m_dist) * m_speed;
            let step_y = (dy / m_dist) * m_speed;
            let n_mx = mx + step_x;
            let n_my = my + step_y;
            if can_move_to_radius(n_mx, my, 0.2) {
                mx = n_mx;
            }
            if can_move_to_radius(mx, n_my, 0.2) {
                my = n_my;
            }
        }

        let actual_dx = px - mx;
        let actual_dy = py - my;
        let actual_dist = float_sqrt(actual_dx * actual_dx + actual_dy * actual_dy);

        if actual_dist < 0.4 {
            for y in 0..1080 {
                for x in 0..1920 {
                    draw_pixel(x, y, 0x880000);
                }
            }
            let msg = b"YOU WERE CAUGHT BY THE ENTITY!";
            let start_x = (1920 - msg.len() as u32 * 16) / 2;
            let start_y = 500;
            for (i, &byte) in msg.iter().enumerate() {
                draw_str_byte(byte, start_x + (i as u32 * 16), start_y, 0xFFFFFF, 0x880000);
            }
            loop {
                if let Some(ascii) = poll_key() {
                    if ascii == b'q' || ascii == b'Q' || ascii == 10 {
                        crate::drivers::vga::WRITER.lock().clear_screen();
                        return;
                    }
                }
                for _ in 0..1_000_000 {
                    core::hint::spin_loop();
                }
            }
        }
        let px_idx = px as usize;
        let py_idx = py as usize;
        if px_idx < MAP_WIDTH && py_idx < MAP_HEIGHT && MAP[py_idx * MAP_WIDTH + px_idx] == 2 {
            for y in 0..1080 {
                for x in 0..1920 {
                    draw_pixel(x, y, 0x008800);
                }
            }
            let msg = b"YOU ESCAPED THE BACKROOMS!";
            let start_x = (1920 - msg.len() as u32 * 16) / 2;
            let start_y = 500;
            for (i, &byte) in msg.iter().enumerate() {
                draw_str_byte(byte, start_x + (i as u32 * 16), start_y, 0xFFFFFF, 0x008800);
            }
            loop {
                if let Some(ascii) = poll_key() {
                    if ascii == b'q' || ascii == b'Q' || ascii == 10 {
                        crate::drivers::vga::WRITER.lock().clear_screen();
                        return;
                    }
                }
                for _ in 0..1_000_000 {
                    core::hint::spin_loop();
                }
            }
        }
        frame_count = frame_count.wrapping_add(1);
        for _ in 0..10_000 {
            core::hint::spin_loop();
        }
    }
}

fn poll_key() -> Option<u8> {
    unsafe {
        let status: u8;
        core::arch::asm!("in al, 0x64", out("al") status);
        if (status & 1) == 1 {
            let scancode: u8;
            core::arch::asm!("in al, 0x60", out("al") scancode);
            crate::drivers::keyboard::scancode_to_ascii(scancode)
        } else {
            None
        }
    }
}

fn draw_str_byte(c: u8, x: u32, y: u32, fg_color: u32, bg_color: u32) {
    const FONT_DATA: &[u8; 16384] = include_bytes!("assets/font_ter16x32.bin");
    let offset = (c as usize) * 64;
    for row in 0..32 {
        let byte1 = FONT_DATA[offset + row * 2];
        let byte2 = FONT_DATA[offset + row * 2 + 1];
        let row_data = ((byte1 as u32) << 8) | (byte2 as u32);
        for col in 0..16 {
            let bit = (row_data >> (15 - col)) & 1;
            let color = if bit == 1 { fg_color } else { bg_color };
            draw_pixel(x + col as u32, y + row as u32, color);
        }
    }
}

fn blit_backbuffer() {
    let pitch = get_pitch() as usize;
    let lfb = 0xA00000 as *mut u8;
    unsafe {
        for y in 0..270 {
            let src_row = &BACKBUFFER[y * 480..];
            for dy in 0..4 {
                let screen_y = y * 4 + dy;
                let mut dest_ptr = lfb.add(screen_y * pitch) as *mut u64;
                for x in 0..480 {
                    let color = src_row[x];
                    let color64 = ((color as u64) << 32) | (color as u64);
                    dest_ptr.write(color64);
                    dest_ptr.add(1).write(color64);
                    dest_ptr = dest_ptr.add(2);
                }
            }
        }
    }
}
