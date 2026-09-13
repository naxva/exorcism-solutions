module Bob (responseFor) where
import Data.Char

responseFor :: String -> String
responseFor input
    | null text = "Fine. Be that way!"
    | isYelling && isQuestion = "Calm down, I know what I'm doing!"
    | isYelling = "Whoa, chill out!"
    | isQuestion = "Sure."
    | otherwise = "Whatever."
    where
        text = filter (not . isSpace) input
        letters = filter isLetter text
        isYelling = all isUpper letters && any isUpper letters
        isQuestion = last text == '?'
