# This is a quick and dirty script for making images for the docs
# Run the rust code with: cargo run --example simple > data
# Text strings and file names are fixed up by hand
set term svg size 400,300
set output "foo.svg"
set style data lines
set title "PolyHarmonic(2), order=2"
#set key at 0.98,0.95
#set key at 0.98,1.3
set key bottom right
plot "data" using 1:2 title "thin plate interp", "data" using 1:3 title "true function"
