#TODO

# Example usage

`cargo run --release -- <Path>/openvpn_mips -i 32 -c 6 -e big --left-shift-call-operand 2 --addressing_mode absolute`
Finds correct with probability 0.62

`cargo run --release -- <Path>/ffmpeg_aarch64 -i 32 -c 6 -e little --left-shift-call-operand 2 --addressing_mode relative`
Finds correct with probability 0.8
