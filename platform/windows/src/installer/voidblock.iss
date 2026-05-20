[Setup]
AppName=VoidBlock
AppVersion=0.1.0
DefaultDirName={pf}\VoidBlock
DefaultGroupName=VoidBlock
OutputBaseFilename=voidblock-setup
Compression=lzma2
SolidCompression=yes
PrivilegesRequired=admin

[Files]
Source: "..\\..\\build\\Release\\voidblock_proxy.exe"; DestDir: "{app}"; Flags: ignoreversion

[Run]
Filename: "{app}\\voidblock_proxy.exe"; Description: "Launch VoidBlock"; Flags: postinstall nowait skipifsilent

[UninstallRun]
Filename: "reg.exe"; Parameters: "add HKLM\\SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters /v ServerAddresses /t REG_SZ /d \"\" /f"; Flags: runhidden
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
