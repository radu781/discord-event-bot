@echo off

cd C:\Users\Radu\source\repos\event-bot\

echo %date% >> logs.txt
echo %time% >> logs.txt
.\target\debug\event-bot  %* >> logs.txt 2>>&1
echo ----- >> logs.txt
