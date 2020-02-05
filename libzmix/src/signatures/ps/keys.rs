use amcl_wrapper::field_elem::FieldElement;
use amcl_wrapper::group_elem::GroupElement;

use super::errors::{PSError, PSErrorKind};
use super::{OtherGroup, SignatureGroup};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sigkey {
    pub x: FieldElement,
    pub y: Vec<FieldElement>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Verkey {
    pub X_tilde: OtherGroup,
    pub Y_tilde: Vec<OtherGroup>,
}

// Parameters generated by random oracle.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub g: SignatureGroup,
    pub g_tilde: OtherGroup,
}

impl Params {
    /// Generate g1, g2. These are shared by signer and all users.
    pub fn new(label: &[u8]) -> Self {
        let g = SignatureGroup::from_msg_hash(&[label, " : g".as_bytes()].concat());
        let g_tilde = OtherGroup::from_msg_hash(&[label, " : g_tilde".as_bytes()].concat());
        Self { g, g_tilde }
    }
}

impl Verkey {
    pub fn msg_count(&self) -> usize {
        self.Y_tilde.len()
    }
}

pub fn keygen(count_messages: usize, params: &Params) -> (Verkey, Sigkey) {
    let x = FieldElement::random();
    let X_tilde = &params.g_tilde * &x;
    let mut y = vec![];
    let mut Y_tilde = vec![];
    for _ in 0..count_messages {
        let y_i = FieldElement::random();
        Y_tilde.push(&params.g_tilde * &y_i);
        y.push(y_i);
    }
    (Verkey { X_tilde, Y_tilde }, Sigkey { x, y })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        let count_msgs = 5;
        let params = Params::new("test".as_bytes());
        let (vk, sk) = keygen(count_msgs, &params);
        assert_eq!(sk.y.len(), count_msgs);
        assert_eq!(vk.Y_tilde.len(), count_msgs);
    }
}
