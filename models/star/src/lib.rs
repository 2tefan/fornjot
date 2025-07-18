use std::f64::consts::PI;

use fj::{
    core::{
        operations::{
            build::{BuildCycle, BuildRegion, BuildSketch},
            reverse::Reverse,
            sweep::SweepSketch,
            update::{UpdateRegion, UpdateSketch},
        },
        topology::{Cycle, Region, Sketch, Solid},
    },
    math::Vector,
};

pub fn model(
    coords: Vec<[f64; 2]>,
    h: f64,
    core: &mut fj::core::Core,
) -> Solid {
    let bottom_surface = core.layers.topology.surfaces.xy_plane();
    let sweep_path = Vector::from([0., 0., h]);
    println!("Object has {} coords", coords.len());

    Sketch::empty(&core.layers.topology)
        .add_regions(
            [Region::polygon(
                coords,
                core.layers.topology.surfaces.space_2d(),
                core,
            )],
            core,
        )
        .sweep_sketch(bottom_surface, sweep_path, core)
}
