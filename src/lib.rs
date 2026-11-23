use core::f64::consts::{PI, TAU};
use std::{
    cmp::Ordering, fmt::{Debug, Display}, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign} 
};

// █▀▄▀█ ▄▀█ █▀▀ █▀█ █▀█ █▀ 
// █░▀░█ █▀█ █▄▄ █▀▄ █▄█ ▄█ 

/// Defines a public constant of `Unit` in current scope.
///
/// Syntax: name of the constant, its dimension, name of the unit, its symbol,
///     function converting to base unit, function converting from base unit 
///
/// Example usage:
/// ```rust
/// define_unit!(KILOMETER, Length, "kilometer", "km", |x| x * 1e3, |x| x / 1e3);
/// ```
/// Note that the predefined units use an internal macro that implements the `Unit` constant to the `Value`. 
///
/// - `type_name`: identificator of the constant
/// - `value`: the type of the dimension; `Value<L, M, T, I, TH, N, J, A>`
/// - `name`: the name of the unit; `&str`
/// - `symbol`: the shortcut to write the unit; `&str`
/// - `to_base`: function taking the value and converting it to base SI unit; `fn`
/// - `from_base`: function taking the base SI value converting it to this unit; `fn`
#[macro_export]
macro_rules! define_unit {
    ($type_name:ident, $value:ty, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        pub const $type_name: $crate::Unit<
            { <$value>::DIMS[0] }, { <$value>::DIMS[1] },
            { <$value>::DIMS[2] }, { <$value>::DIMS[3] },
            { <$value>::DIMS[4] }, { <$value>::DIMS[5] },
            { <$value>::DIMS[6] }, { <$value>::DIMS[7] },
        > = $crate::Unit::new($name, $symbol, $to_base, $from_base);
    };
}


/// Implements cross unit multiplication and division (Length * Length = Area).
///
/// Syntax: muliplier 1, muliplier 1 base unit, muliplier 2, muliplier 2 base unit, product, product base unit
///   - In case the relation is squaring, the `Value` and its `Unit` are inputted only once
///
/// Example usage
/// ```rust
/// priv_cross_mul_div!(Length, Length::METER, Area, Area::SQUARE_METER)
/// priv_cross_mul_div!(Time, Time::SECOND, Velocity, Velocity::METER_PER_SECOND, Length, Length::METER)
/// ```
/// `mult`, `mult_1`, `mult_2`, `prod` define the dimensions; `Value<L, M, T, I, TH, N, J, A>`
/// `mult_out`, `mult_1_out`, `mult_2_out`, `prod_out` define the unit outputted; `Unit<L, M, T, I, TH, N, J, A>`
macro_rules! priv_cross_mul_div {
    ($mult_1:ty, $mult_1_out:expr, $mult_2:ty, $mult_2_out:expr, $prod:ty, $prod_out:expr) => {
        impl Mul<$mult_1> for $mult_2 {
            type Output = $prod;
            fn mul(self, rhs: $mult_1) -> Self::Output {
                let self_base = self.get_unit().get_to_base();
                let rhs_base = rhs.get_unit().get_to_base();
                let base_out = $prod_out.get_from_base();
                Self::Output::new(base_out(self_base(self.get_value()) * rhs_base(rhs.get_value())), $prod_out)
            }
        }
        impl Mul<$mult_2> for $mult_1 {
            type Output = $prod;
            fn mul(self, rhs: $mult_2) -> Self::Output {
                let self_base = self.get_unit().get_to_base();
                let rhs_base = rhs.get_unit().get_to_base();
                let base_out = $prod_out.get_from_base();
                Self::Output::new(base_out(self_base(self.get_value()) * rhs_base(rhs.get_value())), $prod_out)
            }
        }
        impl Div<$mult_1> for $prod {
            type Output = $mult_2;
            fn div(self, rhs: $mult_1) -> Self::Output {
                let self_base = self.get_unit().get_to_base();
                let rhs_base = rhs.get_unit().get_to_base();
                let base_out = $mult_2_out.get_from_base();
                Self::Output::new(base_out(self_base(self.get_value()) / rhs_base(rhs.get_value())), $mult_2_out)
            }
        }
        impl Div<$mult_2> for $prod {
            type Output = $mult_1;
            fn div(self, rhs: $mult_2) -> Self::Output {
                let self_base = self.get_unit().get_to_base();
                let rhs_base = rhs.get_unit().get_to_base();
                let base_out = $mult_1_out.get_from_base();
                Self::Output::new(base_out(self_base(self.get_value()) / rhs_base(rhs.get_value())), $mult_1_out)
            }
        }
    };
    ($mult:ty, $mult_out:expr, $prod:ty, $prod_out:expr) => {
        impl Mul for $mult {
            type Output = $prod;
            fn mul(self, rhs: Self) -> Self::Output {
                let self_base = self.get_unit().get_to_base();
                let rhs_base = rhs.get_unit().get_to_base();
                let base_out = $prod_out.get_from_base();
                Self::Output::new(base_out(self_base(self.get_value()) * rhs_base(rhs.get_value())),$prod_out)
            }
        }
        impl Div<$mult> for $prod {
            type Output = $mult;
            fn div(self, rhs: $mult) -> Self::Output {
                let self_base = self.get_unit().get_to_base();
                let rhs_base = rhs.get_unit().get_to_base();
                let base_out = $mult_out.get_from_base();
                Self::Output::new(base_out(self_base(self.get_value()) / rhs_base(rhs.get_value())), $mult_out)
            }
        }
    };
}

/// Implements `Unit` as public constant for its dimension.
///
/// Syntax: name of the constant, its dimension, name of the unit, its symbol,
///     function converting to base unit, function converting from base unit 
///
/// Example usage:
/// ```rust
/// priv_define_unit!(KILOMETER, Length, "kilometer", "km", |x| x * 1e3, |x| x / 1e3);
/// ```
/// - `type_name`: identificator of the constant
/// - `value`: the type of the dimension; `Value<L, M, T, I, TH, N, J, A>`
/// - `name`: the name of the unit; `&str`
/// - `symbol`: the shortcut to write the unit; `&str`
/// - `to_base`: function taking the value and converting it to base SI unit; `fn`
/// - `from_base`: function taking the base SI value converting it to this unit; `fn`
macro_rules! priv_define_unit {
    ($type_name:ident, $value:ty, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        impl $value {
            pub const $type_name: $crate::Unit<
                { <$value>::DIMS[0] }, { <$value>::DIMS[1] },
                { <$value>::DIMS[2] }, { <$value>::DIMS[3] },
                { <$value>::DIMS[4] }, { <$value>::DIMS[5] },
                { <$value>::DIMS[6] }, { <$value>::DIMS[7] },
            > = $crate::Unit { name: $name, symbol: $symbol, to_base: $to_base, from_base: $from_base };
        }
    };
}

