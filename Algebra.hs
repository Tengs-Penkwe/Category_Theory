type Action = Int
data State = State ([Action],[Action]) [Action]
         deriving (Eq, Show)
data Result = EndOfGame Double State
            | ContinueGame State
         deriving (Eq, Show)


maybe2either :: Maybe a -> Either () a
maybe2either Nothing = Left ()
maybe2either (Just a) = Right a

either2maybe :: Either () a -> Maybe a
either2maybe (Left ()) = Nothing
either2maybe (Right a) = Just a
