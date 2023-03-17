<template>
  <main>
    <DataSourcesList />
    <DataSourceMap />
  </main>
</template>

<script>
import DataSourceMap from '@/components/DataSourceMap.vue';
import DataSourcesList from '@/components/DataSourcesList.vue';

export default {
  components: {
    DataSourcesList,
    DataSourceMap,
  },
};
</script>
