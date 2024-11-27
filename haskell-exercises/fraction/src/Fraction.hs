module Fraction (Fraction, add, sub, mul, divide, hcf) where

type Fraction = (Int, Int)

-- Implement the `add` Function

add :: Fraction -> Fraction -> Fraction
add (n1, d1) (n2, d2) = (n1 * d2 + n2 * d1, d1 * d2)
-- Implement the `sub` Function

sub :: Fraction -> Fraction -> Fraction
sub (n1, d1) (n2, d2) = (n1 * d2 - n2 * d1, d1 * d2)

-- Implement the `mul` Function

mul :: Fraction -> Fraction -> Fraction
mul (n1, d1) (n2, d2) = (n1 * n2, d1 * d2)

-- Implement the `divide` Function

divide :: Fraction -> Fraction -> Fraction
divide (n1, d1) (n2, d2) = (n1 * d2, d1 * n2)

-- Implement the `hcf` Function

hcf :: Int -> Int -> Int
hcf a b
    | a == 0 = b
    | b == 0 = a
    | otherwise = hcf b (a `mod` b)

-- Implement the `simplify` Function

simplify :: Fraction -> Fraction
simplify (n, d) = (n `div` h, d `div` h)
    where h = hcf n d