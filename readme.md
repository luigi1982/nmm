# Play the Game inside the Webbrowser

http://212.227.91.135:3000/


# Nine Men's Morris — Terminal AI (Rust)

A terminal-based implementation of **Nine Men's Morris** featuring AI players built with classical game-tree search techniques.

The project implements:

- Alpha–Beta pruning
- Principal Variation Search (PVS)
- Heuristic evaluation functions for leaf nodes

The game is played entirely in the terminal.

---

## Running the Program

The executable requires **four command-line arguments**:

```bash
nmm.exe <human_a> <human_b> <time_a> <time_b>
```

human_a:	1 = Player A is human, 0 = AI <br>
human_b:	1 = Player B is human, 0 = AI <br>
time_a:	Thinking time for Player A (milliseconds), only relevant if Player A is AI <br>
time_b:	Thinking time for Player B (milliseconds), only relevant if Player B is AI

---

## Playing the Game

- first stage: type the field as a single number and enter return 
- second and third stage: type the field of the stone you wish to move and where it should go seperated by a space and enter return 
- captures: if you closed a mill you are asked to remove a piece, type the number of the field on which the piece is located you wish to remove and enter return