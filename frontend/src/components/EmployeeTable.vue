<template>
  <table class="min-w-full bg-white/10 rounded-xl mt-6">
    <thead>
      <tr><th></th><th>Employee ID</th><th>Public Key</th></tr>
    </thead>
    <tbody>
      <tr v-for="row in rows" :key="row.emp_id" class="hover:bg-white/5">
        <td>
          <input type="checkbox"
                 :checked="selected?.emp_id === row.emp_id"
                 @change="$emit('select', row)" />
        </td>
        <td>{{ row.emp_id }}</td>
        <td class="font-mono break-all">{{ row.pub_key }}</td>
      </tr>
    </tbody>
  </table>
</template>

<script setup lang="ts">
defineProps<{ rows: any[]; selected: any }>();
defineEmits(['select']);
</script>
