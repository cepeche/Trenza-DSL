"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode = __importStar(require("vscode"));
const cp = __importStar(require("child_process"));
const path = __importStar(require("path"));
function activate(context) {
    const outputChannel = vscode.window.createOutputChannel('Trenza');
    const diagnosticCollection = vscode.languages.createDiagnosticCollection('trenza');
    outputChannel.appendLine('Trenza Extension is activating...');
    context.subscriptions.push(vscode.workspace.onDidSaveTextDocument(document => {
        if (document.languageId === 'trenza') {
            runValidation(document, diagnosticCollection, outputChannel);
        }
    }), vscode.workspace.onDidOpenTextDocument(document => {
        if (document.languageId === 'trenza') {
            runValidation(document, diagnosticCollection, outputChannel);
        }
    }));
    // Initial validation for open documents
    vscode.workspace.textDocuments.forEach(doc => {
        if (doc.languageId === 'trenza') {
            runValidation(doc, diagnosticCollection, outputChannel);
        }
    });
    outputChannel.appendLine('Trenza Extension is now active.');
}
exports.activate = activate;
function runValidation(document, collection, output) {
    const config = vscode.workspace.getConfiguration('trenza');
    let compilerPath = config.get('compilerPath') || 'trenza-cli';
    // Improved search: look for compiler in common locations
    if (!path.isAbsolute(compilerPath)) {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        const exeExt = process.platform === 'win32' ? '.exe' : '';
        const binaryName = 'trenza-cli' + exeExt;
        const searchPaths = [];
        if (workspaceFolders) {
            output.appendLine(`Workspace folder found: ${workspaceFolders[0].uri.fsPath}`);
            // Case 1: Root is Trenza-DSL
            searchPaths.push(path.join(workspaceFolders[0].uri.fsPath, 'trenza-cli', 'target', 'release', binaryName));
            // Case 2: Root is editors/vscode
            searchPaths.push(path.join(workspaceFolders[0].uri.fsPath, '..', '..', 'trenza-cli', 'target', 'release', binaryName));
        }
        else {
            output.appendLine('No workspace folder open. Trying to find compiler based on document path.');
            // Fallback: search up from the document path
            let currentDir = path.dirname(document.uri.fsPath);
            while (currentDir !== path.parse(currentDir).root) {
                const potentialPath = path.join(currentDir, 'trenza-cli', 'target', 'release', binaryName);
                searchPaths.push(potentialPath);
                currentDir = path.dirname(currentDir);
            }
        }
        output.appendLine(`Searching for compiler in ${searchPaths.length} locations...`);
        for (const p of searchPaths) {
            output.appendLine(`Checking: ${p}`);
            if (require('fs').existsSync(p)) {
                compilerPath = p;
                output.appendLine(`SUCCESS: Found compiler at: ${compilerPath}`);
                break;
            }
        }
    }
    const docPath = document.uri.fsPath;
    const cmd = `"${compilerPath}" check --format=json "${docPath}"`;
    output.appendLine(`Running: ${cmd}`);
    cp.exec(cmd, (error, stdout, stderr) => {
        if (error) {
            output.appendLine(`Error executing compiler (exit code ${error.code}): ${error.message}`);
        }
        output.appendLine(`Compiler stdout length: ${stdout.length}`);
        output.appendLine(`Compiler stderr length: ${stderr.length}`);
        if (stderr) {
            output.appendLine(`Compiler stderr: ${stderr}`);
        }
        const diagnostics = [];
        try {
            // Find the JSON part in stdout (it might have some messages before/after)
            const jsonStart = stdout.indexOf('[');
            const jsonEnd = stdout.lastIndexOf(']') + 1;
            if (jsonStart !== -1 && jsonEnd !== 0) {
                const jsonStr = stdout.substring(jsonStart, jsonEnd);
                output.appendLine(`Found JSON in stdout (chars ${jsonStart} to ${jsonEnd})`);
                const results = JSON.parse(jsonStr);
                output.appendLine(`Parsed ${results.length} diagnostic(s)`);
                for (const diag of results) {
                    const startLine = Math.max(0, diag.span.start.line - 1);
                    const startCol = Math.max(0, diag.span.start.col - 1);
                    const endLine = Math.max(0, diag.span.end.line - 1);
                    const endCol = Math.max(0, diag.span.end.col - 1);
                    const range = new vscode.Range(startLine, startCol, endLine, endCol);
                    const severity = diag.severity === 'error' ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning;
                    diagnostics.push(new vscode.Diagnostic(range, diag.message, severity));
                }
            }
            else {
                output.appendLine('No JSON array found in compiler output.');
                if (stdout.trim()) {
                    output.appendLine(`Raw stdout: ${stdout}`);
                }
            }
        }
        catch (e) {
            output.appendLine(`Error parsing Trenza diagnostics: ${e.message}`);
        }
        output.appendLine(`Setting ${diagnostics.length} diagnostics for ${document.uri.toString()}`);
        collection.set(document.uri, diagnostics);
    });
}
function deactivate() { }
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map