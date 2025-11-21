@echo off

REM Intercept and return a fake status for CI, but allow any other commands through to the real
REM tailscale binary.

IF "%1"=="status" IF "%2"=="--json" (
  echo { "Peer": { "1": { "HostName": "app-prod-1", "Online": true }, "2": { "HostName": "app-prod-2", "Online": true }, "3": { "HostName": "app-staging-1", "Online": true } } }
  EXIT /B 0
)
"C:\Program Files\Tailscale\tailscale.exe" %*
