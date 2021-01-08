<template>
  <header>
    <div class='header-inner'>
      <div class='header-left'>
        <img class='logo' src="@/assets/CHARM_logo.png">
      </div>
      <div class='header-middle'>
        <p>Catalogue</p>
      </div>
      <div class='header-right'>
      </div>
    </div>
  </header>

</template>

<script>
export default {
  name: 'Header'
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.header-inner {
  background: black;
  padding-left: 80px;
  padding-right: 80px;
  padding-top: 15px;
}

.logo {
  height: 60px;
  padding: 6.5px 1px;
}

header > div {
  display: flex;
  justify-content: space-between;
}
p {
  color: white;
  font-size: 2em;
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