/// Implements for type with the 8 constant parameters.
///
/// Syntax: name of the type, path of the trait, implementing block
///   - if no traits is being implemented, the parameters is omitted
///
/// Example usage:
/// ```rust
/// priv_impl!(Value, Display, {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{} {}", self.value, self.unit.symbol)
///     }
/// });
/// priv_impl!(Unit, {
///     pub const fn new(name: &'static str, symbol: &'static str, to_base: fn(f64) -> f64, from_base: fn(f64) -> f64) -> Self {
///         Unit { name: name, symbol: symbol, to_base: to_base, from_base: from_base }
///     }
/// });
/// ```
///
/// - `for_type`: the type being implemented
/// - `path`: the trait being implemented for `for_type`; optional
/// - `body`: the contents found in in typical impl 
macro_rules! priv_impl {
    ($for_type:ident, { $($body:tt)* }) => {
        impl<const L: i32, const M: i32, const T: i32, const I: i32,
            const TH: i32, const N: i32, const J: i32, const A: i32,
        > $for_type<L, M, T, I, TH, N, J, A> {
            $($body)*
        }
    };
    ($for_type:ident, $trait:path, { $($body:tt)* }) => {
        impl<const L: i32, const M: i32, const T: i32, const I: i32,
            const TH: i32, const N: i32, const J: i32, const A: i32,
        > $trait for $for_type<L, M, T, I, TH, N, J, A> {
            $($body)*
        }
    };
}



// █▀▀ █▀█ █▀█ █▀▀  █▀ ▀█▀ █▀█ █░█ █▀▀ ▀█▀ █▀ 
// █▄▄ █▄█ █▀▄ ██▄  ▄█ ░█░ █▀▄ █▄█ █▄▄ ░█░ ▄█

/// Core struct holding the value and unit.
///
/// Referenced by `Value<L, M, T, I, TH, N, J, A>`, where all generic types are `i32`
/// - each number represents the power each dimension contributes
///     - e.q. `Area` being `Length` * `Length` gets `L=2` and the rest is `0`
///
/// List of parameters: 
/// - `L`: Length (meter)
/// - `M`: Mass (kilogram)
/// - `T`: Time (second)
/// - `I`: Electric current (ampere)
/// - `TH`: Thermodynamic temperature (kelvin)
/// - `N`: Amount of substance (mole)
/// - `J`: Luminous intensity (candela)
/// - `A`: Plane angle (radian)
#[derive(Clone, Copy, Debug)]
pub struct Value<
    const L: i32,
    const M: i32,
    const T: i32,
    const I: i32,
    const TH: i32,
    const N: i32,
    const J: i32,
    const A: i32,
> {
    value: f64,
    unit: Unit<L, M, T, I, TH, N, J, A>,
}
priv_impl!(Value, {
    /// Returns new instance from unit and value
    pub const fn new(value: f64, unit: Unit<L, M, T, I, TH, N, J, A>) -> Self {
        Value { value: value , unit: unit }
    }
    /// Returns new instance of the specified unit
    pub fn convert_to(&self, unit: Unit<L, M, T, I, TH, N, J, A>) -> Self {
        let base_fn = &self.unit.to_base;
        let to_fn = &unit.from_base;
        Value { value: to_fn(base_fn(self.value)), unit: unit }
    }
    /// Returns the stored `f64`
    pub const fn get_value(&self) -> f64 {
        self.value
    }
    /// Returns the unit representing the value
    pub fn get_unit(&self) -> Unit<L, M, T, I, TH, N, J, A> {
        self.unit
    }
    /// Compares 2 values with dynamic margin
    /// Returns `true` if base value difference is smaller than margin
    pub fn approx_eq(&self, other: &Self) -> bool {
        let self_base = &self.unit.to_base;
        let other_base = &other.unit.to_base;
        let a = self_base(self.value);
        let b = other_base(other.value);
        (a - b).abs() <= 1e-12_f64.max(1e-6 * a.abs().max(b.abs()))
    }
    pub const DIMS: [i32; 8] = [L, M, T, I, TH, N, J, A];
});
priv_impl!(Value, Display, {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol)
    }
});
priv_impl!(Value, PartialEq,{
    fn eq(&self, other: &Self) -> bool {
        (self.unit.to_base)(self.value) == (other.unit.to_base)(other.value)
    }
});
priv_impl!(Value, PartialOrd, {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.unit.to_base)(self.value).partial_cmp(&(other.unit.to_base)(other.value))
    }
});

/// Core struct representing the `f64` held by `Value`.
///
/// Referenced by `Unit<L, M, T, I, TH, N, J, A>`, where all generic types are `i32`
/// - each number represents the power each dimension contributes
///     - e.q. `Area` being `Length` * `Length` gets `L=2` and the rest is `0`
#[derive(Clone, Copy, Debug)]
pub struct Unit<
    const L: i32,
    const M: i32,
    const T: i32,
    const I: i32,
    const TH: i32,
    const N: i32,
    const J: i32,
    const A: i32,
> {
    name: &'static str,
    symbol: &'static str,
    to_base: fn(f64) -> f64,
    from_base: fn(f64) -> f64,
}
priv_impl!(Unit, {
    /// Returns new instance from name, symbol and functions converting to and from base unit
    pub const fn new(name: &'static str, symbol: &'static str, to_base: fn(f64) -> f64, from_base: fn(f64) -> f64) -> Self {
        Unit { name: name, symbol: symbol, to_base: to_base, from_base: from_base }
    }
    /// Returns the set name as `&str`
    pub const fn get_name(&self) -> &str {
        self.name
    }
    /// Returns the set symbol as `&str`
    pub const fn get_symbol(&self) -> &str {
        self.symbol
    }
    /// Returns the functions used to convert value of this unit to its base unit value
    pub const fn get_to_base(&self) -> fn(f64) -> f64 {
        self.to_base
    }
    /// Returns the functions used to convert from base value to this units value
    pub const fn get_from_base(&self) -> fn(f64) -> f64 {
        self.from_base
    }
});
priv_impl!(Unit, Display, {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unit: {} [{}], to base: {}, from base: {}", self.name, self.symbol, (self.to_base)(1_f64), (self.from_base)(1_f64))
    }
});

// █▀█ █▀█ █▀▀ █▀█ ▄▀█ ▀█▀ █▀█ █▀█ █▀ 
// █▄█ █▀▀ ██▄ █▀▄ █▀█ ░█░ █▄█ █▀▄ ▄█

priv_impl!(Value, Add, {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let self_base = &self.unit.to_base;
        let self_self = &self.unit.from_base;
        let rhs_base = rhs.unit.to_base;
        Self { value: self_self(self_base(self.value) + rhs_base(rhs.value)), unit: self.unit }
    }
});
priv_impl!(Value, AddAssign, {
    fn add_assign(&mut self, rhs: Self) {
        let self_base = &self.unit.to_base;
        let self_self = &self.unit.from_base;
        let rhs_base = rhs.unit.to_base;
        self.value = self_self(self_base(self.value) + rhs_base(rhs.value));
    }
});
priv_impl!(Value, Sub, {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let self_base = &self.unit.to_base;
        let self_self = &self.unit.from_base;
        let rhs_base = rhs.unit.to_base;
        Self { value: self_self(self_base(self.value) - rhs_base(rhs.value)), unit: self.unit }
    }
});
priv_impl!(Value, SubAssign, {
    fn sub_assign(&mut self, rhs: Self) {
        let self_base = &self.unit.to_base;
        let self_self = &self.unit.from_base;
        let rhs_base = rhs.unit.to_base;
        self.value = self_self(self_base(self.value) - rhs_base(rhs.value));
    }
});
priv_impl!(Value, Mul<f64>, {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { value: self.value * rhs, unit: self.unit }
    }
});
priv_impl!(Value, MulAssign<f64>, {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs; 
    }
});
priv_impl!(Value, Div<f64>, {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self { value: self.value / rhs, unit: self.unit }
    }
});
priv_impl!(Value, DivAssign<f64>, {
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs; 
    }
});
priv_impl!(Value, Neg, {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { value: -self.value, unit: self.unit }
    }
});

