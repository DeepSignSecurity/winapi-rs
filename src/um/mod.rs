// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Headers for user mode only
pub mod gl;
#[cfg(feature = "accctrl")] pub mod accctrl;
#[cfg(feature = "aclapi")] pub mod aclapi;
#[cfg(feature = "adhoc")] pub mod adhoc;
#[cfg(feature = "appmgmt")] pub mod appmgmt;
#[cfg(feature = "audioclient")] pub mod audioclient;
#[cfg(feature = "audiosessiontypes")] pub mod audiosessiontypes;
#[cfg(feature = "avrt")] pub mod avrt;
#[cfg(feature = "bits")] pub mod bits;
#[cfg(feature = "bits10_1")] pub mod bits10_1;
#[cfg(feature = "bits1_5")] pub mod bits1_5;
#[cfg(feature = "bits2_0")] pub mod bits2_0;
#[cfg(feature = "bits2_5")] pub mod bits2_5;
#[cfg(feature = "bits3_0")] pub mod bits3_0;
#[cfg(feature = "bits4_0")] pub mod bits4_0;
#[cfg(feature = "bits5_0")] pub mod bits5_0;
#[cfg(feature = "bitscfg")] pub mod bitscfg;
#[cfg(feature = "bitsmsg")] pub mod bitsmsg;
#[cfg(feature = "bluetoothapis")] pub mod bluetoothapis;
#[cfg(feature = "bluetoothleapis")] pub mod bluetoothleapis;
#[cfg(feature = "bthledef")] pub mod bthledef;
#[cfg(feature = "cfgmgr32")] pub mod cfgmgr32;
#[cfg(feature = "cguid")] pub mod cguid;
#[cfg(feature = "combaseapi")] pub mod combaseapi;
#[cfg(feature = "coml2api")] pub mod coml2api;
#[cfg(feature = "commapi")] pub mod commapi;
#[cfg(feature = "commctrl")] pub mod commctrl;
#[cfg(feature = "commdlg")] pub mod commdlg;
#[cfg(feature = "commoncontrols")] pub mod commoncontrols;
#[cfg(feature = "consoleapi")] pub mod consoleapi;
#[cfg(feature = "corsym")] pub mod corsym;
#[cfg(feature = "d2d1")] pub mod d2d1;
#[cfg(feature = "d2d1_1")] pub mod d2d1_1;
#[cfg(feature = "d2d1_2")] pub mod d2d1_2;
#[cfg(feature = "d2d1_3")] pub mod d2d1_3;
#[cfg(feature = "d2d1effectauthor")] pub mod d2d1effectauthor;
#[cfg(feature = "d2d1effects")] pub mod d2d1effects;
#[cfg(feature = "d2d1effects_1")] pub mod d2d1effects_1;
#[cfg(feature = "d2d1effects_2")] pub mod d2d1effects_2;
#[cfg(feature = "d2d1svg")] pub mod d2d1svg;
#[cfg(feature = "d2dbasetypes")] pub mod d2dbasetypes;
#[cfg(feature = "d3d")] pub mod d3d;
#[cfg(feature = "d3d10")] pub mod d3d10;
#[cfg(feature = "d3d10_1")] pub mod d3d10_1;
#[cfg(feature = "d3d10_1shader")] pub mod d3d10_1shader;
#[cfg(feature = "d3d10effect")] pub mod d3d10effect;
#[cfg(feature = "d3d10misc")] pub mod d3d10misc;
#[cfg(feature = "d3d10sdklayers")] pub mod d3d10sdklayers;
#[cfg(feature = "d3d10shader")] pub mod d3d10shader;
#[cfg(feature = "d3d11")] pub mod d3d11;
#[cfg(feature = "d3d11_1")] pub mod d3d11_1;
#[cfg(feature = "d3d11_2")] pub mod d3d11_2;
#[cfg(feature = "d3d11_3")] pub mod d3d11_3;
#[cfg(feature = "d3d11_4")] pub mod d3d11_4;
#[cfg(feature = "d3d11on12")] pub mod d3d11on12;
#[cfg(feature = "d3d11sdklayers")] pub mod d3d11sdklayers;
#[cfg(feature = "d3d11shader")] pub mod d3d11shader;
#[cfg(feature = "d3d11tokenizedprogramformat")] pub mod d3d11tokenizedprogramformat;
#[cfg(feature = "d3d12")] pub mod d3d12;
#[cfg(feature = "d3d12sdklayers")] pub mod d3d12sdklayers;
#[cfg(feature = "d3d12shader")] pub mod d3d12shader;
#[cfg(feature = "d3dcommon")] pub mod d3dcommon;
#[cfg(feature = "d3dcompiler")] pub mod d3dcompiler;
#[cfg(feature = "d3dcsx")] pub mod d3dcsx;
#[cfg(feature = "d3dx10core")] pub mod d3dx10core;
#[cfg(feature = "d3dx10math")] pub mod d3dx10math;
#[cfg(feature = "d3dx10mesh")] pub mod d3dx10mesh;
#[cfg(feature = "datetimeapi")] pub mod datetimeapi;
#[cfg(feature = "davclnt")] pub mod davclnt;
#[cfg(feature = "dbghelp")] pub mod dbghelp;
#[cfg(feature = "dbt")] pub mod dbt;
#[cfg(feature = "dcommon")] pub mod dcommon;
#[cfg(feature = "dcomp")] pub mod dcomp;
#[cfg(feature = "dcompanimation")] pub mod dcompanimation;
#[cfg(feature = "dde")] pub mod dde;
#[cfg(feature = "ddraw")] pub mod ddraw;
#[cfg(feature = "ddrawi")] pub mod ddrawi;
#[cfg(feature = "ddrawint")] pub mod ddrawint;
#[cfg(feature = "debugapi")] pub mod debugapi;
#[cfg(feature = "devicetopology")] pub mod devicetopology;
#[cfg(feature = "dinput")] pub mod dinput;
#[cfg(feature = "dispex")] pub mod dispex;
#[cfg(feature = "dmksctl")] pub mod dmksctl;
#[cfg(feature = "dmusicc")] pub mod dmusicc;
#[cfg(feature = "docobj")] pub mod docobj;
#[cfg(feature = "documenttarget")] pub mod documenttarget;
#[cfg(feature = "dot1x")] pub mod dot1x;
#[cfg(feature = "dpa_dsa")] pub mod dpa_dsa;
#[cfg(feature = "dpapi")] pub mod dpapi;
#[cfg(feature = "dsgetdc")] pub mod dsgetdc;
#[cfg(feature = "dsound")] pub mod dsound;
#[cfg(feature = "dsrole")] pub mod dsrole;
#[cfg(feature = "dvp")] pub mod dvp;
#[cfg(feature = "dwmapi")] pub mod dwmapi;
#[cfg(feature = "dwrite")] pub mod dwrite;
#[cfg(feature = "dwrite_1")] pub mod dwrite_1;
#[cfg(feature = "dwrite_2")] pub mod dwrite_2;
#[cfg(feature = "dwrite_3")] pub mod dwrite_3;
#[cfg(feature = "dxdiag")] pub mod dxdiag;
#[cfg(feature = "dxfile")] pub mod dxfile;
#[cfg(feature = "dxgidebug")] pub mod dxgidebug;
#[cfg(feature = "dxva2api")] pub mod dxva2api;
#[cfg(feature = "dxvahd")] pub mod dxvahd;
#[cfg(feature = "eaptypes")] pub mod eaptypes;
#[cfg(feature = "enclaveapi")] pub mod enclaveapi;
#[cfg(feature = "endpointvolume")] pub mod endpointvolume;
#[cfg(feature = "errhandlingapi")] pub mod errhandlingapi;
#[cfg(feature = "evntcons")] pub mod evntcons;
#[cfg(feature = "exdisp")] pub mod exdisp;
#[cfg(feature = "fibersapi")] pub mod fibersapi;
#[cfg(feature = "fileapi")] pub mod fileapi;
#[cfg(feature = "functiondiscoverykeys_devpkey")] pub mod functiondiscoverykeys_devpkey;
#[cfg(feature = "handleapi")] pub mod handleapi;
#[cfg(feature = "heapapi")] pub mod heapapi;
#[cfg(feature = "highlevelmonitorconfigurationapi")] pub mod highlevelmonitorconfigurationapi;
#[cfg(feature = "http")] pub mod http;
#[cfg(feature = "imm")] pub mod imm;
#[cfg(feature = "interlockedapi")] pub mod interlockedapi;
#[cfg(feature = "ioapiset")] pub mod ioapiset;
#[cfg(feature = "ipexport")] pub mod ipexport;
#[cfg(feature = "iphlpapi")] pub mod iphlpapi;
#[cfg(feature = "iptypes")] pub mod iptypes;
#[cfg(feature = "jobapi")] pub mod jobapi;
#[cfg(feature = "jobapi2")] pub mod jobapi2;
#[cfg(feature = "knownfolders")] pub mod knownfolders;
#[cfg(feature = "ktmw32")] pub mod ktmw32;
#[cfg(feature = "l2cmn")] pub mod l2cmn;
#[cfg(feature = "libloaderapi")] pub mod libloaderapi;
#[cfg(feature = "lmaccess")] pub mod lmaccess;
#[cfg(feature = "lmalert")] pub mod lmalert;
#[cfg(feature = "lmapibuf")] pub mod lmapibuf;
#[cfg(feature = "lmat")] pub mod lmat;
#[cfg(feature = "lmdfs")] pub mod lmdfs;
#[cfg(feature = "lmerrlog")] pub mod lmerrlog;
#[cfg(feature = "lmjoin")] pub mod lmjoin;
#[cfg(feature = "lmmsg")] pub mod lmmsg;
#[cfg(feature = "lmremutl")] pub mod lmremutl;
#[cfg(feature = "lmrepl")] pub mod lmrepl;
#[cfg(feature = "lmserver")] pub mod lmserver;
#[cfg(feature = "lmshare")] pub mod lmshare;
#[cfg(feature = "lmstats")] pub mod lmstats;
#[cfg(feature = "lmsvc")] pub mod lmsvc;
#[cfg(feature = "lmuse")] pub mod lmuse;
#[cfg(feature = "lmwksta")] pub mod lmwksta;
#[cfg(feature = "lowlevelmonitorconfigurationapi")] pub mod lowlevelmonitorconfigurationapi;
#[cfg(feature = "lsalookup")] pub mod lsalookup;
#[cfg(feature = "memoryapi")] pub mod memoryapi;
#[cfg(feature = "minschannel")] pub mod minschannel;
#[cfg(feature = "minwinbase")] pub mod minwinbase;
#[cfg(feature = "mmdeviceapi")] pub mod mmdeviceapi;
#[cfg(feature = "mmeapi")] pub mod mmeapi;
#[cfg(feature = "mmsystem")] pub mod mmsystem;
#[cfg(feature = "msaatext")] pub mod msaatext;
#[cfg(feature = "mscat")] pub mod mscat;
#[cfg(feature = "mschapp")] pub mod mschapp;
#[cfg(feature = "mssip")] pub mod mssip;
#[cfg(feature = "mswsock")] pub mod mswsock;
#[cfg(feature = "namedpipeapi")] pub mod namedpipeapi;
#[cfg(feature = "namespaceapi")] pub mod namespaceapi;
#[cfg(feature = "nb30")] pub mod nb30;
#[cfg(feature = "ncrypt")] pub mod ncrypt;
#[cfg(feature = "ntlsa")] pub mod ntlsa;
#[cfg(feature = "ntsecapi")] pub mod ntsecapi;
#[cfg(feature = "oaidl")] pub mod oaidl;
#[cfg(feature = "objbase")] pub mod objbase;
#[cfg(feature = "objidl")] pub mod objidl;
#[cfg(feature = "objidlbase")] pub mod objidlbase;
#[cfg(feature = "ocidl")] pub mod ocidl;
#[cfg(feature = "ole2")] pub mod ole2;
#[cfg(feature = "oleacc")] pub mod oleacc;
#[cfg(feature = "oleauto")] pub mod oleauto;
#[cfg(feature = "olectl")] pub mod olectl;
#[cfg(feature = "oleidl")] pub mod oleidl;
#[cfg(feature = "opmapi")] pub mod opmapi;
#[cfg(feature = "pdh")] pub mod pdh;
#[cfg(feature = "perflib")] pub mod perflib;
#[cfg(feature = "pchannel")] pub mod pchannel;
#[cfg(feature = "physicalmonitorenumerationapi")] pub mod physicalmonitorenumerationapi;
#[cfg(feature = "playsoundapi")] pub mod playsoundapi;
#[cfg(feature = "portabledevice")] pub mod portabledevice;
#[cfg(feature = "portabledeviceapi")] pub mod portabledeviceapi;
#[cfg(feature = "portabledevicetypes")] pub mod portabledevicetypes;
#[cfg(feature = "powerbase")] pub mod powerbase;
#[cfg(feature = "powersetting")] pub mod powersetting;
#[cfg(feature = "powrprof")] pub mod powrprof;
#[cfg(feature = "processenv")] pub mod processenv;
#[cfg(feature = "processsnapshot")] pub mod processsnapshot;
#[cfg(feature = "processthreadsapi")] pub mod processthreadsapi;
#[cfg(feature = "processtopologyapi")] pub mod processtopologyapi;
#[cfg(feature = "profileapi")] pub mod profileapi;
#[cfg(feature = "propidl")] pub mod propidl;
#[cfg(feature = "propkey")] pub mod propkey;
#[cfg(feature = "propkeydef")] pub mod propkeydef;
#[cfg(feature = "propsys")] pub mod propsys;
#[cfg(feature = "prsht")] pub mod prsht;
#[cfg(feature = "psapi")] pub mod psapi;
#[cfg(feature = "realtimeapiset")] pub mod realtimeapiset;
#[cfg(feature = "reason")] pub mod reason;
#[cfg(feature = "restartmanager")] pub mod restartmanager;
#[cfg(feature = "restrictederrorinfo")] pub mod restrictederrorinfo;
#[cfg(feature = "rmxfguid")] pub mod rmxfguid;
#[cfg(feature = "rtinfo")] pub mod rtinfo;
#[cfg(feature = "sapi")] pub mod sapi;
#[cfg(feature = "sapi51")] pub mod sapi51;
#[cfg(feature = "sapi53")] pub mod sapi53;
#[cfg(feature = "sapiddk")] pub mod sapiddk;
#[cfg(feature = "sapiddk51")] pub mod sapiddk51;
#[cfg(feature = "schannel")] pub mod schannel;
#[cfg(feature = "securityappcontainer")] pub mod securityappcontainer;
#[cfg(feature = "securitybaseapi")] pub mod securitybaseapi;
#[cfg(feature = "servprov")] pub mod servprov;
#[cfg(feature = "setupapi")] pub mod setupapi;
#[cfg(feature = "shellapi")] pub mod shellapi;
#[cfg(feature = "shellscalingapi")] pub mod shellscalingapi;
#[cfg(feature = "shlobj")] pub mod shlobj;
#[cfg(feature = "shobjidl")] pub mod shobjidl;
#[cfg(feature = "shobjidl_core")] pub mod shobjidl_core;
#[cfg(feature = "shtypes")] pub mod shtypes;
#[cfg(feature = "softpub")] pub mod softpub;
#[cfg(feature = "spapidef")] pub mod spapidef;
#[cfg(feature = "spellcheck")] pub mod spellcheck;
#[cfg(feature = "sporder")] pub mod sporder;
#[cfg(feature = "sql")] pub mod sql;
#[cfg(feature = "sqlext")] pub mod sqlext;
#[cfg(feature = "sqltypes")] pub mod sqltypes;
#[cfg(feature = "sqlucode")] pub mod sqlucode;
#[cfg(feature = "sspi")] pub mod sspi;
#[cfg(feature = "stringapiset")] pub mod stringapiset;
#[cfg(feature = "strmif")] pub mod strmif;
#[cfg(feature = "subauth")] pub mod subauth;
#[cfg(feature = "synchapi")] pub mod synchapi;
#[cfg(feature = "sysinfoapi")] pub mod sysinfoapi;
#[cfg(feature = "systemtopologyapi")] pub mod systemtopologyapi;
#[cfg(feature = "taskschd")] pub mod taskschd;
#[cfg(feature = "textstor")] pub mod textstor;
#[cfg(feature = "threadpoolapiset")] pub mod threadpoolapiset;
#[cfg(feature = "threadpoollegacyapiset")] pub mod threadpoollegacyapiset;
#[cfg(feature = "timeapi")] pub mod timeapi;
#[cfg(feature = "timezoneapi")] pub mod timezoneapi;
#[cfg(feature = "tlhelp32")] pub mod tlhelp32;
#[cfg(feature = "uiautomationclient")] pub mod uiautomationclient;
#[cfg(feature = "uiautomationcore")] pub mod uiautomationcore;
#[cfg(feature = "uiautomationcoreapi")] pub mod uiautomationcoreapi;
#[cfg(feature = "unknwnbase")] pub mod unknwnbase;
#[cfg(feature = "urlhist")] pub mod urlhist;
#[cfg(feature = "urlmon")] pub mod urlmon;
#[cfg(feature = "userenv")] pub mod userenv;
#[cfg(feature = "usp10")] pub mod usp10;
#[cfg(feature = "utilapiset")] pub mod utilapiset;
#[cfg(feature = "uxtheme")] pub mod uxtheme;
#[cfg(feature = "vsbackup")] pub mod vsbackup;
#[cfg(feature = "vss")] pub mod vss;
#[cfg(feature = "vsserror")] pub mod vsserror;
#[cfg(feature = "vswriter")] pub mod vswriter;
#[cfg(feature = "wbemads")] pub mod wbemads;
#[cfg(feature = "wbemcli")] pub mod wbemcli;
#[cfg(feature = "wbemdisp")] pub mod wbemdisp;
#[cfg(feature = "wbemprov")] pub mod wbemprov;
#[cfg(feature = "wbemtran")] pub mod wbemtran;
#[cfg(feature = "wct")] pub mod wct;
#[cfg(feature = "werapi")] pub mod werapi;
#[cfg(feature = "winbase")] pub mod winbase;
#[cfg(feature = "wincodec")] pub mod wincodec;
#[cfg(feature = "wincodecsdk")] pub mod wincodecsdk;
#[cfg(feature = "wincon")] pub mod wincon;
#[cfg(feature = "wincontypes")] pub mod wincontypes;
#[cfg(feature = "wincred")] pub mod wincred;
#[cfg(feature = "wincrypt")] pub mod wincrypt;
#[cfg(feature = "windowsceip")] pub mod windowsceip;
#[cfg(feature = "winefs")] pub mod winefs;
#[cfg(feature = "winevt")] pub mod winevt;
#[cfg(feature = "wingdi")] pub mod wingdi;
#[cfg(feature = "winhttp")] pub mod winhttp;
#[cfg(feature = "wininet")] pub mod wininet;
#[cfg(feature = "winineti")] pub mod winineti;
#[cfg(feature = "winioctl")] pub mod winioctl;
#[cfg(feature = "winnetwk")] pub mod winnetwk;
#[cfg(feature = "winnls")] pub mod winnls;
#[cfg(feature = "winnt")] pub mod winnt;
#[cfg(feature = "winreg")] pub mod winreg;
#[cfg(feature = "winsafer")] pub mod winsafer;
#[cfg(feature = "winscard")] pub mod winscard;
#[cfg(feature = "winsmcrd")] pub mod winsmcrd;
#[cfg(feature = "winsock2")] pub mod winsock2;
#[cfg(feature = "winspool")] pub mod winspool;
#[cfg(feature = "winsvc")] pub mod winsvc;
#[cfg(feature = "wintrust")] pub mod wintrust;
#[cfg(feature = "winusb")] pub mod winusb;
#[cfg(feature = "winuser")] pub mod winuser;
#[cfg(feature = "winver")] pub mod winver;
#[cfg(feature = "wlanapi")] pub mod wlanapi;
#[cfg(feature = "wlanihv")] pub mod wlanihv;
#[cfg(feature = "wlanihvtypes")] pub mod wlanihvtypes;
#[cfg(feature = "wlclient")] pub mod wlclient;
#[cfg(feature = "wow64apiset")] pub mod wow64apiset;
#[cfg(feature = "wpdmtpextensions")] pub mod wpdmtpextensions;
#[cfg(feature = "ws2bth")] pub mod ws2bth;
#[cfg(feature = "ws2spi")] pub mod ws2spi;
#[cfg(feature = "ws2tcpip")] pub mod ws2tcpip;
#[cfg(feature = "wtsapi32")] pub mod wtsapi32;
#[cfg(feature = "xinput")] pub mod xinput;
