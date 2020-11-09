# Vue + Rust mininal example


## Prepare a Rust library

Step 1. Create a rust library called **myrustlib**:
```
cargo new myrustlib --lib
cd myrustlib
```

Step 2. Add `wasm-bindgen = "0.2"` to the dependencied in **Cargo.toml**.

Step 3. Add some code into **lib.rs**:
```rust
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

Step 4. Check the test works fine:
```
cargo test
```

Step 5. Install **wasm-pack** if it is not installed yet. For more documentation read https://rustwasm.github.io/wasm-pack/book/.
```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Step 6. Pack the library for **nodejs**:
```
wasm-pack build --release --target nodejs --out-dir pkg-node --no-typescript
```

Step 7. Pack the library for **web**:
```
wasm-pack build --release --target bundler --out-dir pkg-web --no-typescript
```

Step 8. If everything is correct, you can use the built module in **nodejs**:
```
> const myrustlib = require('./pkg-node')
undefined
> myrustlib.add(2, 3)
5
```


## Prepare a Vue project

Step 1. Create a new Vue project:
```
vue create web-example
cd web-example
```

Step 2. Turn off **ESLint**:
```
npm remove @vue/cli-plugin-eslint
```

Step 2. Install **myrustlib** as a JS package:
```
npm i ../myrustlib/pkg-web
```

Step 3. Import **myrustlib** in **main.js**:
```js
import('myrustlib').then(myrustlib => {
  Vue.prototype.$myrustlib = myrustlib

  new Vue({
    render: h => h(App)
  }).$mount('#app')
})
```

Step 4. Now you can access myrustlib in any component you want. For example (in App.vue):
```js
...
<h2>Result is {{ getResult() }}</h2>
...
<script>
...
export default {
  ...
  methods: {
    getResult() {
      return this.$myrustlib.add(2, 3)
    },
  },
}
</script>
```
