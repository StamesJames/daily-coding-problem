module Haskell.Utils.Queue(Queue, emptyQueue, enqueue, dequeue, peek, queCount) where

import qualified Data.Sequence as Seq

newtype Queue a = Queue (Seq.Seq a)

instance Functor Queue where
  fmap f (Queue q)= Queue (fmap f q)


emptyQueue = Queue Seq.empty
enqueue x (Queue q) = Queue $ q Seq.|> x 

dequeue (Queue q) = 
        case Seq.viewl q of 
            a Seq.:< newQ -> Just (a, Queue newQ)
            Seq.EmptyL -> Nothing

peek (Queue q) = 
    case Seq.viewl q of 
        a Seq.:< _ -> Just (a, Queue q)
        Seq.EmptyL -> Nothing

queCount (Queue q) = length q