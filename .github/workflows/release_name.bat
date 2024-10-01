@echo off
FOR /F "tokens=*" %%i IN ('git describe --exact-match --tags 2^>nul') DO SET TAG=%%i

SET TAG=%TAG:~0,10%

echo TAG=%TAG%>> %GITHUB_ENV%
