fetch('./checkers_test.wasm').then(response =>
    response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes)).then(results => {
    console.log("Loaded wasm module")
    instance = results.instance
    console.log("instance", instance)
    let white = 2
    let black = 1
    let crowned_white = 6
    let crowned_black = 5
    console.log("Calling offset")
    let offset = instance.exports.offsetForPosition(3, 4)
    console.log("Offset for 3,4 is ", offset)
    console.debug("White is white?", instance.exports.isWhite(white))
    console.debug("Black is black?", instance.exports.isBlack(black))
    console.debug("Black is white?", instance.exports.isWhite(black))
    console.debug("crowned white", crowned_white)
    console.debug("Uncrowned white",
        instance.exports.isWhite(instance.exports.withoutCrown(crowned_white)))
    console.debug("crowned black", crowned_black)
    console.debug("Uncrowned black",
        instance.exports.isBlack(instance.exports.withoutCrown(crowned_black)))
    console.debug("Crowned is crowned",
        instance.exports.isCrowned(crowned_white))
    console.debug("Crowned is crowned (b)",
        instance.exports.isCrowned(crowned_black))
})

