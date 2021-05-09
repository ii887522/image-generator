docker image inspect ms-build-tools
if %ERRORLEVEL% neq 0 docker build -t ms-build-tools:latest .
