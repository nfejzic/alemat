use super::Operator;

impl Operator {
    /// Create a '&#x002B;' operator.
    pub fn plus() -> Self {
        Self::from("\u{002B}")
    }

    /// Create a '&#x002D;' operator.
    pub fn minus() -> Self {
        Self::from("\u{002D}")
    }

    /// Create a '&#x22C5;' operator.
    pub fn dot() -> Self {
        Self::from("\u{22C5}")
    }

    /// Create a '&#x00A8;' operator.
    pub fn double_dot() -> Self {
        Self::from("\u{00A8}")
    }

    /// Create a '&#x2217;' operator.
    pub fn asterisk() -> Self {
        Self::from("\u{2217}")
    }

    /// Create a '&#x22C6;' operator.
    pub fn star() -> Self {
        Self::from("\u{22C6}")
    }

    /// Create a '&#x002F;' operator.
    pub fn solidus() -> Self {
        Self::from("\u{002F}")
    }

    /// Create a '&#x2216;' operator.
    pub fn set_minus() -> Self {
        Self::from("\u{2216}")
    }

    /// Create a '&#x00D7;' operator.
    pub fn mult() -> Self {
        Self::from("\u{00D7}")
    }

    /// Create a '&#x00F7;' operator.
    pub fn div() -> Self {
        Self::from("\u{00F7}")
    }

    /// Create a '&#x22C9;' operator.
    pub fn lfactor() -> Self {
        Self::from("\u{22C9}")
    }

    /// Create a '&#x22CA;' operator.
    pub fn rfactor() -> Self {
        Self::from("\u{22CA}")
    }

    /// Create a '&#x22C8;' operator.
    pub fn bowtie() -> Self {
        Self::from("\u{22C8}")
    }

    /// Create a '&#x2218;' operator.
    pub fn ring() -> Self {
        Self::from("\u{2218}")
    }

    /// Create a '&#x2295;' operator.
    pub fn circle_plus() -> Self {
        Self::from("\u{2295}")
    }

    /// Create a '&#x2297;' operator.
    pub fn circle_times() -> Self {
        Self::from("\u{2297}")
    }

    /// Create a '&#x2299;' operator.
    pub fn circle_dot() -> Self {
        Self::from("\u{2299}")
    }

    /// Create a '&#x2211;' operator.
    pub fn sum() -> Self {
        Self::from("\u{2211}")
    }

    /// Create a '&#x220F;' operator.
    pub fn prod() -> Self {
        Self::from("\u{220F}")
    }

    /// Create a '&#x2227;' operator.
    pub fn wedge() -> Self {
        Self::from("\u{2227}")
    }

    /// Create a '&#x22C0;' operator.
    pub fn big_wedge() -> Self {
        Self::from("\u{22C0}")
    }

    /// Create a '&#x2228;' operator.
    pub fn vee() -> Self {
        Self::from("\u{2228}")
    }

    /// Create a '&#x22C1;' operator.
    pub fn big_vee() -> Self {
        Self::from("\u{22C1}")
    }

    /// Create a '&#x2229;' operator.
    pub fn cap() -> Self {
        Self::from("\u{2229}")
    }

    /// Create a '&#x22C2;' operator.
    pub fn big_cap() -> Self {
        Self::from("\u{22C2}")
    }

    /// Create a '&#x222A;' operator.
    pub fn cup() -> Self {
        Self::from("\u{222A}")
    }

    /// Create a '&#x22C3;' operator.
    pub fn big_cup() -> Self {
        Self::from("\u{22C3}")
    }

    /// Create a '&#x222B;' operator.
    pub fn integral() -> Self {
        Self::from("\u{222B}")
    }

    /// Create a '&#x222E;' operator.
    pub fn circle_integral() -> Self {
        Self::from("\u{222E}")
    }

    /// Create a '&#x2202;' operator.
    pub fn partial_diff() -> Self {
        Self::from("\u{2202}")
    }

