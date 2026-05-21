//! # ElementSymbol
//! The elements up to Oganesson are defined.
//!
//! There is a None variant for the case where the element to express is not in the periodic table.
//!
//! A simple API to obtain an `ElementSymbol` from `&str` is provided as `.from_str()`.
//!
//! A simple API to retrieve the atomic weight of the element is provided as `.atomic_weight()`.
//!
//! # Example
//! ```
//! use chemical_formula::prelude::*;
//! use approx::assert_abs_diff_eq;
//!
//! let o = ElementSymbol::from_str("O");
//! assert_eq!(o, ElementSymbol::O);
//!
//! let h = ElementSymbol::H;
//! approx::assert_abs_diff_eq!(h.atomic_weight(), 1.008, epsilon = 1e-6);
//! ```
//!
//! # ChemicalFormula
//!
//! The `ChemicalFormula` struct is used to represent a chemical formula.
//! The `ChemicalFormula` struct has the following fields:
//! * `element` - A `HashSet` of `ElementSymbol` enums.
//! * `stoichiometry` - A `HashMap` of `ElementSymbol` enums and the stoichiometry.
//! * `wt_percent` - A `HashMap` of `ElementSymbol` enums and the wt%.
//!
//! The `ChemicalFormula` struct has the following methods:
//! * `add_element` - Add an element to the formula.
//! * `add_wt_percent` - Add an element to the formula by wt%.
//! * `multiply` - Multiply the stoichiometry and wt% by a multiplier.
//! * `to_molecular_formula` - Convert the formula to molecular formula.
//! * `to_mol_percent` - Convert the formula to mol%.
//! * `molecular_weight` - Calculate the molecular weight of the formula.
//! * `to_wt` - Calculate the molecular weight representation of the formula.
//! * `to_wt_percent` - Convert the formula to wt%.
//! * `multiply_wt_percent` - Multiply the wt% by a multiplier.
//! * `add_formula` - Add another formula to the formula.
//!
//! Please refer to the API reference for more details.
//!
//! The following is a simple example of adding elements to the formula and adding another formula to it.
//!
//! # Example
//! ```
//! use chemical_formula::prelude::*;
//! use approx::assert_abs_diff_eq;
//!
//! let mut formula = ChemicalFormula::new();
//! let mut formula2 = ChemicalFormula::new();
//!
//! formula.add_element(ElementSymbol::O, 1.0);
//! formula.add_wt_percent(ElementSymbol::H, 10.0);
//! formula.add_wt_percent(ElementSymbol::N, 20.0);
//! formula2.add_element(ElementSymbol::O, 1.0);
//! formula2.add_wt_percent(ElementSymbol::H, 10.0);
//! formula2.add_wt_percent(ElementSymbol::N, 20.0);
//! formula.add_formula(&formula2);
//!
//!
//! assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 2.0, epsilon = 1e-6);
//! assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::H], 20.0, epsilon = 1e-6);
//! assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::N], 40.0, epsilon = 1e-6);
//! ```

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

const ZERO_TOL: f64 = 1e-12;

/// Error type used in chemical-formula-rs
///
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormulaError {
    FileIOError,
    FileParseError,
    ParseError {
        input: String,
        position: Option<usize>,
        reason: String,
    },
    InvalidElementSymbol(String),
    InvalidNumber(String),
    WeightPercentOverflow,
    DivisionByZero,
    NoFormula,
}

impl std::error::Error for FormulaError {}

impl fmt::Display for FormulaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormulaError::FileIOError => write!(f, "file I/O error"),
            FormulaError::FileParseError => write!(f, "file parse error"),
            FormulaError::ParseError {
                position, reason, ..
            } => {
                if let Some(position) = position {
                    write!(f, "parse error at byte {}: {}", position, reason)
                } else {
                    write!(f, "parse error: {}", reason)
                }
            }
            FormulaError::InvalidElementSymbol(symbol) => {
                write!(f, "invalid element symbol: {}", symbol)
            }
            FormulaError::InvalidNumber(raw) => write!(f, "invalid number: {}", raw),
            FormulaError::WeightPercentOverflow => write!(f, "weight percent overflow"),
            FormulaError::DivisionByZero => write!(f, "division by zero"),
            FormulaError::NoFormula => write!(f, "no formula"),
        }
    }
}

