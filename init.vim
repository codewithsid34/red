function Config()
  compiler rustc
  set makeprg=make
endfunction
autocmd BufNewFile,BufRead *.rs call Config()
