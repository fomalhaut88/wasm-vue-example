import Vue from 'vue'
import App from './App.vue'

Vue.config.productionTip = false

import('myrustlib').then(myrustlib => {
  Vue.prototype.$myrustlib = myrustlib

  new Vue({
    render: h => h(App)
  }).$mount('#app')
})
