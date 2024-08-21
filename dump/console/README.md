# Instructions for dumping recipes from console
For people helping out with dumping, please follow the instructions below.
Please reach out to me on discord if you want to help.

## Hardware Requirement
You need
- A modded switch with Atmosphere installed
- A PC (preferably running linux)
- Your PC and switch should be connected to the same network

## Software Requirement
You need the following homebrew apps installed on the switch
- [Simple Mod Manager](https://github.com/nadrino/SimpleModManager)
- `ftpd` (install from hbappstore)

You need the following game installed on the switch
- Breath of the Wild 1.6.0

You are strongly recommended to installed the following programs on the PC.
But these are optional, and the instructions will cover how to do without them.
- [`lftp`](https://lftp.yar.ru/)
- Python
- [`task`](https://taskfile.dev/) for running the scripts
- `git` for cloning the repository

If you don't install `lftp`, you need another FTP client.

Additionally, you should have received the 2 mods from me needed to run the dump

## Installing the mod
1. If you have any existing mods installed not through Simple Mod Manager, remove/move them.
2. Extract the 2 zip files and drag the content to the root of your sd card
   - the `mods` directory should be in the root.
3. Open Simple Mod Manager
4. Select BOTW and disable all mods
5. Enable the 1.5 downgrade patch and then the dumper mod. The dumper mod must be installed last.
6. Launch the game, you should see the following displayed on the title screen:
```
[0/0] C=0000 000000/409600 :^ 0
```
- `[0/0]` is the total progress (how many chunks finished/how many total chunks)
- `C=0000` is the current chunk number
- `000000/409600` is the current progress in the current chunk
- `:^ 0` is the current status code

## Setting up the PC
1. Close the game and open `ftpd` on the switch
2. On your PC, open a terminal and run the following commands. Please
   make sure you know which directory you are running the commands in.
    ```bash
    git clone https://github.com/Pistonight/botw-recipe
    ```
    **Alternative to git**: Click on `<> Code` on this page, then download the repo
    as a zip file and extract it.
3. Enter the directory in terminal or open it in your file manager
    ```bash
    cd botw-recipe
    ```
4. **For those with `task` installed**: Create a file called `.env`
    in the repo, then add the following line to it. Change the IP address
    and port to the address displayed in `ftpd` on the switch.
    ```
    CONSOLE_ADDR: 192.168.0.161:5000
    ```

## Configuring the dump
First you need to determine which chunks to dump. Please ask me so I can
assign you the numbers for CHUNK_START and CHUNK_END. After dumping is done,
if you want to dump more, you can ask me for more numbers, and repeat these steps.

Make sure you have `ftpd` started on the switch.

### With `task` + `python` + `lftp`
Run the following command in the terminal
```bash
task configure -- CHUNK_START CHUNK_END
```
Replace `CHUNK_START` and `CHUNK_END` with the numbers I assigned you.
For example, if I assigned you 50 and 100, you would run `task configure -- 50 100`

### Without `task`
Go to the `/dump/console` directory in the repo, then run the following command
```bash
python scripts/config.py CHUNK_START CHUNK_END
```
Replace `CHUNK_START` and `CHUNK_END` with the numbers I assigned you.
For example, if I assigned you 50 and 100, you would run `python scripts/config.py 50 100`

Then run the following:
```bash
lftp <YOUR_SWITCH_IP> < scripts/lftp-config.sh
```
Replace `<YOUR_SWITCH_IP>` with the IP address and port of your switch.
For example `192.168.178.11:5000`

### Through other FTP clients
If you don't have any of the tools installed:
1. Create a file called `config.txt` in the `/dump/console/scripts` directory
2. Open the file and type the following
    ```
    XXXXYYYY
    ```
    where `XXXX` is the CHUNK_START and `YYYY` is CHUNK_END - CHUNK_START.
    If the number is not 4 digits, pad it with zeros.
    For example, if CHUNK_START is 50 and CHUNK_END is 100, you would type `00500050`
3. Save and close `config.txt`
3. Open your FTP client and connect to the switch
4. Put `config.txt` in the `/botwrdump` directory at the root of the SD card

## Start the dump
After configuring which chunks to dump, all you have to do is exit `ftpd`
and start the game. The dump will start at the title screen after a 5 second countdown
displayed in the status bar.

Note that the console might go to sleep after a while, so make sure to
touch the controller every now and then.

You should see the status code change to `Y` when the dump is done.
Other codes:
- `^`: Cannot find config file
- `7`: Failed to read config
- `8`: Config file is invalid
- `9`: Numbers in the config file are wrong
- `o`: Failed to open a chunk for writing
- `s`: Failed to write to a chunk
- `R`: Ready to start
- `5` `4` `3` `2` `1` `0`: Countdown to start
- `D`: Dumping in progress
- `Y`: Done
- `U`: Uninitialized

## Downloading the chunks
After the dump is done, you can download the chunks from the switch.

Make sure to open `ftpd` on the switch again.

### With `task` + `python` + `lftp`
Run the following command in the terminal in the repo directory
```bash
task download
```
The chunks will be stored in `/dump/console/data`, please zip them and send them to me.

### Without `task`
Go to the `/dump/console` directory in the repo, then run the following command
```bash
mkdir -p raw
lftp <YOUR_SWITCH_IP> < scripts/lftp-download.sh
python scripts/rename.py
```
The chunks will be stored in `/dump/console/data`, please zip them and send them to me.

### Through other FTP clients
If you don't have any of the tools installed:
1. Open your FTP client and connect to the switch
2. Go to the `/botwrdump` directory at the root of the SD card
3. You should see a bunch of files named `ck_XXXX.bin` where XXXX are the chunk numbers
4. Download all of the `.bin` files somewhere on your PC
5. zip them and send them to me

