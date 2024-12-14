Part 1: https://research.metastate.dev/plonk-by-hand-part-1/
    
PLONK can do much more compared to older SNARKs.
If combined with KZG, it becomes a universal SNARK meaning the reference string created in a trusted setup can be re-used for any other PLONK circuit of the same size (or less) than the original.
    - The reference string can be also updated, with each update reducing the chance that the setup was compromised by collusion
    - Other polynomial commitment schemes can eliminate the need of trusted setup such as FRI (STARKs), or IPA (Kimchi).

PLONK needs a polynomial commitment scheme and KZG is used in this example.

KZG requires an elliptic curve on which we can compute pairings. Usually BN254, BLS12-381 or some other well-known pairing-friendly curve would be picked. (pairing-friendly means pairing function is efficiently computable without sacrificing security)

An elliptic curve is an equation of the form y^2 = x^3 + ax + b where a, b, x and y are field elements. In a cryptography setting the field will be a finite field.
    - Any (x, y) pairs satisfying this equation are points on the curve.

In this example, 𝔽₁₀₁ (Field 101) is used to make the computations easy.
Over this field, the elliptic curve y^2 = x^3 + 3 is going to be used with (1, 2) as the generator since it's a point easy to find.

Fun fact: The curve alt_bn128 (implemented on Ethereum) uses the same equation over a much larger field.

Points on elliptic curves from a group which means two points can be "added" to get another point on the curve. We can also "double" a point by adding it to itself or invert a point to find its additive inverse.

Point Doubling:
for P = (x,  y), 2P = (m ^ 2 - 2x, m( 3x - m^2) - y) where m = 3x ^ 2 / 2y

_This doubling formula only works for curves with a = 0 like ours._

Point Inversion:
for P = (x, y), -P = (x, -y)

Fun fact: Elliptic curves also have an abstract "point at infinity" which serves as the group identity.

TODO: explain more :)