extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate float;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;


// please do not change these values for the time being
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const ROWS: i32 = 100;
const COLUMNS: i32 = 100;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    cell_state : [[f32;100];100],
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        //const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        //const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32;4] = [0.0,0.0,0.0,1.0];
        const WHITE: [f32;4] = [1.0,1.0,1.0,1.0];

        let square = rectangle::square(0.0, 0.0, f64::from(WIDTH/ROWS));
        let (x, y) = (args.window_size[0] / f64::from(COLUMNS) , args.window_size[1] / f64::from(ROWS));

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            for i in 0..self.cell_state.len(){
                for j in 0..self.cell_state[i].len(){
                    let transform = c
                        .transform
                        .trans(x*(i as f64),y*(j as f64));
                    let mut color: [f32;4] = WHITE;
                    for item in color.iter_mut() {
                        *item = (*item)*self.cell_state[i][j];
                    }
                    rectangle(color,square,transform,gl);

                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        for i in 0..self.cell_state.len(){
            for j in 0..self.cell_state[i].len(){
                let mut n_sum = 0.0;
                n_sum += self.cell_state[(i+1)%100][j];
                n_sum += self.cell_state[(i+99)%100][j];
                n_sum += self.cell_state[(i+1)%100][(j+1)%100];
                n_sum += self.cell_state[(i+99)%100][(j+1)%100];
                n_sum += self.cell_state[(i+1)%100][(j+99)%100];
                n_sum += self.cell_state[(i+99)%100][(j+99)%100];
                n_sum += self.cell_state[i][(j+1)%100];
                n_sum += self.cell_state[i][(j+99)%100];
                if n_sum>=2.0 && n_sum<=3.0 {
                    self.cell_state[i][j] += n_sum/3.0;
                    self.cell_state[i][j] %= 1.0;
                }
                else if n_sum<2.0{
                    self.cell_state[i][j] -= (2.0-n_sum)/(2.0*10.0) ;
                    self.cell_state[i][j] %= 1.0;
                }
                else{
                    self.cell_state[i][j] -= n_sum/(8.0*5.0);
                    self.cell_state[i][j] %= 1.0;
                }
            }
        }
    }
}

fn randomize_cell(cell: &mut [[f32;100];100]){
    for i in 0..cell.len(){
        for j in 0..cell[i].len(){
            use rand::Rng;
            cell[i][j] = rand::thread_rng().gen_range(0..2) as f32;
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("smooth-cellular-automata", [1000, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut init_state: [[f32;100];100] = [[0.0;100];100];
    randomize_cell(&mut init_state);
    let mut app = App {
        gl: GlGraphics::new(opengl),
        cell_state: init_state,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        //use std::{thread,time};
        //let sleep_time = time::Duration::from_millis(1000);
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        //thread::sleep(sleep_time);
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
