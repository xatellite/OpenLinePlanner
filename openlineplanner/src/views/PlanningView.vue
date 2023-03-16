<template>
  <main>
    <LineList />
    <InteractiveMap />
  </main>
</template>

<script>
import InteractiveMap from '@/components/InteractiveMap.vue';
import LineList from '@/components/LineList.vue';

export default {
  components: {
    LineList,
    InteractiveMap,
  },
};
</script>