/// Hashmap of atomic weight
///
/// The key is the `ElementSymbol` enums and the value is the atomic weight.
///
///
/// # Example
/// ```
/// use chemical_formula::element::ATOMIC_WEIGHT;
/// use chemical_formula::prelude::*;
/// use approx::assert_abs_diff_eq;
///
///
/// assert_abs_diff_eq!(ATOMIC_WEIGHT[&ElementSymbol::H], 1.008, epsilon = 1e-6)
/// ```
///
/// # Reference
/// [ATOMIC WEIGHTS OF THE ELEMENTS 2021](https://iupac.qmul.ac.uk/AtWt/)
pub static ATOMIC_WEIGHT: Lazy<HashMap<ElementSymbol, f64>> = Lazy::new(|| {
    HashMap::from([
        (ElementSymbol::None, 0.0),
        (ElementSymbol::H, 1.008),
        (ElementSymbol::He, 4.002602),
        (ElementSymbol::Li, 6.94),
        (ElementSymbol::Be, 9.0121831),
        (ElementSymbol::B, 10.81),
        (ElementSymbol::C, 12.011),
        (ElementSymbol::N, 14.007),
        (ElementSymbol::O, 15.999),
        (ElementSymbol::F, 18.998403163),
        (ElementSymbol::Ne, 20.1797),
        (ElementSymbol::Na, 22.98976928),
        (ElementSymbol::Mg, 24.305),
        (ElementSymbol::Al, 26.9815384),
        (ElementSymbol::Si, 28.085),
        (ElementSymbol::P, 30.973761998),
        (ElementSymbol::S, 32.06),
        (ElementSymbol::Cl, 35.45),
        (ElementSymbol::Ar, 39.95),
        (ElementSymbol::K, 39.0983),
        (ElementSymbol::Ca, 40.078),
        (ElementSymbol::Sc, 44.955907),
        (ElementSymbol::Ti, 47.867),
        (ElementSymbol::V, 50.9415),
        (ElementSymbol::Cr, 51.9961),
        (ElementSymbol::Mn, 54.938043),
        (ElementSymbol::Fe, 55.845),
        (ElementSymbol::Co, 58.933194),
        (ElementSymbol::Ni, 58.6934),
        (ElementSymbol::Cu, 63.546),
        (ElementSymbol::Zn, 65.38),
        (ElementSymbol::Ga, 69.723),
        (ElementSymbol::Ge, 72.630),
        (ElementSymbol::As, 74.921595),
        (ElementSymbol::Se, 78.971),
        (ElementSymbol::Br, 79.904),
        (ElementSymbol::Kr, 83.798),
        (ElementSymbol::Rb, 85.4678),
        (ElementSymbol::Sr, 87.62),
        (ElementSymbol::Y, 88.905838),
        (ElementSymbol::Zr, 91.224),
        (ElementSymbol::Nb, 92.90637),
        (ElementSymbol::Mo, 95.95),
        (ElementSymbol::Tc, 97.0),
        (ElementSymbol::Ru, 101.07),
        (ElementSymbol::Rh, 102.90549),
        (ElementSymbol::Pd, 106.42),
        (ElementSymbol::Ag, 107.8682),
        (ElementSymbol::Cd, 112.414),
        (ElementSymbol::In, 114.818),
        (ElementSymbol::Sn, 118.710),
        (ElementSymbol::Sb, 121.760),
        (ElementSymbol::Te, 127.60),
        (ElementSymbol::I, 126.90447),
        (ElementSymbol::Xe, 131.293),
        (ElementSymbol::Cs, 132.90545196),
        (ElementSymbol::Ba, 137.327),
        (ElementSymbol::La, 138.90547),
        (ElementSymbol::Ce, 140.116),
        (ElementSymbol::Pr, 140.90766),
        (ElementSymbol::Nd, 144.242),
        (ElementSymbol::Pm, 145.0),
        (ElementSymbol::Sm, 150.36),
        (ElementSymbol::Eu, 151.964),
        (ElementSymbol::Gd, 157.25),
        (ElementSymbol::Tb, 158.925354),
        (ElementSymbol::Dy, 162.500),
        (ElementSymbol::Ho, 164.930329),
        (ElementSymbol::Er, 167.259),
        (ElementSymbol::Tm, 168.934219),
        (ElementSymbol::Yb, 173.045),
        (ElementSymbol::Lu, 174.9668),
        (ElementSymbol::Hf, 178.486),
        (ElementSymbol::Ta, 180.94788),
        (ElementSymbol::W, 183.84),
        (ElementSymbol::Re, 186.207),
        (ElementSymbol::Os, 190.23),
        (ElementSymbol::Ir, 192.217),
        (ElementSymbol::Pt, 195.084),
        (ElementSymbol::Au, 196.966570),
        (ElementSymbol::Hg, 200.592),
        (ElementSymbol::Tl, 204.38),
        (ElementSymbol::Pb, 207.2),
        (ElementSymbol::Bi, 208.98040),
        (ElementSymbol::Po, 209.0),
        (ElementSymbol::At, 210.0),
        (ElementSymbol::Rn, 222.0),
        (ElementSymbol::Fr, 223.0),
        (ElementSymbol::Ra, 226.0),
        (ElementSymbol::Ac, 227.0),
        (ElementSymbol::Th, 232.0377),
        (ElementSymbol::Pa, 231.03588),
        (ElementSymbol::U, 238.02891),
        (ElementSymbol::Np, 237.0),
        (ElementSymbol::Pu, 244.0),
        (ElementSymbol::Am, 243.0),
        (ElementSymbol::Cm, 247.0),
        (ElementSymbol::Bk, 247.0),
        (ElementSymbol::Cf, 251.0),
        (ElementSymbol::Es, 252.0),
        (ElementSymbol::Fm, 257.0),
        (ElementSymbol::Md, 258.0),
        (ElementSymbol::No, 259.0),
        (ElementSymbol::Lr, 262.0),
        (ElementSymbol::Rf, 267.0),
        (ElementSymbol::Db, 270.0),
        (ElementSymbol::Sg, 269.0),
        (ElementSymbol::Bh, 270.0),
        (ElementSymbol::Hs, 270.0),
        (ElementSymbol::Mt, 278.0),
        (ElementSymbol::Ds, 281.0),
        (ElementSymbol::Rg, 281.0),
        (ElementSymbol::Cn, 285.0),
        (ElementSymbol::Nh, 286.0),
        (ElementSymbol::Fl, 289.0),
        (ElementSymbol::Mc, 289.0),
        (ElementSymbol::Lv, 293.0),
        (ElementSymbol::Ts, 293.0),
        (ElementSymbol::Og, 294.0),
    ])
});

