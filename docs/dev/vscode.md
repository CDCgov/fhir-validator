# VS Code Setup

## Extensions

- rust-analyzer (language support)
- codeLLDB (debugger)

## .vscode/launch.json

```json
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'fhir-validator'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=fhir-validator",
                    "--package=fhir-validator"
                ],
                "filter": {
                    "name": "fhir-validator",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable 'fhir-validator'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=fhir-validator",
                    "--package=fhir-validator"
                ],
                "filter": {
                    "name": "fhir-validator",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

## Optional

### Format on Save

File > Preference > Settings > Editor: Format on save (will run cargo fmt on save)

settings.json
```
"editor.formatOnSave": true,
```

### Disable Inlay Type Hints
settings.json
```
"rust-analyzer.inlayHints.parameterHints.enable": false,
"rust-analyzer.inlayHints.typeHints.enable": false,
```


