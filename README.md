**Purpose:** Single file, single click installer of linux apps for any distro.
#
**NO THIS DOES NOT WORK YET**

## Todos
- ~~Safely check tables and values exist~~
- Create spec for .ulai 
- Map data from toml struct to normal internal data struct
- - Distro info inheritance in the .ulai file with overrides
- Detect user distro and match to a set of package mangers
- Handle dependencies
- - handle 32 bit dependencies
- Handle installation
- Handle installation errors or complications
- Handle updates
- ~~Separate validation command for authors~~
#
A lot of normal people who don't regularly use linux find it overly complicated to install apps because of there's different installation steps for different distros and versions of distros.
####
Ulai aims to solve this by providing a single .ulai file for linux apps, which it can use to install the app on any distro (within reason).
.ulai files are used like an .exe installer on windows or .dmg installer on macOS... but for any linux distro, hopefully simplifying the linux experience for newcomers, and making it faster to install your apps on linux by decreasing installation to just executing ulai with its .ulai file, no matter your distro.
####
This allows for you to set ulai as the default program for .ulai files, handling .ulai files as soon as you click on them, and installing the app just like that.