/// Enum of element symbol
///
/// The elements up to Oganesson are defined.
///
/// There is a None variant for the case where the element to express is not in the periodic table.
///
/// A simple API to obtain an `ElementSymbol` from `&str` is provided as `.from_str()`.
///
/// A simple API to retrieve the atomic weight of the element is provided as `.atomic_weight()`.
///
/// # Example
/// ```
/// use chemical_formula::prelude::*;
/// use approx::assert_abs_diff_eq;
///
/// let o = ElementSymbol::from_str("O");
/// assert_eq!(o, ElementSymbol::O);
///
/// let h = ElementSymbol::H;
/// approx::assert_abs_diff_eq!(h.atomic_weight(), 1.008, epsilon = 1e-6);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementSymbol {
    #[default]
    None = 0,
    H = 1,
    He = 2,
    Li = 3,
    Be = 4,
    B = 5,
    C = 6,
    N = 7,
    O = 8,
    F = 9,
    Ne = 10,
    Na = 11,
    Mg = 12,
    Al = 13,
    Si = 14,
    P = 15,
    S = 16,
    Cl = 17,
    Ar = 18,
    K = 19,
    Ca = 20,
    Sc = 21,
    Ti = 22,
    V = 23,
    Cr = 24,
    Mn = 25,
    Fe = 26,
    Co = 27,
    Ni = 28,
    Cu = 29,
    Zn = 30,
    Ga = 31,
    Ge = 32,
    As = 33,
    Se = 34,
    Br = 35,
    Kr = 36,
    Rb = 37,
    Sr = 38,
    Y = 39,
    Zr = 40,
    Nb = 41,
    Mo = 42,
    Tc = 43,
    Ru = 44,
    Rh = 45,
    Pd = 46,
    Ag = 47,
    Cd = 48,
    In = 49,
    Sn = 50,
    Sb = 51,
    Te = 52,
    I = 53,
    Xe = 54,
    Cs = 55,
    Ba = 56,
    La = 57,
    Ce = 58,
    Pr = 59,
    Nd = 60,
    Pm = 61,
    Sm = 62,
    Eu = 63,
    Gd = 64,
    Tb = 65,
    Dy = 66,
    Ho = 67,
    Er = 68,
    Tm = 69,
    Yb = 70,
    Lu = 71,
    Hf = 72,
    Ta = 73,
    W = 74,
    Re = 75,
    Os = 76,
    Ir = 77,
    Pt = 78,
    Au = 79,
    Hg = 80,
    Tl = 81,
    Pb = 82,
    Bi = 83,
    Po = 84,
    At = 85,
    Rn = 86,
    Fr = 87,
    Ra = 88,
    Ac = 89,
    Th = 90,
    Pa = 91,
    U = 92,
    Np = 93,
    Pu = 94,
    Am = 95,
    Cm = 96,
    Bk = 97,
    Cf = 98,
    Es = 99,
    Fm = 100,
    Md = 101,
    No = 102,
    Lr = 103,
    Rf = 104,
    Db = 105,
    Sg = 106,
    Bh = 107,
    Hs = 108,
    Mt = 109,
    Ds = 110,
    Rg = 111,
    Cn = 112,
    Nh = 113,
    Fl = 114,
    Mc = 115,
    Lv = 116,
    Ts = 117,
    Og = 118,
}

