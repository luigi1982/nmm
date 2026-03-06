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
let moves = [];

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
    const m = line.match(/^player\s+(-?\d+)\s+placed stone on field\s+(\d+)\s*$/i);
    if (m) {
      const player = Number(m[1]);
      const field = Number(m[2]);
      console.log("placement event", { player, field });
      moves.push([player, field]);
    }

    if (line.match(/^not a valid move!$/i)) {
      moves.push([0, 0]);
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
  const move = (moves.length > 0) ? moves[moves.length - 1] : null;
  res.json({ move, waitingForMove });
});

app.listen(3000, () => console.log("Backend running at http://localhost:3000"));