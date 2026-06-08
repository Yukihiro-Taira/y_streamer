use rand::seq::SliceRandom;
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    prelude::Color,
    style::Style,
    widgets::Clear,
};

use crate::app::App;

const FPS: f32 = 18.0;
const GRAVITY: f32 = 9.81;
const HEAD: char = '▄';
const TAIL: char = '│';
const EXPLOSION_CHARS: [char; 3] = ['+', '*', '•'];
const LAUNCH_GAP: u64 = 6;
const LAUNCH_COUNT: usize = 6;
const ROCKET_ASCENT_TICKS: u64 = 16;
const EXPLOSION_PARTICLES: usize = 56;
const COLORS: [Color; 5] = [
    Color::Rgb(168, 100, 253),
    Color::Rgb(41, 205, 255),
    Color::Rgb(120, 255, 68),
    Color::Rgb(255, 113, 141),
    Color::Rgb(253, 255, 106),
];

#[derive(Clone, Copy, Debug)]
struct Projectile {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    dt: f32,
}

impl Projectile {
    fn new(x: f32, y: f32, vx: f32, vy: f32, ax: f32, ay: f32) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            ax,
            ay,
            dt: 1.0 / FPS,
        }
    }

    fn update(&mut self) {
        self.x += self.vx * self.dt;
        self.y += self.vy * self.dt;
        self.vx += self.ax * self.dt;
        self.vy += self.ay * self.dt;
    }
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    glyph: char,
    color: Color,
    physics: Projectile,
    shooting: bool,
    tail: Option<char>,
}

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    if !app.should_celebrate() {
        return;
    }

    frame.render_widget(Clear, area);
    let inner = area;

    if inner.width < 10 || inner.height < 10 {
        return;
    }

    let mut particles = simulate_particles(app.frame_tick(), inner);
    render_particles(frame.buffer_mut(), inner, &mut particles);
}

fn simulate_particles(tick: u64, area: Rect) -> Vec<Particle> {
    let mut particles = Vec::new();

    for launch in 0..LAUNCH_COUNT {
        let start_tick = launch as u64 * LAUNCH_GAP;
        let local_tick = tick.saturating_sub(start_tick);
        if local_tick > ROCKET_ASCENT_TICKS + 18 {
            continue;
        }

        let base_x = launch_x(area, launch);
        let mut rocket = Particle {
            glyph: HEAD,
            color: sample_color(launch as usize),
            physics: Projectile::new(
                base_x,
                area.height.saturating_sub(1) as f32,
                0.0,
                -(22.0 + launch as f32 * 1.6),
                0.0,
                GRAVITY,
            ),
            shooting: true,
            tail: Some(TAIL),
        };

        for _ in 0..local_tick.min(ROCKET_ASCENT_TICKS) {
            rocket.physics.update();
        }

        if local_tick < ROCKET_ASCENT_TICKS {
            particles.push(rocket);
            continue;
        }

        let explosion_tick = local_tick - ROCKET_ASCENT_TICKS;
        let explosion_speed = 13.0 + launch as f32 * 0.7;
        let vertical_scale = 0.55 + stable_unit(launch as u64, 800) * 0.65;
        let horizontal_scale = 0.8 + stable_unit(launch as u64, 801) * 0.5;
        for i in 0..EXPLOSION_PARTICLES {
            let base_angle = i as f32 * std::f32::consts::TAU / EXPLOSION_PARTICLES as f32;
            let angle_jitter = (stable_unit(launch as u64, i as u64) - 0.5) * 0.28;
            let angle = base_angle + angle_jitter;
            let speed_variation = 0.72 + stable_unit(launch as u64 + 17, i as u64 + 37) * 0.75;
            let drift = (stable_unit(launch as u64 + 99, i as u64 + 7) - 0.5) * 1.6;
            let mut p = Particle {
                glyph: sample_glyph(i as usize + launch as usize),
                color: rocket.color,
                physics: Projectile::new(
                    rocket.physics.x,
                    rocket.physics.y,
                    angle.cos() * explosion_speed * speed_variation * horizontal_scale + drift,
                    angle.sin() * explosion_speed * speed_variation * vertical_scale,
                    0.0,
                    GRAVITY,
                ),
                shooting: false,
                tail: None,
            };

            for _ in 0..explosion_tick {
                p.physics.update();
            }
            particles.push(p);
        }
    }

    particles
}

fn render_particles(buf: &mut Buffer, area: Rect, particles: &mut [Particle]) {
    buf.set_style(area, Style::new().bg(Color::Rgb(8, 12, 24)));

    for particle in particles.iter() {
        let x = area.x + particle.physics.x.max(0.0).floor() as u16;
        let y = area.y + particle.physics.y.max(0.0).floor() as u16;

        if x >= area.x + area.width || y >= area.y + area.height {
            continue;
        }

        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(particle.glyph);
            cell.set_style(Style::new().fg(particle.color));
        }

        if particle.shooting {
            let trail = (-particle.physics.vy).max(0.0) as u16;
            for i in 1..trail.min(8) {
                let ty = y.saturating_add(i);
                if ty >= area.y + area.height {
                    break;
                }
                if let Some(cell) = buf.cell_mut((x, ty)) {
                    cell.set_char(particle.tail.unwrap_or(TAIL));
                    cell.set_style(Style::new().fg(particle.color));
                }
            }
        }
    }
}

fn launch_x(area: Rect, launch: usize) -> f32 {
    let anchors = [0.12_f32, 0.28, 0.46, 0.62, 0.78, 0.88];
    area.width as f32 * anchors[launch % anchors.len()]
}

fn sample_color(seed: usize) -> Color {
    let mut rng = rand::thread_rng();
    *COLORS.choose(&mut rng).unwrap_or(&COLORS[seed % COLORS.len()])
}

fn sample_glyph(seed: usize) -> char {
    let mut rng = rand::thread_rng();
    *EXPLOSION_CHARS
        .choose(&mut rng)
        .unwrap_or(&EXPLOSION_CHARS[seed % EXPLOSION_CHARS.len()])
}

fn stable_unit(a: u64, b: u64) -> f32 {
    let mut x = a.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ b.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    (x as f64 / u64::MAX as f64) as f32
}
