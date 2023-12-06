use super::Operator;

#[allow(missing_docs)]
impl Operator {
    pub fn plus() -> Self {
        Self::from("\u{002B}")
    }

    pub fn minus() -> Self {
        Self::from("\u{002D}")
    }

    pub fn dot() -> Self {
        Self::from("\u{22C5}")
    }

    pub fn asterisk() -> Self {
        Self::from("\u{2217}")
    }

    pub fn star() -> Self {
        Self::from("\u{22C6}")
    }

    pub fn solidus() -> Self {
        Self::from("\u{002F}")
    }

    pub fn set_minus() -> Self {
        Self::from("\u{2216}")
    }

    pub fn mult() -> Self {
        Self::from("\u{00D7}")
    }

    pub fn div() -> Self {
        Self::from("\u{00F7}")
    }

    pub fn lfactor() -> Self {
        Self::from("\u{22C9}")
    }

    pub fn rfactor() -> Self {
        Self::from("\u{22CA}")
    }

    pub fn bowtie() -> Self {
        Self::from("\u{22C8}")
    }

    pub fn ring() -> Self {
        Self::from("\u{2218}")
    }

    pub fn circle_plus() -> Self {
        Self::from("\u{2295}")
    }

    pub fn circle_times() -> Self {
        Self::from("\u{2297}")
    }

    pub fn circle_dot() -> Self {
        Self::from("\u{2299}")
    }

    pub fn sum() -> Self {
        Self::from("\u{2211}")
    }

    pub fn prod() -> Self {
        Self::from("\u{220F}")
    }

    pub fn wedge() -> Self {
        Self::from("\u{2227}")
    }

    pub fn big_wedge() -> Self {
        Self::from("\u{22C0}")
    }

    pub fn vee() -> Self {
        Self::from("\u{2228}")
    }

    pub fn big_vee() -> Self {
        Self::from("\u{22C1}")
    }

    pub fn cap() -> Self {
        Self::from("\u{2229}")
    }

    pub fn big_cap() -> Self {
        Self::from("\u{22C2}")
    }

    pub fn cup() -> Self {
        Self::from("\u{222A}")
    }

    pub fn big_cup() -> Self {
        Self::from("\u{22C3}")
    }

    pub fn integral() -> Self {
        Self::from("\u{222B}")
    }

    pub fn circle_integral() -> Self {
        Self::from("\u{222E}")
    }

    pub fn partial_diff() -> Self {
        Self::from("\u{2202}")
    }

    pub fn nabla() -> Self {
        Self::from("\u{2207}")
    }

    pub fn plus_minus() -> Self {
        Self::from("\u{00B1}")
    }

    pub fn therefore() -> Self {
        Self::from("\u{2234}")
    }

    pub fn because() -> Self {
        Self::from("\u{2235}")
    }

    pub fn angle() -> Self {
        Self::from("\u{2220}")
    }

    pub fn lfloor() -> Self {
        Self::from("\u{230A}")
    }

    pub fn rfloor() -> Self {
        Self::from("\u{230B}")
    }

    pub fn lceiling() -> Self {
        Self::from("\u{2308}")
    }

    pub fn rceiling() -> Self {
        Self::from("\u{2309}")
    }

    pub fn eq() -> Self {
        Self::from("\u{003D}")
    }

    pub fn not_eq() -> Self {
        Self::from("\u{2260}")
    }

    pub fn lt() -> Self {
        Self::from("\u{003C}")
    }

    pub fn rt() -> Self {
        Self::from("\u{003E}")
    }

    pub fn le() -> Self {
        Self::from("\u{2264}")
    }

    pub fn ge() -> Self {
        Self::from("\u{2265}")
    }

    pub fn prec() -> Self {
        Self::from("\u{227A}")
    }

    pub fn succ() -> Self {
        Self::from("\u{227B}")
    }

    pub fn preceq() -> Self {
        Self::from("\u{227C}")
    }

    pub fn succeq() -> Self {
        Self::from("\u{227D}")
    }

    pub fn in_set() -> Self {
        Self::from("\u{2208}")
    }

    pub fn not_in_set() -> Self {
        Self::from("\u{2209}")
    }

    pub fn subset() -> Self {
        Self::from("\u{2282}")
    }

    pub fn supset() -> Self {
        Self::from("\u{2283}")
    }

    pub fn subseteq() -> Self {
        Self::from("\u{2286}")
    }

    pub fn supseteq() -> Self {
        Self::from("\u{2287}")
    }

    pub fn equivalent() -> Self {
        Self::from("\u{2261}")
    }

    pub fn congruent() -> Self {
        Self::from("\u{2245}")
    }

    pub fn approx() -> Self {
        Self::from("\u{2248}")
    }

    pub fn propto() -> Self {
        Self::from("\u{221D}")
    }

    pub fn not() -> Self {
        Self::from("\u{00AC}")
    }

    pub fn implies() -> Self {
        Self::from("\u{21D2}")
    }

    pub fn log_if() -> Self {
        Self::from("\u{21D0}")
    }

    pub fn iff() -> Self {
        Self::from("\u{21D4}")
    }

    pub fn forall() -> Self {
        Self::from("\u{2200}")
    }

    pub fn exists() -> Self {
        Self::from("\u{2203}")
    }

    pub fn bottom() -> Self {
        Self::from("\u{22A5}")
    }

    pub fn top() -> Self {
        Self::from("\u{22A4}")
    }

    pub fn vdash() -> Self {
        Self::from("\u{22A2}")
    }

    pub fn models() -> Self {
        Self::from("\u{22A8}")
    }

    pub fn lparens() -> Self {
        Self::from("\u{0028}")
    }

    pub fn rparens() -> Self {
        Self::from("\u{0029}")
    }

    pub fn oparens() -> Self {
        Self::from("\u{2322}")
    }

    pub fn uparens() -> Self {
        Self::from("\u{2323}")
    }

    pub fn lbracket() -> Self {
        Self::from("\u{005B}")
    }

    pub fn rbracket() -> Self {
        Self::from("\u{005D}")
    }

    pub fn obracket() -> Self {
        Self::from("\u{23B4}")
    }

    pub fn ubracket() -> Self {
        Self::from("\u{23B5}")
    }

    pub fn lbrace() -> Self {
        Self::from("\u{007B}")
    }

    pub fn rbrace() -> Self {
        Self::from("\u{007D}")
    }

    pub fn obrace() -> Self {
        Self::from("\u{23DE}")
    }

    pub fn ubrace() -> Self {
        Self::from("\u{23DF}")
    }

    pub fn langle() -> Self {
        Self::from("\u{27E8}")
    }

    pub fn rangle() -> Self {
        Self::from("\u{27E9}")
    }

    pub fn vert_bar() -> Self {
        Self::from("\u{007C}")
    }

    pub fn hat() -> Self {
        Self::from("\u{005E}")
    }

    pub fn circumflex() -> Self {
        Self::hat()
    }

    pub fn bar() -> Self {
        Self::from("\u{00AF}")
    }

    pub fn macron() -> Self {
        Self::bar()
    }

    pub fn rarrow() -> Self {
        Self::from("\u{2192}")
    }

    pub fn larrow() -> Self {
        Self::from("\u{2190}")
    }
}