    /// Create a '&#x2207;' operator.
    pub fn nabla() -> Self {
        Self::from("\u{2207}")
    }

    /// Create a '&#x00B1;' operator.
    pub fn plus_minus() -> Self {
        Self::from("\u{00B1}")
    }

    /// Create a '&#x2234;' operator.
    pub fn therefore() -> Self {
        Self::from("\u{2234}")
    }

    /// Create a '&#x2235;' operator.
    pub fn because() -> Self {
        Self::from("\u{2235}")
    }

    /// Create a '&#x2220;' operator.
    pub fn angle() -> Self {
        Self::from("\u{2220}")
    }

    /// Create a '&#x230A;' operator.
    pub fn lfloor() -> Self {
        Self::from("\u{230A}")
    }

    /// Create a '&#x230B;' operator.
    pub fn rfloor() -> Self {
        Self::from("\u{230B}")
    }

    /// Create a '&#x2308;' operator.
    pub fn lceiling() -> Self {
        Self::from("\u{2308}")
    }

    /// Create a '&#x2309;' operator.
    pub fn rceiling() -> Self {
        Self::from("\u{2309}")
    }

    /// Create a '&#x2254;' operator.
    pub fn assign() -> Self {
        Self::from("\u{2254}")
    }

    /// Create a '&#x003D;' operator.
    pub fn eq() -> Self {
        Self::from("\u{003D}")
    }

    /// Create a '&#x2260;' operator.
    pub fn not_eq() -> Self {
        Self::from("\u{2260}")
    }

    /// Create a '&#x003C;' operator.
    pub fn lt() -> Self {
        Self::from("\u{003C}")
    }

    /// Create a '&#x003E;' operator.
    pub fn gt() -> Self {
        Self::from("\u{003E}")
    }

    /// Create a '&#x2264;' operator.
    pub fn le() -> Self {
        Self::from("\u{2264}")
    }

    /// Create a '&#x2265;' operator.
    pub fn ge() -> Self {
        Self::from("\u{2265}")
    }

    /// Create a '&#x227A;' operator.
    pub fn prec() -> Self {
        Self::from("\u{227A}")
    }

    /// Create a '&#x227B;' operator.
    pub fn succ() -> Self {
        Self::from("\u{227B}")
    }

    /// Create a '&#x227C;' operator.
    pub fn preceq() -> Self {
        Self::from("\u{227C}")
    }

    /// Create a '&#x227D;' operator.
    pub fn succeq() -> Self {
        Self::from("\u{227D}")
    }

    /// Create a '&#x2208;' operator.
    pub fn in_set() -> Self {
        Self::from("\u{2208}")
    }

    /// Create a '&#x2209;' operator.
    pub fn not_in_set() -> Self {
        Self::from("\u{2209}")
    }

    /// Create a '&#x2282;' operator.
    pub fn subset() -> Self {
        Self::from("\u{2282}")
    }

    /// Create a '&#x2283;' operator.
    pub fn supset() -> Self {
        Self::from("\u{2283}")
    }

    /// Create a '&#x2286;' operator.
    pub fn subseteq() -> Self {
        Self::from("\u{2286}")
    }

    /// Create a '&#x2287;' operator.
    pub fn supseteq() -> Self {
        Self::from("\u{2287}")
    }

    /// Create a '&#x2261;' operator.
    pub fn equivalent() -> Self {
        Self::from("\u{2261}")
    }

    /// Create a '&#x2245;' operator.
    pub fn congruent() -> Self {
        Self::from("\u{2245}")
    }

    /// Create a '&#x2248;' operator.
    pub fn approx() -> Self {
        Self::from("\u{2248}")
    }

    /// Create a '&#x221D;' operator.
    pub fn propto() -> Self {
        Self::from("\u{221D}")
    }

    /// Create a '&#x00AC;' operator.
    pub fn not() -> Self {
        Self::from("\u{00AC}")
    }

    /// Create a '&#x21D2;' operator.
    pub fn implies() -> Self {
        Self::from("\u{21D2}")
    }

