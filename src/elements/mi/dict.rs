use super::Ident;

impl Ident {
    pub fn empty_set() -> Self {
        Self::from("\u{2205}")
    }

    pub fn infinity() -> Self {
        Self::from("∞")
    }

    pub fn aleph() -> Self {
        Self::from("ℵ")
    }

    pub fn set_natural() -> Self {
        Self::from("\u{2115}")
    }

    pub fn set_integer() -> Self {
        Self::from("\u{2124}")
    }

    pub fn set_rational() -> Self {
        Self::from("\u{211A}")
    }

    pub fn set_irrational() -> Self {
        Self::from("\u{211D}")
    }

    pub fn set_complex() -> Self {
        Self::from("\u{2102}")
    }

    pub fn set_body() -> Self {
        Self::from("\u{1D542}")
    }

    pub fn alpha() -> Self {
        Self::from("\u{1D6FC}")
    }

    pub fn beta() -> Self {
        Self::from("\u{1D6FD}")
    }

    pub fn gamma() -> Self {
        Self::from("\u{1D6FE}")
    }

    pub fn big_gamma() -> Self {
        Self::from("\u{0393}")
    }

    pub fn delta() -> Self {
        Self::from("\u{1D6FF}")
    }

    pub fn big_delta() -> Self {
        Self::from("\u{0394}")
    }

    pub fn epsilon() -> Self {
        Self::from("\u{1D700}")
    }

    pub fn varepsilon() -> Self {
        Self::from("\u{1D716}")
    }

    pub fn zeta() -> Self {
        Self::from("\u{1D701}")
    }

    pub fn eta() -> Self {
        Self::from("\u{1D702}")
    }

    pub fn theta() -> Self {
        Self::from("\u{1D703}")
    }

    pub fn big_theta() -> Self {
        Self::from("\u{0398}")
    }

    pub fn vartheta() -> Self {
        Self::from("\u{1D717}")
    }

    pub fn iota() -> Self {
        Self::from("\u{1D704}")
    }

    pub fn kappa() -> Self {
        Self::from("\u{1D705}")
    }

    pub fn lambda() -> Self {
        Self::from("\u{1D706}")
    }

    pub fn big_lambda() -> Self {
        Self::from("\u{039B}")
    }

    pub fn mu() -> Self {
        Self::from("\u{1D707}")
    }

    pub fn nu() -> Self {
        Self::from("\u{1D708}")
    }

    pub fn xi() -> Self {
        Self::from("\u{1D709}")
    }

    pub fn big_xi() -> Self {
        Self::from("\u{039E}")
    }

    pub fn pi() -> Self {
        Self::from("\u{1D70B}")
    }

    pub fn big_pi() -> Self {
        Self::from("\u{03A0}")
    }

    pub fn rho() -> Self {
        Self::from("\u{1D70C}")
    }

    pub fn sigma() -> Self {
        Self::from("\u{1D70E}")
    }

    pub fn big_sigma() -> Self {
        Self::from("\u{03A3}")
    }

    pub fn tau() -> Self {
        Self::from("\u{1D70F}")
    }

    pub fn upsilon() -> Self {
        Self::from("\u{1D710}")
    }

    pub fn phi() -> Self {
        Self::from("\u{1D711}")
    }

    pub fn big_phi() -> Self {
        Self::from("\u{03A6}")
    }

    pub fn varphi() -> Self {
        Self::from("\u{1D719}")
    }

    pub fn chi() -> Self {
        Self::from("\u{1D713}")
    }

    pub fn psi() -> Self {
        Self::from("\u{1D714}")
    }

    pub fn big_psi() -> Self {
        Self::from("\u{03A8}")
    }

    pub fn omega() -> Self {
        Self::from("\u{1D715}")
    }

    pub fn big_omega() -> Self {
        Self::from("\u{03A9}")
    }
}
