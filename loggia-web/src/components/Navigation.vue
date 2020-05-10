<template>
  <div id="nav">
    <router-link to="/">Home</router-link>
    |
    <router-link to="/about">About</router-link>
  </div>
</template>

<style scoped lang="scss">
  #nav {
    padding: 30px;
  }

  a {
    font-weight: bold;
    color: $text-color;
  }

  .router-link-exact-active {
    color: $primary-color;
  }
</style>
