use super::Ident;

impl Ident {
    /// Create a '&#x2205;' identifier.
    pub fn empty_set() -> Self {
        Self::from("\u{2205}")
    }

    /// Create a '∞' identifier.
    pub fn infinity() -> Self {
        Self::from("∞")
    }

    /// Create a 'ℵ' identifier.
    pub fn aleph() -> Self {
        Self::from("ℵ")
    }

    /// Create a '&#x2115;' identifier.
    pub fn set_natural() -> Self {
        Self::from("\u{2115}")
    }

    /// Create a '&#x2124;' identifier.
    pub fn set_integer() -> Self {
        Self::from("\u{2124}")
    }

    /// Create a '&#x211A;' identifier.
    pub fn set_rational() -> Self {
        Self::from("\u{211A}")
    }

    /// Create a '&#x211D;' identifier.
    pub fn set_irrational() -> Self {
        Self::from("\u{211D}")
    }

    /// Create a '&#x2102;' identifier.
    pub fn set_complex() -> Self {
        Self::from("\u{2102}")
    }

    /// Create a '&#x1D542;' identifier.
    pub fn set_body() -> Self {
        Self::from("\u{1D542}")
    }

    /// Create a '&#x1D6FC;' identifier.
    pub fn alpha() -> Self {
        Self::from("\u{1D6FC}")
    }

    /// Create a '&#x1D6FD;' identifier.
    pub fn beta() -> Self {
        Self::from("\u{1D6FD}")
    }

    /// Create a '&#x1D6FE;' identifier.
    pub fn gamma() -> Self {
        Self::from("\u{1D6FE}")
    }

    /// Create a '&#x0393;' identifier.
    pub fn big_gamma() -> Self {
        Self::from("\u{0393}")
    }

    /// Create a '&#x1D6FF;' identifier.
    pub fn delta() -> Self {
        Self::from("\u{1D6FF}")
    }

    /// Create a '&#x0394;' identifier.
    pub fn big_delta() -> Self {
        Self::from("\u{0394}")
    }

    /// Create a '&#x1D700;' identifier.
    pub fn epsilon() -> Self {
        Self::from("\u{1D700}")
    }

    /// Create a '&#x1D716;' identifier.
    pub fn varepsilon() -> Self {
        Self::from("\u{1D716}")
    }

    /// Create a '&#x1D701;' identifier.
    pub fn zeta() -> Self {
        Self::from("\u{1D701}")
    }

    /// Create a '&#x1D702;' identifier.
    pub fn eta() -> Self {
        Self::from("\u{1D702}")
    }

    /// Create a '&#x1D703;' identifier.
    pub fn theta() -> Self {
        Self::from("\u{1D703}")
    }

    /// Create a '&#x0398;' identifier.
    pub fn big_theta() -> Self {
        Self::from("\u{0398}")
    }

    /// Create a '&#x1D717;' identifier.
    pub fn vartheta() -> Self {
        Self::from("\u{1D717}")
    }

    /// Create a '&#x1D704;' identifier.
    pub fn iota() -> Self {
        Self::from("\u{1D704}")
    }

    /// Create a '&#x1D705;' identifier.
    pub fn kappa() -> Self {
        Self::from("\u{1D705}")
    }

    /// Create a '&#x1D706;' identifier.
    pub fn lambda() -> Self {
        Self::from("\u{1D706}")
    }

    /// Create a '&#x039B;' identifier.
    pub fn big_lambda() -> Self {
        Self::from("\u{039B}")
    }

    /// Create a '&#x1D707;' identifier.
    pub fn mu() -> Self {
        Self::from("\u{1D707}")
    }

    /// Create a '&#x1D708;' identifier.
    pub fn nu() -> Self {
        Self::from("\u{1D708}")
    }

    /// Create a '&#x1D709;' identifier.
    pub fn xi() -> Self {
        Self::from("\u{1D709}")
    }

    /// Create a '&#x039E;' identifier.
    pub fn big_xi() -> Self {
        Self::from("\u{039E}")
    }

    /// Create a '&#x1D70B;' identifier.
    pub fn pi() -> Self {
        Self::from("\u{1D70B}")
    }

    /// Create a '&#x03A0;' identifier.
    pub fn big_pi() -> Self {
        Self::from("\u{03A0}")
    }

    /// Create a '&#x1D70C;' identifier.
    pub fn rho() -> Self {
        Self::from("\u{1D70C}")
    }

    /// Create a '&#x1D70E;' identifier.
    pub fn sigma() -> Self {
        Self::from("\u{1D70E}")
    }

    /// Create a '&#x03A3;' identifier.
    pub fn big_sigma() -> Self {
        Self::from("\u{03A3}")
    }

    /// Create a '&#x1D70F;' identifier.
    pub fn tau() -> Self {
        Self::from("\u{1D70F}")
    }

    /// Create a '&#x1D710;' identifier.
    pub fn upsilon() -> Self {
        Self::from("\u{1D710}")
    }

    /// Create a '&#x1D711;' identifier.
    pub fn phi() -> Self {
        Self::from("\u{1D711}")
    }

    /// Create a '&#x03A6;' identifier.
    pub fn big_phi() -> Self {
        Self::from("\u{03A6}")
    }

    /// Create a '&#x1D719;' identifier.
    pub fn varphi() -> Self {
        Self::from("\u{1D719}")
    }

    /// Create a '&#x1D713;' identifier.
    pub fn chi() -> Self {
        Self::from("\u{1D713}")
    }

    /// Create a '&#x03C8;' identifier.
    pub fn psi() -> Self {
        Self::from("\u{03C8}")
    }

    /// Create a '&#x03A8;' identifier.
    pub fn big_psi() -> Self {
        Self::from("\u{03A8}")
    }

    /// Create a '&#x03C9;' identifier.
    pub fn omega() -> Self {
        Self::from("\u{03C9}")
    }

    /// Create a '&#x03A9;' identifier.
    pub fn big_omega() -> Self {
        Self::from("\u{03A9}")
    }
}
