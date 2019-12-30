# Project Init
`$ nodenv local 13.1.0`  
`$ cargo new backend`  
`$ npx create-next-app --example with-typescript frontend`

# [Frontend Dependencies Install](https://qiita.com/yohei_nakamura/items/2365682720ffd2fb3424)
### ESLint
`$ yarn add -D eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin eslint-plugin-react@latest`
### Prettier
`$ yarn add -D prettier eslint-config-prettier eslint-plugin-prettier`
### Redux
`$ yarn add redux react-redux @types/react-redux next-redux-wrapper redux-thunk`
### Sass
`$ yarn add -D @zeit/next-sass node-sass`
### Axios
`$ yarn add axios`

# Reflect Module
### ESLint
`$ eslint --init`

# Setup Project
### Next
`$ cd frontend && yarn dev`  
You can see the UI on port 3000
### Rocket
`$ cd backend && cargo run`
API server is listening on port 8000
### MySQL
`$ mysql < database/create_tables.sql`  
`$ mysql < database/test_user.sql`

# Reference
- [redux-thunk](https://qiita.com/hiroya8649/items/c202742c99d2cc6159b8)
- [random-string](https://qiita.com/aoyagikouhei/items/b796632ff6581197737c)
- [sha256](http://kizkoh.hatenablog.com/entry/2016/06/02/154525)
