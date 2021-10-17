test:
	cargo test

preview:
	cargo snippet -t ultisnips

convert:
	cargo snippet -t ultisnips > ~/dotfiles/configs/vim/ultisnips/rust.snippets
