use core::fmt;
use std::marker::PhantomData;

use serde::{
    de::{Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

pub fn divide_i16_by<'de, D, const N: i16>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = i16::deserialize(deserializer)?;
    Ok(f32::from(value) / f32::from(N))
}

pub fn divide_u16_by<'de, D, const N: i16>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u16::deserialize(deserializer)?;
    Ok(f32::from(value) / f32::from(N))
}

pub fn divide_u32_by<'de, D, const N: i16>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u32::deserialize(deserializer)?;
    Ok(f64::from(value) / f64::from(N))
}

pub fn parse_string<'de, D, const LENGTH: usize>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct ArrayVisitor<T, const M: usize>(PhantomData<T>);

    impl<'de, T, const M: usize> Visitor<'de> for ArrayVisitor<T, M>
    where
        T: Default + Copy + Deserialize<'de>,
    {
        type Value = [T; M];

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_fmt(format_args!("an array of size {}", M))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut arr = [T::default(); M];
            for i in 0..M {
                arr[i] = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(i, &self))?;
            }
            Ok(arr)
        }
    }

    let value: [u8; LENGTH] = deserializer.deserialize_tuple(LENGTH, ArrayVisitor(PhantomData))?;
    String::from_utf8(value.to_vec()).map_err(Error::custom)
}
