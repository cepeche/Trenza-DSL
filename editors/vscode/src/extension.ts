import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';

export function activate(context: vscode.ExtensionContext) {
    const outputChannel = vscode.window.createOutputChannel('Trenza');
    const diagnosticCollection = vscode.languages.createDiagnosticCollection('trenza');
    
    outputChannel.appendLine('Trenza Extension is activating...');
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(document => {
            if (document.languageId === 'trenza') {
                runValidation(document, diagnosticCollection, outputChannel);
            }
        }),
        vscode.workspace.onDidOpenTextDocument(document => {
            if (document.languageId === 'trenza') {
                runValidation(document, diagnosticCollection, outputChannel);
            }
        }),
        vscode.commands.registerCommand('trenza.validate', () => {
            const activeEditor = vscode.window.activeTextEditor;
            if (activeEditor && activeEditor.document.languageId === 'trenza') {
                runValidation(activeEditor.document, diagnosticCollection, outputChannel);
            }
        })
    );
    
    // Initial validation for open documents
    vscode.workspace.textDocuments.forEach(doc => {
        if (doc.languageId === 'trenza') {
            runValidation(doc, diagnosticCollection, outputChannel);
        }
    });

    outputChannel.appendLine('Trenza Extension is now active.');
}

function runValidation(document: vscode.TextDocument, collection: vscode.DiagnosticCollection, output: vscode.OutputChannel) {
    const config = vscode.workspace.getConfiguration('trenza');
    let compilerPath = config.get<string>('compilerPath') || 'trenza-cli';
    
    // Improved search: look for compiler in common locations
    if (!path.isAbsolute(compilerPath)) {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        const exeExt = process.platform === 'win32' ? '.exe' : '';
        const binaryName = 'trenza-cli' + exeExt;

        const searchPaths: string[] = [];
        if (workspaceFolders) {
            output.appendLine(`Workspace folder found: ${workspaceFolders[0].uri.fsPath}`);
            // Case 1: Root is Trenza-DSL
            searchPaths.push(path.join(workspaceFolders[0].uri.fsPath, 'target', 'debug', binaryName));
            searchPaths.push(path.join(workspaceFolders[0].uri.fsPath, 'target', 'release', binaryName));
            searchPaths.push(path.join(workspaceFolders[0].uri.fsPath, 'trenza-cli', 'target', 'debug', binaryName));
            searchPaths.push(path.join(workspaceFolders[0].uri.fsPath, 'trenza-cli', 'target', 'release', binaryName));
            // Case 2: Root is editors/vscode
            searchPaths.push(path.join(workspaceFolders[0].uri.fsPath, '..', '..', 'target', 'debug', binaryName));
            searchPaths.push(path.join(workspaceFolders[0].uri.fsPath, '..', '..', 'target', 'release', binaryName));
        } else {
            output.appendLine('No workspace folder open. Trying to find compiler based on document path.');
            let currentDir = path.dirname(document.uri.fsPath);
            while (currentDir !== path.parse(currentDir).root) {
                searchPaths.push(path.join(currentDir, 'target', 'debug', binaryName));
                searchPaths.push(path.join(currentDir, 'target', 'release', binaryName));
                searchPaths.push(path.join(currentDir, 'trenza-cli', 'target', 'debug', binaryName));
                searchPaths.push(path.join(currentDir, 'trenza-cli', 'target', 'release', binaryName));
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
        
        const diagnostics: vscode.Diagnostic[] = [];
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
            } else {
                output.appendLine('No JSON array found in compiler output.');
                if (stdout.trim()) {
                    output.appendLine(`Raw stdout: ${stdout}`);
                }
            }
        } catch (e: any) {
            output.appendLine(`Error parsing Trenza diagnostics: ${e.message}`);
        }
        
        output.appendLine(`Setting ${diagnostics.length} diagnostics for ${document.uri.toString()}`);
        collection.set(document.uri, diagnostics);
    });
}

export function deactivate() {}
