let express = require("express")
let fs = require("fs")

let app = express()

app.get("/i/:script?", (req, res) => {
    let str = fs.readFileSync("./runner.lua", "utf-8")
    str = str.replace("[SCRIPT]", req.params.script || "script.lua")
    res.send(str)
})

let promise = (_x) => void 0

app.get("/script/:script", async (req, res) => {
    console.log("requested", req.params.script)
    let text = await new Promise(res => promise = res)
    console.log("responding", req.params.script)
    res.send(text)
})

fs.watch("script.lua", () => {
    promise(fs.readFileSync("script.lua", "utf-8"))
})

app.listen(9999, () => {
    console.log(`loadstring(http.get("http://localhost:9999/i/").readAll())()`)
})