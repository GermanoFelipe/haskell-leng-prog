module Lists (member, union, intersection, difference,
              insert, insertionSort, firsts,
              binaryToDecimal, toDecimal, toDec, decimal,
              binaryAdd) where
  
import Data.Char(digitToInt)  

member:: Int -> [Int] -> Bool
member _ []      = False
member e (x:xs)  = e == x || member e xs


union:: [Int] -> [Int] -> [Int]
union [] ys     = ys
union (x:xs) ys 
  | member x ys = union xs ys
  | otherwise   = x : union xs ys

intersection:: [Int] -> [Int] -> [Int]
intersection [] _ = []
intersection (x: xs) ys
   | member x ys = x : intersection xs ys
   | otherwise   = intersection xs ys

difference:: [Int] -> [Int] -> [Int]
difference [] _ = []
difference (x: xs) ys
  | member x ys = difference xs ys
  | otherwise = x : difference xs ys

insert:: Int -> [Int] -> [Int]
insert w [] = [w]
insert w (x: xs)
  | w < x   = w : (x:xs)
  | otherwise = x: insert w xs

insertionSort :: [Int] -> [Int]
insertionSort [] = []
insertionSort (x: xs) = insert x (insertionSort xs)

binaryToDecimal :: [Int] -> Int
binaryToDecimal xs = sum [x * (2 ^ i)  | (x, i) <- zip (reverse xs) [0..]]
    
toDecimal :: Int -> [Int] -> Int
toDecimal a xs = sum [x * (a ^ i) | (x, i) <- zip (reverse xs) [0..]]
    
toDec::Int -> String -> Int
toDec base s = toDecimal base (map digitToInt s)

-- Same as `toDec` But use a list comprehension

decimal::Int -> String -> Int
decimal base s = toDecimal base [ digitToInt x | x <- s]

firsts::[a] -> [[a]]
firsts xs = [take i xs | (i,_) <- zip [1..] xs]

-- Given two String that represents numbers in binary implement the 'binaryAdd' function
-- DO NOT USE a predefined '+' operation

binaryAdd::String -> String -> String
binaryAdd  = error "Implement it"