    /// Create a '&#x21D0;' operator.
    pub fn log_if() -> Self {
        Self::from("\u{21D0}")
    }

    /// Create a '&#x21D4;' operator.
    pub fn iff() -> Self {
        Self::from("\u{21D4}")
    }

    /// Create a '&#x2200;' operator.
    pub fn forall() -> Self {
        Self::from("\u{2200}")
    }

    /// Create a '&#x2203;' operator.
    pub fn exists() -> Self {
        Self::from("\u{2203}")
    }

    /// Create a '&#x22A5;' operator.
    pub fn bottom() -> Self {
        Self::from("\u{22A5}")
    }

    /// Create a '&#x22A4;' operator.
    pub fn top() -> Self {
        Self::from("\u{22A4}")
    }

    /// Create a '&#x22A2;' operator.
    pub fn vdash() -> Self {
        Self::from("\u{22A2}")
    }

    /// Create a '&#x22A8;' operator.
    pub fn models() -> Self {
        Self::from("\u{22A8}")
    }

    /// Create a '&#x0028;' operator.
    pub fn lparens() -> Self {
        Self::from("\u{0028}")
    }

    /// Create a '&#x0029;' operator.
    pub fn rparens() -> Self {
        Self::from("\u{0029}")
    }

    /// Create a '&#x2322;' operator.
    pub fn oparens() -> Self {
        Self::from("\u{2322}")
    }

    /// Create a '&#x2323;' operator.
    pub fn uparens() -> Self {
        Self::from("\u{2323}")
    }

    /// Create a '&#x005B;' operator.
    pub fn lbracket() -> Self {
        Self::from("\u{005B}")
    }

    /// Create a '&#x005D;' operator.
    pub fn rbracket() -> Self {
        Self::from("\u{005D}")
    }

    /// Create a '&#x23B4;' operator.
    pub fn obracket() -> Self {
        Self::from("\u{23B4}")
    }

    /// Create a '&#x23B5;' operator.
    pub fn ubracket() -> Self {
        Self::from("\u{23B5}")
    }

    /// Create a '&#x007B;' operator.
    pub fn lbrace() -> Self {
        Self::from("\u{007B}")
    }

    /// Create a '&#x007D;' operator.
    pub fn rbrace() -> Self {
        Self::from("\u{007D}")
    }

    /// Create a '&#x23DE;' operator.
    pub fn obrace() -> Self {
        Self::from("\u{23DE}")
    }

    /// Create a '&#x23DF;' operator.
    pub fn ubrace() -> Self {
        Self::from("\u{23DF}")
    }

    /// Create a '&#x27E8;' operator.
    pub fn langle() -> Self {
        Self::from("\u{27E8}")
    }

    /// Create a '&#x27E9;' operator.
    pub fn rangle() -> Self {
        Self::from("\u{27E9}")
    }

    /// Create a '&#x007C;' operator.
    pub fn vert_bar() -> Self {
        Self::from("\u{007C}")
    }

    /// Create a '&#x2225;' operator.
    pub fn norm() -> Self {
        Self::from("\u{2225}")
    }

    /// Create a '&#x005E;' operator.
    pub fn hat() -> Self {
        Self::from("\u{005E}")
    }

    /// Create a '&#x005E;' operator.
    pub fn circumflex() -> Self {
        Self::hat()
    }

    /// Create a '&#x00AF;' operator.
    pub fn bar() -> Self {
        Self::from("\u{00AF}")
    }

    /// Create a '&#x00AF;' operator.
    pub fn macron() -> Self {
        Self::bar()
    }

    /// Create a '&#x2192;' operator.
    pub fn rarrow() -> Self {
        Self::from("\u{2192}")
    }

    /// Create a '&#x2190;' operator.
    pub fn larrow() -> Self {
        Self::from("\u{2190}")
    }

    /// Create a '&#x007E;' operator.
    pub fn tilde() -> Self {
        Self::from("\u{007E}")
    }
}
