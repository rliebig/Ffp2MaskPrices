# FFP 2 Mask Prices

With medical masks becoming mandatory in public transit in Germany, I was curious how market dynamics would actually play out.

This is a crude, smol tool written in rust to monitor Amazon prices for FFP" masks over the coming weeks and months.

It utilizes tui-rs for graphics, reqwest for HTTP and select-rs for parsing.

Still a work in progress, will probably update this in the near future.

## Setup
Starting in your home directory

``shell
$ mkdir Ffp2MaskPrices
$ cd Ffp2MaskPrices
$ git init
$ git pull https://github.com/rliebig/Ffp2MaskPrices
$ cargo build --release
$ crontab -e``

Now add the following to your cronfile for hourly data scrapping:

``
0 * * * * * ~/Ffp2MaskPrices/target/release/Ffp2MaskPrices``

## Todos
[ ] tui-rs rendering should work

[ ] check the reason why reqwest is taking so long

[ ] Improve shell interface

[ ] add gnu plot scripts

