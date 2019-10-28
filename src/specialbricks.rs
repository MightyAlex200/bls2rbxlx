use crate::types::{CFrame, Item, Property, Vector3};
use crate::{BRICK_HEIGHT, CONE_RESOLUTION, CONE_WALL_WIDTH};
use nalgebra::{Point3, Rotation3, Vector3 as NVector3};

use std::f32::consts::{FRAC_PI_2, PI};

pub const TWO_PI: f32 = 2. * PI;

pub struct SpecialBricksCache {
    cone2x2x2: Option<Item>,
}

fn generate_cone_2x2x2() -> Item {
    let mut item = Item::default("Model".to_string());

    // Helper function for creating sides
    fn create_wedge(
        percent: f32,
        wedge_size: f32,
        rotation: f32,
        offset: f32,
    ) -> (Vector3, CFrame) {
        let orig_outer_point =
            Rotation3::new(NVector3::new(0., percent * TWO_PI, 0.)) * Point3::new(0., 0., 1.);
        let orig_inner_point = orig_outer_point * 0.5 + NVector3::new(0., 2. * BRICK_HEIGHT, 0.);
        let mid_point = Point3::from((orig_outer_point.coords + orig_inner_point.coords) / 2.);

        let rot_out = Rotation3::new(NVector3::new(0., (percent + offset) * TWO_PI, 0.));
        let outer_point = rot_out * Point3::new(0., 0., 1.);
        let inner_point = outer_point * 0.5 + NVector3::new(0., 2. * BRICK_HEIGHT, 0.);

        let towards_inner = inner_point - outer_point;
        let looking_towards_inner = Rotation3::face_towards(&towards_inner, &NVector3::y());
        let size = Vector3::new(
            CONE_WALL_WIDTH,
            towards_inner.magnitude(),
            1. / CONE_RESOLUTION as f32 * PI * wedge_size,
        );
        let cframe = CFrame {
            vector: Vector3(mid_point.coords - NVector3::new(0., BRICK_HEIGHT, 0.)),
            rotation: looking_towards_inner
                * Rotation3::from_scaled_axis(NVector3::z() * FRAC_PI_2)
                * Rotation3::from_scaled_axis(NVector3::x() * (FRAC_PI_2 + rotation))
                * Rotation3::from_scaled_axis(NVector3::y() * offset * PI * PI),
        };
        (size, cframe)
    }

    // Actually all the create sides
    for i in 0..CONE_RESOLUTION {
        let percent = i as f32 / CONE_RESOLUTION as f32;
        let mut wedge1 = Item::default("WedgePart".to_string());
        let (size1, cframe1) = create_wedge(percent, 2., 0., 0.);
        wedge1
            .properties
            .insert("size".to_string(), Property::Vector3(size1));
        wedge1
            .properties
            .insert("CFrame".to_string(), Property::CFrame(cframe1));
        let mut wedge2 = Item::default("WedgePart".to_string());
        let (size2, cframe2) = create_wedge(percent, 1., PI, 1. / CONE_RESOLUTION as f32);
        wedge2
            .properties
            .insert("size".to_string(), Property::Vector3(size2));
        wedge2
            .properties
            .insert("CFrame".to_string(), Property::CFrame(cframe2));
        item.children.push(wedge1);
        item.children.push(wedge2);
    }

    // Create top and bottom of cone
    let cylinder_mesh = Item::default("CylinderMesh".to_string());
    let mut cap_bottom = Item::default("Part".to_string());
    cap_bottom.properties.insert(
        "size".to_string(),
        Property::Vector3(Vector3::new(2., CONE_WALL_WIDTH, 2.)),
    );
    cap_bottom.properties.insert(
        "CFrame".to_string(),
        Property::CFrame(CFrame {
            vector: Vector3::new(0., -BRICK_HEIGHT, 0.),
            rotation: Rotation3::identity(),
        }),
    );
    cap_bottom.children.push(cylinder_mesh.clone());
    let mut cap_top = Item::default("Part".to_string());
    cap_top.properties.insert(
        "size".to_string(),
        Property::Vector3(Vector3::new(1., CONE_WALL_WIDTH, 1.)),
    );
    cap_top.properties.insert(
        "CFrame".to_string(),
        Property::CFrame(CFrame {
            vector: Vector3::new(0., BRICK_HEIGHT, 0.),
            rotation: Rotation3::identity(),
        }),
    );
    cap_top.children.push(cylinder_mesh.clone());
    item.children.push(cap_bottom);
    item.children.push(cap_top);
    item
}

impl SpecialBricksCache {
    pub fn new() -> Self {
        SpecialBricksCache { cone2x2x2: None }
    }

    pub fn cone2x2x2(&mut self) -> Item {
        match &self.cone2x2x2 {
            Some(cone) => cone.clone(),
            None => {
                let cone = generate_cone_2x2x2();
                self.cone2x2x2 = Some(cone.clone());
                cone
            }
        }
    }
}
