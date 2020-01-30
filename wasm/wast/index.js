fetch('./checkers.wasm').then(resp =>
    resp.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes, {
    events: {
        piecemoved: (fX, fY, tX, tY) => {
            console.log("A piece moved from (" + fX + "," + fY +
                ") to (" + tX + "," + tY + ")")
        },
        piececrowned: (x, y) => {
            console.log("A piece was crowned at (" + x + "," + y + ")")
        }
    },
})).then(results => {
    instance = results.instance

    instance.exports.initBoard()
    console.log("At start, turn owner is " + instance.exports.getTurnOwner())
    instance.exports.move(0, 5, 0, 4) // B
    instance.exports.move(1, 0, 1, 1) // W
    instance.exports.move(0, 4, 0, 3) // B
    instance.exports.move(1, 1, 1, 0) // W
    instance.exports.move(0, 3, 0, 2) // B
    instance.exports.move(1, 0, 1, 1) // W
    instance.exports.move(0, 2, 0, 0) // B
    instance.exports.move(1, 1, 1, 0) // W
    // B - move the crowned piece out
    let res = instance.exports.move(0, 0, 0, 2)
    document.getElementById("container").innerText = res;
    console.log("At end, turn owner is " + instance.exports.getTurnOwner())
}).catch(console.error)
