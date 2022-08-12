if exists('g:loaded_jox_dark') | finish | endif " prevent loading file twice

let s:save_cpo = &cpo " save user coptions
set cpo&vim           " reset them to defaults

" command to run our plugin
command! JoxIsItDark lua require("dark").setBackround()
command! JoxSetColo lua require("dark").setColorscheme()

let &cpo = s:save_cpo " and restore after
unlet s:save_cpo

let g:loaded_jox_dark = 1

lua << EOF
require("dark").setBackround()
EOF
