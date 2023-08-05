https://adventofcode.com/2022/day/2

**--- Day 2: Rock Paper Scissors ---**

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: **Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock**. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an **encrypted strategy guide** (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your **total score** is the **sum** of your scores for **each round**. The score for a single round is the score for the **shape you selected** (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the **outcome of the round** (0 if you lost, 3 if the round was a draw, and 6 if you won).

*Le vainqueur de tout le tournoi est le joueur avec le score le plus élevé. Votre **score total** est la **somme** de vos scores pour **chaque tour**. Le score d'un seul tour est le score de la **forme que vous avez sélectionnée** (1 pour la pierre, 2 pour le papier et 3 pour les ciseaux) plus le **score du résultat du tour** (0 si vous avez perdu, 3 si le tour était un match nul , et 6 si vous avez gagné).*

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:
```
A Y
B X
C Z
```

`A = ROCK`
`B = PAPER`
`C = SCISSORS`

`X => ROCK`
`Y => PAPER`
`Z => SCISSORS`

Score pour **un round**
- 1 si ROCK
- 2 si PAPER
- 3 si SCISSORS

Plus **score résultat du tour**
- 0 si échec
- 3 si match nul
- 6 si réussite


| Round | Opponent going to play | I should to play in response | Result                        |
| ----- | ---------------------- | ---------------------------- | ----------------------------- |
| #1    | A                      | Y                            | {PAPER + WIN} 2+6 = **8**     |
| #2    | B                      | X                            | {ROCK + FAIL} 1+0 = **1**     |
| #3    | C                      | Z                            | {SCISSORS + DRAW} 3+3 = **6** |

This strategy guide predicts and recommends the following:

- In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
- In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of **1** (1 + 0).
- The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = **6**.

In this example, if you were to follow the strategy guide, you would get a total score of **15** (**8 + 1 + 6**).

**What would your total score be if everything goes exactly according to your strategy guide?**

To begin, get your puzzle input.

https://adventofcode.com/2022/day/2/input


Your puzzle answer was 10595.

The first half of this puzzle is complete! It provides one gold star: *


https://adventofcode.com/2022/day/2#part2


**--- Part Two ---**

The Elf finishes helping with the tent and sneaks back over to you. "An  yway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

- In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = **4**.
- In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = **1**.
- In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = **7**.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of **12**.

Following the Elf's instructions for the second column, what would your **total score be if everything goes exactly according to your strategy guide?**

