use uuid::Uuid;

pub trait Prop: ToString {}

impl<T> Prop for T where T: ToString {}

pub struct RBXUUID(pub Uuid);

pub struct Item {
    class: &'static str,
    referent: RBXUUID,
    properties: Vec<Box<dyn Prop>>,
}

impl ToString for Item {
    fn to_string(&self) -> String {
        format!(
            "<Item class=\"{}\" referent=\"{}\">
    <Properties>
        {}
    </Properties>
</Item>",
            self.class,
            self.referent.0.to_simple_ref().to_string(),
            self.properties
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Item {
    pub fn new(
        class: &'static str,
        referent: RBXUUID,
        name: Name,
        cframe: CoordinateFrame,
        b_size: size,
        color: Color3uint8,
    ) -> Item {
        Item {
            class,
            referent,
            properties: vec![
                // <bool name="Anchored">false</bool>
                Box::new(Anchored(true)),
                // <float name="BackParamA">-0.5</float>
                Box::new(BackParamA(-0.5)),
                // <float name="BackParamB">0.5</float>
                Box::new(BackParamB(0.5)),
                // <token name="BackSurface">0</token>
                Box::new(BackSurface(0)),
                // <token name="BackSurfaceInput">0</token>
                Box::new(BackSurfaceInput(0)),
                // <float name="BottomParamA">-0.5</float>
                Box::new(BottomParamA(-0.5)),
                // <float name="BottomParamB">0.5</float>
                Box::new(BottomParamB(0.5)),
                // <token name="BottomSurface">4</token>
                Box::new(BottomSurface(4)),
                // <token name="BottomSurfaceInput">0</token>
                Box::new(BottomSurfaceInput(0)),
                // <int name="BrickColor">1004</int>
                Box::new(BrickColor(1004)),
                // <CoordinateFrame name="CFrame">
                // 	<X>-14</X>
                // 	<Y>0.600012004</Y>
                // 	<Z>23</Z>
                // 	<R00>1</R00>
                // 	<R01>0</R01>
                // 	<R02>0</R02>
                // 	<R10>0</R10>
                // 	<R11>1</R11>
                // 	<R12>0</R12>
                // 	<R20>0</R20>
                // 	<R21>0</R21>
                // 	<R22>1</R22>
                // </CoordinateFrame>
                Box::new(cframe),
                // <bool name="CanCollide">true</bool>
                Box::new(CanCollide(true)),
                // <Color3uint8 name="Color3uint8">4294901760</Color3uint8>
                Box::new(color),
                // <PhysicalProperties name="CustomPhysicalProperties">
                // 	<CustomPhysics>false</CustomPhysics>
                // </PhysicalProperties>
                Box::new(CustomPhysicalProperties(false)),
                // <float name="Elasticity">0.5</float>
                Box::new(Elasticity(0.5)),
                // <float name="Friction">0.300000012</float>
                Box::new(Friction(0.5)),
                // <float name="FrontParamA">-0.5</float>
                Box::new(FrontParamA(-0.5)),
                // <float name="FrontParamB">0.5</float>
                Box::new(FrontParamB(0.5)),
                // <token name="FrontSurface">0</token>
                Box::new(FrontSurface(0)),
                // <token name="FrontSurfaceInput">0</token>
                Box::new(FrontSurfaceInput(0)),
                // <float name="LeftParamA">-0.5</float>
                Box::new(LeftParamA(-0.5)),
                // <float name="LeftParamB">0.5</float>
                Box::new(LeftParamB(0.5)),
                // <token name="LeftSurface">0</token>
                Box::new(LeftSurface(0)),
                // <token name="LeftSurfaceInput">0</token>
                Box::new(LeftSurfaceInput(0)),
                // <bool name="Locked">false</bool>
                Box::new(Locked(false)),
                // <token name="Material">256</token>
                Box::new(Material(256)),
                // <string name="Name">Part</string>
                Box::new(name),
                // <float name="Reflectance">0</float>
                Box::new(Reflectance(0.)),
                // <float name="RightParamA">-0.5</float>
                Box::new(RightParamA(-0.5)),
                // <float name="RightParamB">0.5</float>
                Box::new(RightParamB(0.5)),
                // <token name="RightSurface">0</token>
                Box::new(RightSurface(0)),
                // <token name="RightSurfaceInput">0</token>
                Box::new(RightSurfaceInput(0)),
                // <Vector3 name="RotVelocity">
                // 	<X>0</X>
                // 	<Y>0</Y>
                // 	<Z>0</Z>
                // </Vector3>
                Box::new(RotVelocity(Vector3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                })),
                // <float name="TopParamA">-0.5</float>
                Box::new(TopParamA(-0.5)),
                // <float name="TopParamB">0.5</float>
                Box::new(TopParamB(0.5)),
                // <token name="TopSurface">3</token>
                Box::new(TopSurface(3)),
                // <token name="TopSurfaceInput">0</token>
                Box::new(TopSurfaceInput(0)),
                // <float name="Transparency">0</float>
                Box::new(Transparency(0.)),
                // <Vector3 name="Velocity">
                // 	<X>0</X>
                // 	<Y>0</Y>
                // 	<Z>0</Z>
                // </Vector3>
                Box::new(Velocity(Vector3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                })),
                // <token name="formFactorRaw">1</token>
                Box::new(formFactorRaw(1)),
                // <token name="shape">1</token>
                Box::new(shape(1)),
                // <Vector3 name="size">
                // 	<X>4</X>
                // 	<Y>1.20000005</Y>
                // 	<Z>2</Z>
                // </Vector3>
                Box::new(b_size),
            ],
        }
    }
}

#[derive(Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

macro_rules! vec3_prop {
    ($i:ident) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct $i(pub Vector3);

        impl ToString for $i {
            fn to_string(&self) -> String {
                format!(
                    "<Vector3 name=\"{i}\"><X>{x}</X><Y>{y}</Y><Z>{z}</Z></{i}>",
                    i = stringify!($i),
                    x = self.0.x,
                    y = self.0.y,
                    z = self.0.z
                )
            }
        }
    };
}

