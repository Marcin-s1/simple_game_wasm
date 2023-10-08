import * as wasm from "wasm-game-of-life";
import { CheckersView, CheckerCell, GameState2 } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/tic_tac_toe_bg";

const GRID_COLOR = "#CCCCCC";
const GAME_NAME = "Checker";

const canvas = document.getElementById("checkers-canvas");
const ctx = canvas.getContext('2d');

const checkersView = CheckersView.new(300, 300);

const game_selector = document.getElementById("game_type");

const width = checkersView.get_width();
const height = checkersView.get_height();
const cell_size = width / 8;
let state = GameState2.NotStarted;
let color = CheckerCell.White;

let selected_field = null;
let possible_field = null;

const isCheckerGame = () => {
    return document.getElementById("game_type").value == GAME_NAME;
}

game_selector.addEventListener("change", event => {
    if (isCheckerGame() === false) {
      canvas.style.display = "none";
    }
    else {
      canvas.style.display = "block";
    }
});

const start_button = document.getElementById("start-button");
start_button.addEventListener("click", event => {
    if (isCheckerGame() === false) {
        return;
    }

    let player1_name = document.getElementById("player1").value;
    let player2_name = document.getElementById("player2").value;
    checkersView.join_player1(player1_name);
    checkersView.join_player2(player2_name);
    state = checkersView.start();
    console.error("State: " + state);
    if ((state === GameState2.MovePlayer1) || (state === GameState2.MovePlayer2))
    {
      document.getElementById("state").value = "Playing";
    }
});


canvas.addEventListener("mousemove", event => {
    if (isCheckerGame() === false) {
        return;
    }


    possible_field = {
        x : Math.trunc(event.offsetY / cell_size),
        y : Math.trunc(event.offsetX / cell_size)
    }
});

canvas.addEventListener("mouseleave", event => {
    possible_field = null;
});

canvas.addEventListener("click", event => {
    if (isCheckerGame() === false) {
        return;
    }

    const x = Math.trunc(event.offsetY / cell_size);
    const y = Math.trunc(event.offsetX / cell_size);

    const cellsPtr = checkersView.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, 64);
    
    if (selected_field == null)
    {
        let temp_color = cells[y + x * 8];
        if (temp_color == CheckerCell.White || temp_color == CheckerCell.WhiteKing)
        {
            temp_color = CheckerCell.White;
        }
        else if (temp_color == CheckerCell.Black || temp_color == CheckerCell.BlackKing) {
            temp_color = CheckerCell.Black;
        }
    
        
        
        if (color != temp_color) {
            selected_field == null;
            return;
        }
        
        selected_field = {
            x:  Math.trunc(event.offsetY / cell_size),
            y:  Math.trunc(event.offsetX / cell_size),
        };
        return;
    }

    if ((x == selected_field.x) && (y == selected_field.y))
    {
        selected_field = null;
        return;
    }

    console.error(
        "selected_field.x = " + selected_field.x + " selected_field.y = " + selected_field.y + " x = " + x + " y = " + y);
    console.error("State1: " + state);
    let new_state = state;
    if (state === GameState2.MovePlayer1)
    {
      new_state = checkersView.move_player1(selected_field.x, selected_field.y, x, y);
    }
    else if (state === GameState2.MovePlayer2)
    {
      new_state = checkersView.move_player2(selected_field.x, selected_field.y, x, y);
    }

    if (new_state != state) {
        selected_field = null;
        state = new_state;
    }

    if (state === GameState2.MovePlayer1)
    {
        color = CheckerCell.White;
    }
    else if (state === GameState2.MovePlayer2)
    {
        color = CheckerCell.Black;
    }

    if (state === GameState2.Finished)
    {
      let winner = checkersView.get_winner();
      document.getElementById("state").value = "Finished - Won Player: " + winner;
    }
    console.error("State2: " + state);
    

    drawField();
    drawCells();
});


const drawPiece = (i, j, color) => {
    
    ctx.beginPath();
    ctx.fillStyle = color;
    let centerX = i % 8 * cell_size + cell_size / 2;
    let centerY = j % 8 * cell_size + cell_size / 2;
    const radius = cell_size / 2 - 5;
    ctx.arc(centerX, centerY, radius, 0, 2 * Math.PI, false);
    ctx.fill();
    ctx.closePath();

    ctx.stroke();
}

