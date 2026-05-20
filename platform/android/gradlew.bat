@echo off
where gradle >nul 2>nul
if errorlevel 1 (
  echo gradle not found in PATH 1>&2
  exit /b 1
)
gradle %*
