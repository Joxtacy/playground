function! ReloadJox()
lua << EOF
  for k in pairs(package.loaded) do
    if k:match("^dark") then
      package.loaded[k] = nil
    end
  end
EOF
endfunction

" Reload the plugin
nnoremap <Leader>pra :call ReloadJox()<CR>

" Test the plugin
nnoremap <Leader>ptt :lua require("dark").setBackround()<CR>
