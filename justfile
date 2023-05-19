default: edit

alias c := cargo-edit
alias ci := cargo-install
alias cb := cargo-build 
alias cr := cargo-run
alias dm := diesel-migration-generate
alias drm := diesel-migration-run

alias f := flutter-edit
alias fr := flutter-run 
alias fb := flutter-build
alias fi := flutter-install

alias w := web-edit
alias wr := web-run
alias wb := web-build
alias wi := web-install

edit:
  nvim justfile

cargo-edit:
  cd server && nvim src/main.rs
cargo-install argument: 
  cd server && cargo add {{argument}} 
cargo-build: 
  cd server && cargo build
cargo-run: 
  cd server && cargo run
diesel-migration-generate argument:
  cd server && diesel migration generate {{argument}}
diesel-migration-run:
  cd server && diesel migration run

flutter-edit:
  cd redir && nvim lib/main.dart
flutter-install: 
  cd redir && flutter pub add
flutter-build: 
  cd redir && flutter build
flutter-run: 
  cd redir && flutter run

web-edit:
  cd redir-web && nvim src/App.tsx
web-run: 
  cd redir-web && pnpm run dev
web-build:
  cd redir-web && pnpm build
web-install: 
  cd redir-web && pnpm install
