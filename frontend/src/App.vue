<!-- src/App.vue -->
<template>
  <!-- full-screen brand gradient wrapper -->
  <div class="min-h-screen bg-gradient-to-br from-primary via-secondary to-accent">
    <!-- all routed pages render here -->
    <router-view />
  </div>
</template>

<script setup lang="ts">
// no script needed — routing handles page swaps
</script>
