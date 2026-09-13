module Darts (score) where
import GHC.Float

distance :: Float -> Float -> Float
distance x y = sqrtFloat (x * x + y * y)

score :: Float -> Float -> Int
score x y
    | dist <= 1 = 10
    | dist <= 5 = 5
    | dist <= 10 = 1
    | otherwise = 0
    where dist = distance x y
