use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::mem;

#[derive(Debug)]
pub enum FormulaError {
    FileIOError,
    FileParseError,
    WeightPercentOverflow,
    NoFormula,
}

impl Error for FormulaError {
    fn description(&self) -> &str {
        match *self {
            FormulaError::FileIOError => "File IO error",
            FormulaError::FileParseError => "File parse error",
            FormulaError::WeightPercentOverflow => "Weight percent overflow",
            FormulaError::NoFormula => "No formula",
        }
    }
}

impl std::fmt::Display for FormulaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FormulaError::FileIOError => write!(f, "File IO error"),
            FormulaError::FileParseError => write!(f, "File parse error"),
            FormulaError::WeightPercentOverflow => write!(f, "Weight percent overflow"),
            FormulaError::NoFormula => write!(f, "No formula"),
        }
    }
}

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

#[derive(Debug, Clone, Default)]
pub struct ChemicalFormula {
    pub element: HashSet<ElementSymbol>,
    pub stoichiometry: HashMap<ElementSymbol, f64>,
    pub wt_percent: HashMap<ElementSymbol, f64>,
}

impl ChemicalFormula {
    pub fn new() -> Self {
        ChemicalFormula {
            element: HashSet::new(),
            stoichiometry: HashMap::new(),
            wt_percent: HashMap::new(),
        }
    }
    pub fn add_element(&mut self, element: ElementSymbol, stoichiometry: f64) -> &mut Self {
        self.element.insert(element);

        self.stoichiometry
            .entry(element)
            .and_modify(|e| *e += stoichiometry)
            .or_insert(stoichiometry);

        self
    }

    pub fn add_wt_percent(&mut self, element: ElementSymbol, wt_ratio: f64) -> &mut Self {
        self.element.insert(element);

        self.wt_percent
            .entry(element)
            .and_modify(|e| *e += wt_ratio)
            .or_insert(wt_ratio);
        self
    }

    pub fn multiply(&mut self, multiplier: f64) -> &mut Self {
        for (element, stoichiometry) in self.stoichiometry.iter_mut() {
            *stoichiometry *= multiplier;
        }

        for (element, wt_ratio) in self.wt_percent.iter_mut() {
            *wt_ratio *= multiplier;
        }
        self
    }