macro_rules! simple_property {
    (string, $name:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $name(pub String);

        impl ToString for $name {
            fn to_string(&self) -> String {
                format!(
                    "<string name=\"{n}\">{c}</string>",
                    c = self.0,
                    n = stringify!($name)
                )
            }
        }
    };
    (int, $name:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $name(pub i64);

        impl ToString for $name {
            fn to_string(&self) -> String {
                format!(
                    "<int name=\"{n}\">{c}</int>",
                    c = self.0,
                    n = stringify!($name)
                )
            }
        }
    };
    (token, $name:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $name(pub i64);

        impl ToString for $name {
            fn to_string(&self) -> String {
                format!(
                    "<token name=\"{n}\">{c}</token>",
                    c = self.0,
                    n = stringify!($name)
                )
            }
        }
    };
    (float, $name:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $name(pub f64);

        impl ToString for $name {
            fn to_string(&self) -> String {
                format!(
                    "<float name=\"{n}\">{c}</float>",
                    c = self.0,
                    n = stringify!($name)
                )
            }
        }
    };
    ($t:ty, $name:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $name(pub $t);

        impl ToString for $name {
            fn to_string(&self) -> String {
                format!(
                    "<{t} name=\"{n}\">{c}</{t}>",
                    t = stringify!($t),
                    c = self.0,
                    n = stringify!($name)
                )
            }
        }
    };
}

simple_property!(bool, Anchored);
simple_property!(float, BackParamA);
simple_property!(float, BackParamB);
simple_property!(token, BackSurface);
simple_property!(token, BackSurfaceInput);
simple_property!(float, BottomParamA);
simple_property!(float, BottomParamB);
simple_property!(token, BottomSurface);
simple_property!(token, BottomSurfaceInput);
simple_property!(int, BrickColor);
pub struct CoordinateFrame {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub r00: f64,
    pub r01: f64,
    pub r02: f64,
    pub r10: f64,
    pub r11: f64,
    pub r12: f64,
    pub r20: f64,
    pub r21: f64,
    pub r22: f64,
}

impl ToString for CoordinateFrame {
    fn to_string(&self) -> String {
        format!(
            "<CoordinateFrame name=\"CFrame\">
    <X>{x}</X>
    <Y>{y}</Y>
    <Z>{z}</Z>
    <R00>{r00}</R00>
    <R01>{r01}</R01>
    <R02>{r02}</R02>
    <R10>{r10}</R10>
    <R11>{r11}</R11>
    <R12>{r12}</R12>
    <R20>{r20}</R20>
    <R21>{r21}</R21>
    <R22>{r22}</R22>
</CoordinateFrame>",
            x = self.x,
            y = self.y,
            z = self.z,
            r00 = self.r00,
            r01 = self.r01,
            r02 = self.r02,
            r10 = self.r10,
            r11 = self.r11,
            r12 = self.r12,
            r20 = self.r20,
            r21 = self.r21,
            r22 = self.r22
        )
    }
}
simple_property!(bool, CanCollide);
pub struct Color3uint8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ToString for Color3uint8 {
    fn to_string(&self) -> String {
        format!(
            "<Color3uint8 name=\"Color3uint8\">{}</Color3uint8>",
            ((self.a as u32) << 24)
                + ((self.r as u32) << 16)
                + ((self.g as u32) << 8)
                + (self.b as u32)
        )
    }
}

pub struct CustomPhysicalProperties(bool);

impl ToString for CustomPhysicalProperties {
    fn to_string(&self) -> String {
        format!("<PhysicalProperties name=\"CustomPhysicalProperties\"><CustomPhysics>{}</CustomPhysics></PhysicalProperties>", self.0)
    }
}

// <PhysicalProperties name="CustomPhysicalProperties">
// 	<CustomPhysics>false</CustomPhysics>
// </PhysicalProperties>
simple_property!(float, Elasticity);
simple_property!(float, Friction);
simple_property!(float, FrontParamA);
simple_property!(float, FrontParamB);
simple_property!(token, FrontSurface);
simple_property!(token, FrontSurfaceInput);
simple_property!(float, LeftParamA);
simple_property!(float, LeftParamB);
simple_property!(token, LeftSurface);
simple_property!(token, LeftSurfaceInput);
simple_property!(bool, Locked);
simple_property!(token, Material);
simple_property!(string, Name);
simple_property!(float, Reflectance);
simple_property!(float, RightParamA);
simple_property!(float, RightParamB);
simple_property!(token, RightSurface);
simple_property!(token, RightSurfaceInput);
vec3_prop!(RotVelocity);
simple_property!(float, TopParamA);
simple_property!(float, TopParamB);
simple_property!(token, TopSurface);
simple_property!(token, TopSurfaceInput);
simple_property!(float, Transparency);
vec3_prop!(Velocity);
simple_property!(token, formFactorRaw);
simple_property!(token, shape);
vec3_prop!(size);
