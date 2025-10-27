RTSP Frame Grabber

Description:

Connects to an CP-E28A and saves one frame per minute as a JPG image locally. Uses opencv and chrono crates to decode/decrypt the frames.

Prerequisites:

Rust: Install via rustup.

OpenCV Dev Libs: On Debian/Ubuntu: sudo apt update && sudo apt install -y build-essential cmake pkg-config libopencv-dev

Network Setup:
Section 1:
This is CP-E28A camera’s RTSP live video URL
rtsp://192.168.137.192:5543/live/channel0

Section 2:
This is the RTSP live video URL we will use when programming in rust or accessing using 3rd party software's.
rtsp://admin:Spiderman1215@192.168.137.192:5543/live/channel0

Here,
1. admin is the ONVIF Username found in ODM v2.2.250 (Camlytics Service)
   ODM is a ONVIF protocol implementation of Network Video Client (NVC) to manage Network Video Transmitters (NVT), Network Video Storage (NVS) and Network Video Analytics (NVA) devices.

2. Spiderman1215 is password set in the ezykam app of CP-E28A.

3. 192.168.137.192 is the IP address of the CP-E28A camera.
   This is the IP from the WIFI/Hotspot being shared to it.
   It can be found in Onvif section in the ezykam app.
   Changes based on the network being used.

4. 5543 is the port being used, can be found in ODM v2.2.250 app.

5. /live/channel0 is the stream path, also found in ODM 2.2.250 app

Configuration:

Edit constants in src/main.rs:
1. rtsp_url: Your camera's full RTSP stream URL.
2. output_dir: Folder name for saved frames (e.g., "captured_frames").
3. frame_interval_ms: Interval in milliseconds (e.g., 60000 for 1/min).

Running the program:

1. Clone the entire repository onto your device command:


2. Build Command: cargo build
3. Run Command: cargo run

Output Format

Filenames: SEQUENTIAL_NUMBER_YYYYMMDD.jpg (e.g., 1_20251027.jpg)

Dependencies:
Managed via Cargo.toml (opencv, chrono)
