#!/usr/bin/env ruby

# Ruby script to build all WAD maps and convert the SVGs to example PNGs

wads = [
  "doom.wad",
  "doom2.wad",
  "heretic.wad",
  "hexen.wad",
  "hexdd.wad",
  "strife.wad",
  #"chex3.wad", #disabled for now
]

examples = "examples"

# remove the examples directory entirely
remove = `rm -rf examples`
Dir.mkdir("#{examples}")

wads.each do |wad|

  build = `cargo run #{wad}`

  wad_raw = wad.sub(/.wad/, '')

  map_dir = wad + ".maps"
  ex_dir = "#{examples}/#{wad_raw}"
  mkv = Dir.mkdir(ex_dir)

  Dir.foreach(wad+".maps") do |map|
    if map.length > 2
      new_fname = map.sub(/svg/, "png")
      puts new_fname
      cnv = `convert #{map_dir}/#{map} #{ex_dir}/#{new_fname}`

    end
  end



end
