[Setup]
AppName=Crosshair Overlay
AppVersion=1.0
DefaultDirName={pf}\CrosshairOverlay
DefaultGroupName=Crosshair Overlay
OutputDir=target\installer
OutputBaseFilename=Crosshair-overlay-setup
Compression=lzma
SolidCompression=yes

[Files]
Source: "target\release\crosshair-overlay.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Crosshair Overlay"; Filename: "{app}\crosshair-overlay.exe";
Name: "{userdesktop}\Crosshair Overlay"; Filename: "{app}\crosshair-overlay.exe"; Tasks: desktopicon;

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop icon"; GroupDescription: "Additional icons:"

[Run]
Filename: "{app}\crosshair-overlay.exe"; Description: "Launch Crosshair Overlay"; Flags: nowait postinstall skipifsilent runhidden shellexec