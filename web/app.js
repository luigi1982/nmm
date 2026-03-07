const express = require("express");
const { spawn } = require("child_process");
const readline = require("readline");
const path = require("path");
const fs = require("fs");

const app = express();
app.use(express.json());

const RUST_BIN = "../nmm-rust/target/release/nmm.exe";

console.log("RUST_BIN =", RUST_BIN);
console.log("exists? =", fs.existsSync(RUST_BIN));

// Keep ONE running game here (single-session version)
let rustProc = null;
let lastOutput = "";
let waitingForMove = false;
let move_history = [];
let num_send_moves = 0;

function appendOutput(s) {
  lastOutput += s;
  if (lastOutput.length > 200_000) lastOutput = lastOutput.slice(-200_000);
}

// Serve frontend
app.use(express.static(path.join(__dirname, "frontend")));

app.get("/api/start_game", (req, res) => {
  // Prevent starting multiple games accidentally
  if (rustProc && !rustProc.killed) {
    return res.json({ ok: true, alreadyRunning: true });
  }

  console.log("Starting Rust process...");
  rustProc = spawn(RUST_BIN, ["1", "0", "1000", "1000"], {
    stdio: ["pipe", "pipe", "pipe"],
    env: process.env,
  });

  rustProc.stdout.setEncoding("utf8");
  rustProc.stderr.setEncoding("utf8");

  const rl = readline.createInterface({ input: rustProc.stdout });

  rl.on("line", (line) => {
    console.log("[rust]", line);
    appendOutput(line + "\n");

    // Example: detect when input is needed
    if (line.toLowerCase().includes("place a piece")) {
      waitingForMove = true;
    }

    // Parse placement event
    const m_place = line.match(/^player\s+(-?\d+)\s+placed stone on field\s+(\d+)\s*$/i);
    const m_move = line.match(/^player\s+(-?\d+)\s+moved stone from field\s+(\d+)\s+to\s+(\d+)*$/i);
    const m_player_remove = line.match(/^Remove one of your opponents pieces!*$/i)
    const m_remove = line.match(/^player\s+(-?\d+)\s+removes:\s+(\d+)*$/i);
    const m_failed = line.match(/^not a valid move!$/i);
    const too_few_pieces = line.match(/^match ended after \d+ plies with score \d-\d*$/i);
    const no_moves = line.match(/^gameover player (-?\d+) ran out of moves*$/i);

    if (m_place) {

      const player = Number(m_place[1]);
      const field = Number(m_place[2]);
      console.log(`Move - ${player} placed stone on field ${field}`);
      move_history.push([player, field, -1]);

    } else if (m_move) {

      const player = Number(m_move[1]);
      const from_field = Number(m_move[2]);
      const to_field = Number(m_move[3]);
      console.log(`Move - ${player} moved stone from field ${from_field} to ${to_field}`);
      move_history.push([player, from_field, to_field]);

    } else if (m_player_remove) {

      move_history.push([1, 100, -1]);

    } else if (m_remove) {
      
      const player = Number(m_remove[1]);
      const field = Number(m_remove[2]);
      console.log(`Move - ${player} removed stone on field ${field}`);
      move_history.push([10, field, -1]);

    } else if (m_failed) {

      move_history.push([0, 0, -1]);

    } else if (too_few_pieces || too_few_pieces) {
      move_history.push([1000, -1, -1])
    }
  });

  rustProc.stderr.on("data", (chunk) => {
    appendOutput("[stderr] " + chunk + "\n");
  });

  rustProc.on("exit", (code) => {
    appendOutput(`\n[Rust exited with code ${code}]\n`);
    waitingForMove = false;
    rustProc = null;
  });

  // IMPORTANT: always respond to the HTTP request
  res.json({ ok: true, started: true });
});

app.post("/api/move", (req, res) => {
  if (!rustProc || rustProc.killed) {
    return res.status(400).json({ ok: false, error: "No running game. Call /api/start_game first." });
  }

  const move = req.body.move;


  // Adjust validation for your stage rules as needed
  //if (!Number.isInteger(n) || n < 0 || n > 23) {
  //  return res.status(400).json({ ok: false, error: "move must be an integer 1..24" });
  //}

  // Send like user typing + pressing Enter
  rustProc.stdin.write(move + "\n");
  waitingForMove = false;

  res.json({ ok: true });
});

app.get("/api/game_output", (req, res) => {

  moves = move_history.slice(num_send_moves, move_history.length);
  num_send_moves += moves.length;
  res.json({ moves , waitingForMove });

});

app.listen(3000, () => console.log("Backend running at http://localhost:3000"));