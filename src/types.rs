use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct Vector3(nalgebra::Vector3<f32>);

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3(nalgebra::Vector3::<f32>::new(x, y, z))
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn z(&self) -> f32 {
        self.0.z
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3(self.0 + other.0)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3(self.0 - other.0)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3(self.0 * other)
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Vector3 {
        Vector3(self.0 / other)
    }
}

impl ToString for Vector3 {
    fn to_string(&self) -> String {
        format!("<X>{}</X><Y>{}</Y><Z>{}</Z>", self.0.x, self.0.y, self.0.z)
    }
}

#[derive(Clone)]
pub struct Color3 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<(f32, f32, f32, f32)> for Color3 {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Color3 {
        let f = |v| (v * 255.) as u8;
        Color3 {
            r: f(r),
            g: f(g),
            b: f(b),
            a: f(a),
        }
    }
}

impl ToString for Color3 {
    fn to_string(&self) -> String {
        (((self.a as u32) << 24)
            + ((self.r as u32) << 16)
            + ((self.g as u32) << 8)
            + (self.b as u32))
            .to_string()
    }
}

#[derive(Clone)]
pub struct CFrame {
    pub vector: Vector3,
    pub rotation: nalgebra::Rotation3<f32>,
}

impl std::ops::Add<Vector3> for CFrame {
    type Output = Self;

    fn add(mut self, other: Vector3) -> Self {
        self.vector.0.x += other.0.x;
        self.vector.0.y += other.0.y;
        self.vector.0.z += other.0.z;
        self
    }
}

impl std::ops::Sub<Vector3> for CFrame {
    type Output = Self;

    fn sub(mut self, other: Vector3) -> Self {
        self.vector.0.x -= other.0.x;
        self.vector.0.y -= other.0.y;
        self.vector.0.z -= other.0.z;
        self
    }
}

impl ToString for CFrame {
    fn to_string(&self) -> String {
        format!(
            "<X>{}</X>
<Y>{}</Y>
<Z>{}</Z>
<R00>{}</R00>
<R01>{}</R01>
<R02>{}</R02>
<R10>{}</R10>
<R11>{}</R11>
<R12>{}</R12>
<R20>{}</R20>
<R21>{}</R21>
<R22>{}</R22>",
            self.vector.0.x,
            self.vector.0.y,
            self.vector.0.z,
            self.rotation[(0, 0)],
            self.rotation[(0, 1)],
            self.rotation[(0, 2)],
            self.rotation[(1, 0)],
            self.rotation[(1, 1)],
            self.rotation[(1, 2)],
            self.rotation[(2, 0)],
            self.rotation[(2, 1)],
            self.rotation[(2, 2)]
        )
    }
}

pub struct PhysicalProperties(pub bool);

impl ToString for PhysicalProperties {
    fn to_string(&self) -> String {
        format!("<CustomPhysics>{}</CustomPhysics>", self.0)
    }
}

pub struct RbxUuid(pub Uuid);

impl ToString for RbxUuid {
    fn to_string(&self) -> String {
        let mut s = "RBX".to_string();
        s.push_str(&self.0.to_simple_ref().to_string());
        s
    }
}

pub struct Item {
    pub class: String,
    pub referent: RbxUuid,
    pub properties: HashMap<String, Property>,
    pub children: Vec<Item>,
}

impl Item {
    pub fn default(class: String) -> Item {
        let mut properties: HashMap<String, Property> = HashMap::new();
        vec![
            ("Anchored", Property::Bool(true)),
            ("BackParamA", Property::Float(-0.5)),
            ("BackParamB", Property::Float(0.5)),
            ("BackSurface", Property::Token(0)),
            ("BackSurfaceInput", Property::Token(0)),
            ("BottomParamA", Property::Float(-0.5)),
            ("BottomParamB", Property::Float(0.5)),
            ("BottomSurface", Property::Token(4)),
            ("BottomSurfaceInput", Property::Token(0)),
            // ("BrickColor", Property::Int(1004)),
            (
                "CustomPhysicalProperties",
                Property::PhysProps(PhysicalProperties(false)),
            ),
            ("Elasticity", Property::Float(0.5)),
            ("Friction", Property::Float(0.5)),
            ("FrontParamA", Property::Float(0.5)),
            ("FrontParamB", Property::Float(0.5)),
            ("FrontSurface", Property::Token(0)),
            ("FrontSurfaceInput", Property::Token(0)),
            ("LeftParamA", Property::Float(0.5)),
            ("LeftParamB", Property::Float(0.5)),
            ("LeftSurface", Property::Token(0)),
            ("LeftSurfaceInput", Property::Token(0)),
            ("Locked", Property::Bool(false)),
            ("Material", Property::Token(256)),
            ("Reflectance", Property::Float(0.)),
            ("RightParamA", Property::Float(0.5)),
            ("RightParamB", Property::Float(0.5)),
            ("RightSurface", Property::Token(0)),
            ("RightSurfaceInput", Property::Token(0)),
            ("RotVelocity", Property::Vector3(Vector3::new(0., 0., 0.))),
            ("TopParamA", Property::Float(-0.5)),
            ("TopParamB", Property::Float(0.5)),
            ("TopSurface", Property::Token(3)),
            ("TopSurfaceInput", Property::Token(0)),
            ("Velocity", Property::Vector3(Vector3::new(0., 0., 0.))),
            ("formFactorRaw", Property::Token(1)),
            ("shape", Property::Token(1)),
        ]
        .into_iter()
        .for_each(|(n, p)| {
            properties.insert(n.to_string(), p);
        });
        Item {
            class,
            referent: RbxUuid(Uuid::new_v4()),
            properties,
            children: vec![],
        }
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        format!(
            "<Item class=\"{class}\" referent=\"{referent}\"><Properties>{props}</Properties>{children}</Item>",
            class=self.class,
            referent=self.referent.to_string(),
            props=self.properties.iter().map(|(k, v)| v.property_to_string(k)).collect::<Vec<_>>().join("\n"),
            children=self.children.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
        )
    }
}

macro_rules! define_property_enum {
    ($($tag:expr => $t:ident($backing:ty);)*) => {
        #[allow(dead_code)]
        pub enum Property {
            $($t($backing)),*
        }

        impl Property {
            pub fn property_to_string(&self, prop_name: &str) -> String {
                match self {
                    $(Property::$t(v) =>
                        format!(
                            "<{t} name=\"{n}\">{v}</{t}>",
                            t = $tag,
                            n = prop_name,
                            v = v.to_string()
                        )
                    ),*
                }
            }
        }
    };
}

define_property_enum! {
    "bool" => Bool(bool);
    "float" => Float(f32);
    "token" => Token(u32);
    "int" => Int(i64);
    "CoordinateFrame" => CFrame(CFrame);
    "Color3uint8" => Color3(Color3);
    "PhysicalProperties" => PhysProps(PhysicalProperties);
    "string" => String(String);
    "Vector3" => Vector3(Vector3);
}
