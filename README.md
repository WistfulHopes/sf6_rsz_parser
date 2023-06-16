# sf6_rsz_parser
A parser for Street Fighter 6 RSZ files. Currently can only parse the FChar files; rebuilding and support for more file types to come.

## How to use:

Grab rszsf6.json from https://github.com/alphazolam/RE_RSZ. Rip files from game using RETool (https://www.patreon.com/posts/retool-modding-36746173). First argument should be the file you wish to parse, second argument should be the RSZ json, third argument should be the output json.

Example: ```sf6_rsz_parser 000.fchar.17 rszsf6.json esf000.json```
