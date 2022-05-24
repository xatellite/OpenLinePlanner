<template>
  <svg
    width="20"
    height="18"
    viewBox="0 0 20 18"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      d="M2 16L4.16958 9.76245C5.07039 7.17264 8.24288 6.2253 10.4163 7.89712V7.89712C11.5734 8.78726 13.2303 8.58718 14.1423 7.44716L18.5 2"
      :stroke="color"
      stroke-width="3"
      stroke-linecap="round"
    />
  </svg>
</template>

<script>
export default {
  props: {
    color: String,
  },
};
</script>
