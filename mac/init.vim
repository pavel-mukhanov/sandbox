call plug#begin('~/.vim/plugged')

Plug 'challenger-deep-theme/vim', { 'as': 'challenger-deep' }
Plug 'raichoo/purescript-vim'
Plug 'cespare/vim-toml'
Plug 'nightsense/carbonized'
Plug 'vim-airline/vim-airline'
Plug 'racer-rust/vim-racer'
Plug 'rust-lang/rust.vim'

call plug#end()

" RUST
au FileType rust nmap gd <Plug>(rust-def)
let g:rustfmt_autosave = 1

" Appearence
set termguicolors
colorscheme challenger_deep

set number relativenumber
