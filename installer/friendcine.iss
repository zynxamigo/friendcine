#define AppName "FriendCine"
#define AppVersion "0.2.0"
#define AppPublisher "FriendCine"
#define AppExeName "friendcine.exe"

[Setup]
AppId={{7E5D5B64-3A15-4D7D-8C3C-21A8B8C1D902}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
DefaultDirName={localappdata}\Programs\FriendCine
DefaultGroupName=FriendCine
DisableProgramGroupPage=yes
OutputDir=..\dist
OutputBaseFilename=FriendCine-Setup
SetupIconFile=..\static\icon.svg
UninstallDisplayIcon={app}\{#AppExeName}
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=lowest
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
CloseApplications=yes
RestartApplications=no

[Languages]
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"

[Tasks]
Name: "desktopicon"; Description: "Criar atalho na área de trabalho"; GroupDescription: "Atalhos adicionais:"; Flags: unchecked

[Files]
Source: "..\target\release\friendcine.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\FriendCine"; Filename: "{app}\{#AppExeName}"; WorkingDir: "{app}"
Name: "{autodesktop}\FriendCine"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon; WorkingDir: "{app}"

[Run]
Filename: "{app}\{#AppExeName}"; Description: "Abrir FriendCine"; Flags: postinstall nowait skipifsilent
