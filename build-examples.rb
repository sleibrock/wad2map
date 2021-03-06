#!/usr/bin/env ruby

# Ruby script to build all WAD maps and convert the SVGs to example PNGs

examples = "examples"
wads = [
  "doom.wad",
  "doom2.wad",
  "heretic.wad",
  "hexen.wad",
  "hexdd.wad",
  "strife.wad",
  "chex3.wad",

  # Non-core wads
  "freedoom1.wad",
  "freedoom2.wad",
  "pirates.wad",
  "scythe2.wad",
  "aoddoom2.wad",
]

# remove the examples directory entirely
remove = `rm -rf examples`
Dir.mkdir("#{examples}")

# begin mainloop
wads.each do |wad|
  build   = `cargo run #{wad}`
  wad_raw = wad.sub(/.wad/, '')
  map_dir = wad + ".maps"
  ex_dir  = "#{examples}/#{wad_raw}"
  mkv     = Dir.mkdir(ex_dir)

  Dir.foreach(wad+".maps") do |map|
    if map.length > 2
      new_fname = map.sub(/svg/, "png")
      cnv = `convert #{map_dir}/#{map} #{ex_dir}/#{new_fname}`
    end
  end
end

# end buildscript