impl<const L: i32, const M: i32, const T: i32, const I: i32,
    const TH: i32, const N: i32, const J: i32,const A: i32,
> Mul<Value<L, M, T, I, TH, N, J, A>> for f64 {
    type Output = Value<L, M, T, I, TH, N, J, A>;
    fn mul(self, rhs: Value<L, M, T, I, TH, N, J, A>) -> Self::Output {
        Value { value: rhs.value * self, unit: rhs.unit }
    }
}

// █▄▄ ▄▀█ █▀ █▀▀  █▀ █ 
// █▄█ █▀█ ▄█ ██▄  ▄█ █ 

pub type Dimensionless = Value<0,0,0,0,0,0,0,0>;
pub type Length = Value<1,0,0,0,0,0,0,0>;
pub type Mass = Value<0,1,0,0,0,0,0,0>;
pub type Time = Value<0,0,1,0,0,0,0,0>;
pub type ElectricCurrent = Value<0,0,0,1,0,0,0,0>;
pub type Temperature = Value<0,0,0,0,1,0,0,0>;
pub type AmountOfSubstance = Value<0,0,0,0,0,1,0,0>;
pub type LuminousIntensity = Value<0,0,0,0,0,0,1,0>;
pub type Angle = Value<0,0,0,0,0,0,0,1>;

priv_define_unit!(ONE, Dimensionless, "one", "", |x| x, |x| x);
priv_define_unit!(METER, Length, "meter", "m", |x| x, |x| x);
priv_define_unit!(KILOGRAM, Mass, "kilogram", "kg", |x| x, |x| x);
priv_define_unit!(SECOND, Time, "second", "s", |x| x, |x| x);
priv_define_unit!(AMPERE, ElectricCurrent, "ampere", "A", |x| x, |x| x);
priv_define_unit!(KELVIN, Temperature, "kelvin", "K", |x| x, |x| x);
priv_define_unit!(MOLE, AmountOfSubstance, "mole", "mol", |x| x, |x| x);
priv_define_unit!(CANDELA, LuminousIntensity, "candela", "cd", |x| x, |x| x);
priv_define_unit!(RADIAN, Angle, "radian", "rad", |x| x, |x| x);

// █▀▄ █▀▀ █▀█ █ █░█ █▀▀ █▀▄ 
// █▄▀ ██▄ █▀▄ █ ▀▄▀ ██▄ █▄▀ 

// Geometry & Kinematics
pub type Area = Value<2,0,0,0,0,0,0,0>;
pub type Volume = Value<3,0,0,0,0,0,0,0>;
pub type Velocity = Value<1,0,-1,0,0,0,0,0>;
pub type Acceleration = Value<1,0,-2,0,0,0,0,0>;
pub type Frequency = Value<0,0,-1,0,0,0,0,0>;
pub type AngularVelocity = Value<0,0,-1,0,0,0,0,1>;
pub type AngularAcceleration = Value<0,0,-2,0,0,0,0,1>;

// Mechanics & Fluid Dynamics
pub type Density = Value<-3,1,0,0,0,0,0,0>;
pub type Momentum = Value<1,1,-1,0,0,0,0,0>;
pub type Force = Value<1,1,-2,0,0,0,0,0>;
pub type Pressure = Value<-1,1,-2,0,0,0,0,0>;
pub type Energy = Value<2,1,-2,0,0,0,0,0>;
pub type Torque = Value<2,1,-2,0,0,0,0,-1>;
pub type Power = Value<2,1,-3,0,0,0,0,0>;
pub type VolumetricFlowRate = Value<3,0,-1,0,0,0,0,0>;
pub type MassFlowRate = Value<0,1,-1,0,0,0,0,0>;
pub type DynamicViscosity = Value<-1,1,-1,0,0,0,0,0>;
pub type KinematicViscosity = Value<2,0,-1,0,0,0,0,0>;

// Electromagnetism
pub type ElectricCharge = Value<0,0,1,1,0,0,0,0>;
pub type Voltage = Value<2,1,-3,-1,0,0,0,0>;
pub type ElectricResistance = Value<2,1,-3,-2,0,0,0,0>;
pub type ElectricConductance = Value<-2,-1,3,2,0,0,0,0>;
pub type Capacitance = Value<-2,-1,4,2,0,0,0,0>;
pub type MagneticFlux = Value<2,1,-2,-1,0,0,0,0>;
pub type MagneticFluxDensity = Value<0,1,-2,-1,0,0,0,0>;
pub type Inductance = Value<2,1,-2,-2,0,0,0,0>;

// Photometry
pub type LuminousFlux = Value<0,0,0,0,0,0,1,2>;
pub type Illuminance = Value<-2,0,0,0,0,0,1,2>;

