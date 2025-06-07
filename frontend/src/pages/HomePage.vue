<template>
    <div class="h-screen flex flex-col justify-center items-center gap-8">
      <h1 class="text-4xl font-bold text-gradient">boost</h1>
  
      <div class="flex gap-6">
        <RouterLink to="/add"    class="btn-primary">Add employees</RouterLink>
        <RouterLink to="/verify" class="btn-primary">Verify wallet</RouterLink>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  /* purely presentational */
  </script>
  