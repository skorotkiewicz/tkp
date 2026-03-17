import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  const command = vscode.workspace
    .getConfiguration("tkp")
    .get<string>("serverPath", "tkp");

  const serverOptions: ServerOptions = {
    run: { command, args: ["lsp"] },
    debug: { command, args: ["lsp"] },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "tkp" }],
  };

  client = new LanguageClient("tkp-lsp", "TKP Language Server", serverOptions, clientOptions);
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) return undefined;
  return client.stop();
}
