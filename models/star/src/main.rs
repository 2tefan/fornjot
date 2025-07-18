use std::{fs::File, io::BufReader};

use clap::Parser;
use fj::{core::{operations::{build::{BuildRegion, BuildSketch, BuildSolid}, merge::Merge, sweep::SweepSketch, transform::TransformObject, update::UpdateSketch}, topology::{Region, Sketch, Solid}}, math::{Scalar, Vector}};

#[derive(Parser)]
struct Building {
    #[clap(skip)]
    coords: Vec<[f64; 2]>,

    /// Height of the star
    #[arg(long, default_value = "5.0")]
    height: f64,

    #[command(flatten)]
    fj: fj::Args,
}

pub fn model_cubio(size: impl Into<Vector<3>>, core: &mut fj::core::Core) -> Solid {
    let [x, y, z] = size.into().components;

    let bottom_surface = core.layers.topology.surfaces.xy_plane();
    let sweep_path = Vector::from([Scalar::ZERO, Scalar::ZERO, z]);

    Sketch::empty(&core.layers.topology)
        .add_regions(
            [Region::polygon(
                [
                    [-x / 2., -y / 2.],
                    [x / 2., -y / 2.],
                    [x / 2., y / 2.],
                    [-x / 2., y / 2.],
                ],
                core.layers.topology.surfaces.space_2d(),
                core,
            )],
            core,
        )
        .sweep_sketch(bottom_surface, sweep_path, core)
}


fn load_coords(path: &str) -> std::io::Result<Vec<Vec<[f64; 2]>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let coords = serde_json::from_reader(reader)?;
    Ok(coords)
}

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let params = Building::parse();

    let osm_coords_a = vec![
        [[0.0, 0.0], [1.0, -0.2], [1.0, 2.], [0.5, 4.2]],
        [[3.0, 0.0], [1.2, 0.0], [1.2, 0.5], [0.0, 0.2]],
        [[0.1, 2.0], [1.2, 0.0], [4.2, 0.5], [2.0, 0.2]],
    ];
    let osm_coords = load_coords("/tmp/out.json").expect("Failed to load JSON");

    let mut all = Solid::empty();

    let mut counter = 0;
    let total = osm_coords.len();
    for coords in osm_coords {
        counter += 1;
        //let vec_coords = coords.to_vec();
        println!("{:#?}", coords);
        println!("[{}/{}]", counter, total);
        let model = star::model(
            coords,
            2.0,
            &mut fj.core,
        );

        all = all.merge(&model, &mut fj.core);
    }


    let base_plate = model_cubio([210.0, 210.0, -1.], &mut fj.core);
    all = all.merge(&base_plate, &mut fj.core);

    fj.process_model_args(&all, params.fj)?;

    Ok(())
}