impl ElementSymbol {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> ElementSymbol {
        match s {
            "H" => ElementSymbol::H,
            "He" => ElementSymbol::He,
            "Li" => ElementSymbol::Li,
            "Be" => ElementSymbol::Be,
            "B" => ElementSymbol::B,
            "C" => ElementSymbol::C,
            "N" => ElementSymbol::N,
            "O" => ElementSymbol::O,
            "F" => ElementSymbol::F,
            "Ne" => ElementSymbol::Ne,
            "Na" => ElementSymbol::Na,
            "Mg" => ElementSymbol::Mg,
            "Al" => ElementSymbol::Al,
            "Si" => ElementSymbol::Si,
            "P" => ElementSymbol::P,
            "S" => ElementSymbol::S,
            "Cl" => ElementSymbol::Cl,
            "Ar" => ElementSymbol::Ar,
            "K" => ElementSymbol::K,
            "Ca" => ElementSymbol::Ca,
            "Sc" => ElementSymbol::Sc,
            "Ti" => ElementSymbol::Ti,
            "V" => ElementSymbol::V,
            "Cr" => ElementSymbol::Cr,
            "Mn" => ElementSymbol::Mn,
            "Fe" => ElementSymbol::Fe,
            "Co" => ElementSymbol::Co,
            "Ni" => ElementSymbol::Ni,
            "Cu" => ElementSymbol::Cu,
            "Zn" => ElementSymbol::Zn,
            "Ga" => ElementSymbol::Ga,
            "Ge" => ElementSymbol::Ge,
            "As" => ElementSymbol::As,
            "Se" => ElementSymbol::Se,
            "Br" => ElementSymbol::Br,
            "Kr" => ElementSymbol::Kr,
            "Rb" => ElementSymbol::Rb,
            "Sr" => ElementSymbol::Sr,
            "Y" => ElementSymbol::Y,
            "Zr" => ElementSymbol::Zr,
            "Nb" => ElementSymbol::Nb,
            "Mo" => ElementSymbol::Mo,
            "Tc" => ElementSymbol::Tc,
            "Ru" => ElementSymbol::Ru,
            "Rh" => ElementSymbol::Rh,
            "Pd" => ElementSymbol::Pd,
            "Ag" => ElementSymbol::Ag,
            "Cd" => ElementSymbol::Cd,
            "In" => ElementSymbol::In,
            "Sn" => ElementSymbol::Sn,
            "Sb" => ElementSymbol::Sb,
            "Te" => ElementSymbol::Te,
            "I" => ElementSymbol::I,
            "Xe" => ElementSymbol::Xe,
            "Cs" => ElementSymbol::Cs,
            "Ba" => ElementSymbol::Ba,
            "La" => ElementSymbol::La,
            "Ce" => ElementSymbol::Ce,
            "Pr" => ElementSymbol::Pr,
            "Nd" => ElementSymbol::Nd,
            "Pm" => ElementSymbol::Pm,
            "Sm" => ElementSymbol::Sm,
            "Eu" => ElementSymbol::Eu,
            "Gd" => ElementSymbol::Gd,
            "Tb" => ElementSymbol::Tb,
            "Dy" => ElementSymbol::Dy,
            "Ho" => ElementSymbol::Ho,
            "Er" => ElementSymbol::Er,
            "Tm" => ElementSymbol::Tm,
            "Yb" => ElementSymbol::Yb,
            "Lu" => ElementSymbol::Lu,
            "Hf" => ElementSymbol::Hf,
            "Ta" => ElementSymbol::Ta,
            "W" => ElementSymbol::W,
            "Re" => ElementSymbol::Re,
            "Os" => ElementSymbol::Os,
            "Ir" => ElementSymbol::Ir,
            "Pt" => ElementSymbol::Pt,
            "Au" => ElementSymbol::Au,
            "Hg" => ElementSymbol::Hg,
            "Tl" => ElementSymbol::Tl,
            "Pb" => ElementSymbol::Pb,
            "Bi" => ElementSymbol::Bi,
            "Po" => ElementSymbol::Po,
            "At" => ElementSymbol::At,
            "Rn" => ElementSymbol::Rn,
            "Fr" => ElementSymbol::Fr,
            "Ra" => ElementSymbol::Ra,
            "Ac" => ElementSymbol::Ac,
            "Th" => ElementSymbol::Th,
            "Pa" => ElementSymbol::Pa,
            "U" => ElementSymbol::U,
            "Np" => ElementSymbol::Np,
            "Pu" => ElementSymbol::Pu,
            "Am" => ElementSymbol::Am,
            "Cm" => ElementSymbol::Cm,
            "Bk" => ElementSymbol::Bk,
            "Cf" => ElementSymbol::Cf,
            "Es" => ElementSymbol::Es,
            "Fm" => ElementSymbol::Fm,
            "Md" => ElementSymbol::Md,
            "No" => ElementSymbol::No,
            "Lr" => ElementSymbol::Lr,
            "Rf" => ElementSymbol::Rf,
            "Db" => ElementSymbol::Db,
            "Sg" => ElementSymbol::Sg,
            "Bh" => ElementSymbol::Bh,
            "Hs" => ElementSymbol::Hs,
            "Mt" => ElementSymbol::Mt,
            "Ds" => ElementSymbol::Ds,
            "Rg" => ElementSymbol::Rg,
            "Cn" => ElementSymbol::Cn,
            "Nh" => ElementSymbol::Nh,
            "Fl" => ElementSymbol::Fl,
            "Mc" => ElementSymbol::Mc,
            "Lv" => ElementSymbol::Lv,
            "Ts" => ElementSymbol::Ts,
            "Og" => ElementSymbol::Og,
            _ => ElementSymbol::None,
        }
    }

    pub fn atomic_weight(&self) -> f64 {
        ATOMIC_WEIGHT[self]
    }
}

impl FromStr for ElementSymbol {
    type Err = FormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let symbol = ElementSymbol::from_str(s);
        if symbol == ElementSymbol::None {
            return Err(FormulaError::InvalidElementSymbol(s.to_owned()));
        }

        Ok(symbol)
    }
}

impl fmt::Display for ElementSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == ElementSymbol::None {
            return Ok(());
        }

        write!(f, "{:?}", self)
    }
}

/// Struct to represent the chemical formula along wt%  
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct ChemicalFormula {
    pub element: HashSet<ElementSymbol>,
    pub stoichiometry: HashMap<ElementSymbol, f64>,
    pub wt_percent: HashMap<ElementSymbol, f64>,
}

impl ChemicalFormula {
    /// Create a empty ChemicalFormula
    ///
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// let formula = ChemicalFormula::new();
    ///
    /// assert!(formula.element.is_empty());
    /// assert!(formula.stoichiometry.is_empty());
    /// assert!(formula.wt_percent.is_empty());
    /// ```
    pub fn new() -> Self {
        ChemicalFormula {
            element: HashSet::new(),
            stoichiometry: HashMap::new(),
            wt_percent: HashMap::new(),
        }
    }

    /// Add an element to the formula
    ///
    /// # Arguments
    /// * `element` - A `ElementSymbol` enum
    /// * `stoichiometry` - A f64 value that represents the stoichiometry
    ///
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    ///
    /// let mut formula = ChemicalFormula::new();
    /// formula.add_element(ElementSymbol::O, 1.0);
    ///
    /// assert_eq!(formula.element.len(), 1);
    /// assert_eq!(formula.stoichiometry.len(), 1);
    ///
    /// assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 1.0);
    /// ```
    pub fn add_element(&mut self, element: ElementSymbol, stoichiometry: f64) -> &mut Self {
        self.element.insert(element);

        self.stoichiometry
            .entry(element)
            .and_modify(|e| *e += stoichiometry)
            .or_insert(stoichiometry);

        self
    }

    /// Add an element to the formula by wt%
    ///
    /// # Arguments
    /// * `element` - A `ElementSymbol` enum
    /// * `wt_ratio` - A f64 value that represents the wt% ratio (0.0 - 100.0)
    ///
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    ///
    /// let mut formula = ChemicalFormula::new();
    /// formula.add_element(ElementSymbol::O, 1.0);
    /// formula.add_wt_percent(ElementSymbol::O, 10.0);
    /// formula.add_wt_percent(ElementSymbol::H, 20.0);
    ///
    ///
    /// assert_eq!(formula.element.len(), 2);
    /// assert_eq!(formula.wt_percent.len(), 2);
    /// assert_eq!(formula.stoichiometry.len(), 1);
    ///
    /// assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::O], 10.0);
    /// assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::H], 20.0);
    /// ```
    ///
    pub fn add_wt_percent(&mut self, element: ElementSymbol, wt_ratio: f64) -> &mut Self {
        self.element.insert(element);

        self.wt_percent
            .entry(element)
            .and_modify(|e| *e += wt_ratio)
            .or_insert(wt_ratio);
        self
    }

