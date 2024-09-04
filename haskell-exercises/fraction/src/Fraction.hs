module Fraction (Fraction, add, sub, mul, divide, hcf) where

type Fraction = (Int, Int)

-- Implement the `add` Function

add :: Fraction -> Fraction -> Fraction
add a b
    | d1 == d2 = simplify(n1 + n2, d1)
    | otherwise = simplify(n1 * d2 + n2 * d1, d1 * d2)
    where
      n1 = fst a
      n2 = fst b
      d1 = snd a
      d2 = snd b

-- Implement the `sub` Function

sub :: Fraction -> Fraction -> Fraction
sub a b
    | d1 == d2 = simplify(n1 - n2, d1)
    | otherwise = simplify(n1 * d2 - n2 * d1, d1 * d2)
    where
      n1 = fst a
      n2 = fst b
      d1 = snd a
      d2 = snd b

-- Implement the `mul` Function

mul :: Fraction -> Fraction -> Fraction
mul a b
    | d1 == d2 = simplify(n1 * n2, d1 * d1)
    | otherwise = simplify(n1 * n2, d1 * d2)
    where
      n1 = fst a
      n2 = fst b
      d1 = snd a
      d2 = snd b

-- Implement the `divide` Function

divide :: Fraction -> Fraction -> Fraction
divide a b = simplify(n1 * d2, d1 * n2)
    where
      n1 = fst a
      n2 = fst b
      d1 = snd a
      d2 = snd b

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