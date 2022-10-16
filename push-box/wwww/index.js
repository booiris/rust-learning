import init from "../pkg/push_box.js";
import { Game, Cell, wasm } from "../pkg/push_box.js";

const run = () => {

    const CELL_SIZE = 50; // px
    const LINE_SIZE = 4
    const LINE_COLOR = "#EEEEEE";
    const BACK_COLOR = "#022222";

    const game = Game.new();
    const height = game.height();
    const width = game.width();

    const person = new Image();
    person.src = "./img/spy.svg"
    const key = new Image();
    key.src = "./img/key.svg"
    const aim = new Image();
    aim.src = "./img/strongbox.svg"
    const wall = new Image();
    wall.src = "./img/brick-wall.svg"

    const canvas = document.getElementById("game-canvas");
    canvas.height = (CELL_SIZE + LINE_SIZE) * height + LINE_SIZE;
    canvas.width = (CELL_SIZE + LINE_SIZE) * width + LINE_SIZE;
    let trueHeight = canvas.height
    let trueWidth = canvas.width


    const ctx = canvas.getContext("2d");
    ctx.canvas.style.imageRendering = 'auto'

    let devicePixelRatio = window.devicePixelRatio || 1
    let backingStoreRatio = ctx.webkitBackingStorePixelRatio || ctx.mozBackingStorePixelRatio || ctx.msBackingStorePixelRatio ||
        ctx.oBackingStorePixelRatio || ctx.backingStorePixelRatio || 1
    let ratio = devicePixelRatio / backingStoreRatio
    canvas.width = canvas.width * ratio
    canvas.height = canvas.height * ratio
    ctx.scale(ratio, ratio);

    canvas.style.width = `${trueWidth}px`
    canvas.style.height = `${trueHeight}px`

    let iter = 0

    const renderLoop = () => {

        while (true) {
            if (game.game_over()) {
                if (iter % 500 == 0) {
                    console.log(iter + 2 + " begin!")
                }

                iter += 1;
                game.restart()
            }

            if (iter > 5000) {
                console.log("end!!!!")
                break
            }

            game.tick();

            if (iter % 500 == 0) {
                drawGrid();
                drawMap();

                game.set_is_log(false)
                setTimeout(() => {
                    requestAnimationFrame(renderLoop);
                }, 100);
                break
            } else {
                game.set_is_log(false)
            }
        }

    };

    const drawGrid = () => {
        ctx.fillStyle = BACK_COLOR;
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        ctx.beginPath();
        ctx.lineWidth = LINE_SIZE;
        ctx.strokeStyle = LINE_COLOR;

        // Vertical lines.
        for (let i = 0; i <= width; i++) {
            ctx.moveTo(i * (CELL_SIZE + LINE_SIZE) + LINE_SIZE / 2, 0);
            ctx.lineTo(i * (CELL_SIZE + LINE_SIZE) + LINE_SIZE / 2, (CELL_SIZE + LINE_SIZE) * height + LINE_SIZE);
        }

        // Horizontal lines.
        for (let j = 0; j <= height; j++) {
            ctx.moveTo(0, j * (CELL_SIZE + LINE_SIZE) + LINE_SIZE / 2);
            ctx.lineTo((CELL_SIZE + LINE_SIZE) * width + LINE_SIZE, j * (CELL_SIZE + LINE_SIZE) + LINE_SIZE / 2);
        }

        ctx.stroke();
    };

    const getIndex = (row, column) => {
        return row * width + column;
    };

    const drawItem = (img, y, x) => {
        ctx.drawImage(img, 7 + (CELL_SIZE + LINE_SIZE) * x, 7 + (CELL_SIZE + LINE_SIZE) * y, CELL_SIZE - 5, CELL_SIZE - 5)
    }


    const drawMap = () => {
        const roomPtr = game.room();
        const room = new Uint8Array(wasm.memory.buffer, roomPtr, width * height);

        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);
                if (room[idx] == Cell.Empty) {
                    continue;
                } else if (room[idx] == Cell.Person) {
                    drawItem(person, row, col)
                } else if (room[idx] == Cell.Key) {
                    drawItem(key, row, col)
                } else if (room[idx] == Cell.Aim) {
                    drawItem(aim, row, col)
                } else if (room[idx] == Cell.Wall) {
                    drawItem(wall, row, col)
                }
            }
        }
    }

    drawGrid();
    drawMap();
    renderLoop();

};

init().then(run);
