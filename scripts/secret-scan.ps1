$ErrorActionPreference = 'Stop'
$scanner = Get-Command gitleaks -ErrorAction SilentlyContinue
if (-not $scanner) {
    throw 'gitleaks is not installed or not on PATH. See https://github.com/gitleaks/gitleaks.'
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$scanRoot = Join-Path ([System.IO.Path]::GetTempPath()) ('ai-usage-widget-scan-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $scanRoot | Out-Null

try {
    Push-Location $repoRoot
    $files = git ls-files --cached --others --exclude-standard
    if ($LASTEXITCODE -ne 0) { throw 'Could not enumerate the Git file set.' }
    foreach ($file in $files) {
        $source = Join-Path $repoRoot $file
        if (-not (Test-Path -LiteralPath $source)) { continue }
        $destination = Join-Path $scanRoot $file
        $parent = Split-Path -Parent $destination
        if ($parent) { New-Item -ItemType Directory -Force -Path $parent | Out-Null }
        Copy-Item -LiteralPath $source -Destination $destination
    }
    & $scanner.Source dir $scanRoot --no-banner --redact --config (Join-Path $repoRoot '.gitleaks.toml')
    if ($LASTEXITCODE -ne 0) { throw 'Secret scan failed.' }
}
finally {
    Pop-Location
    $resolvedTemp = [System.IO.Path]::GetFullPath([System.IO.Path]::GetTempPath())
    $resolvedScan = [System.IO.Path]::GetFullPath($scanRoot)
    if ($resolvedScan.StartsWith($resolvedTemp) -and (Split-Path -Leaf $resolvedScan).StartsWith('ai-usage-widget-scan-')) {
        Remove-Item -LiteralPath $resolvedScan -Recurse -Force -ErrorAction SilentlyContinue
    }
}
