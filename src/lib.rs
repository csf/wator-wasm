#[macro_use]
extern crate lazy_static;

use std::os::raw::c_double;
use std::os::raw::c_uint;
use std::sync::Mutex;

mod world;
mod rand;
use self::world::WorldState;

extern "C" {
    fn clear_screen();
    fn draw_fish(_: c_double, _: c_double);
    fn draw_shark(_: c_double, _: c_double);
    fn draw_status(_: c_double, _: c_double);
    fn draw_debug(_: c_double, _: c_double);
}

lazy_static! {
    static ref WORLD_DATA: Mutex<WorldData> = Mutex::new(new_world_data(100, 50, 100, 20, 5, 3, 5));
}

pub struct WorldData {
    world_state: WorldState,
}

pub fn new_world_data(width: u32, height: u32, fish: u32, sharks: u32, fish_spawn: u32, shark_spawn: u32, shark_starve: u32) -> WorldData {
    WorldData {
        world_state: WorldState::new(width, height, fish, sharks, fish_spawn, shark_spawn, shark_starve),
    }
}
    
#[no_mangle]
pub extern "C" fn resize(width_px: c_double, height_px: c_double) {
    let data = &mut WORLD_DATA.lock().unwrap();
    data.world_state.resize(width_px as f64, height_px as f64);
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    let data = &mut WORLD_DATA.lock().unwrap();
    clear_screen();

    let mut f = 0;
    let mut s = 0;
    for fish in data.world_state.fish_list.items.iter() {
                let x = fish.col * 5;
                let y = fish.row * 5 + 30;
                f += 1;
                draw_fish(x as f64, y as f64);
    }
    for shark in data.world_state.shark_list.items.iter() {
                let x = shark.col * 5;
                let y = shark.row * 5 + 30;
                s += 1;
                draw_shark(x as f64, y as f64);
    }
     
    draw_status(f as f64, s as f64);
    draw_debug(data.world_state.time, (data.world_state.epoch as f64) / data.world_state.time as f64);
}

#[no_mangle]
pub extern "C" fn update(progress: c_double) {
    let data: &mut WorldData = &mut WORLD_DATA.lock().unwrap();
    data.world_state.update_time(progress);
}

#[no_mangle]
pub extern "C" fn init(seed: c_uint, width: c_uint, height: c_uint, fish: c_uint, sharks: c_uint, fish_spawn: c_uint, shark_spawn: c_uint, shark_starve: c_uint) {
    *WORLD_DATA.lock().unwrap() = new_world_data(width, height, fish, sharks, fish_spawn, shark_spawn, shark_starve);
    let data: &mut WorldData = &mut WORLD_DATA.lock().unwrap();
    data.world_state.init(seed);
}
