<template>
  <form @submit.prevent="save" class="space-y-4 bg-white/10 p-4 rounded-xl">
    <input v-model="emp_id" maxlength="8" pattern="\\d{8}"
           class="input" placeholder="Employee ID (8 digits)" required />
    <input v-model="pub_key"
           class="input ml-2" placeholder="Solana Pubkey" required />
    <button class="btn ml-2">Save</button>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import axios from 'axios';

const emp_id = ref('');
const pub_key = ref('');
const emit = defineEmits(['submitted']);

async function save() {
  if (!/^\d{8}$/.test(emp_id.value)) {
    alert('Emp ID must be 8 digits');
    return;
  }
  await axios.post('http://localhost:3001/api/employees', {
    emp_id: emp_id.value,
    pub_key: pub_key.value,
  });
  emp_id.value = '';
  pub_key.value = '';
  emit('submitted');
}
</script>

<style scoped>
.input { @apply rounded-md p-2 border; }
.btn   { @apply bg-logo text-white rounded-md px-4 py-2; }
</style>
