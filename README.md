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

# Setup Project
### ESLint
`$ eslint --init`
### MySQL
`$ mysql database/test_user.sql`  
`$ mysql < test_user.sql`

# Reference
About [redux-thunk](https://qiita.com/hiroya8649/items/c202742c99d2cc6159b8)
