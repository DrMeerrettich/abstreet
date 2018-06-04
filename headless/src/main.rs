// Copyright 2018 Google LLC, licensed under http://www.apache.org/licenses/LICENSE-2.0

extern crate control;
extern crate geom;
extern crate map_model;
extern crate sim;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "headless")]
struct Flags {
    /// ABST input to load
    #[structopt(name = "abst_input")]
    abst_input: String,

    /// Optional RNG seed
    #[structopt(long = "rng_seed")]
    rng_seed: Option<u8>,
}

fn main() {
    let flags = Flags::from_args();

    println!("Opening {}", flags.abst_input);
    let data = map_model::load_pb(&flags.abst_input).expect("Couldn't load pb");
    let map = map_model::Map::new(&data);
    let geom_map = geom::GeomMap::new(&map);
    // TODO could load savestate
    let control_map = control::ControlMap::new(&map, &geom_map);
    let mut sim = sim::straw_model::Sim::new(&map, &geom_map, flags.rng_seed);
    // TODO need a notion of scenarios
    sim.spawn_many_on_empty_roads(100000);

    let mut counter = 0;
    let mut benchmark = sim.start_benchmark();
    loop {
        counter += 1;
        sim.step(&geom_map, &map, &control_map);
        if counter % 1000 == 0 {
            let speed = sim.measure_speed(&mut benchmark);
            println!("{0}, speed = {1:.2}x", sim.summary(), speed);
        }
    }
}
