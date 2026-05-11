[Setup]
AppName=VoidBlock
AppVersion=0.1.0
DefaultDirName={autopf}\VoidBlock
DefaultGroupName=VoidBlock
OutputBaseFilename=VoidBlock-Installer
Compression=lzma
SolidCompression=yes
PrivilegesRequired=admin

[Files]
Source: "..\\..\\build\\voidblock-proxy.exe"; DestDir: "{app}"; Flags: ignoreversion

[Run]
Filename: "{app}\\voidblock-proxy.exe"; Description: "Launch VoidBlock"; Flags: nowait postinstall skipifsilent

[UninstallRun]
Filename: "reg.exe"; Parameters: "add HKLM\\SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters /v ServerAddresses /t REG_SZ /d \"\" /f"; Flags: runhidden
