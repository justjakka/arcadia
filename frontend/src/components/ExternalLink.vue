<template>
  <a target="_blank" :href="link"><img :src="'/logos/external_links/' + getLinkLogo()" /></a>
</template>

<script setup lang="ts">
const props = defineProps<{
  link: string
}>()
const getLinkLogo = () => {
  switch (true) {
    case props.link.includes('wikipedia.org'):
      return 'wikipedia.png'
    case props.link.includes('openlibrary.org'):
      return 'open-library.svg'
    case props.link.includes('musicbrainz.org'):
      return 'musicbrainz.svg'
    case props.link.includes('discogs.com'):
      return 'discogs.svg'
    case props.link.includes('imdb.com'):
      return 'imdb.svg'
    case props.link.includes('themoviedb.org'):
      return 'tmdb.svg'
    case props.link.includes('thetvdb.com'):
      return 'tvdb.svg'
    case props.link.includes('store.steampowered.com'):
      return 'steam.svg'
    case props.link.includes('comicvine.gamespot.com'):
      return 'comic_vine.svg'
    case props.link.includes('redacted.sh'):
      return 'redacted.ico'
    case props.link.includes('orpheus.network'):
      return 'orpheus.ico'
  }
}
</script>
<style scoped>
img {
  width: 30px;
}
a {
  background-color: transparent;
}
</style>
