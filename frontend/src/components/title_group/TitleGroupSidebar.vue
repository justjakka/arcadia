<template>
  <div id="title-group-sidebar">
    <Galleria
      :value="title_group.covers"
      :numVisible="1"
      :circular="true"
      :showItemNavigators="false"
      :showThumbnails="false"
      :showIndicators="true"
      class="carousel"
    >
      <template #item="slotProps">
        <Image :src="slotProps.item" preview>
          <template #previewicon>
            <i class="pi pi-search"></i>
          </template>
        </Image>
      </template>
    </Galleria>
    <ContentContainer :container-title="t('general.link', 2)">
      <div class="external-links links">
        <ExternalLink v-for="link in title_group.external_links" :key="link" :link="link" />
      </div>
    </ContentContainer>
    <ContentContainer :container-title="t('artist.artist', 2)">
      <template #top-right><i class="pi pi-pen-to-square cursor-pointer" @click="emit('editAffiliatedArtistsClicked')" /></template>
      <template v-if="title_group.affiliated_artists.length != 0">
        <div class="affiliated-artists">
          <AffiliatedArtist class="affiliated-artist" v-for="artist in title_group.affiliated_artists" :key="artist.artist_id" :affiliated_artist="artist" />
        </div>
      </template>
      <div class="wrapper-center" v-else>
        {{ t('artist.no_affiliated_artist_registered') }}
      </div>
    </ContentContainer>
    <ContentContainer :container-title="t('entity.entity', 2)" v-if="title_group.affiliated_entities.length != 0">
      <!-- <template #top-right><i class="pi pi-pen-to-square cursor-pointer" @click="emit('editAffiliatedArtistsClicked')" /></template> -->
      <div class="affiliated-entities">
        <AffiliatedEntity class="affiliated-entity" v-for="entity in title_group.affiliated_entities" :key="entity.entity_id" :affiliatedEntity="entity" />
      </div>
    </ContentContainer>
    <ContentContainer
      :container-title="`${t('master_group.in_same_master_group')} (${title_group.master_group_id})`"
      v-if="title_group.in_same_master_group.length != 0"
    >
      <div class="flex justify-content-center links">
        <MasterGroupLink v-for="tg in title_group.in_same_master_group" :key="tg.id" :title_group="tg" />
      </div>
    </ContentContainer>
    <ContentContainer :container-title="t('general.series')" v-if="title_group.series.id">
      <RouterLink :to="`/series/${title_group.series.id}`">
        {{ title_group.series.name }}
      </RouterLink>
    </ContentContainer>
    <ContentContainer :container-title="t('general.tags')">
      <div class="tags">
        <div v-for="tag in title_group.tags" :key="tag">{{ tag }}</div>
      </div>
    </ContentContainer>
  </div>
</template>
<script setup lang="ts">
import { Galleria } from 'primevue'
import Image from 'primevue/image'
import AffiliatedArtist from '@/components/artist/AffiliatedArtist.vue'
import ExternalLink from '@/components/ExternalLink.vue'
import MasterGroupLink from '@/components/MasterGroupLink.vue'
import ContentContainer from '../ContentContainer.vue'
import type { TitleGroupAndAssociatedData } from '@/services/api/torrentService'
import { useI18n } from 'vue-i18n'
import AffiliatedEntity from '../artist/AffiliatedEntity.vue'

const { t } = useI18n()

const emit = defineEmits<{
  editAffiliatedArtistsClicked: []
}>()

defineProps<{
  title_group: TitleGroupAndAssociatedData
}>()
</script>
<style scoped>
#title-group-sidebar {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  .content-container {
    width: 100%;
    margin-top: 20px;
  }
}
.p-galleria {
  border: none;
}
.affiliated-artists {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  .affiliated-artist {
    margin: 0px 4px;
    margin-bottom: 15px;
  }
}
.links {
  a {
    margin: 0px 5px;
  }
}
.external-links {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
<style>
#title-group-sidebar .p-galleria-content img {
  /* height: 20em !important; */
  width: 100% !important;
  border-radius: 7px;
}
</style>
