use rand::ComplementaryMultiplyWithCarryGen;

#[derive(Clone,Debug)]
pub enum CellType {
    Fish,
    Shark,
}

#[derive(Copy,Clone,Debug)]
pub struct Particle {
    pub row: u32,
    pub col: u32,
    born: u32,
    ate: u32,
    reproduced: u32,
}

pub struct ParticleList {
    pub items: Vec<Particle>,
}

impl ParticleList {
    pub fn new() -> ParticleList {
        ParticleList { items: Vec::new() }
    }

    pub fn add(&mut self, p: Particle) {
        self.items.retain(|x| ((x.row != p.row) || (x.col != p.col)));
        self.items.push(p);
    }

    pub fn _len(&self) -> usize {
        self.items.len()
    }

    pub fn remove_at(&mut self, row: u32, col: u32) {
        self.items.retain(|x| x.row != row || x.col != col);
    }

    pub fn find(&self, row: u32, col: u32) -> Option<Particle> {
        for p in self.items.iter() {
            if p.row == row && p.col == col {
                return Some(p.clone());
            }
        }
        None
    }
}

pub struct WorldState {
    pub epoch: u32,
    pub time: f64,
    pub width_px: f64,
    pub height_px: f64,
    pub width: u32,
    pub height: u32,
    pub fish: u32,
    pub sharks: u32,
    pub fish_spawn: u32,
    pub shark_spawn: u32,
    pub shark_starve: u32,
    pub fish_list: ParticleList,
    pub shark_list: ParticleList,
    initialized: bool,
    pub seed: u32,
}

impl WorldState {
    pub fn new(width: u32, height: u32, fish: u32, sharks: u32, fish_spawn: u32, shark_spawn: u32, shark_starve: u32)-> WorldState {
    WorldState {
            epoch: 0,
            time: 0.0,
            width_px: 0.0,
            height_px: 0.0,
            width,
            height,
            fish,
            sharks,
            fish_spawn,
            shark_spawn,
            shark_starve,
            fish_list: ParticleList::new(),
            shark_list: ParticleList::new(),
            initialized: false,
            seed: 0,
        }
    }

    pub fn update_time(&mut self, increment: f64) {
        self.time += increment;
        self.epoch += 1;
        self.advance();
    }

    fn find_random_empty_cell(&self, rng: &mut ComplementaryMultiplyWithCarryGen) -> (u32,u32) {
        loop {
            let r = (rng.random() % self.height) as u32;
            let c = (rng.random() % self.width) as u32;
            if self.fish_list.find(r,c).is_none() && self.shark_list.find(r,c).is_none() {
                return (r,c);
            }
        }
    }

    fn initial_placement(&mut self, cell_type: Option<CellType>, rng: &mut ComplementaryMultiplyWithCarryGen) {
        let (r,c) = self.find_random_empty_cell(rng);
        let epoch = self.epoch;
        let fish_spawn = rng.random()  % self.fish_spawn; 
        let shark_spawn = rng.random()  % self.shark_spawn;
        let shark_starve = rng.random()  % self.shark_starve;

        match cell_type {
            Some(CellType::Fish) => {
                self.fish_list.add(Particle { row: r, col: c, born: epoch, reproduced: fish_spawn, ate: 0 }); 
            },
            Some(CellType::Shark) => { 
                self.shark_list.add(Particle { row: r, col: c, born: epoch, reproduced: shark_spawn, ate: shark_starve }); 
            }
            None => { }
        }
    }
    
    pub fn init(&mut self, seed: u32) {
        if !(self.initialized) {
            self.seed = seed;
            let mut rng = ComplementaryMultiplyWithCarryGen::new(self.seed);
            for _ in 0..self.fish {
                self.initial_placement(Some(CellType::Fish), &mut rng);
            }
            for _ in 0..self.sharks {
                self.initial_placement(Some(CellType::Shark), &mut rng);
            }
        }
        self.initialized = true;
    }

    pub fn resize(&mut self, width_px: f64, height_px: f64) {
        self.width_px = width_px;
        self.height_px = height_px;
    }

