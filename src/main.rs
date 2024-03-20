use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::traits::{AsBytes, ByteConversion};
use lambdaworks_math::{cyclic_group::IsGroup, elliptic_curve::traits::IsEllipticCurve};
use lambdaworks_math::unsigned_integer::element::U256;




fn main() {

    const PRIVATE_KEY: u64= 0x6C616D6264617370;
    let generator = BLS12381Curve::generator();
    let public_key = generator.operate_with_self(PRIVATE_KEY);
    let public_key_from_bytes =
    U256::from_bytes_be(&public_key.as_bytes()).expect("Type conversion failed");

    println!("0x{}",public_key_from_bytes.to_hex().to_uppercase());
}


