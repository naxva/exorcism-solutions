module DNA (nucleotideCounts, Nucleotide (..)) where

import Data.Map (Map)
import qualified Data.Map as M

data Nucleotide = A | C | G | T deriving (Eq, Ord, Show)

toNucleotide :: Char -> Either String Nucleotide
toNucleotide 'A' = Right A
toNucleotide 'C' = Right C
toNucleotide 'G' = Right G
toNucleotide 'T' = Right T
toNucleotide c = Left ("invalid nucleotide: " ++ [c])

nucleotideCounts :: String -> Either String (Map Nucleotide Int)
nucleotideCounts xs = do
  ns <- traverse toNucleotide xs

  pure (M.unionWith (+) zeros (M.fromListWith (+) [(n, 1) | n <- ns]))
  where
    zeros = M.fromList [(n, 0) | n <- [A, C, G, T]]