    /// Multiply the stoichiometry and wt% by a multiplier
    ///
    /// This method will simply multiply the stoichiometry and wt% by a multiplier.
    /// The method will not check if the wt% sum is over 100.
    ///
    /// For example, if the formula has 10 wt% of H and the multiplier is 2, the resulting wt% of H
    /// will be 20wt%. If the formula has stoichiometry of 1.0 of O and the multiplier is 2, the resulting
    /// stoichiometry of O will be 2.0.
    ///
    /// # Arguments
    /// * `multiplier` - A f64 value that represents the multiplier
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    ///
    /// let mut formula = ChemicalFormula::new();
    /// formula.add_element(ElementSymbol::O, 1.0);
    /// formula.add_wt_percent(ElementSymbol::H, 10.0);
    /// formula.multiply(2.0);
    ///
    /// assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::H], 20.0, epsilon = 1e-6);
    /// assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 2.0, epsilon = 1e-6);
    /// ```
    pub fn multiply(&mut self, multiplier: f64) -> &mut Self {
        for (_element, stoichiometry) in self.stoichiometry.iter_mut() {
            *stoichiometry *= multiplier;
        }

        for (_element, wt_ratio) in self.wt_percent.iter_mut() {
            *wt_ratio *= multiplier;
        }
        self
    }

    /// Convert the formula to molecular formula
    ///
    /// This method will convert the wt% representation to molecular formula representation and
    /// adds the stoichiometry.
    ///
    /// For example, Pt5%SiO2 will be converted to Pt0.016209751480873558SiO2.
    ///
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    ///
    /// let mut formula = ChemicalFormula::new();
    ///
    /// formula.add_element(ElementSymbol::Si, 1.0);
    /// formula.add_element(ElementSymbol::O, 2.0);
    /// formula.add_wt_percent(ElementSymbol::Pt, 5.0);
    ///
    /// let molecular_formula = formula.to_molecular_formula().unwrap();
    /// assert_abs_diff_eq!(molecular_formula.stoichiometry[&ElementSymbol::Pt], 0.016209751480873558, epsilon=1e-6);
    /// assert_abs_diff_eq!(molecular_formula.stoichiometry[&ElementSymbol::Si], 1.0);
    /// assert_abs_diff_eq!(molecular_formula.stoichiometry[&ElementSymbol::O], 2.0);
    /// ```
    ///
    /// # Note
    /// When ChemicalFormula has no stoichiometry, the stoichiometry is a relative value.
    /// This is because the wt% is a relative term and the absolute value of the stoichiometry cannot be determined.
    pub fn to_molecular_formula(&self) -> Result<ChemicalFormula, FormulaError> {
        if self.wt_percent.is_empty() {
            return Ok(self.clone());
        }

        let mut wt_ratio_sum = 0.0;
        let mut wt_ratio_molecular_weight_sum = 0.0;

        for (element, wt_ratio) in &self.wt_percent {
            wt_ratio_sum += wt_ratio;
            wt_ratio_molecular_weight_sum += wt_ratio / element.atomic_weight();
        }

        if wt_ratio_sum > 100. {
            return Err(FormulaError::WeightPercentOverflow);
        }

        if wt_ratio_molecular_weight_sum.abs() <= ZERO_TOL {
            return Err(FormulaError::DivisionByZero);
        }

        if self.stoichiometry.is_empty() {
            let mut stoichiometry = HashMap::new();
            for (element, wt_ratio) in &self.wt_percent {
                stoichiometry.insert(*element, wt_ratio / element.atomic_weight());
            }

            return Ok(ChemicalFormula {
                element: self.element.clone(),
                stoichiometry,
                wt_percent: HashMap::new(),
            });
        }

        let residue = 100.0 - wt_ratio_sum;
        if residue.abs() <= ZERO_TOL {
            return Err(FormulaError::DivisionByZero);
        }

        let molecular_weight_residue = self
            .stoichiometry
            .iter()
            .map(|(element, stoichiometry)| ATOMIC_WEIGHT[element] * stoichiometry)
            .sum::<f64>();

        if molecular_weight_residue.abs() <= ZERO_TOL {
            return Err(FormulaError::DivisionByZero);
        }

        let molecular_weight_main =
            molecular_weight_residue * wt_ratio_molecular_weight_sum / residue;

        let mut stoichiometry = self.stoichiometry.clone();

        for (element, wt_ratio) in &self.wt_percent {
            stoichiometry
                .entry(*element)
                .and_modify(|e| {
                    *e *= molecular_weight_main * wt_ratio
                        / element.atomic_weight()
                        / wt_ratio_molecular_weight_sum
                })
                .or_insert(
                    molecular_weight_main * wt_ratio
                        / element.atomic_weight()
                        / wt_ratio_molecular_weight_sum,
                );
        }

        Ok(ChemicalFormula {
            element: self.element.clone(),
            stoichiometry,
            wt_percent: HashMap::new(),
        })
    }

