<template>
  <div class="min-h-screen bg-gradient-to-br from-primary via-secondary to-accent">
    <div class="flex justify-between items-center p-6 text-logo text-2xl font-bold">
      boost
      <WalletConnect />
    </div>

    <div class="max-w-3xl mx-auto p-4">
      <EmployeeEntry @submitted="fetchRows" />
      <EmployeeTable :rows="rows" :selected="selected" @select="selectRow" />
      <WalletCheck v-if="selected" :emp="selected" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import axios from 'axios';

import WalletConnect   from './components/WalletConnect.vue';
import EmployeeEntry   from './components/EmployeeEntry.vue';
import EmployeeTable   from './components/EmployeeTable.vue';
import WalletCheck     from './components/WalletCheck.vue';

const rows = ref<any[]>([]);
const selected = ref<any | null>(null);

async function fetchRows() {
  const { data } = await axios.get('http://localhost:3001/api/employees');
  rows.value = data;
  selected.value = null;
}

function selectRow(row: any) { selected.value = row; }

onMounted(fetchRows);
</script>
