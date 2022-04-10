extern crate test;
use test::Bencher;
use crate::FluidSim;
use core_simd::*;
use std::cmp;

const grid_size: usize = 300;
const particle_count: usize = 2000;
const max_particles_per_cell: usize = 2;
const gravity: f32x2 = Simd::from_array([0.0, 0.3]);
const radius: f32 = 1.0;
const dist_squared_max: f32 = (radius + radius) * (radius + radius);


fn create_fluid_sim(add_points: bool) -> FluidSim {
    let mut f = FluidSim::new(grid_size, grid_size, max_particles_per_cell * 2);
    if add_points {
        let pts = f.generate_random_points(particle_count);
        f.add_points(&pts);
    }
    return f;
}

#[bench]
fn add_points(b: &mut Bencher) {
    let mut f = create_fluid_sim(false);
    let pts = f.generate_random_points(particle_count);
    b.iter(|| {
        f.add_points(&pts);
    });
}

#[bench]
fn add_points_simd(b: &mut Bencher) {
    let mut f = create_fluid_sim(false);
    let pts = f.generate_random_points(particle_count);
    b.iter(|| {
        f.add_points_simd(&pts);
    });
}

#[bench]
fn update_velocity_from_collisions(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.update_velocity_from_collisions();
    });
}

#[bench]
fn update_velocity_from_collisions_simd(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.update_velocity_from_collisions_simd();
    });
}

#[bench]
fn add_uniform_velocity(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.add_uniform_velocity(gravity);
    });
}

#[bench]
fn add_uniform_velocity_simd(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.add_uniform_velocity_simd(gravity);
    });
}


#[bench]
fn apply_velocity(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.apply_velocity(1.0);
    });
}

#[bench]
fn apply_velocity_simd(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.apply_velocity_simd(1.0);
    });
}

#[bench]
fn swap(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.swap();
    });
}

#[bench]
fn clear_next(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.clear_next(false);
    });
}

#[bench]
fn clear_next_simd(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.clear_next_simd(false);
    });
}


#[bench]
fn for_each_pos(b: &mut Bencher) {
    // THIS IS HORRIBLY SLOW! rethink how we do this
    // simulate rendering of particles
    let render_c = #[inline(always)] |pt: f32x2| {
        const scale: f32 = 1.0;
        const x_offset: f32 = 1.0;
        const y_offset: f32 = 1.0;
        let x2 = pt[0] * scale + x_offset;
        let y2 = pt[1] * scale + y_offset;
        let radius2 = 1.0 * scale;
        // draw
    };

    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.for_each_pos(render_c);
    });
}


////////////////////////

#[bench]
fn lots_of_small_loops(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {
        f.update_velocity_from_collisions();
        f.add_uniform_velocity(gravity); // some gravity
        //f.apply_velocity(0.005); // reducing this step increase sim stability
        //f.swap();
        //f.clear_next_simd(false);
    });
}

#[bench]
fn iter(b: &mut Bencher) {
    let mut f = create_fluid_sim(true);
    b.iter(|| {

        let mut buff = f.buffer.current.borrow_mut();
        for cell in f.iter() {
            // for each bucket within the x/y cell....
            let bucket_length = buff.bucket_sz[cell.i as usize] as isize;
            if bucket_length <= 0 {
                continue;
            }


            // update_velocity_from_collisions - part 1
            const cells_to_scan: isize = 3; // need to account for up to 2 * radius

            let iy2_start = if cell.iy < cells_to_scan { 0 } else { cell.iy - cells_to_scan }; //cmp::max(iy - cells_to_scan, 0);
            let iy2_end = cmp::min(cell.fluid_sim.y_size as isize, cell.iy + cells_to_scan);

            let ix2_start = if cell.ix < cells_to_scan { 0 } else { cell.ix - cells_to_scan }; //cmp::max(ix - cells_to_scan, 0);
            let ix2_end = cmp::min(cell.fluid_sim.x_size as isize, cell.ix + cells_to_scan);
            
            //let mut check_icell = cell.ix + (cell.iy * cell.fluid_sim.y_size as isize);
            //check_icell *= cell.fluid_sim.bucket_size as isize;

            for b in cell.icell..(cell.icell + bucket_length) {
                // add_uniform_velocity
                buff.vel[b as usize] += gravity;

                //
                // // update_velocity_from_collisions - part 2
                // get the position of the circle
                let pt = buff.pos[b as usize];
                
                for iy2 in iy2_start..iy2_end {
                    for ix2 in ix2_start..ix2_end {
                        let mut cell2 = ix2 + (iy2 * cell.fluid_sim.y_size as isize);

                        let bucket2_length = buff.bucket_sz[cell2 as usize];
                        cell2 *= cell.fluid_sim.bucket_size as isize;
                        for b2 in 0..bucket2_length {
                            let bucket_cell2 = cell2 + b2 as isize;

                            // don't compare against ourself
                            if b == bucket_cell2 {
                                continue;
                            }

                            // get the position of the circle
                            let pt2 = buff.pos[bucket_cell2 as usize];
                            //let pt_y2 = buff.pos[bucket_cell2+1];


                            // now do a radius check for other circles
                            //print!("  compare against -> pt: {:?},{:?} from pts[{:?}]", pt_x, pt_y, bucket_cell2);

                            // compute dist between
                            let a = pt2 - pt;
                            let dist_squared = (a[0] * a[0]) + (a[1] * a[1]);
                            if dist_squared >= dist_squared_max {
                                // no collision
                                //println!(" -> NO collision");
                                continue;
                            }

                            // collision!
                            //println!(" -> collision");

                            // compute and apply velocity to each circle
                            let dist = dist_squared.sqrt();
                            let dist_to_move = dist * 0.5;

                            /*
                            if (dist_to_move < 0.5) {
                                println!("to much intersection")
                            }*/

                            // as the points get closer, the velocity increases
                            // exponentially
                            // https://www.wolframalpha.com/input?i2d=true&i=plot+Divide%5B1%2Cx%5D
                            let mut vel_mag = 1.0 / dist_to_move;

                            let vel_m: f32x2 = Simd::from_array([vel_mag, vel_mag]);

                            // lose or gain energy in the outgoing velocity
                            let vel = (a * vel_m) * Simd::from_array([cell.fluid_sim.elasticity, cell.fluid_sim.elasticity]);
                            //let vel_y = (a[1] * vel_mag) * self.elasticity;

                            // loose some energy from the incoming velocity
                            //buff.vel[bucket_cell] *= self.collision_energy_loss;
                            //buff.vel[bucket_cell+1] *= self.collision_energy_loss;

                            //buff.vel[bucket_cell2] *= self.collision_energy_loss;
                            //buff.vel[bucket_cell2+1] *= self.collision_energy_loss;

                            buff.vel[b as usize] -= vel;
                            //buff.vel[bucket_cell+1] -= vel_y;

                            //buff.vel[bucket_cell2] += vel_x;
                            //buff.vel[bucket_cell2+1] += vel_y;

                            //println!("done");
                        }
                    }
                }
            }
        }

    });
}