    /// Convert the formula to mol%
    ///
    /// This method will convert the wt% representation to mol% representation.
    /// The method will first convert the wt% to molecular formula representation by calling `to_molecular_formula()`
    /// and then normalize to mol%.
    ///
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    /// let mut formula = ChemicalFormula::new();
    /// formula.add_element(ElementSymbol::O, 1.0);
    /// formula.add_element(ElementSymbol::H, 2.0);
    ///
    /// let expected_O = 1./3. * 100.;
    /// let expected_H = 2./3. * 100.;
    ///
    /// let mol_percent = formula.to_mol_percent().unwrap();
    ///
    /// assert_abs_diff_eq!(mol_percent.stoichiometry[&ElementSymbol::O], expected_O, epsilon = 1e-6);
    /// assert_abs_diff_eq!(mol_percent.stoichiometry[&ElementSymbol::H], expected_H, epsilon = 1e-6);
    /// ```
    pub fn to_mol_percent(&self) -> Result<ChemicalFormula, FormulaError> {
        let mut formula = self.to_molecular_formula()?;

        if formula.stoichiometry.is_empty() {
            return Ok(ChemicalFormula::new());
        }

        let molecular_formula_sum = formula.stoichiometry.values().sum::<f64>();
        if molecular_formula_sum.abs() <= ZERO_TOL {
            return Err(FormulaError::DivisionByZero);
        }

        formula.multiply(100. / molecular_formula_sum);

        Ok(formula)
    }

    /// Calculate the molecular weight of the formula
    ///
    /// This method will calculate the molecular weight of the formula.
    ///
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    /// let mut formula = ChemicalFormula::new();
    ///
    /// formula.add_element(ElementSymbol::O, 1.0);
    /// formula.add_element(ElementSymbol::H, 2.0);
    ///
    /// let molecular_weight = formula.molecular_weight().unwrap();
    /// assert_abs_diff_eq!(molecular_weight, 18.015, epsilon = 1e-6);
    /// ```
    ///
    pub fn molecular_weight(&self) -> Result<f64, FormulaError> {
        let stoichiometry = if self.wt_percent.is_empty() {
            self.stoichiometry.clone()
        } else {
            self.to_molecular_formula()?.stoichiometry
        };

        Ok(stoichiometry
            .iter()
            .map(|(element, stoichiometry)| ATOMIC_WEIGHT[element] * stoichiometry)
            .sum())
    }

    /// Calculate the molecular weight representation of the formula
    ///
    /// This method will calculate the weight ratio of the formula.
    /// This method will not normalize to 100wt%.
    /// Please use `to_wt()` or `to_wt_percent()` to normalize to 100wt%.
    ///
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    /// let mut formula = ChemicalFormula::new();
    ///
    /// formula.add_wt_percent(ElementSymbol::Na, 10.0);
    ///
    /// let wt_percent = formula.to_wt().unwrap();
    /// assert_abs_diff_eq!(wt_percent.wt_percent[&ElementSymbol::Na], 10.0, epsilon = 1e-6);
    /// ```
    pub fn to_wt(&self) -> Result<ChemicalFormula, FormulaError> {
        if self.stoichiometry.is_empty() {
            return Ok(self.clone());
        }

        let formula = self.to_molecular_formula()?;
        let molecular_weight = formula.molecular_weight()?;
        if molecular_weight.abs() <= ZERO_TOL {
            return Err(FormulaError::DivisionByZero);
        }

        let mut wt_ratio = HashMap::new();

        for (element, stoichiometry) in &formula.stoichiometry {
            wt_ratio.insert(
                *element,
                stoichiometry * ATOMIC_WEIGHT[element] * 100. / molecular_weight,
            );
        }

        Ok(ChemicalFormula {
            element: formula.element,
            stoichiometry: HashMap::new(),
            wt_percent: wt_ratio,
        })
    }

    /// Convert the formula to wt%
    ///
    /// This method will convert the stoichiometry representation to wt% representation.
    ///
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    /// let mut formula = ChemicalFormula::new();
    ///
    /// formula.add_element(ElementSymbol::O, 1.0);
    /// formula.add_element(ElementSymbol::H, 2.0);
    /// formula.add_wt_percent(ElementSymbol::Na, 10.0);
    ///
    /// let wt_percent = formula.to_wt().unwrap();
    /// assert_abs_diff_eq!(wt_percent.wt_percent[&ElementSymbol::Na], 10.0, epsilon = 1e-6);
    /// assert_abs_diff_eq!(wt_percent.wt_percent[&ElementSymbol::H] +
    /// wt_percent.wt_percent[&ElementSymbol::O], 90.0, epsilon = 1e-6);;
    ///
    ///
    /// let mut formula = ChemicalFormula::new();
    /// formula.add_wt_percent(ElementSymbol::H, 10.0);
    /// formula.add_wt_percent(ElementSymbol::N, 20.0);
    ///
    /// let expected_H = 1.0/3.0 * 100.0;
    /// let expected_N = 2.0/3.0 * 100.0;
    ///
    /// let wt_percent = formula.to_wt_percent().unwrap();
    /// assert_abs_diff_eq!(wt_percent.wt_percent[&ElementSymbol::H], expected_H, epsilon = 1e-6);
    /// assert_abs_diff_eq!(wt_percent.wt_percent[&ElementSymbol::N], expected_N, epsilon = 1e-6);
    /// ```
    pub fn to_wt_percent(&self) -> Result<ChemicalFormula, FormulaError> {
        let mut formula = self.to_wt()?;

        let wt_total = formula.wt_percent.values().sum::<f64>();
        if wt_total.abs() <= ZERO_TOL {
            return Err(FormulaError::DivisionByZero);
        }

        formula.multiply(100. / wt_total);

        Ok(formula)
    }

