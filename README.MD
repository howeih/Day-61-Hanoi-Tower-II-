Day 61: Hanoi Tower (II)
<br>
In an optimal game (with the least number of moves), find how many moves have been played given towers’ configuration. And conversely, given towers’ configuration find how many moves have been played.
<br>
The key observation here is the number of moves in optimal game. For 1 disk, it’s 1 move. For n disks, it’s 2^n — 1 moves. This can be proved by induction.
<br>
Solving n-disk game, transfer n-1 disks to auxiliary rod, move the largest disk and transfer n-1 disks from auxiliary to target rod. Following the induction.
<br>
![Alt text](hanoiII.png?raw=true "hanoi")
The proof also gives a direct solution to the problem. In n-disk game, the largest disk is transferred exactly on move 2^(n-1)! After a single check on moves >= 2 ** (n-1) we immediately know where the largest disk is. And we can proceed recursively on a n-1-disk game.
<br>
result:
```

 0 moves -- [4, 3, 2, 1] [] []
 1 moves -- [4, 3, 2] [1] []
 2 moves -- [4, 3] [1] [2]
 3 moves -- [4, 3] [] [2, 1]
 4 moves -- [4] [3] [2, 1]
 5 moves -- [4, 1] [3] [2]
 6 moves -- [4, 1] [3, 2] []
 7 moves -- [4] [3, 2, 1] []
 8 moves -- [] [3, 2, 1] [4]
 9 moves -- [] [3, 2] [4, 1]
10 moves -- [2] [3] [4, 1]
11 moves -- [2, 1] [3] [4]
12 moves -- [2, 1] [] [4, 3]
13 moves -- [2] [1] [4, 3]
14 moves -- [] [1] [4, 3, 2]
15 moves -- [] [] [4, 3, 2, 1]

```