const selectFeild = () => {
    if (possible_field === null) {
        return;
    }
    
    ctx.beginPath();
    
    let beginX = possible_field.x % 8 * cell_size;
    let beginY = possible_field.y % 8 * cell_size;

    const cellsPtr = checkersView.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, 64);
    
    let temp_color;
    if (color == CheckerCell.White || color == CheckerCell.WhiteKing)
    {
        temp_color = CheckerCell.White;
    }
    else if (color == CheckerCell.Black || color == CheckerCell.BlackKing) {
        temp_color = CheckerCell.Black;
    }

    if (cells[possible_field.y + possible_field.x * 8] === temp_color) {
       // alert("test1");
        ctx.fillStyle = "orange";
        ctx.globalAlpha = 0.7;
    } else {
        ctx.fillStyle = "orange";
        ctx.globalAlpha = 0.1;
    }

    

    

    ctx.rect(beginY, beginX, cell_size, cell_size);
    ctx.fill();
    ctx.globalAlpha = 1;

    ctx.stroke();
}

const drawField = () => {
    ctx.beginPath();

    const cellsPtr = checkersView.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, 64);

    ctx.rect(0, 0, width, height);
    ctx.fillStyle = "white";
    ctx.fill();
  
    for (let i = 0; i * cell_size < width; i++)
    {
        ctx.moveTo(i * cell_size, 0);
        ctx.lineTo(i * cell_size, height);
    }

    for (let j = 0; j * cell_size < width; j++)
    {
        ctx.moveTo(0, j * cell_size);
        ctx.lineTo(width, j * cell_size);
    }

    

    
  
    ctx.stroke();

    for (let i = 0; i <  8; i++)
        
        for (let j = 0; j <  8; j++)
        {
            ctx.beginPath();
            ctx.rect(i * cell_size, j * cell_size, cell_size, cell_size);
            //console.error("rect: [" + i * cell_size + ", " + j * cell_size + ", " + cell_size + ", " + cell_size + "]");
            let index = i*8 + j;
           // console.error("i = " + i + " j= " + j + " i*8 + j= " + index);
            if ((i*8 + j + i % 2) % 2 == 1)
            {
               // console.error("black");
                ctx.fillStyle = "green";
            }
            else {
                //console.error("white");
                ctx.fillStyle = "white";
            }
            if (selected_field != null)
            {
                if (i === selected_field.y && j === selected_field.x) {
                    let index = i + j * 8;
                    if (cells[index] === color) {
                        ctx.fillStyle = "yellow";
                    }
                    
                }
            }
            
            
            ctx.fill();
            ctx.stroke();
        }
}


const drawCells = () => {
    const cellsPtr = checkersView.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, 64);

    for (let i = 0; i < 64; i++)
    {
      // console.error("drawCells: " + " " + i +  " " + cells[i]);
    }
    
    


    for (let i = 0; i < 64; i++)
    {
        //console.error("drawCells: " + Math.floor(i / 8) + ", " + i % 8 + " - value: " + cells[i]);
        if (cells[i] === CheckerCell.Black)
        {
           // console.error("drawCells: " + Math.floor(i / 8) + ", " + i % 8);
            drawPiece(i % 8, Math.floor(i / 8), "black");
        }
        else if (cells[i] === CheckerCell.White)
        {
            drawPiece(i % 8, Math.floor(i / 8), "white");
        }
        else if (cells[i] === CheckerCell.WhiteKing)
        {
            drawPiece(i % 8, Math.floor(i / 8), "Yellow");
        }
        else if (cells[i] === CheckerCell.BlackKing)
        {
            drawPiece(i % 8, Math.floor(i / 8), "Gray");
        }
        
    }
}

drawField();
drawCells();
//drawPiece(0, 0, "black");
//drawPiece(0, 1, "black");
const renderLoop = () => {
    
    
    drawField();
    selectFeild();
    drawCells();
    
    
    
  
    requestAnimationFrame(renderLoop);
  };
  
  requestAnimationFrame(renderLoop);