    /// multiplier by wt%
    ///
    /// # Arguments
    /// * `multiplier` - A f64 value that represents the wt% multiplier
    ///
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    ///
    /// let mut formula = ChemicalFormula::new();
    /// formula.add_element(ElementSymbol::O, 1.0);
    /// formula.add_wt_percent(ElementSymbol::H, 10.0);
    /// formula.add_wt_percent(ElementSymbol::N, 20.0);
    /// formula.multiply_wt_percent(2.0);
    ///
    /// formula.to_wt_percent().unwrap();
    ///
    /// let expected_H = 10.0 * 2.0/100.0;
    ///
    /// assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::H], expected_H , epsilon = 1e-6);
    /// ```
    pub fn multiply_wt_percent(&mut self, multiplier: f64) -> Result<&mut Self, FormulaError> {
        let formula = self.to_molecular_formula()?;
        let molecular_weight = formula.molecular_weight()?;
        if molecular_weight.abs() <= ZERO_TOL {
            return Err(FormulaError::DivisionByZero);
        }

        let mut wt_ratio = HashMap::new();

        for (element, stoichiometry) in &formula.stoichiometry {
            wt_ratio.insert(
                *element,
                stoichiometry * ATOMIC_WEIGHT[element] / molecular_weight * multiplier,
            );
        }

        *self = ChemicalFormula {
            element: formula.element,
            stoichiometry: HashMap::new(),
            wt_percent: wt_ratio,
        };

        Ok(self)
    }

    /// Add another formula to the current formula
    /// # Arguments
    /// * `formula` - A `ChemicalFormula` to be added
    /// # Example
    /// ```
    /// use chemical_formula::prelude::*;
    /// use approx::assert_abs_diff_eq;
    ///
    /// let mut formula = ChemicalFormula::new();
    /// let mut formula2 = ChemicalFormula::new();
    ///
    /// formula.add_element(ElementSymbol::O, 1.0);
    /// formula.add_wt_percent(ElementSymbol::H, 10.0);
    /// formula.add_wt_percent(ElementSymbol::N, 20.0);
    /// formula2.add_element(ElementSymbol::O, 1.0);
    /// formula2.add_wt_percent(ElementSymbol::H, 10.0);
    /// formula2.add_wt_percent(ElementSymbol::N, 20.0);
    /// formula.add_formula(&formula2);
    ///
    ///
    /// assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 2.0, epsilon = 1e-6);
    /// assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::H], 20.0, epsilon = 1e-6);
    /// assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::N], 40.0, epsilon = 1e-6);
    /// ```
    pub fn add_formula(&mut self, formula: &ChemicalFormula) -> &mut Self {
        for (element, stoichiometry) in &formula.stoichiometry {
            self.element.insert(*element);
            self.stoichiometry
                .entry(*element)
                .and_modify(|e| *e += stoichiometry)
                .or_insert(*stoichiometry);
        }

        for (element, wt_ratio) in &formula.wt_percent {
            self.element.insert(*element);

            self.wt_percent
                .entry(*element)
                .and_modify(|e| *e += wt_ratio)
                .or_insert(*wt_ratio);
        }

        self
    }
}

fn format_number(value: f64) -> String {
    let normalized = if value.abs() <= ZERO_TOL { 0.0 } else { value };
    if (normalized - normalized.round()).abs() <= ZERO_TOL {
        return format!("{:.0}", normalized.round());
    }

    let formatted = format!("{:.12}", normalized);
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

fn sorted_entries(map: &HashMap<ElementSymbol, f64>) -> Vec<(ElementSymbol, f64)> {
    let mut entries: Vec<(ElementSymbol, f64)> = map.iter().map(|(k, v)| (*k, *v)).collect();
    entries.sort_by_key(|(symbol, _)| *symbol as u16);
    entries
}

impl fmt::Display for ChemicalFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for (element, wt_ratio) in sorted_entries(&self.wt_percent) {
            if element == ElementSymbol::None {
                continue;
            }
            output.push_str(&element.to_string());
            output.push_str(&format_number(wt_ratio));
            output.push_str("wt%");
        }

        for (element, stoichiometry) in sorted_entries(&self.stoichiometry) {
            if element == ElementSymbol::None {
                continue;
            }
            output.push_str(&element.to_string());
            if (stoichiometry - 1.0).abs() > ZERO_TOL {
                output.push_str(&format_number(stoichiometry));
            }
        }

        write!(f, "{}", output)
    }
}

