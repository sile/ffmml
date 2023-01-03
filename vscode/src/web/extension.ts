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

  context.subscriptions.push(
    vscode.commands.registerCommand("ffmml.playMusic", () => {
      provider.playMusic();
    })
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
    context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken
  ) {
    this._view = webviewView;

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [this._extensionUri],
    };

    webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

    webviewView.webview.onDidReceiveMessage((data) => {
      // switch (data.type) {
      //   case "colorSelected": {
      //     vscode.window.activeTextEditor?.insertSnippet(
      //       new vscode.SnippetString(`#${data.value}`)
      //     );
      //     break;
      //   }
      // }
    });
  }

  public playMusic() {
    if (this._view) {
      this._view.show(true);
      this._view.webview.postMessage({ type: "playMusic" });
    }
  }

  private _getHtmlForWebview(webview: vscode.Webview) {
    const scriptUri = webview.asWebviewUri(
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
<!--
				<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource}; script-src 'nonce-${nonce}';">
				<meta name="viewport" content="width=device-width, initial-scale=1.0">
-->
				<link href="${styleVSCodeUri}" rel="stylesheet">

				<title>FFMML Player</title>
			</head>
			<body>
				<ul class="color-list">
				</ul>
				<button class="play-music-button">Play Music</button>
<!--
				<script nonce="${nonce}" src="${scriptUri}"></script>
-->
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
