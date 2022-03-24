const { tetris } = wasm_bindgen;

let counter = 2

const start_ai = async () => {
    await wasm_bindgen('/static/js/pkg/tetris_ai_bg.wasm')
    setTimeout(async () => {
        let game = globalThis.game.games;
        let matrix = JSON.stringify(game.current.data.game.droppedMatrix)
        let tetrium = null
        // game.current.data.piece.matrix = [
        //     [6, 0],
        //     [6, 0],
        //     [6, 6]
        // ]
        let tetrium_identifier = game.current.data.piece.matrix[0]
        let tetris_id;

        if (tetrium_identifier.includes(1)) {
            tetrium = "TETRIS_STAIR_REVERSE"
            tetris_id = 1;
        } else if (tetrium_identifier.includes(2)) {
            tetrium = "TETRIS_STAIR"
            tetris_id = 2;
        } else if (tetrium_identifier.includes(3)) {
            tetrium = "TETRIS_SQUARE"
            tetris_id = 3;
        } else if (tetrium_identifier.includes(4)) {
            tetrium = "TETRIS_T"
            tetris_id = 4;
        } else if (tetrium_identifier.includes(5)) {
            tetrium = "TETRIS_LINE"
            tetris_id = 5;
        } else if (tetrium_identifier.includes(6)) {
            tetrium = "TETRIS_L"
            tetris_id = 6;
        } else if (tetrium_identifier.includes(7)) {
            tetrium = "TETRIS_L_REVERSE"
            tetris_id = 7;
        }

        let order = JSON.parse(tetris(matrix, tetrium))
        console.log(order)

        let bounds = order.bounds.map(element => element.map(item => item == 1 ? tetris_id : 0))

        let target_x = order.x + 1
        console.log(target_x)

        game.current.data.piece.x = target_x
        game.current.data.piece.matrix = bounds

        setTimeout(() => {
            window.dispatchEvent(new KeyboardEvent("keydown", {
                "code": "Space"
            }))
        }, 140)
        setTimeout(() => {
            window.dispatchEvent(new KeyboardEvent("keyup", {
                "code": "Space"
            }))
        }, 280)

        await start_ai()
    }, 1000)
}