impl FromStr for ChemicalFormula {
    type Err = FormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::parse_formula(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use std::str::FromStr;

    #[test]
    fn test_formula() {
        let mut formula = ChemicalFormula::new();

        formula.add_element(ElementSymbol::O, 1.0);
        formula.add_wt_percent(ElementSymbol::H, 10.0);
        formula.add_wt_percent(ElementSymbol::N, 20.0);

        let expected_o = 1.0;
        let expected_h = 10.0;
        let expected_n = 20.0;

        let molecular_formula = formula.to_molecular_formula().unwrap();
        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_eq!(
            molecular_formula.stoichiometry[&ElementSymbol::O],
            expected_o
        );

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_h,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_n,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_formula_no_element() {
        let mut formula = ChemicalFormula::new();

        formula.add_wt_percent(ElementSymbol::H, 10.0);
        formula.add_wt_percent(ElementSymbol::N, 20.0);

        let expected_h = 10.0;
        let expected_n = 20.0;
        let expected_h_after_wt_percent = 1.0 / 3.0 * 100.0;
        let expected_n_after_wt_percent = 2.0 / 3.0 * 100.0;

        let wt_ratio = formula.to_wt().unwrap();

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_h,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_n,
            epsilon = 1e-6
        );

        let wt_percent = formula.to_wt_percent().unwrap();

        assert_abs_diff_eq!(
            wt_percent.wt_percent[&ElementSymbol::H],
            expected_h_after_wt_percent,
            epsilon = 1e-6
        );

        assert_abs_diff_eq!(
            wt_percent.wt_percent[&ElementSymbol::N],
            expected_n_after_wt_percent,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_no_wt_percent() {
        let mut formula = ChemicalFormula::new();

        formula.add_element(ElementSymbol::O, 1.0);

        let expected_o = 1.0;

        let molecular_formula = formula.to_molecular_formula().unwrap();
        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_eq!(
            molecular_formula.stoichiometry[&ElementSymbol::O],
            expected_o
        );

        assert_abs_diff_eq!(wt_ratio.wt_percent[&ElementSymbol::O], 100., epsilon = 1e-6);
    }

    #[test]
    fn test_add_formula() {
        let mut formula = ChemicalFormula::new();
        let mut formula2 = ChemicalFormula::new();

        formula.add_element(ElementSymbol::O, 1.0);
        formula.add_wt_percent(ElementSymbol::H, 10.0);
        formula.add_wt_percent(ElementSymbol::N, 20.0);

        formula2.add_element(ElementSymbol::O, 1.0);
        formula2.add_wt_percent(ElementSymbol::H, 10.0);
        formula2.add_wt_percent(ElementSymbol::N, 20.0);

        let expected_o = 2.0;
        let expected_h = 20.0;
        let expected_n = 40.0;

        formula.add_formula(&formula2);

        let molecular_formula = formula.to_molecular_formula().unwrap();
        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_eq!(
            molecular_formula.stoichiometry[&ElementSymbol::O],
            expected_o
        );

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_h,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_n,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_multiply_formula() {
        let mut formula = ChemicalFormula::new();
        let multiplier = 2.0;

        formula.add_element(ElementSymbol::O, 1.0);
        formula.add_wt_percent(ElementSymbol::H, 10.0);
        formula.add_wt_percent(ElementSymbol::N, 20.0);

        let expected_o = 2.0;
        let expected_h = 20.0;
        let expected_n = 40.0;

        formula.multiply(multiplier);

        let molecular_formula = formula.to_molecular_formula().unwrap();
        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_eq!(
            molecular_formula.stoichiometry[&ElementSymbol::O],
            expected_o
        );

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_h,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_n,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_wt_multiply_formula() {
        let mut formula = ChemicalFormula::new();
        let multiplier = 2.0;

        formula.add_element(ElementSymbol::O, 1.0);
        formula.add_wt_percent(ElementSymbol::H, 10.0);
        formula.add_wt_percent(ElementSymbol::N, 20.0);

        let expected_h = 10.0 * 2.0 / 100.0;
        let expected_n = 20.0 * 2.0 / 100.0;

        let expected_h_after_wt_ratio = 10.0;
        let expected_n_after_wt_ratio = 20.0;

        formula.multiply_wt_percent(multiplier).unwrap();

        assert_abs_diff_eq!(
            formula.wt_percent[&ElementSymbol::H],
            expected_h,
            epsilon = 1e-6
        );

        assert_abs_diff_eq!(
            formula.wt_percent[&ElementSymbol::N],
            expected_n,
            epsilon = 1e-6
        );

        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_h_after_wt_ratio,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_n_after_wt_ratio,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_element_display() {
        assert_eq!(ElementSymbol::O.to_string(), "O");
        assert_eq!(ElementSymbol::None.to_string(), "");
    }

    #[test]
    fn test_element_from_str_trait() {
        assert_eq!("O".parse::<ElementSymbol>().unwrap(), ElementSymbol::O);
        assert!(matches!(
            "Xx".parse::<ElementSymbol>(),
            Err(FormulaError::InvalidElementSymbol(symbol)) if symbol == "Xx"
        ));
    }

    #[test]
    fn test_formula_display() {
        let formula = crate::parser::parse_formula("Pt5wt%/SiO2").unwrap();
        let display = formula.to_string();

        assert!(display.contains("Pt5wt%"));

        let reparsed: ChemicalFormula = display.parse().unwrap();
        let wt_percent = reparsed.to_wt_percent().unwrap().wt_percent;
        assert_abs_diff_eq!(wt_percent[&ElementSymbol::Pt], 5.0, epsilon = 1e-6);
    }

    #[test]
    fn test_formula_from_str() {
        let formula = ChemicalFormula::from_str("H2O").unwrap();
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::H], 2.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 1.0);
    }

    #[test]
    fn test_formula_roundtrip() {
        let original = crate::parser::parse_formula("(NH4)2SO4").unwrap();
        let serialized = original.to_string();
        let reparsed: ChemicalFormula = serialized.parse().unwrap();

        assert_abs_diff_eq!(
            original.molecular_weight().unwrap(),
            reparsed.molecular_weight().unwrap(),
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_division_by_zero_errors() {
        let mut no_mass_formula = ChemicalFormula::new();
        no_mass_formula.add_element(ElementSymbol::O, 0.0);
        assert!(matches!(
            no_mass_formula.to_wt_percent(),
            Err(FormulaError::DivisionByZero)
        ));

        let mut no_residue_formula = ChemicalFormula::new();
        no_residue_formula.add_element(ElementSymbol::Si, 1.0);
        no_residue_formula.add_wt_percent(ElementSymbol::Pt, 100.0);
        assert!(matches!(
            no_residue_formula.to_molecular_formula(),
            Err(FormulaError::DivisionByZero)
        ));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_roundtrip() {
        let formula = crate::parser::parse_formula("H2O").unwrap();
        let serialized = serde_json::to_string(&formula).unwrap();
        let deserialized: ChemicalFormula = serde_json::from_str(&serialized).unwrap();

        assert_abs_diff_eq!(deserialized.stoichiometry[&ElementSymbol::H], 2.0);
        assert_abs_diff_eq!(deserialized.stoichiometry[&ElementSymbol::O], 1.0);
    }
}
