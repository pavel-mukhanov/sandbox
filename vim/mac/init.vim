set nocompatible

"Enable filetypes
filetype on
filetype plugin on
filetype indent on
syntax on
"
"Display current cursor position in lower right corner.
set ruler

"Want a different map leader than \
let mapleader = ","

"Ever notice a slight lag after typing the leader key + command? This lowers
"the timeout.
set timeoutlen=500

"clear selection
nnoremap <cr> :noh<CR><CR>:<backspace>

"Set font type and size. Depends on the resolution. Larger screens, prefer h20
set guifont=Menlo:h14

"Tab stuff
set tabstop=3
set shiftwidth=3
set softtabstop=3

"Show command in bottom right portion of the screen
set showcmd

"Show lines numbers
set number
set relativenumber

"Allows to change cursor position by mouse click
set mouse=a

"Indent stuff
set smartindent
set autoindent

"Always show the status line
set laststatus=2

"Prefer a slightly higher line height
set linespace=3

"Better line wrapping
set wrap
set textwidth=79
set formatoptions=qrn1

"Set incremental searching"
set incsearch

"Highlight searching
set hlsearch

" case insensitive search
set ignorecase
set smartcase

" Source the vimrc file after saving it. This way, you don't have to reload Vim to see the changes.
if has("autocmd")
augroup myvimrchooks
au!
autocmd bufwritepost .vimrc source ~/.vimrc
augroup END
endif

set showmatch " show matching brackets

" print empty <a> tag

" standatr copy and paste
nnoremap <C-y> "+y
vnoremap <C-y> "+y
nnoremap <C-p> "+gP
vnoremap <C-p> "+gP

if $VIM_CRONTAB == "true"
set nobackup
set nowritebackup
endif

set laststatus=2

map  <Leader>f <Plug>(easymotion-bd-f)
nmap <Leader>f <Plug>(easymotion-overwin-f)

:au BufAdd,BufNewFile * nested tab sball "always open file in new tab
let g:ctrlp_map = '<c-p>'
let g:ctrlp_cmd = 'CtrlP'
set wildignore+=*.class
set wildignore+=*/target/*
set wildignore+=*/node_modules/*<Paste>

set langmap=ёйцукенгшщзхъфывапролджэячсмитьбюЁЙЦУКЕHГШЩЗХЪФЫВАПРОЛДЖЭЯЧСМИТЬБЮ;`qwertyuiop[]asdfghjkl\\;‘zxcvbnm\\,.~QWERTYUIOP{}ASDFGHJKL:\\“ZXCVBNM<>
" http://stackoverflow.com/questions/20186975/vim-mac-how-to-copy-to-clipboard-without-pbcopy
" works only for OS X
let os=substitute(system('uname'), '\n', '', '')
if os == 'Darwin' || os == 'Mac'
  set clipboard^=unnamed
  set clipboard^=unnamedplus"
endif

call plug#begin('~/.vim/plugged')
Plug 'rust-lang/rust.vim'
Plug 'autozimu/LanguageClient-neovim', { 'do': ':UpdateRemotePlugins' }
Plug 'challenger-deep-theme/vim', { 'as': 'challenger-deep' }
Plug 'itchyny/lightline.vim'
Plug 'raichoo/purescript-vim'
Plug 'easymotion/vim-easymotion'
Plug 'ctrlpvim/ctrlp.vim'
Plug 'racer-rust/vim-racer'
Plug 'cespare/vim-toml'
Plug 'junegunn/goyo.vim'
Plug 'junegunn/limelight.vim'
Plug 'junegunn/seoul256.vim'
Plug 'godlygeek/tabular'
Plug 'plasticboy/vim-markdown'
call plug#end()

" RUST
autocmd BufReadPost *.rs setlocal filetype=rust

" Required for operations modifying multiple buffers like rename.
set hidden

let g:LanguageClient_serverCommands = {
    \ 'rust': ['rustup', 'run', 'nightly', 'rls'],
    \ }

" Automatically start language servers.
let g:LanguageClient_autoStart = 1
" Maps K to hover, gd to goto definition, F2 to rename

nnoremap <silent> K :call LanguageClient_textDocument_hover()
nnoremap <silent> <F2> :call LanguageClient_textDocument_rename()
au FileType rust nmap gd <Plug>(rust-def)
au FileType rust nmap gs <Plug>(rust-def-split)
au FileType rust nmap gx <Plug>(rust-def-vertical)
au FileType rust nmap <leader>gd <Plug>(rust-doc)

colorscheme challenger_deep
let g:lightline = { 'colorscheme': 'challenger_deep'}
set termguicolors

let g:rustfmt_autosave = 1
