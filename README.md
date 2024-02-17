# Unifi-Protect footage bulk download tool
This CLI-tool allows you to download all footage from your Unifi-Protect NVR. It is written in Rust and uses the [unifi-protect](https://github.com/xlfpx/unifi-protect-rust) crate to communicate with the Unifi-Protect API.

# Installation
1. Install rust & cargo if not installed: https://rust-lang.org/tools/install
2. Install this CLI-tool: `cargo install unifi-protect-bulk-download`

### Docker
Alternatively, you can also use Docker to run the tool without installing Rust: `docker run -it unifiprotect/unifi-protect-bulk-download download <uri> <username> <password> <path> <mode> <recording_type> <start_date> <end_date>`

# Usage
`unifi_protect_bulk_download download <uri> <username> <password> <path> <mode> <recording_type> <start_date> <end_date>`

Arguments:
- \<uri>             The uri of the unifi protect server
- \<username>        The username for logging into the unifi protect server
- \<password>        The password for logging into the unifi protect server
- \<path>            The path to the directory to download the files to
- \<mode>            The mode to download the files in (daily or hourly) [possible values: daily, hourly]
- \<recording_type>  The type of recording to download (rotating or timelapse) [possible values: rotating, timelapse]
- \<start_date>      The start date to download the files from (YYYY-MM-DD)
- \<end_date>        The end date to download the files from (YYYY-MM-DD)


# Example
For example, to download all footage from your Unifi-Protect NVR, for all cameras, for the months of June and July 2023, run the following command:
```bash
download https://<Unifi-Protect-IP-Addr> <username> <password> /path/to/destination/folder daily rotating 2023-06-01 2023-07-31
```
In the above example, replace:
1. __\<Unifi-Protect-IP-Addr\>__ with the IP-Address of your unifi-protect system
2. __\<username\>__ with the username of your unifi-protect account
3. __\<password\>__ with the password of your unifi-protect account
4. __/path/to/destination/folder__ with the path to the folder where you want to download the footage to
5. __daily__ with __hourly__ in case you want one video per camera per hour, rather than per day of footage
6. __rotating__ with __timelapse__ in case you want to download timelapse footage rather than real time recordings
6. __2023-06-01__ with the start date of the footage you want to download
6. __2023-07-31__ with the end date of the footage you want to download

## GPL3 LICENSE SYNOPSIS
TL;DR* Here's what the license entails:

1. Anyone can copy, modify and distribute this software.
2. You have to include the license and copyright notice with each and every distribution.
3. You can use this software privately.
4. You can use this software for commercial purposes.
5. If you dare build your business solely from this code, you risk open-sourcing the whole code base.
6. If you modify it, you have to indicate changes made to the code.
7. Any modifications of this code base MUST be distributed with the same license, GPLv3.
8. This software is provided without warranty.
9. The software author or license can not be held liable for any damages inflicted by the software.
   More information on about the LICENSE can be found [here](https://www.gnu.org/licenses/gpl-3.0.en.html)