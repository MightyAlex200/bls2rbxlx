use crate::types::{CFrame, Item, Property, Vector3};
use crate::{BRICK_HEIGHT, CONE_RESOLUTION, CONE_WALL_WIDTH, WEDGE_LIP_SIZE};
use nalgebra::{Point3, Rotation3, Vector3 as NVector3};

use std::f32::consts::{FRAC_PI_2, PI};

pub const TWO_PI: f32 = 2. * PI;

pub const SPAWN_HEIGHT: f32 = 0.2;
pub const WINDOW_RIM_WIDTH: f32 = 0.1;

pub struct SpecialBricksCache {
    cone2x2x2: Option<Item>,
    cone1x1: Option<Item>,
    castle_wall: Option<Item>,
    spawn_point: Option<Item>,
    window_1x4x3: Option<Item>,
    crest_corner_25: Option<Item>,
    crest_end_25: Option<Item>,
    crest_corner_45: Option<Item>,
    crest_end_45: Option<Item>,
}

fn generate_cone(cone_size: f32) -> Item {
    let mut item = Item::default("Model");

    // Helper function for creating sides
    fn create_wedge(
        percent: f32,
        cone_size: f32,
        wedge_size: f32,
        rotation: f32,
        offset: f32,
    ) -> (Vector3, CFrame) {
        let orig_outer_point = Rotation3::new(NVector3::new(0., percent * TWO_PI, 0.))
            * Point3::new(0., 0., cone_size / 2.);
        let orig_inner_point =
            orig_outer_point * 0.5 + NVector3::new(0., cone_size * BRICK_HEIGHT, 0.);
        let mid_point = Point3::from((orig_outer_point.coords + orig_inner_point.coords) / 2.);

        let rot_out = Rotation3::new(NVector3::new(0., (percent + offset) * TWO_PI, 0.));
        let outer_point = rot_out * Point3::new(0., 0., cone_size / 2.);
        let inner_point = outer_point * 0.5 + NVector3::new(0., cone_size * BRICK_HEIGHT, 0.);

        let towards_inner = inner_point - outer_point;
        let looking_towards_inner = Rotation3::face_towards(&towards_inner, &NVector3::y());
        let size = Vector3::new(
            CONE_WALL_WIDTH,
            towards_inner.magnitude(),
            1. / CONE_RESOLUTION as f32 * PI * wedge_size,
        );
        let cframe = CFrame {
            vector: Vector3(
                mid_point.coords - NVector3::new(0., cone_size / 2. * BRICK_HEIGHT, 0.),
            ),
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
        let mut wedge1 = Item::default("WedgePart");
        let (size1, cframe1) = create_wedge(percent, cone_size, 2., 0., 0.);
        wedge1.properties.insert("size", Property::Vector3(size1));
        wedge1
            .properties
            .insert("CFrame", Property::CFrame(cframe1));
        let mut wedge2 = Item::default("WedgePart");
        let (size2, cframe2) =
            create_wedge(percent, cone_size, 1., PI, 1. / CONE_RESOLUTION as f32);
        wedge2.properties.insert("size", Property::Vector3(size2));
        wedge2
            .properties
            .insert("CFrame", Property::CFrame(cframe2));
        item.children.push(wedge1);
        item.children.push(wedge2);
    }

    // Create top and bottom of cone
    let half_cone_size = cone_size / 2.;
    let cylinder_mesh = Item::default("CylinderMesh");
    let mut cap_bottom = Item::default("Part");
    cap_bottom.properties.insert(
        "size",
        Property::Vector3(Vector3::new(cone_size, CONE_WALL_WIDTH, cone_size)),
    );
    cap_bottom.properties.insert(
        "CFrame",
        Property::CFrame(CFrame {
            vector: Vector3::new(0., -BRICK_HEIGHT * half_cone_size, 0.),
            rotation: Rotation3::identity(),
        }),
    );
    cap_bottom.children.push(cylinder_mesh.clone());
    let mut cap_top = Item::default("Part");
    cap_top.properties.insert(
        "size",
        Property::Vector3(Vector3::new(
            half_cone_size,
            CONE_WALL_WIDTH,
            half_cone_size,
        )),
    );
    cap_top.properties.insert(
        "CFrame",
        Property::CFrame(CFrame {
            vector: Vector3::new(0., BRICK_HEIGHT * half_cone_size, 0.),
            rotation: Rotation3::identity(),
        }),
    );
    cap_top.children.push(cylinder_mesh);
    item.children.push(cap_bottom);
    item.children.push(cap_top);
    item
}

fn generate_castle_wall() -> Item {
    let mut model = Item::default("Model");

    let mut bottom = Item::default("Part");
    bottom.properties.insert(
        "size",
        Property::Vector3(Vector3::new(1., 3. * BRICK_HEIGHT, 3.)),
    );
    bottom.properties.insert(
        "CFrame",
        Property::CFrame(CFrame {
            vector: Vector3::new(0., -1.5 * BRICK_HEIGHT, 0.),
            rotation: Rotation3::identity(),
        }),
    );

    fn create_wall(z: f32) -> Item {
        let mut wall = Item::default("Part");
        wall.properties.insert(
            "size",
            Property::Vector3(Vector3::new(1., 5. / 3. * BRICK_HEIGHT, 1.)),
        );
        wall.properties.insert(
            "CFrame",
            Property::CFrame(CFrame {
                vector: Vector3::new(0., 5. / 6. * BRICK_HEIGHT, z),
                rotation: Rotation3::identity(),
            }),
        );
        wall
    }

    let left = create_wall(1.);
    let right = create_wall(-1.);

    let mut top = Item::default("Part");
    top.properties.insert(
        "size",
        Property::Vector3(Vector3::new(1., 4. / 3. * BRICK_HEIGHT, 3.)),
    );
    top.properties.insert(
        "CFrame",
        Property::CFrame(CFrame {
            vector: Vector3::new(0., 7. / 3. * BRICK_HEIGHT, 0.),
            rotation: Rotation3::identity(),
        }),
    );

    fn create_corner(zpos: f32, yrot: f32) -> Item {
        let mut corner = Item::default("WedgePart");
        corner.properties.insert(
            "size",
            Property::Vector3(Vector3::new(1., BRICK_HEIGHT / 3., 1. / 3.)),
        );
        corner.properties.insert(
            "CFrame",
            Property::CFrame(CFrame {
                vector: Vector3::new(0., 3. / 2. * BRICK_HEIGHT, zpos),
                rotation: Rotation3::from_scaled_axis(NVector3::x() * PI)
                    * Rotation3::from_scaled_axis(NVector3::y() * yrot),
            }),
        );
        corner
    }

    let left_corner = create_corner(2. / 6., PI);
    let right_corner = create_corner(-2. / 6., 0.);

    model.children.push(bottom);
    model.children.push(left);
    model.children.push(right);
    model.children.push(left_corner);
    model.children.push(right_corner);
    model.children.push(top);
    model
}

fn generate_spawn_point() -> Item {
    let mut model = Item::default("Model");

    let mut spawnpoint = Item::default("SpawnLocation");
    spawnpoint.properties.insert(
        "size",
        Property::Vector3(Vector3::new(3., SPAWN_HEIGHT * BRICK_HEIGHT, 3.)),
    );
    spawnpoint.properties.insert(
        "CFrame",
        Property::CFrame(CFrame {
            vector: Vector3::new(0., (-2.5 + SPAWN_HEIGHT / 2.) * BRICK_HEIGHT, 0.),
            rotation: Rotation3::from_scaled_axis(NVector3::y() * FRAC_PI_2),
        }),
    );

    let mut cover = Item::default("Part");
    cover.properties.insert(
        "size",
        Property::Vector3(Vector3::new(3., (5. - SPAWN_HEIGHT) * BRICK_HEIGHT, 3.)),
    );
    cover.properties.insert(
        "CFrame",
        Property::CFrame(CFrame {
            vector: Vector3::new(0., SPAWN_HEIGHT / 2. * BRICK_HEIGHT, 0.),
            rotation: Rotation3::identity(),
        }),
    );
    cover
        .properties
        .insert("Transparency", Property::Float(0.5));
    cover.properties.insert("CanCollide", Property::Bool(false));

    model.children.push(spawnpoint);
    model.children.push(cover);
    model
}

fn generate_window() -> Item {
    let mut model = Item::default("Model");

    fn create_horizontal(ypos: f32) -> Item {
        let mut part = Item::default("Part");
        part.properties.insert(
            "size",
            Property::Vector3(Vector3::new(4., WINDOW_RIM_WIDTH * BRICK_HEIGHT, 1.)),
        );
        part.properties.insert(
            "CFrame",
            Property::CFrame(CFrame {
                vector: Vector3::new(0., ypos, 0.),
                rotation: Rotation3::identity(),
            }),
        );
        part
    }

    fn create_vertical(xpos: f32) -> Item {
        let mut part = Item::default("Part");
        part.properties.insert(
            "size",
            Property::Vector3(Vector3::new(
                WINDOW_RIM_WIDTH,
                (5. - WINDOW_RIM_WIDTH * 2.) * BRICK_HEIGHT,
                1.,
            )),
        );
        part.properties.insert(
            "CFrame",
            Property::CFrame(CFrame {
                vector: Vector3::new(xpos, 0., 0.),
                rotation: Rotation3::identity(),
            }),
        );
        part
    }

    let top = create_horizontal((5. - WINDOW_RIM_WIDTH) * BRICK_HEIGHT / 2.);
    let bottom = create_horizontal((-5. + WINDOW_RIM_WIDTH) * BRICK_HEIGHT / 2.);
    let left = create_vertical((4. - WINDOW_RIM_WIDTH) / 2.);
    let right = create_vertical((-4. + WINDOW_RIM_WIDTH) / 2.);

    let mut window = Item::default("Part");
    window.properties.insert(
        "size",
        Property::Vector3(Vector3::new(
            4. - WINDOW_RIM_WIDTH * 2.,
            (5. - WINDOW_RIM_WIDTH * 2.) * BRICK_HEIGHT,
            1.,
        )),
    );
    window.properties.insert(
        "CFrame",
        Property::CFrame(CFrame {
            vector: Vector3::new(0., 0., 0.),
            rotation: Rotation3::identity(),
        }),
    );
    window
        .properties
        .insert("Transparency", Property::Float(0.5));

    model.children.push(bottom);
    model.children.push(top);
    model.children.push(left);
    model.children.push(right);
    model.children.push(window);
    model
}

fn generate_crest_lip(x: f32, z: f32) -> Item {
    let mut lip = Item::default("Part");
    lip.properties.insert(
        "size",
        Property::Vector3(Vector3::new(x, WEDGE_LIP_SIZE * BRICK_HEIGHT, z)),
    );
    lip.properties.insert(
        "CFrame",
        Property::CFrame(CFrame {
            vector: Vector3::new(0., (-1. + WEDGE_LIP_SIZE) * BRICK_HEIGHT / 2., 0.),
            rotation: Rotation3::identity(),
        }),
    );
    lip
}

pub fn generate_crest(height: f32, length: u8) -> Item {
    let mut model = Item::default("Model");

    let lip = generate_crest_lip(length as f32, 2.);

    fn generate_wedge(z: f32, yrot: f32, height: f32, length: u8) -> Item {
        let mut wedge = Item::default("WedgePart");

        wedge.properties.insert(
            "size",
            Property::Vector3(Vector3::new(
                length as f32,
                (height - WEDGE_LIP_SIZE) * BRICK_HEIGHT,
                1.,
            )),
        );
        wedge.properties.insert(
            "CFrame",
            Property::CFrame(CFrame {
                vector: Vector3::new(0., (-1. + WEDGE_LIP_SIZE + height) * BRICK_HEIGHT / 2., z),
                rotation: Rotation3::from_scaled_axis(NVector3::y() * yrot),
            }),
        );

        wedge
    }

    let wedge_1 = generate_wedge(0.5, PI, height, length);
    let wedge_2 = generate_wedge(-0.5, 0., height, length);

    model.children.push(wedge_1);
    model.children.push(wedge_2);
    model.children.push(lip);
    model
}

fn generate_crest_corner(height: f32) -> Item {
    let mut model = Item::default("Model");

    let lip = generate_crest_lip(2., 2.);

    fn generate_wedge(x: f32, z: f32, yrot: f32, height: f32) -> Item {
        let mut wedge = Item::default("WedgePart");

        wedge.properties.insert(
            "size",
            Property::Vector3(Vector3::new(
                1.,
                (height - WEDGE_LIP_SIZE) * BRICK_HEIGHT,
                1.,
            )),
        );
        wedge.properties.insert(
            "CFrame",
            Property::CFrame(CFrame {
                vector: Vector3::new(x, (-1. + WEDGE_LIP_SIZE + height) * BRICK_HEIGHT / 2., z),
                rotation: Rotation3::from_scaled_axis(NVector3::y() * yrot),
            }),
        );

        wedge
    }

    let wedge_1 = generate_wedge(0.5, 0.5, PI, height);
    let wedge_2 = generate_wedge(0.5, -0.5, 0., height);
    let wedge_3 = generate_wedge(-0.5, 0.5, FRAC_PI_2, height);
    let wedge_4 = generate_wedge(0.5, 0.5, 3. * FRAC_PI_2, height);
    let mut wedge_corner = Item::default("CornerWedgePart");
    wedge_corner.properties.insert(
        "size",
        Property::Vector3(Vector3::new(
            1.,
            (height - WEDGE_LIP_SIZE) * BRICK_HEIGHT,
            1.,
        )),
    );
    wedge_corner.properties.insert(
        "CFrame",
        Property::CFrame(CFrame {
            vector: Vector3::new(
                -0.5,
                (-1. + WEDGE_LIP_SIZE + height) * BRICK_HEIGHT / 2.,
                -0.5,
            ),
            rotation: Rotation3::from_scaled_axis(NVector3::y() * -FRAC_PI_2),
        }),
    );

    model.children.push(wedge_1);
    model.children.push(wedge_2);
    model.children.push(wedge_3);
    model.children.push(wedge_4);
    model.children.push(wedge_corner);
    model.children.push(lip);
    model
}

fn generate_crest_end(height: f32) -> Item {
    let mut model = Item::default("Model");

    let lip = generate_crest_lip(1., 2.);

    fn generate_wedge(z: f32, yrot: f32, height: f32) -> Item {
        let mut wedge = Item::default("CornerWedgePart");

        wedge.properties.insert(
            "size",
            Property::Vector3(Vector3::new(
                1.,
                (height - WEDGE_LIP_SIZE) * BRICK_HEIGHT,
                1.,
            )),
        );
        wedge.properties.insert(
            "CFrame",
            Property::CFrame(CFrame {
                vector: Vector3::new(0., (-1. + WEDGE_LIP_SIZE + height) * BRICK_HEIGHT / 2., z),
                rotation: Rotation3::from_scaled_axis(NVector3::y() * yrot),
            }),
        );

        wedge
    }

    let wedge_1 = generate_wedge(0.5, FRAC_PI_2, height);
    let wedge_2 = generate_wedge(-0.5, -PI, height);

    model.children.push(wedge_1);
    model.children.push(wedge_2);
    model.children.push(lip);
    model
}

impl SpecialBricksCache {
    pub fn new() -> Self {
        SpecialBricksCache {
            cone2x2x2: None,
            cone1x1: None,
            castle_wall: None,
            spawn_point: None,
            window_1x4x3: None,
            crest_corner_25: None,
            crest_end_25: None,
            crest_corner_45: None,
            crest_end_45: None,
        }
    }

    pub fn cone2x2x2(&mut self) -> Item {
        match &self.cone2x2x2 {
            Some(cone) => cone.clone(),
            None => {
                let cone = generate_cone(2.);
                self.cone2x2x2 = Some(cone.clone());
                cone
            }
        }
    }

    pub fn cone1x1(&mut self) -> Item {
        match &self.cone1x1 {
            Some(cone) => cone.clone(),
            None => {
                let cone = generate_cone(1.);
                self.cone1x1 = Some(cone.clone());
                cone
            }
        }
    }

    pub fn castle_wall(&mut self) -> Item {
        match &self.castle_wall {
            Some(wall) => wall.clone(),
            None => {
                let wall = generate_castle_wall();
                self.castle_wall = Some(wall.clone());
                wall
            }
        }
    }

    pub fn spawn_point(&mut self) -> Item {
        match &self.spawn_point {
            Some(spawn) => spawn.clone(),
            None => {
                let spawn = generate_spawn_point();
                self.spawn_point = Some(spawn.clone());
                spawn
            }
        }
    }

    pub fn window_1x4x3(&mut self) -> Item {
        match &self.window_1x4x3 {
            Some(window) => window.clone(),
            None => {
                let window = generate_window();
                self.window_1x4x3 = Some(window.clone());
                window
            }
        }
    }

    pub fn crest_corner_25(&mut self) -> Item {
        match &self.crest_corner_25 {
            Some(crest) => crest.clone(),
            None => {
                let crest = generate_crest_corner(2. / 3.);
                self.crest_corner_25 = Some(crest.clone());
                crest
            }
        }
    }

    pub fn crest_end_25(&mut self) -> Item {
        match &self.crest_end_25 {
            Some(crest) => crest.clone(),
            None => {
                let crest = generate_crest_end(2. / 3.);
                self.crest_end_25 = Some(crest.clone());
                crest
            }
        }
    }

    pub fn crest_corner_45(&mut self) -> Item {
        match &self.crest_corner_45 {
            Some(crest) => crest.clone(),
            None => {
                let crest = generate_crest_corner(1.);
                self.crest_corner_45 = Some(crest.clone());
                crest
            }
        }
    }

    pub fn crest_end_45(&mut self) -> Item {
        match &self.crest_end_45 {
            Some(crest) => crest.clone(),
            None => {
                let crest = generate_crest_end(1.);
                self.crest_end_45 = Some(crest.clone());
                crest
            }
        }
    }
}