    #[inline]
    pub fn neighbors(&self, row: u32, col: u32) -> Vec<(u32,u32)> {
        let up = ((row + self.height + 1) % self.height, col);
        let down = ((row + self.height - 1) % self.height, col);
        let left = (row, (col + self.width - 1) % self.width);
        let right = (row, (col + self.width + 1) % self.width);
        vec!(up, down, left, right)
    }

    pub fn advance(&mut self) {
        let mut next_fish_list = ParticleList::new();
        let mut next_shark_list = ParticleList::new();
        let mut rng = ComplementaryMultiplyWithCarryGen::new(self.time as u32);

        for f in self.fish_list.items.iter() {
            let n = self.neighbors(f.row, f.col);
            let mut move_targets = Vec::new();
            for &(r,c) in n.iter() {
                if self.fish_list.find(r,c).is_none() && self.shark_list.find(r,c).is_none() && next_fish_list.find(r,c).is_none() {
                    move_targets.push((r,c));
                }
            }
            
            match move_targets.len() {
                0 => {
                    // age it in place
                    let next_fish = Particle { row: f.row, col: f.col, ate: 0, born: f.born, reproduced: f.reproduced + 1 };
                    next_fish_list.items.push(next_fish);
                    },
                _ => { // pick a random target, move the fish to it and age it
                    let move_target_index =  rng.random() as usize % move_targets.len();
                    let (r,c) = move_targets[move_target_index];
                    if f.reproduced+1 > self.fish_spawn {
                        let baby_fish = Particle { row: f.row, col: f.col, born: self.epoch, ate: 0, reproduced: 0 };
                        next_fish_list.items.push(baby_fish);
                    let next_fish = Particle { row: r, col: c, ate: 0, born: f.born, reproduced: 0 };
                    next_fish_list.items.push(next_fish);
                    } else {
                        let next_fish = Particle { row: r, col: c, ate: 0, born: f.born, reproduced: f.reproduced+1 };
                        next_fish_list.items.push(next_fish);
                    }
                }
            }

        }

        for s in self.shark_list.items.iter() {
            let n = self.neighbors(s.row, s.col);
            let mut eat_targets = Vec::new();
            for &(r,c) in n.iter() {
                if (next_fish_list.find(r,c).is_some()) && self.shark_list.find(r,c).is_none() {
                    eat_targets.push((r,c));
                }
            }

            match eat_targets.len() {
                0 => {
                        let mut move_targets = Vec::new();
                        for &(r,c) in n.iter() {
                            if self.shark_list.find(r,c).is_none() && next_shark_list.find(r,c).is_none() {
                                move_targets.push((r,c));
                            }
                        }
                        match move_targets.len() {
                            0 => { // age in place
                                if s.ate+1 < self.shark_starve {
                                    let next_shark = Particle { row: s.row, col: s.col, ate: s.ate+1, born: s.born, reproduced: s.reproduced + 1 };
                                    next_shark_list.items.push(next_shark);
                                }
                            },
                            _ => { // Move and reproduce if necessary
                                    let move_target_index = rng.random() as usize % move_targets.len();
                                    let (r,c) = move_targets[move_target_index];
                                    if s.reproduced+1 > self.shark_spawn {
                                        let baby_shark = Particle { row: s.row, col: s.col, born: self.epoch, ate: 0, reproduced: 0 };
                                        next_shark_list.items.push(baby_shark);
                                        if s.ate+1 < self.shark_starve {
                                            let next_shark = Particle { row: r, col: c, ate: s.ate+1, born: s.born, reproduced: 0 };
                                            next_shark_list.items.push(next_shark);
                                        }   
                                    }
                                    else {
                                        if s.ate+1 < self.shark_starve {
                                            let next_shark = Particle { row: r, col: c, ate: s.ate+1, born: s.born, reproduced: s.reproduced+1 };
                                            next_shark_list.items.push(next_shark);
                                        }
                                    }
                                }
                            };
                        },
                _ => { // pick a random, move it, eat the target and age it
                        let eat_target_index =  rng.random() as usize % eat_targets.len();
                        let (r,c) = eat_targets[eat_target_index];
                        
                        // eat
                        next_fish_list.remove_at(r,c);

                        if s.reproduced+1 > self.shark_spawn {
                            let baby_shark = Particle { row: s.row, col: s.col, born: self.epoch, ate: 0, reproduced: 0 };
                            next_shark_list.items.push(baby_shark);
                            let next_shark = Particle { row: r, col: c, ate: 0, born: s.born, reproduced: 0 };
                            next_shark_list.items.push(next_shark);
                        } else {
                            let next_shark = Particle { row: r, col: c, ate: 0, born: s.born, reproduced: s.reproduced+1 };
                            next_shark_list.items.push(next_shark);
                        }
                    }
            };
        }
        self.fish_list = next_fish_list;
        self.shark_list = next_shark_list;
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_initial_placement() {
        let mut w: WorldState = WorldState::new(5,5,1,1,1,1,1);
        let mut rng = ComplementaryMultiplyWithCarryGen::new(42 as u32);

        w.initial_placement(Some(CellType::Fish), &mut rng);
        w.initial_placement(Some(CellType::Shark), &mut rng);

        assert!(w.fish_list._len() == 1);
        assert!(w.shark_list._len() == 1);
    }

    #[test]
    fn test_neighbors() {
        let w: WorldState = WorldState::new(5,5,1,1,1,1,1);
        let mut corners = Vec::new();
        corners.push(w.neighbors(0,0));
        corners.push(w.neighbors(0, w.width - 1));
        corners.push(w.neighbors(w.height - 1, 0));
        corners.push(w.neighbors(w.height - 1, w.width - 1));

        for v in corners.iter() {
            assert!(v.len() == 4);
        }
        
        let fix = vec![(4,0),(1,0),(0,4),(0,1)];
        for f in fix.iter() {
            assert!(corners[0].contains(f));
        }
        let fix = vec![(4,4),(1,4),(0,3),(0,0)];
        for f in fix.iter() {
            assert!(corners[1].contains(f));
        }
        let fix = vec![(3,0),(0,0),(4,4),(4,1)];
        for f in fix.iter() {
            assert!(corners[2].contains(f));
        }
        let fix = vec![(3,4),(0,4),(4,3),(4,0)];
        for f in fix.iter() {
            assert!(corners[3].contains(f));
        }
    }

    #[test]
    fn test_init() {
        let mut w: WorldState = WorldState::new(5,5,1,1,1,1,1);
        assert!(!w.initialized);
        w.init(42);
        assert!(w.initialized);

        assert!(w.fish_list._len() == 1);
        assert!(w.shark_list._len() == 1);

    }

    #[test]
    fn find_random_empty_cell() {
        let w: WorldState = WorldState::new(5,5,1,1,1,1,1);
        let mut rng = ComplementaryMultiplyWithCarryGen::new(42 as u32);
        let (r,c) = w.find_random_empty_cell(&mut rng);
        assert!(w.fish_list.find(r,c).is_none());
        assert!(w.shark_list.find(r,c).is_none());
        assert!(w.fish_list._len() == 0);
        assert!(w.shark_list._len() == 0);

        let mut w: WorldState = WorldState::new(5,5,1,1,1,1,1);
        for i in 0..24 {
            w.fish_list.add( Particle { row: i / 5, col: i % 5, born: 0, ate: 0, reproduced: 0 } )
        }
        assert!(w.fish_list._len() == 24);
        assert!(w.fish_list.find(4,4).is_none());
        let (r,c) = w.find_random_empty_cell(&mut rng);
        assert!(r == 4);
        assert!(c == 4);
        assert!(w.shark_list.find(r,c).is_none());
    }

    #[test]
    fn test_particle_list_remove_at() {
        let mut p = ParticleList::new();
        p.add( Particle { row: 0, col: 0, ate: 0, reproduced: 0, born: 0} );
        p.add( Particle { row: 1, col: 0, ate: 0, reproduced: 0, born: 0} );
        p.add( Particle { row: 0, col: 1, ate: 0, reproduced: 0, born: 0} );
        p.add( Particle { row: 1, col: 1, ate: 0, reproduced: 0, born: 0} );
        assert!(p._len() == 4);
        p.remove_at(1,1);
        assert!(p._len() == 3);
        assert!(p.find(1,1).is_none());
    }
}
