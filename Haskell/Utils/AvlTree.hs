module Haskell.Utils.AvlTree(AvlTree(Leaf, Node), insert) where

data AvlTree a = Leaf | Node {myElem::a, left::AvlTree a, right::AvlTree a, myHeight::Integer, myCount::Integer} 

instance Show a => Show (AvlTree a) where
  show Leaf = "()"
  show Node {myElem=_myElem, left=_left, right=_right, myHeight=_myHeight, myCount=_myCount} = "(" ++ show _myElem ++ " " ++ show _left ++ " " ++ show _right ++ ")"
    

height Leaf = 0
height Node {myElem=_myElem, left=_left, right=_right, myHeight=_myHeight, myCount=_myCount} = _myHeight

count Leaf = 0
count Node {myElem=_myElem, left=_left, right=_right, myHeight=_myHeight, myCount=_myCount} = _myCount

isBalanced Leaf = True 
isBalanced Node {myElem=_elem, left=_left, right=_right, myHeight=_height, myCount=_count} = height _right <= height _left + 1 && height _left <= height _right + 1 && isBalanced _left && isBalanced _right

rotateRight Leaf = Leaf
rotateRight Node {myElem=x, left=Node {myElem=y, left=a, right=b, myHeight=_height_l, myCount=_count_l}, right=c, myHeight=_height_n, myCount=_count_n} = 
    Node {myElem=y, left=a, right=Node {myElem=x, left=b, right=c, myHeight=newRightHeight, myCount=newRightCount}, myHeight=newRootHeight, myCount=newRootCount}
    where
        newRightHeight = max (height b) (height c) + 1
        newRightCount = count b + count c + 1
        newRootHeight = max newRightCount (height a) + 1 
        newRootCount = newRightCount + count a + 1
rotateRight _ = Leaf

rotateLeft Leaf = Leaf
rotateLeft Node {myElem=y, left=a, right=Node {myElem=x, left=b, right=c, myHeight=_myHeight_r, myCount=_myCount_r}, myHeight=_myHeight_n, myCount=_myCount_n} = 
    Node {myElem=x, left=Node {myElem=y, left=a, right=b, myHeight=newLeftHeight, myCount=newLeftHeight}, right=c, myHeight=newRootHeight, myCount=newRootCount}
    where
        newLeftHeight = max (height a) (height b) + 1
        newLeftCount = count a + count b + 1
        newRootHeight = max newLeftHeight (height a) + 1
        newRootCount = newLeftCount + count a + 1
rotateLeft _ = Leaf

balance Leaf = Leaf
balance tree@Node {myElem=_myElem_n, left=_left_n, right=_right_n, myHeight=_myHeight_n, myCount=_myCount_n}
    | height _left_n > height _right_n + 1 =
        case _left_n of 
            Leaf -> Leaf
            Node {myElem=_myElem_l, left=_left_l, right=_right_l, myHeight=_myHeight_l, myCount=_myCount_l} -> 
                if height _left_l >= height _right_l then
                    rotateRight tree
                else
                    rotateRight tree{left= rotateLeft _left_n}
    | height _right_n > height _left_n + 1 =
        case _right_n of 
            Leaf -> Leaf
            Node {myElem=_myElem_r, left=_left_r, right=_right_r, myHeight=_myHeight_r, myCount=_myCount_r} -> 
                if height _right_r >= height _left_r then
                    rotateLeft tree
                else
                    rotateLeft tree{right= rotateRight _right_n}
    | otherwise =
        tree

insert x Leaf = (0,True, Node {myElem=x, left=Leaf, right=Leaf, myHeight=1, myCount=1})
insert x tree @ Node {myElem=_myElem, left=_left, right=_right} 
    | x < _myElem = 
        let
            (index, inserted, t) = insert x _left
            newTree = balance tree{left=t, myCount = count _right + count t + 1, myHeight = max (height _right) (height t) + 1} 
        in (index, inserted, newTree)
    | x > _myElem = 
        let
            (i,inserted,t) = insert x _right
            index = count _left + 1 + i
            newTree = balance tree{right=t, myCount = count _left + count t + 1, myHeight = max (height _left) (height t) + 1}
        in (index, inserted, newTree) 
    | otherwise = (count _left, False, tree)



(i1,b1,tree1) = insert 1 Leaf
(i2,b2,tree2) = insert 2 tree1
(i3,b3,tree3) = insert 3 tree2
(i4,b4,tree4) = insert 4 tree3
(i5,b5,tree5) = insert 5 tree4
(i6,b6,tree6) = insert 6 tree5
(i7,b7,tree7) = insert 7 tree6