const positions = [
    [25, 25], [200, 25], [375, 25],
    [25, 200], [375, 200],
    [25, 375], [200, 375], [375, 375],
    [75, 75], [200, 75], [325, 75],
    [75, 200], [325, 200],
    [75, 325], [200, 325], [325, 325],
    [125, 125], [200, 125], [275, 125],
    [125, 200], [275, 200],
    [125, 275], [200, 275], [275, 275]
];

const numbers = [
    7, 0, 1,
    6, 2,
    5, 4, 3,
    15, 8, 9, 
    14, 10, 
    13, 12, 11,
    23, 16, 17, 
    22, 18, 
    21, 20, 19
];


document.addEventListener("DOMContentLoaded", () => {

    const svg = document.getElementById("board");
    var circles = [];
    var pieces = [];
    var moving = false;
    var moving_piece = null;
    var is_player_turn = true;
    var player_send_move = false;
    var start_position = null;
    var start_num = null;
    var target_position = null;
    var target_num = null;

    // create all of the positions
    for (let i=0; i < positions.length; i++) {
        circles.push(create_circle(positions[i], numbers[i]));
    }

    // draw them on the board
    for (let i=0; i < circles.length; i++) {
        svg.appendChild(circles[i]);
    }

    // start the game
    startGame();

    // add functionaliy when clicked
    document.querySelectorAll(".board-point").forEach(point => {
        point.addEventListener("click", async (e) => {

            if (is_player_turn && !player_send_move) {

                // convert position to integer
                const num = Number(e.target.dataset.number);
                const pos = e.target.dataset.position;
                const str_positions = pos.split(",");
                target_position = str_positions.map(function(x) {
                    return Number(x);
                });
                target_num = num;

                if (moving) {

                    await sendMove(moving_piece.num, target_num);

                } else {

                    await sendMove(num);

                }

                player_send_move = true;

            }
        })
    });

    setInterval( async function () {

        const move = await refreshOutput();

        if (is_player_turn) {

            // check if the player already send a move
            // else idle
            if (player_send_move) {

                // check if the send move is valid
                if (move && move[0] == 1) {

                    if (moving) {

                        // move piece on frontend
                        console.log("Moving piece!");
                        moving_piece.num = target_num;
                        moving_piece.position = target_position;
                        moving_piece.circle.setAttribute("cx", target_position[0]);
                        moving_piece.circle.setAttribute("cy", target_position[1]);

                        moving_piece.circle.setAttribute("stroke", "black");
                        moving_piece.circle.setAttribute("stroke-width", 1);
                        moving_piece = null;
                        moving = false;

                    } else {

                        // draw piece on frontend
                        const new_piece = new Stone('White', svg, target_position, target_num);
                        new_piece.circle.addEventListener("click", (e) => {
                            moving = true;

                            // if another piece was clicked before, reset
                            if (moving_piece) {
                                moving_piece.circle.setAttribute("stroke", "black");
                                moving_piece.circle.setAttribute("stroke-width", 1);
                            }

                            // change stroke color to mark moving piece
                            new_piece.circle.setAttribute("stroke", "red");
                            new_piece.circle.setAttribute("stroke-width", 3);

                            moving_piece = new_piece;
                        });
                        new_piece.draw();
                        pieces.push(new_piece);

                    }

                    is_player_turn = false;
                    player_send_move = false;
                    
                } else if (move && move[0] == 0) {

                    // move is not valid
                    // player needs to send a new move
                    player_send_move = false;
                }
            }

        } else {

            if (move && move[0] == -1) {

                // get the number of the field where stone was placed
                const num = move[1];
                // get the coordinates of the field
                const index = numbers.indexOf(num);
                const pos = positions[index];

                const new_piece = new Stone('Black', svg, pos);
                new_piece.circle.addEventListener("click", (e) => {

                    // here needs to be added logic to remove the enemies piece
                    console.log("removing enemy piece");

                });
                new_piece.draw();
                pieces.push(new_piece);

                is_player_turn = true;
            }
        }
    }, 500);

});

function create_circle(position, number) {

    const x = position[0];
    const y = position[1];

    const circle = document.createElementNS(
        "http://www.w3.org/2000/svg",
        "circle"
    );

    circle.setAttribute("cx", x);
    circle.setAttribute("cy", y);
    circle.setAttribute("r", 8);
    circle.setAttribute("fill", "black");
    circle.setAttribute("stroke", "black");
    circle.setAttribute("class", "board-point");
    circle.dataset.position = position;
    circle.dataset.number = number;

    return circle;
}

// call start game endpoint
async function startGame() {
  const r = await fetch("/api/start_game");
  const data = await r.json();
  console.log(data);
}

// call send move endpoint
async function sendMove(n, target=null) {

    const move = (target) ? String(n) + " " + String(target) : String(n);

    const r = await fetch("/api/move", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ move })
    });
    const data = await r.json();
    console.log(data);
}

// check for moves of the AI
async function refreshOutput() {
  const r = await fetch("/api/game_output");
  const { move, waitingForMove } = await r.json();
  return move;
}

class Stone {

    constructor(color, canvas, position, num) {
        this.color = color;
        this.position = position;
        this.canvas = canvas;
        this.num = num;
        this.circle = document.createElementNS(
            "http://www.w3.org/2000/svg",
            "circle"
        );
    }

    draw() {

        const x = this.position[0];
        const y = this.position[1];

        this.circle.setAttribute("cx", x);
        this.circle.setAttribute("cy", y);
        this.circle.setAttribute("r", 18);
        this.circle.setAttribute("fill", this.color);
        this.circle.setAttribute("stroke", "black");
        this.circle.setAttribute("class", "piece");
        
        this.canvas.appendChild(this.circle);
    }

}