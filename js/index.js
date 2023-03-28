import("../pkg/index.js")
  .then((module) => {
    const { Color, flip_color, evaluate_board, search_with_nega_alpha } =
      module;
    const board = module.Board.new_10x10();
    const humanTurn = Color.Black;
    let lastCoord = { x: -1, y: -1 };
    let gameMsg = "";
    let turn = Color.Black;

    window.board = board;
    window.module = module;

    renderBoard();

    function nextTurn() {
      turn = flip_color(turn);
      renderBoard();

      if (board.is_end()) {
        const white_count = board.count_color(Color.White);
        const black_count = board.count_color(Color.Black);

        if (white_count === black_count) {
          gameMsg = "DRAW!";
        } else if (black_count > white_count) {
          gameMsg = "YOU WIN!";
        } else {
          gameMsg = "CPU WIN!";
        }
        renderBoard();
        return;
      }

      if (!board.has_puttable(turn)) {
        skipStroke += 1;
        gameMsg = `${Color[turn]} pass!`;
        renderBoard();
        setTimeout(() => nextTurn(), 500);
        return;
      } else {
        gameMsg = "";
      }

      if (turn === humanTurn) {
      } else {
        gameMsg = "CPU THINKING...";
        renderBoard();
        setTimeout(() => {
          const { x, y } = search_with_nega_alpha(
            board.clone_for_search(),
            turn,
            6
          );
          board.put(x, y, turn);
          lastCoord = { x, y };
          gameMsg = "";
          nextTurn();
        }, 100);
      }
    }

    function renderBoard() {
      const detailsEl = document.querySelector("#details");
      detailsEl.innerHTML = `
<div>BLACK(YOU): ${board.count_color(Color.Black)}</div>
<div>WHITE(CPU): ${board.count_color(Color.White)}</div>
<div style="opacity: ${gameMsg ? 1 : 0}">${gameMsg || ">GAME MSG<"}</div>
`.trim();

      const el = document.querySelector("#board");
      el.innerHTML = "";

      for (let y = 0; y < board.height; ++y) {
        for (let x = 0; x < board.width; ++x) {
          const tile = board.get(x, y);
          const tileEl = document.createElement("div");
          const stoneEl = document.createElement("div");

          if (lastCoord.x === x && lastCoord.y === y) {
            stoneEl.classList.add("last-coord");
          }

          if (tile === Color.Wall) {
            tileEl.style.visibility = "hidden";
          } else if (tile === Color.Empty) {
            if (board.is_puttable(x, y, turn)) {
              if (turn === Color.White) {
                stoneEl.classList.add("white");
              } else if (turn === Color.Black) {
                stoneEl.classList.add("black");
              }
              stoneEl.classList.add("puttable");

              if (humanTurn === turn) {
                stoneEl.addEventListener("click", () => {
                  board.put(x, y, turn);
                  lastCoord = { x, y };
                  nextTurn();
                });
              }
            }
          } else if (tile === Color.White) {
            stoneEl.classList.add("white");
          } else if (tile === Color.Black) {
            stoneEl.classList.add("black");
          }

          tileEl.appendChild(stoneEl);

          el.appendChild(tileEl);
        }
      }
    }
  })
  .catch(console.error);
