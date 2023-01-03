(function () {
    const vscode = acquireVsCodeApi();

    document.getElementById("play-music-button").addEventListener('click', playMusic);

    let system = undefined;
    let game = undefined;

    function playMusic() {
        window.addEventListener("message", (msg) => {
            if (msg.data.type !== "getMmlScriptResponse") {
                throw new Error(`unexpected message: ${msg.data.type}`);
            }

            const mml=msg.data.value;
            if (!mml) {
                return;
            }
            try {
                game.command(system, "parseScript", new TextEncoder().encode(mml));
                game.command(system, "playAudio", new Uint8Array(0));
            } catch (e) {
                vscode.postMessage({type: "error", error: e.message});
            }
        }, {once: true});
        vscode.postMessage({type: "getMmlScript"});
    }

    window.addEventListener("message", async (msg) => {
        if (msg.data.type !== "getWasmUriResponse") {
            throw new Error(`unexpected message: ${msg.data.type}`);
        }

        game = await Pagurus.Game.load(msg.data.value);
        system = await Pagurus.System.create(game.memory);

        game.initialize(system);
        while (true) {
            const event = await system.nextEvent();
            try {
                if (!game.handleEvent(system, event)) {
                    break;
                }
            } catch (e) {
                console.warn(e);
                vscode.postMessage({type: "error", error: e.message});
            }
        }
    }, {once: true});
    vscode.postMessage({type: "getWasmUri"});
}());