    pub fn to_molecular_formula(&self) -> Result<ChemicalFormula, FormulaError> {
        if self.wt_percent.is_empty() {
            return Ok(self.clone());
        }

        let mut wt_ratio_sum = 0.0;
        let mut wt_ratio_molecular_weight_sum = 0.0;

        for (element, wt_ratio) in self.wt_percent.iter() {
            wt_ratio_sum += wt_ratio;
            wt_ratio_molecular_weight_sum += wt_ratio / element.atomic_weight();
        }

        if wt_ratio_sum > 100. {
            return Err(FormulaError::WeightPercentOverflow);
        }

        let residue = 100. - wt_ratio_sum;

        let mut molecular_weight_residue = if self.stoichiometry.is_empty() {
            100.0
        } else {
            self.stoichiometry
                .iter()
                .map(|(element, stoichiometry)| ATOMIC_WEIGHT[element] * stoichiometry)
                .fold(0.0, |acc, x| acc + x)
        };

        let molecular_weight_main =
            molecular_weight_residue * wt_ratio_molecular_weight_sum / residue;

        let mut stoichiometry: HashMap<ElementSymbol, f64> = self.stoichiometry.clone();

        for (element, wt_ratio) in self.wt_percent.iter() {
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

    pub fn to_mol_percent(&self) -> Result<ChemicalFormula, FormulaError> {
        let mut formula = self.to_molecular_formula()?;

        if formula.stoichiometry.is_empty() {
            return Ok(ChemicalFormula::new());
        }

        let molecular_formula_sum = formula
            .stoichiometry
            .iter()
            .map(|(element, stoichiometry)| stoichiometry)
            .fold(0.0, |acc, x| acc + x);

        formula.multiply(100. / molecular_formula_sum);

        Ok(formula)
    }

    pub fn molecular_weight(&self) -> Result<f64, FormulaError> {
        let stoichiometry = if self.wt_percent.is_empty() {
            self.stoichiometry.clone()
        } else {
            self.to_molecular_formula().unwrap().stoichiometry
        };
        Ok(stoichiometry
            .iter()
            .map(|(element, stoichiometry)| ATOMIC_WEIGHT[element] * stoichiometry)
            .fold(0.0, |acc, x| acc + x))
    }

    pub fn to_wt(&self) -> Result<ChemicalFormula, FormulaError> {
        if self.stoichiometry.is_empty() {
            return Ok(self.clone());
        }

        let formula = self.to_molecular_formula()?;
        let molecular_weight = formula.molecular_weight()?;

        let mut wt_ratio = HashMap::new();

        for (element, stoichiometry) in formula.stoichiometry.iter() {
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

    pub fn to_wt_percent(&self) -> Result<ChemicalFormula, FormulaError> {
        let mut formula = self.to_wt()?;

        let wt_total = formula.wt_percent.iter().fold(0.0, |acc, (_, x)| acc + x);

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

        let mut wt_ratio = HashMap::new();

        for (element, stoichiometry) in formula.stoichiometry.iter() {
            wt_ratio.insert(
                *element,
                stoichiometry * ATOMIC_WEIGHT[element] / molecular_weight * multiplier,
            );
        }

        mem::replace(
            self,
            ChemicalFormula {
                element: formula.element,
                stoichiometry: HashMap::new(),
                wt_percent: wt_ratio,
            },
        );

        Ok(self)
    }

    pub fn add_formula(&mut self, formula: &ChemicalFormula) -> &mut Self {
        for (element, stoichiometry) in formula.stoichiometry.iter() {
            self.element.insert(*element);
            self.stoichiometry
                .entry(*element)
                .and_modify(|e| *e += stoichiometry)
                .or_insert(*stoichiometry);
        }

        for (element, wt_ratio) in formula.wt_percent.iter() {
            self.element.insert(*element);

            self.wt_percent
                .entry(*element)
                .and_modify(|e| *e += wt_ratio)
                .or_insert(*wt_ratio);
        }

        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_formula() {
        let mut formula = ChemicalFormula::new();

        formula.add_element(ElementSymbol::O, 1.0);
        formula.add_wt_percent(ElementSymbol::H, 10.0);
        formula.add_wt_percent(ElementSymbol::N, 20.0);

        let expected_O = 1.0;
        let expected_H = 10.0;
        let expected_N = 20.0;

        let molecular_formula = formula.to_molecular_formula().unwrap();

        let O_mol_ratio = molecular_formula
            .stoichiometry
            .get(&ElementSymbol::O)
            .unwrap();
        let H_mol_ratio = molecular_formula
            .stoichiometry
            .get(&ElementSymbol::H)
            .unwrap();

        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_eq!(
            molecular_formula.stoichiometry[&ElementSymbol::O],
            expected_O
        );

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_H,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_N,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_formula_no_element() {
        let mut formula = ChemicalFormula::new();

        formula.add_wt_percent(ElementSymbol::H, 10.0);
        formula.add_wt_percent(ElementSymbol::N, 20.0);

        let expected_H = 10.;
        let expected_N = 20.;
        let expected_H_after_wt_percent = 1. / 3. * 100.;
        let expected_N_after_wt_percent = 2. / 3. * 100.;

        let molecular_formula = formula.to_molecular_formula().unwrap();

        let H_mol_ratio = molecular_formula
            .stoichiometry
            .get(&ElementSymbol::H)
            .unwrap();

        let wt_ratio = formula.to_wt().unwrap();

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_H,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_N,
            epsilon = 1e-6
        );

        let wt_percent = formula.to_wt_percent().unwrap();

        assert_abs_diff_eq!(
            wt_percent.wt_percent[&ElementSymbol::H],
            expected_H_after_wt_percent,
            epsilon = 1e-6
        );

        assert_abs_diff_eq!(
            wt_percent.wt_percent[&ElementSymbol::N],
            expected_N_after_wt_percent,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_no_wt_percent() {
        let mut formula = ChemicalFormula::new();

        formula.add_element(ElementSymbol::O, 1.0);

        let expected_O = 1.0;

        let molecular_formula = formula.to_molecular_formula().unwrap();

        let O_mol_ratio = molecular_formula
            .stoichiometry
            .get(&ElementSymbol::O)
            .unwrap();

        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_eq!(
            molecular_formula.stoichiometry[&ElementSymbol::O],
            expected_O
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

        let expected_O = 2.0;
        let expected_H = 20.0;
        let expected_N = 40.0;

        formula.add_formula(&formula2);

        let molecular_formula = formula.to_molecular_formula().unwrap();

        let O_mol_ratio = molecular_formula
            .stoichiometry
            .get(&ElementSymbol::O)
            .unwrap();
        let H_mol_ratio = molecular_formula
            .stoichiometry
            .get(&ElementSymbol::H)
            .unwrap();

        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_eq!(
            molecular_formula.stoichiometry[&ElementSymbol::O],
            expected_O
        );

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_H,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_N,
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

        let expected_O = 2.0;
        let expected_H = 20.0;
        let expected_N = 40.0;

        formula.multiply(multiplier);

        let molecular_formula = formula.to_molecular_formula().unwrap();

        let O_mol_ratio = molecular_formula
            .stoichiometry
            .get(&ElementSymbol::O)
            .unwrap();
        let H_mol_ratio = molecular_formula
            .stoichiometry
            .get(&ElementSymbol::H)
            .unwrap();

        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_eq!(
            molecular_formula.stoichiometry[&ElementSymbol::O],
            expected_O
        );

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_H,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_N,
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

        let expected_H = 10.0 * 2.0 / 100.0;
        let expected_N = 20.0 * 2.0 / 100.0;

        let expected_H_after_wt_ratio = 10.;
        let expected_N_after_wt_ratio = 20.;

        formula.multiply_wt_percent(multiplier);

        assert_abs_diff_eq!(
            formula.wt_percent[&ElementSymbol::H],
            expected_H,
            epsilon = 1e-6
        );

        assert_abs_diff_eq!(
            formula.wt_percent[&ElementSymbol::N],
            expected_N,
            epsilon = 1e-6
        );

        let wt_ratio = formula.to_wt_percent().unwrap();

        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::H],
            expected_H_after_wt_ratio,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            wt_ratio.wt_percent[&ElementSymbol::N],
            expected_N_after_wt_ratio,
            epsilon = 1e-6
        );
    }
}
