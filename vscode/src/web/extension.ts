// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  const provider = new PlayerViewProvider(context.extensionUri);

  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(
      PlayerViewProvider.viewType,
      provider
    )
  );
}

// This method is called when your extension is deactivated
export function deactivate() {}

class PlayerViewProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = "ffmml.playerView";
  private _view?: vscode.WebviewView;

  constructor(private readonly _extensionUri: vscode.Uri) {}

  public resolveWebviewView(
    webviewView: vscode.WebviewView,
    _context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken
  ) {
    this._view = webviewView;

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [this._extensionUri],
    };

    webviewView.webview.onDidReceiveMessage((msg) => {
      switch (msg.type) {
        case "getWasmUri": {
          const wasmUri = webviewView.webview.asWebviewUri(
            vscode.Uri.joinPath(this._extensionUri, "media", "ffmml.wasm")
          );
          webviewView.webview.postMessage({
            type: "getWasmUriResponse",
            value: wasmUri.toString(),
          });
          break;
        }
        case "getMmlScript": {
          const editor = vscode.window.activeTextEditor;
          if (!editor) {
            webviewView.webview.postMessage({
              type: "getMmlScriptResponse",
              value: undefined,
            });
          } else {
            webviewView.webview.postMessage({
              type: "getMmlScriptResponse",
              value: editor.document.getText(),
            });
          }
          break;
        }
        case "error":
          const errorMessageLines = JSON.parse(msg.error).message.split("\n");
          const errorMessage = `${
            errorMessageLines[0]
          } (at ${errorMessageLines[1].replace(/.*TEXTAREA:/, "")})`;
          vscode.window.showErrorMessage(errorMessage);
          break;
      }
    });
    webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);
  }

  private _getHtmlForWebview(webview: vscode.Webview) {
    const scriptPagurusUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._extensionUri, "media", "pagurus.js")
    );
    const scriptMainUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._extensionUri, "media", "main.js")
    );
    const styleVSCodeUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._extensionUri, "media", "vscode.css")
    );
    const nonce = getNonce();

    return `<!DOCTYPE html>
			<html lang="en">
			<head>
				<meta charset="UTF-8">
        <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource}; script-src 'nonce-${nonce}' 'wasm-unsafe-eval' blob:; connect-src ${webview.cspSource};">
				<meta name="viewport" content="width=device-width, initial-scale=1.0">
				<link href="${styleVSCodeUri}" rel="stylesheet">

				<title>FFMML Player</title>
			</head>
			<body>
        <button id="play-music-button">Play Music</button>
				<script nonce="${nonce}" src="${scriptPagurusUri}"></script>
        <script nonce="${nonce}" src="${scriptMainUri}"></script>
			</body>
			</html>`;
  }
}

function getNonce() {
  let text = "";
  const possible =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}
