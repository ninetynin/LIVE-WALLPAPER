// Gonna try updating the wallpaper using a gif file way rather than using online gif way to change windows live wallpaper
#include <iostream>
#include <string>
#include <fstream>
#include <windows.h>

using namespace std;

string path = "F:\\LANGS\\C++\\LIVE-WALLPAPER\\wallpaper.gif";

class Console{
    public:
        void crtNewConsole() {
            AllocConsole();
            FILE* f;
            freopen_s(&f, "CONOUT$", "w", stdout);
            }
        void freeConsole() {
            fclose(stdout);
            FreeConsole();
            }
};

class Wallpaper{
    public:
        void setWallpaper() {
            SystemParametersInfo(SPI_SETDESKWALLPAPER, 0, (PVOID)path.c_str(), SPIF_UPDATEINIFILE);
            }
        void getWallpaperLocation( string &path ) {
            char buffer[MAX_PATH];
            SystemParametersInfo(SPI_GETDESKWALLPAPER, MAX_PATH, buffer, 0);
            path = buffer;
            }
};

int main(int argc, char *argv[]) {
    Console console;
    Wallpaper wallpaper;
    string path;
    console.crtNewConsole();
    wallpaper.getWallpaperLocation(path);
    cout << "Current wallpaper location: " << path << endl;
    wallpaper.setWallpaper();
    console.freeConsole();
    return 0;    
}


// Features to implement later on:
// - Uptime Timer / If any time is running ability to show it on desktop
// - Ability to show current time on desktop
// - To show CPU usage & memory usage on desktop {useful for myself not for others most prolly}