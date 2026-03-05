pub mod bit_vector;
pub mod field;
pub mod ring;
pub mod vector;

pub trait PanickingIntegerCasts {
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
    fn as_u32(self) -> u32;
}

impl PanickingIntegerCasts for usize {
    fn as_u8(self) -> u8 {
        self.try_into().unwrap()
    }

    fn as_u16(self) -> u16 {
        self.try_into().unwrap()
    }

    fn as_u32(self) -> u32 {
        self.try_into().unwrap()
    }
}

pub trait ArrayConversion<const LEN: usize> {
    fn as_len_array(&self) -> [u8; LEN];
    fn into_len_array(self) -> [u8; LEN];
    fn into_padded_array(&self) -> [u8; LEN];
}

impl<const LEN: usize> ArrayConversion<LEN> for Vec<u8> {
    fn as_len_array(&self) -> [u8; LEN] {
        self.clone().try_into().unwrap()
    }

    fn into_len_array(self) -> [u8; LEN] {
        self.try_into().unwrap()
    }

    fn into_padded_array(&self) -> [u8; LEN] {
        assert!(self.len() <= LEN);
        let mut out = [0u8; LEN];
        out[0..self.len()].copy_from_slice(self);
        out
    }
}

impl<const LEN: usize> ArrayConversion<LEN> for &[u8] {
    fn as_len_array(&self) -> [u8; LEN] {
        self.to_vec().try_into().unwrap()
    }

    fn into_len_array(self) -> [u8; LEN] {
        self.try_into().unwrap()
    }

    fn into_padded_array(&self) -> [u8; LEN] {
        assert!(self.len() <= LEN);
        let mut out = [0u8; LEN];
        out[0..self.len()].copy_from_slice(self);
        out
    }
}

pub trait ArrayPadding<const LEN: usize> {
    fn into_padded_array<const OLEN: usize>(&self) -> [u8; OLEN];
}

impl<const LEN: usize> ArrayPadding<LEN> for &[u8; LEN] {
    fn into_padded_array<const OLEN: usize>(&self) -> [u8; OLEN] {
        assert!(self.len() <= OLEN);
        let mut out = [0u8; OLEN];
        out[0..self.len()].copy_from_slice(*self);
        out
    }
}

pub trait ArrayUpdate {
    fn update(self, start: usize, other: &[u8]) -> Self;
}

impl<const LEN: usize> ArrayUpdate for [u8; LEN] {
    fn update(mut self, start: usize, other: &[u8]) -> Self {
        self[start..start + other.len()].copy_from_slice(other);
        self
    }
}

pub trait VecUpdate {
    fn concat(self, other: &[u8]) -> Self;
}

impl VecUpdate for Vec<u8> {
    fn concat(mut self, other: &[u8]) -> Self {
        self.extend_from_slice(&other);
        self
    }
}

pub trait UpdatingArray {
    fn push(self, other: &[u8]) -> Self;
}

pub struct UpdatableArray<const LEN: usize> {
    value: [u8; LEN],
    pointer: usize,
}

impl<const LEN: usize> UpdatableArray<LEN> {
    pub fn new(value: [u8; LEN]) -> Self {
        Self { value, pointer: 0 }
    }

    pub fn array(self) -> [u8; LEN] {
        self.value
    }
}

impl<const LEN: usize> UpdatingArray for UpdatableArray<LEN> {
    fn push(mut self, other: &[u8]) -> Self {
        self.value[self.pointer..self.pointer + other.len()].copy_from_slice(other);
        self.pointer += other.len();
        self
    }
}

impl<const LEN: usize> From<[u8; LEN]> for UpdatableArray<LEN> {
    fn from(value: [u8; LEN]) -> Self {
        Self { value, pointer: 0 }
    }
}

impl<const LEN: usize> From<UpdatableArray<LEN>> for [u8; LEN] {
    fn from(value: UpdatableArray<LEN>) -> Self {
        value.value
    }
}

pub trait UpdatingArray2<const OLEN: usize> {
    fn push(self, other: &[u8]) -> [u8; OLEN];
}

impl<const LEN: usize, const OLEN: usize> UpdatingArray2<OLEN> for [u8; LEN] {
    fn push(self, other: &[u8]) -> [u8; OLEN] {
        let mut out = [0u8; OLEN];
        out[0..self.len()].copy_from_slice(&self);
        out[self.len()..].copy_from_slice(other);
        out
    }
}

impl<const OLEN: usize> UpdatingArray2<OLEN> for &[u8] {
    fn push(self, other: &[u8]) -> [u8; OLEN] {
        let mut out = [0u8; OLEN];
        out[0..self.len()].copy_from_slice(&self);
        out[self.len()..].copy_from_slice(other);
        out
    }
}