// Dimensionless
priv_define_unit!(PERCENT, Dimensionless, "percent", "%", |x| x * 1e-2, |x| x * 1e2);
priv_define_unit!(PERMILLE, Dimensionless, "permille", "‰", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(PPM, Dimensionless, "parts per million", "ppm", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(PPB, Dimensionless, "parts per billion", "ppb", |x| x * 1e-9, |x| x * 1e9);

// Angle (base: radian)
priv_define_unit!(DEGREE, Angle, "degree", "°", |x| x * (PI / 180.0), |x| x * (180.0 / PI));
priv_define_unit!(GRADIAN, Angle, "gradian", "gon", |x| x * (PI / 200.0), |x| x * (200.0 / PI));
priv_define_unit!(TURN, Angle, "turn", "tr", |x| x * TAU, |x| x / TAU);
priv_define_unit!(ARCMINUTE, Angle, "arcminute", "′", |x| x * (PI / 10_800.0), |x| x * (10_800.0 / PI));
priv_define_unit!(ARCSECOND, Angle, "arcsecond", "″", |x| x * (PI / 648_000.0), |x| x * (648_000.0 / PI));

// Length (base: meter)
priv_define_unit!(KILOMETER, Length, "kilometer", "km", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(DECIMETER, Length, "decimeter", "dm", |x| x * 1e-1, |x| x * 1e1);
priv_define_unit!(CENTIMETER, Length, "centimeter", "cm", |x| x * 1e-2, |x| x * 1e2);
priv_define_unit!(MILLIMETER, Length, "millimeter", "mm", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(MICROMETER, Length, "micrometer", "µm", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(NANOMETER, Length, "nanometer", "nm", |x| x * 1e-9, |x| x * 1e9);
priv_define_unit!(ANGSTROM, Length, "angstrom", "Å", |x| x * 1e-10, |x| x * 1e10);
priv_define_unit!(INCH, Length, "inch", "in", |x| x * 0.0254, |x| x / 0.0254);
priv_define_unit!(FOOT, Length, "foot", "ft", |x| x * 0.3048, |x| x / 0.3048);
priv_define_unit!(YARD, Length, "yard", "yd", |x| x * 0.9144, |x| x / 0.9144);
priv_define_unit!(MILE, Length, "mile", "mi", |x| x * 1609.344, |x| x / 1609.344);
priv_define_unit!(NAUTICAL_MILE, Length, "nautical mile", "nmi", |x| x * 1852.0, |x| x / 1852.0);
priv_define_unit!(ASTRONOMICAL_UNIT, Length, "astronomical unit", "AU", |x| x * 149_597_870_700.0, |x| x / 149_597_870_700.0);
priv_define_unit!(LIGHT_YEAR, Length, "light-year", "ly", |x| x * 9_460_730_472_580_800.0, |x| x / 9_460_730_472_580_800.0);
priv_define_unit!(PARSEC, Length, "parsec", "pc", |x| x * 3.085_677_581_491_367_3e16, |x| x / 3.085_677_581_491_367_3e16);

// Mass (base: kilogram)
priv_define_unit!(GRAM, Mass, "gram", "g", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(MILLIGRAM, Mass, "milligram", "mg", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(MICROGRAM, Mass, "microgram", "µg", |x| x * 1e-9, |x| x * 1e9);
priv_define_unit!(METRIC_TON, Mass, "metric ton", "t", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(POUND, Mass, "pound", "lb", |x| x * 0.453_592_37, |x| x / 0.453_592_37);
priv_define_unit!(OUNCE, Mass, "ounce", "oz", |x| x * 0.028_349_523_125, |x| x / 0.028_349_523_125);
priv_define_unit!(STONE, Mass, "stone", "st", |x| x * 6.350_293_18, |x| x / 6.350_293_18);
priv_define_unit!(SHORT_TON, Mass, "short ton", "ton", |x| x * 907.184_74, |x| x / 907.184_74);
priv_define_unit!(LONG_TON, Mass, "long ton", "LT", |x| x * 1016.046_908_8, |x| x / 1016.046_908_8);
priv_define_unit!(SLUG, Mass, "slug", "slug", |x| x * 14.593_902_937_206_364, |x| x / 14.593_902_937_206_364);
priv_define_unit!(CARAT, Mass, "carat", "ct", |x| x * 2e-4, |x| x / 2e-4);
priv_define_unit!(DALTON, Mass, "dalton", "Da", |x| x * 1.660_539_066_60e-27, |x| x / 1.660_539_066_60e-27);

// Time (base: second)
priv_define_unit!(MILLISECOND, Time, "millisecond", "ms", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(MICROSECOND, Time, "microsecond", "µs", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(NANOSECOND, Time, "nanosecond", "ns", |x| x * 1e-9, |x| x * 1e9);
priv_define_unit!(MINUTE, Time, "minute", "min", |x| x * 60.0, |x| x / 60.0);
priv_define_unit!(HOUR, Time, "hour", "h", |x| x * 3_600.0, |x| x / 3_600.0);
priv_define_unit!(DAY, Time, "day", "d", |x| x * 86_400.0, |x| x / 86_400.0);
priv_define_unit!(WEEK, Time, "week", "wk", |x| x * 604_800.0, |x| x / 604_800.0);
priv_define_unit!(YEAR, Time, "Julian year", "yr", |x| x * 31_557_600.0, |x| x / 31_557_600.0);

// Temperature (base: kelvin)
priv_define_unit!(CELSIUS, Temperature, "degree Celsius", "°C", |x| x + 273.15, |x| x - 273.15);
priv_define_unit!(FAHRENHEIT, Temperature, "degree Fahrenheit", "°F", |x| (x - 32.0) * (5.0 / 9.0) + 273.15, |x| (x - 273.15) * (9.0 / 5.0) + 32.0);
priv_define_unit!(RANKINE, Temperature, "degree Rankine", "°R", |x| x * (5.0 / 9.0), |x| x * (9.0 / 5.0));

// Amount of Substance (base: mole)
priv_define_unit!(MILLIMOLE, AmountOfSubstance, "millimole", "mmol", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(MICROMOLE, AmountOfSubstance, "micromole", "µmol", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(KILOMOLE, AmountOfSubstance, "kilomole", "kmol", |x| x * 1e3, |x| x / 1e3);

// Area (base: square meter)
priv_define_unit!(SQUARE_METER, Area, "square meter", "m²", |x| x, |x| x);
priv_define_unit!(SQUARE_KILOMETER, Area, "square kilometer", "km²", |x| x * 1e6, |x| x / 1e6);
priv_define_unit!(SQUARE_CENTIMETER, Area, "square centimeter", "cm²", |x| x * 1e-4, |x| x * 1e4);
priv_define_unit!(SQUARE_MILLIMETER, Area, "square millimeter", "mm²", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(HECTARE, Area, "hectare", "ha", |x| x * 1e4, |x| x / 1e4);
priv_define_unit!(ARE, Area, "are", "a", |x| x * 1e2, |x| x / 1e2);
priv_define_unit!(SQUARE_INCH, Area, "square inch", "in²", |x| x * 0.000_645_16, |x| x / 0.000_645_16);
priv_define_unit!(SQUARE_FOOT, Area, "square foot", "ft²", |x| x * 0.092_903_04, |x| x / 0.092_903_04);
priv_define_unit!(SQUARE_YARD, Area, "square yard", "yd²", |x| x * 0.836_127_36, |x| x / 0.836_127_36);
priv_define_unit!(ACRE, Area, "acre", "ac", |x| x * 4_046.856_422_4, |x| x / 4_046.856_422_4);
priv_define_unit!(SQUARE_MILE, Area, "square mile", "mi²", |x| x * 2_589_988.110_336, |x| x / 2_589_988.110_336);

// Volume (base: cubic meter)
priv_define_unit!(CUBIC_METER, Volume, "cubic meter", "m³", |x| x, |x| x);
priv_define_unit!(LITER, Volume, "liter", "L", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(DECILITER, Volume, "deciliter", "dL", |x| x * 1e-4, |x| x * 1e4);
priv_define_unit!(CENTILITER, Volume, "centiliter", "cL", |x| x * 1e-5, |x| x * 1e5);
priv_define_unit!(MILLILITER, Volume, "milliliter", "mL", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(CUBIC_CENTIMETER, Volume, "cubic centimeter", "cm³", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(CUBIC_MILLIMETER, Volume, "cubic millimeter", "mm³", |x| x * 1e-9, |x| x * 1e9);
priv_define_unit!(CUBIC_INCH, Volume, "cubic inch", "in³", |x| x * 1.638_706_4e-5, |x| x / 1.638_706_4e-5);
priv_define_unit!(CUBIC_FOOT, Volume, "cubic foot", "ft³", |x| x * 0.028_316_846_592, |x| x / 0.028_316_846_592);
priv_define_unit!(CUBIC_YARD, Volume, "cubic yard", "yd³", |x| x * 0.764_554_857_984, |x| x / 0.764_554_857_984);
priv_define_unit!(US_GALLON, Volume, "US gallon", "gal", |x| x * 0.003_785_411_784, |x| x / 0.003_785_411_784);
priv_define_unit!(US_QUART, Volume, "US quart", "qt", |x| x * 0.000_946_352_946, |x| x / 0.000_946_352_946);
priv_define_unit!(US_PINT, Volume, "US pint", "pt", |x| x * 0.000_473_176_473, |x| x / 0.000_473_176_473);
priv_define_unit!(US_CUP, Volume, "US cup", "cup", |x| x * 0.000_236_588_236_5, |x| x / 0.000_236_588_236_5);
priv_define_unit!(US_FLUID_OUNCE, Volume, "US fluid ounce", "fl oz", |x| x * 2.957_352_956_25e-5, |x| x / 2.957_352_956_25e-5);
priv_define_unit!(US_TABLESPOON, Volume, "US tablespoon", "tbsp", |x| x * 1.478_676_478_125e-5, |x| x / 1.478_676_478_125e-5);
priv_define_unit!(US_TEASPOON, Volume, "US teaspoon", "tsp", |x| x * 4.928_921_593_75e-6, |x| x / 4.928_921_593_75e-6);
priv_define_unit!(IMPERIAL_GALLON, Volume, "imperial gallon", "imp gal", |x| x * 0.004_546_09, |x| x / 0.004_546_09);
priv_define_unit!(IMPERIAL_PINT, Volume, "imperial pint", "imp pt", |x| x * 0.000_568_261_25, |x| x / 0.000_568_261_25);
priv_define_unit!(OIL_BARREL, Volume, "oil barrel", "bbl", |x| x * 0.158_987_294_928, |x| x / 0.158_987_294_928);

// Velocity (base: meter per second)
priv_define_unit!(METER_PER_SECOND, Velocity, "meter per second", "m/s", |x| x, |x| x);
priv_define_unit!(KILOMETER_PER_HOUR, Velocity, "kilometer per hour", "km/h", |x| x / 3.6, |x| x * 3.6);
priv_define_unit!(MILE_PER_HOUR, Velocity, "mile per hour", "mph", |x| x * 0.447_04, |x| x / 0.447_04);
priv_define_unit!(FOOT_PER_SECOND, Velocity, "foot per second", "ft/s", |x| x * 0.3048, |x| x / 0.3048);
priv_define_unit!(KNOT, Velocity, "knot", "kn", |x| x * (1852.0 / 3600.0), |x| x * (3600.0 / 1852.0));
priv_define_unit!(SPEED_OF_LIGHT, Velocity, "speed of light", "c", |x| x * 299_792_458.0, |x| x / 299_792_458.0);

// Acceleration (base: meter per second squared)
priv_define_unit!(METER_PER_SECOND_SQUARED, Acceleration, "meter per second squared", "m/s²", |x| x, |x| x);
priv_define_unit!(STANDARD_GRAVITY, Acceleration, "standard gravity", "g₀", |x| x * 9.806_65, |x| x / 9.806_65);
priv_define_unit!(FOOT_PER_SECOND_SQUARED, Acceleration, "foot per second squared", "ft/s²", |x| x * 0.3048, |x| x / 0.3048);

// Frequency & Angular Velocity & AngularAcceleration (base: hertz / rad/s)
priv_define_unit!(HERTZ, Frequency, "hertz", "Hz", |x| x, |x| x);
priv_define_unit!(KILOHERTZ, Frequency, "kilohertz", "kHz", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(MEGAHERTZ, Frequency, "megahertz", "MHz", |x| x * 1e6, |x| x / 1e6);
priv_define_unit!(GIGAHERTZ, Frequency, "gigahertz", "GHz", |x| x * 1e9, |x| x / 1e9);
priv_define_unit!(RPM, Frequency, "revolutions per minute", "rpm", |x| x / 60.0, |x| x * 60.0);
priv_define_unit!(RADIAN_PER_SECOND, AngularVelocity, "radian per second", "rad/s", |x| x, |x| x);
priv_define_unit!(DEGREE_PER_SECOND, AngularVelocity, "degree per second", "°/s", |x| x * (PI / 180.0), |x| x * (180.0 / PI));
priv_define_unit!(RADIAN_PER_SECOND_SQUARED, AngularAcceleration, "radian per second squared", "rad/s²", |x| x, |x| x);

// Density (base: kilogram per cubic meter)
priv_define_unit!(KILOGRAM_PER_CUBIC_METER, Density, "kilogram per cubic meter", "kg/m³", |x| x, |x| x);
priv_define_unit!(GRAM_PER_CUBIC_CENTIMETER, Density, "gram per cubic centimeter", "g/cm³", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(POUND_PER_CUBIC_FOOT, Density, "pound per cubic foot", "lb/ft³", |x| x * 16.018_463_373_960_138, |x| x / 16.018_463_373_960_138);

// Momentum (base; kilogram meter per second)
priv_define_unit!(KILOGRAM_METER_PER_SECOND, Momentum, "kilogram meter per second", "kg·m/s", |x| x, |x| x);


// Force (base: newton)
priv_define_unit!(NEWTON, Force, "newton", "N", |x| x, |x| x);
priv_define_unit!(KILONEWTON, Force, "kilonewton", "kN", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(MILLINEWTON, Force, "millinewton", "mN", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(DYNE, Force, "dyne", "dyn", |x| x * 1e-5, |x| x * 1e5);
priv_define_unit!(POUND_FORCE, Force, "pound-force", "lbf", |x| x * 4.448_221_615_260_5, |x| x / 4.448_221_615_260_5);
priv_define_unit!(KILOGRAM_FORCE, Force, "kilogram-force", "kgf", |x| x * 9.806_65, |x| x / 9.806_65);

// Pressure (base: pascal)
priv_define_unit!(PASCAL, Pressure, "pascal", "Pa", |x| x, |x| x);
priv_define_unit!(HECTOPASCAL, Pressure, "hectopascal", "hPa", |x| x * 1e2, |x| x / 1e2);
priv_define_unit!(KILOPASCAL, Pressure, "kilopascal", "kPa", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(MEGAPASCAL, Pressure, "megapascal", "MPa", |x| x * 1e6, |x| x / 1e6);
priv_define_unit!(BAR, Pressure, "bar", "bar", |x| x * 1e5, |x| x / 1e5);
priv_define_unit!(MILLIBAR, Pressure, "millibar", "mbar", |x| x * 1e2, |x| x / 1e2);
priv_define_unit!(ATMOSPHERE, Pressure, "standard atmosphere", "atm", |x| x * 101_325.0, |x| x / 101_325.0);
priv_define_unit!(TORR, Pressure, "torr", "Torr", |x| x * (101_325.0 / 760.0), |x| x * (760.0 / 101_325.0));
priv_define_unit!(PSI, Pressure, "pound per square inch", "psi", |x| x * 6_894.757_293_168_361, |x| x / 6_894.757_293_168_361);
priv_define_unit!(INCH_OF_MERCURY, Pressure, "inch of mercury", "inHg", |x| x * 3_386.389, |x| x / 3_386.389);

// Energy (base: joule)
priv_define_unit!(JOULE, Energy, "joule", "J", |x| x, |x| x);
priv_define_unit!(KILOJOULE, Energy, "kilojoule", "kJ", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(MEGAJOULE, Energy, "megajoule", "MJ", |x| x * 1e6, |x| x / 1e6);
priv_define_unit!(WATT_HOUR, Energy, "watt-hour", "Wh", |x| x * 3_600.0, |x| x / 3_600.0);
priv_define_unit!(KILOWATT_HOUR, Energy, "kilowatt-hour", "kWh", |x| x * 3_600_000.0, |x| x / 3_600_000.0);
priv_define_unit!(CALORIE, Energy, "calorie", "cal", |x| x * 4.184, |x| x / 4.184);
priv_define_unit!(KILOCALORIE, Energy, "kilocalorie", "kcal", |x| x * 4_184.0, |x| x / 4_184.0);
priv_define_unit!(ELECTRONVOLT, Energy, "electronvolt", "eV", |x| x * 1.602_176_634e-19, |x| x / 1.602_176_634e-19);
priv_define_unit!(BTU, Energy, "British thermal unit", "BTU", |x| x * 1_055.055_852_62, |x| x / 1_055.055_852_62);
priv_define_unit!(FOOT_POUND, Energy, "foot-pound", "ft·lbf", |x| x * 1.355_817_948_331_400_4, |x| x / 1.355_817_948_331_400_4);
priv_define_unit!(ERG, Energy, "erg", "erg", |x| x * 1e-7, |x| x * 1e7);

// Torque (base: newton-meter)
priv_define_unit!(NEWTON_METER, Torque, "newton-meter", "N·m", |x| x, |x| x);
priv_define_unit!(POUND_FOOT, Torque, "pound-foot", "lbf·ft", |x| x * 1.355_817_948_331_400_4, |x| x / 1.355_817_948_331_400_4);
priv_define_unit!(POUND_INCH, Torque, "pound-inch", "lbf·in", |x| x * 0.112_984_829_027_616_7, |x| x / 0.112_984_829_027_616_7);

// Power (base: watt)
priv_define_unit!(WATT, Power, "watt", "W", |x| x, |x| x);
priv_define_unit!(MILLIWATT, Power, "milliwatt", "mW", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(KILOWATT, Power, "kilowatt", "kW", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(MEGAWATT, Power, "megawatt", "MW", |x| x * 1e6, |x| x / 1e6);
priv_define_unit!(GIGAWATT, Power, "gigawatt", "GW", |x| x * 1e9, |x| x / 1e9);
priv_define_unit!(HORSEPOWER, Power, "mechanical horsepower", "hp", |x| x * 745.699_871_582_270_2, |x| x / 745.699_871_582_270_2);
priv_define_unit!(METRIC_HORSEPOWER, Power, "metric horsepower", "PS", |x| x * 735.498_75, |x| x / 735.498_75);
priv_define_unit!(BTU_PER_HOUR, Power, "BTU per hour", "BTU/h", |x| x * (1_055.055_852_62 / 3_600.0), |x| x * (3_600.0 / 1_055.055_852_62));

// Flow & Viscosity
priv_define_unit!(CUBIC_METER_PER_SECOND, VolumetricFlowRate, "cubic meter per second", "m³/s", |x| x, |x| x);
priv_define_unit!(LITER_PER_MINUTE, VolumetricFlowRate, "liter per minute", "L/min", |x| x * (1e-3 / 60.0), |x| x * 60_000.0);
priv_define_unit!(GALLON_PER_MINUTE, VolumetricFlowRate, "US gallon per minute", "gpm", |x| x * (0.003_785_411_784 / 60.0), |x| x * (60.0 / 0.003_785_411_784));
priv_define_unit!(KILOGRAM_PER_SECOND, MassFlowRate, "kilogram per second", "kg/s", |x| x, |x| x);
priv_define_unit!(PASCAL_SECOND, DynamicViscosity, "pascal-second", "Pa·s", |x| x, |x| x);
priv_define_unit!(POISE, DynamicViscosity, "poise", "P", |x| x * 0.1, |x| x * 10.0);
priv_define_unit!(CENTIPOISE, DynamicViscosity, "centipoise", "cP", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(SQUARE_METER_PER_SECOND, KinematicViscosity, "square meter per second", "m²/s", |x| x, |x| x);
priv_define_unit!(STOKES, KinematicViscosity, "stokes", "St", |x| x * 1e-4, |x| x * 1e4);

// Electromagnetism
priv_define_unit!(MILLIAMPERE, ElectricCurrent, "milliampere", "mA", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(MICROAMPERE, ElectricCurrent, "microampere", "µA", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(KILOAMPERE, ElectricCurrent, "kiloampere", "kA", |x| x * 1e3, |x| x / 1e3);

priv_define_unit!(COULOMB, ElectricCharge, "coulomb", "C", |x| x, |x| x);
priv_define_unit!(AMPERE_HOUR, ElectricCharge, "ampere-hour", "Ah", |x| x * 3_600.0, |x| x / 3_600.0);
priv_define_unit!(MILLIAMPERE_HOUR, ElectricCharge, "milliampere-hour", "mAh", |x| x * 3.6, |x| x / 3.6);
priv_define_unit!(ELEMENTARY_CHARGE, ElectricCharge, "elementary charge", "e", |x| x * 1.602_176_634e-19, |x| x / 1.602_176_634e-19);

priv_define_unit!(VOLT, Voltage, "volt", "V", |x| x, |x| x);
priv_define_unit!(MILLIVOLT, Voltage, "millivolt", "mV", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(MICROVOLT, Voltage, "microvolt", "µV", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(KILOVOLT, Voltage, "kilovolt", "kV", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(MEGAVOLT, Voltage, "megavolt", "MV", |x| x * 1e6, |x| x / 1e6);

priv_define_unit!(OHM, ElectricResistance, "ohm", "Ω", |x| x, |x| x);
priv_define_unit!(MILLIOHM, ElectricResistance, "milliohm", "mΩ", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(KILOOHM, ElectricResistance, "kiloohm", "kΩ", |x| x * 1e3, |x| x / 1e3);
priv_define_unit!(MEGAOHM, ElectricResistance, "megaohm", "MΩ", |x| x * 1e6, |x| x / 1e6);

priv_define_unit!(SIEMENS, ElectricConductance, "siemens", "S", |x| x, |x| x);
priv_define_unit!(MILLISIEMENS, ElectricConductance, "millisiemens", "mS", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(MICROSIEMENS, ElectricConductance, "microsiemens", "µS", |x| x * 1e-6, |x| x * 1e6);

priv_define_unit!(FARAD, Capacitance, "farad", "F", |x| x, |x| x);
priv_define_unit!(MILLIFARAD, Capacitance, "millifarad", "mF", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(MICROFARAD, Capacitance, "microfarad", "µF", |x| x * 1e-6, |x| x * 1e6);
priv_define_unit!(NANOFARAD, Capacitance, "nanofarad", "nF", |x| x * 1e-9, |x| x * 1e9);
priv_define_unit!(PICOFARAD, Capacitance, "picofarad", "pF", |x| x * 1e-12, |x| x * 1e12);

priv_define_unit!(WEBER, MagneticFlux, "weber", "Wb", |x| x, |x| x);
priv_define_unit!(MAXWELL, MagneticFlux, "maxwell", "Mx", |x| x * 1e-8, |x| x * 1e8);

priv_define_unit!(TESLA, MagneticFluxDensity, "tesla", "T", |x| x, |x| x);
priv_define_unit!(MILLITESLA, MagneticFluxDensity, "millitesla", "mT", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(GAUSS, MagneticFluxDensity, "gauss", "G", |x| x * 1e-4, |x| x * 1e4);

priv_define_unit!(HENRY, Inductance, "henry", "H", |x| x, |x| x);
priv_define_unit!(MILLIHENRY, Inductance, "millihenry", "mH", |x| x * 1e-3, |x| x * 1e3);
priv_define_unit!(MICROHENRY, Inductance, "microhenry", "µH", |x| x * 1e-6, |x| x * 1e6);

// Photometry
priv_define_unit!(LUMEN, LuminousFlux, "lumen", "lm", |x| x, |x| x);
priv_define_unit!(LUX, Illuminance, "lux", "lx", |x| x, |x| x);
priv_define_unit!(FOOT_CANDLE, Illuminance, "foot-candle", "fc", |x| x / 0.092_903_04, |x| x * 0.092_903_04);

// █▀▀ █▀█ █▀█ █▀ █▀  █░█ █▄░█ █ ▀█▀  █▀▄▀█ █░█ █░░  █▀▄ █ █░█ 
// █▄▄ █▀▄ █▄█ ▄█ ▄█  █▄█ █░▀█ █ ░█░  █░▀░█ █▄█ █▄▄  █▄▀ █ ▀▄▀ 

// Geometry & Kinematics
priv_cross_mul_div!(Length, Length::METER, Area, Area::SQUARE_METER);
priv_cross_mul_div!(Length, Length::METER, Area, Area::SQUARE_METER, Volume, Volume::CUBIC_METER);
priv_cross_mul_div!(Velocity, Velocity::METER_PER_SECOND, Time, Time::SECOND, Length, Length::METER);
priv_cross_mul_div!(Acceleration, Acceleration::METER_PER_SECOND_SQUARED, Time, Time::SECOND, Velocity, Velocity::METER_PER_SECOND);
priv_cross_mul_div!(Frequency, Frequency::HERTZ, Time, Time::SECOND, Dimensionless, Dimensionless::ONE);
priv_cross_mul_div!(Length, Length::METER, Frequency, Frequency::HERTZ, Velocity, Velocity::METER_PER_SECOND);
priv_cross_mul_div!(Velocity, Velocity::METER_PER_SECOND, Frequency, Frequency::HERTZ, Acceleration, Acceleration::METER_PER_SECOND_SQUARED);
priv_cross_mul_div!(AngularVelocity, AngularVelocity::RADIAN_PER_SECOND, Time, Time::SECOND, Angle, Angle::RADIAN);
priv_cross_mul_div!(AngularAcceleration, AngularAcceleration::RADIAN_PER_SECOND_SQUARED, Time, Time::SECOND, AngularVelocity, AngularVelocity::RADIAN_PER_SECOND);
priv_cross_mul_div!(Angle, Angle::RADIAN, Frequency, Frequency::HERTZ, AngularVelocity, AngularVelocity::RADIAN_PER_SECOND);
priv_cross_mul_div!(AngularVelocity, AngularVelocity::RADIAN_PER_SECOND, Frequency, Frequency::HERTZ, AngularAcceleration, AngularAcceleration::RADIAN_PER_SECOND_SQUARED);

// Mechanics & Fluid Dynamics
priv_cross_mul_div!(Density, Density::KILOGRAM_PER_CUBIC_METER, Volume, Volume::CUBIC_METER, Mass, Mass::KILOGRAM);
priv_cross_mul_div!(MassFlowRate, MassFlowRate::KILOGRAM_PER_SECOND, Time, Time::SECOND, Mass, Mass::KILOGRAM);
priv_cross_mul_div!(Mass, Mass::KILOGRAM, Frequency, Frequency::HERTZ, MassFlowRate, MassFlowRate::KILOGRAM_PER_SECOND);
priv_cross_mul_div!(VolumetricFlowRate, VolumetricFlowRate::CUBIC_METER_PER_SECOND, Time, Time::SECOND, Volume, Volume::CUBIC_METER);
priv_cross_mul_div!(Volume, Volume::CUBIC_METER, Frequency, Frequency::HERTZ, VolumetricFlowRate, VolumetricFlowRate::CUBIC_METER_PER_SECOND);
priv_cross_mul_div!(Velocity, Velocity::METER_PER_SECOND, Area, Area::SQUARE_METER, VolumetricFlowRate, VolumetricFlowRate::CUBIC_METER_PER_SECOND);
priv_cross_mul_div!(Density, Density::KILOGRAM_PER_CUBIC_METER, VolumetricFlowRate, VolumetricFlowRate::CUBIC_METER_PER_SECOND, MassFlowRate, MassFlowRate::KILOGRAM_PER_SECOND);
priv_cross_mul_div!(Mass, Mass::KILOGRAM, Velocity, Velocity::METER_PER_SECOND, Momentum, Momentum::KILOGRAM_METER_PER_SECOND);
priv_cross_mul_div!(MassFlowRate, MassFlowRate::KILOGRAM_PER_SECOND, Length, Length::METER, Momentum, Momentum::KILOGRAM_METER_PER_SECOND);
priv_cross_mul_div!(Mass, Mass::KILOGRAM, Acceleration, Acceleration::METER_PER_SECOND_SQUARED, Force, Force::NEWTON);
priv_cross_mul_div!(Force, Force::NEWTON, Time, Time::SECOND, Momentum, Momentum::KILOGRAM_METER_PER_SECOND);
priv_cross_mul_div!(Momentum, Momentum::KILOGRAM_METER_PER_SECOND, Frequency, Frequency::HERTZ, Force, Force::NEWTON);
priv_cross_mul_div!(MassFlowRate, MassFlowRate::KILOGRAM_PER_SECOND, Velocity, Velocity::METER_PER_SECOND, Force, Force::NEWTON);
priv_cross_mul_div!(Pressure, Pressure::PASCAL, Area, Area::SQUARE_METER, Force, Force::NEWTON);
priv_cross_mul_div!(Force, Force::NEWTON, Length, Length::METER, Energy, Energy::JOULE);
priv_cross_mul_div!(Pressure, Pressure::PASCAL, Volume, Volume::CUBIC_METER, Energy, Energy::JOULE);
priv_cross_mul_div!(Momentum, Momentum::KILOGRAM_METER_PER_SECOND, Velocity, Velocity::METER_PER_SECOND, Energy, Energy::JOULE);
priv_cross_mul_div!(Torque, Torque::NEWTON_METER, Angle, Angle::RADIAN, Energy, Energy::JOULE);
priv_cross_mul_div!(Power, Power::WATT, Time, Time::SECOND, Energy, Energy::JOULE);
priv_cross_mul_div!(Energy, Energy::JOULE, Frequency, Frequency::HERTZ, Power, Power::WATT);
priv_cross_mul_div!(Force, Force::NEWTON, Velocity, Velocity::METER_PER_SECOND, Power, Power::WATT);
priv_cross_mul_div!(Pressure, Pressure::PASCAL, VolumetricFlowRate, VolumetricFlowRate::CUBIC_METER_PER_SECOND, Power, Power::WATT);
priv_cross_mul_div!(Torque, Torque::NEWTON_METER, AngularVelocity, AngularVelocity::RADIAN_PER_SECOND, Power, Power::WATT);
priv_cross_mul_div!(Pressure, Pressure::PASCAL, Time, Time::SECOND, DynamicViscosity, DynamicViscosity::PASCAL_SECOND);
priv_cross_mul_div!(DynamicViscosity, DynamicViscosity::PASCAL_SECOND, Frequency, Frequency::HERTZ, Pressure, Pressure::PASCAL);
priv_cross_mul_div!(Density, Density::KILOGRAM_PER_CUBIC_METER, KinematicViscosity, KinematicViscosity::SQUARE_METER_PER_SECOND, DynamicViscosity, DynamicViscosity::PASCAL_SECOND);
priv_cross_mul_div!(KinematicViscosity, KinematicViscosity::SQUARE_METER_PER_SECOND, Time, Time::SECOND, Area, Area::SQUARE_METER);
priv_cross_mul_div!(Velocity, Velocity::METER_PER_SECOND, Length, Length::METER, KinematicViscosity, KinematicViscosity::SQUARE_METER_PER_SECOND);

// Electromagnetism
priv_cross_mul_div!(ElectricCurrent, ElectricCurrent::AMPERE, Time, Time::SECOND, ElectricCharge, ElectricCharge::COULOMB);
priv_cross_mul_div!(ElectricCharge, ElectricCharge::COULOMB, Frequency, Frequency::HERTZ, ElectricCurrent, ElectricCurrent::AMPERE);
priv_cross_mul_div!(Voltage, Voltage::VOLT, ElectricCurrent, ElectricCurrent::AMPERE, Power, Power::WATT);
priv_cross_mul_div!(Voltage, Voltage::VOLT, ElectricCharge, ElectricCharge::COULOMB, Energy, Energy::JOULE);
priv_cross_mul_div!(ElectricCurrent, ElectricCurrent::AMPERE, ElectricResistance, ElectricResistance::OHM, Voltage, Voltage::VOLT);
priv_cross_mul_div!(Voltage, Voltage::VOLT, ElectricConductance, ElectricConductance::SIEMENS, ElectricCurrent, ElectricCurrent::AMPERE);
priv_cross_mul_div!(ElectricResistance, ElectricResistance::OHM, ElectricConductance, ElectricConductance::SIEMENS, Dimensionless, Dimensionless::ONE);
priv_cross_mul_div!(Capacitance, Capacitance::FARAD, Voltage, Voltage::VOLT, ElectricCharge, ElectricCharge::COULOMB);
priv_cross_mul_div!(ElectricResistance, ElectricResistance::OHM, Capacitance, Capacitance::FARAD, Time, Time::SECOND);
priv_cross_mul_div!(ElectricConductance, ElectricConductance::SIEMENS, Time, Time::SECOND, Capacitance, Capacitance::FARAD);
priv_cross_mul_div!(Capacitance, Capacitance::FARAD, Frequency, Frequency::HERTZ, ElectricConductance, ElectricConductance::SIEMENS);
priv_cross_mul_div!(Voltage, Voltage::VOLT, Time, Time::SECOND, MagneticFlux, MagneticFlux::WEBER);
priv_cross_mul_div!(MagneticFlux, MagneticFlux::WEBER, Frequency, Frequency::HERTZ, Voltage, Voltage::VOLT);
priv_cross_mul_div!(MagneticFlux, MagneticFlux::WEBER, ElectricCurrent, ElectricCurrent::AMPERE, Energy, Energy::JOULE);
priv_cross_mul_div!(MagneticFluxDensity, MagneticFluxDensity::TESLA, Area, Area::SQUARE_METER, MagneticFlux, MagneticFlux::WEBER);
priv_cross_mul_div!(Inductance, Inductance::HENRY, ElectricCurrent, ElectricCurrent::AMPERE, MagneticFlux, MagneticFlux::WEBER);
priv_cross_mul_div!(ElectricResistance, ElectricResistance::OHM, Time, Time::SECOND, Inductance, Inductance::HENRY);
priv_cross_mul_div!(Inductance, Inductance::HENRY, Frequency, Frequency::HERTZ, ElectricResistance, ElectricResistance::OHM);
priv_cross_mul_div!(Inductance, Inductance::HENRY, ElectricConductance, ElectricConductance::SIEMENS, Time, Time::SECOND);

// Photometry
priv_cross_mul_div!(Illuminance, Illuminance::LUX, Area, Area::SQUARE_METER, LuminousFlux, LuminousFlux::LUMEN);

// Manual impl to omit Velocity / Length as it would colide with Frequency
impl Mul<Length> for AngularVelocity {
    type Output = Velocity;
    fn mul(self, rhs: Length) -> Self::Output {
        let self_base = self.get_unit().get_to_base();
        let rhs_base = rhs.get_unit().get_to_base();
        let base_out = Velocity::METER_PER_SECOND.get_from_base();
        Self::Output::new(base_out(self_base(self.get_value()) * rhs_base(rhs.get_value())), Velocity::METER_PER_SECOND)
    }
}
impl Mul<AngularVelocity> for Length {
    type Output = Velocity;
    fn mul(self, rhs: AngularVelocity) -> Self::Output {
        let self_base = self.get_unit().get_to_base();
        let rhs_base = rhs.get_unit().get_to_base();
        let base_out = Velocity::METER_PER_SECOND.get_from_base();
        Self::Output::new(base_out(self_base(self.get_value()) * rhs_base(rhs.get_value())), Velocity::METER_PER_SECOND)
    }
}
impl Div<AngularVelocity> for Velocity {
    type Output = Length;
    fn div(self, rhs: AngularVelocity) -> Self::Output {
        let self_base = self.get_unit().get_to_base();
        let rhs_base = rhs.get_unit().get_to_base();
        let base_out = Length::METER.get_from_base();
        Self::Output::new(base_out(self_base(self.get_value()) / rhs_base(rhs.get_value())), Length::METER)
    }
}
