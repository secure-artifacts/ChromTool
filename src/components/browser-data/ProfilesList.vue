<script setup lang="ts">
import { computed } from "vue";
import type { ProfileSortKey, ProfileSummary } from "../../types/browser";
import { profileAvatarSrc } from "../../utils/icons";

const props = defineProps<{
  profiles: ProfileSummary[];
  sortKey: ProfileSortKey;
  openProfileError: string;
  browserId: string;
  browserFamilyId: string | null;
  selectedProfileIds: string[];
  openSelectedBusy: boolean;
  isOpeningProfile: (browserId: string, profileId: string) => boolean;
}>();

const emit = defineEmits<{
  "update:sortKey": [value: ProfileSortKey];
  openProfile: [browserId: string, profileId: string];
  toggleProfile: [profileId: string];
  toggleAllProfiles: [];
  openSelected: [];
}>();

const allSelected = computed(
  () =>
    props.profiles.length > 0 &&
    props.profiles.every((profile) => props.selectedProfileIds.includes(profile.id)),
);

function isSelected(profileId: string) {
  return props.selectedProfileIds.includes(profileId);
}
</script>

<template>
  <section class="table-section">
    <div v-if="openProfileError" class="inline-error">
      {{ openProfileError }}
    </div>

    <div v-if="profiles.length" class="data-table">
      <div class="profiles-toolbar">
        <label class="toolbar-checkbox" :class="{ disabled: !profiles.length }">
          <input
            type="checkbox"
            class="native-checkbox"
            :checked="allSelected"
            :disabled="!profiles.length || openSelectedBusy"
            @change="emit('toggleAllProfiles')"
          />
          <span class="custom-checkbox" :class="{ checked: allSelected }" aria-hidden="true">
            <svg viewBox="0 0 16 16">
              <path d="M3.5 8.2L6.4 11.1L12.5 4.9" />
            </svg>
          </span>
          <span>全选</span>
        </label>
        <button
          class="card-action-button toolbar-action-button"
          type="button"
          :disabled="!selectedProfileIds.length || openSelectedBusy"
          @click="emit('openSelected')"
        >
          {{ openSelectedBusy ? "打开中..." : `打开所选（${selectedProfileIds.length}）` }}
        </button>
      </div>

      <div class="data-table-header profiles-grid">
        <div class="header-cell checkbox-cell">选择</div>
        <div class="header-cell icon-cell">头像</div>
        <button class="header-cell sortable" :class="{ active: sortKey === 'name' }" type="button" @click="emit('update:sortKey', 'name')">名称</button>
        <button class="header-cell sortable" :class="{ active: sortKey === 'email' }" type="button" @click="emit('update:sortKey', 'email')">邮箱</button>
        <button class="header-cell sortable" :class="{ active: sortKey === 'id' }" type="button" @click="emit('update:sortKey', 'id')">资料 ID</button>
        <div class="header-cell actions-cell">操作</div>
      </div>
      <div class="data-table-body styled-scrollbar">
        <article v-for="profile in profiles" :key="profile.id" class="data-table-row profiles-grid">
          <div class="row-cell checkbox-cell">
            <label class="table-checkbox" :class="{ disabled: openSelectedBusy }">
              <input
                type="checkbox"
                class="native-checkbox"
                :checked="isSelected(profile.id)"
                :disabled="openSelectedBusy"
                @change="emit('toggleProfile', profile.id)"
              />
              <span class="custom-checkbox" :class="{ checked: isSelected(profile.id) }" aria-hidden="true">
                <svg viewBox="0 0 16 16">
                  <path d="M3.5 8.2L6.4 11.1L12.5 4.9" />
                </svg>
              </span>
            </label>
          </div>
          <div class="profile-avatar table-avatar">
            <img
              v-if="profileAvatarSrc(profile, browserFamilyId)"
              :src="profileAvatarSrc(profile, browserFamilyId) ?? undefined"
              :alt="`${profile.name} avatar`"
            />
            <span v-else>{{ profile.avatarLabel }}</span>
          </div>
          <div class="row-cell primary-cell">
            <strong>{{ profile.name }}</strong>
          </div>
          <div class="row-cell muted-cell" :title="profile.email ?? undefined">
            {{ profile.email || "" }}
          </div>
          <div class="row-cell">
            <span class="badge neutral">{{ profile.id }}</span>
          </div>
          <div class="row-cell actions-cell">
            <button
              class="card-action-button"
              :disabled="openSelectedBusy || isOpeningProfile(browserId, profile.id)"
              type="button"
              @click="emit('openProfile', browserId, profile.id)"
            >
              {{ isOpeningProfile(browserId, profile.id) ? "打开中..." : "打开" }}
            </button>
          </div>
        </article>
      </div>
    </div>
    <div v-else class="empty-card">
      <p>这个浏览器没有找到任何用户资料目录。</p>
    </div>
  </section>
</template>

<style scoped>
.table-section {
  padding: 0;
  height: 100%;
  min-height: 0;
}

.data-table {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-width: 0;
  min-height: 0;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 22px;
  background: var(--panel);
  box-shadow: var(--shadow);
  overflow: hidden;
}

.data-table-body {
  min-height: 0;
  overflow: auto;
  scrollbar-gutter: stable;
}

.profiles-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px 8px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.1);
}

.profiles-grid {
  display: grid;
  grid-template-columns: 52px 64px minmax(180px, 1.2fr) minmax(180px, 1fr) 132px 110px;
  gap: 12px;
  align-items: center;
}

.data-table-header {
  position: sticky;
  top: 0;
  z-index: 2;
  padding: 8px 24px 8px 12px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.14);
  background: rgba(248, 250, 252, 0.94);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.header-cell {
  color: var(--muted);
  font-size: 0.81rem;
  font-weight: 700;
  letter-spacing: 0.02em;
}

.header-cell.sortable {
  padding: 0;
  text-align: left;
  background: transparent;
  cursor: pointer;
}

.header-cell.sortable.active {
  color: var(--text);
}

.toolbar-checkbox {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  color: var(--text);
  font-size: 0.88rem;
  cursor: pointer;
}

.toolbar-checkbox.disabled {
  opacity: 0.55;
  cursor: default;
}

.native-checkbox {
  position: absolute;
  opacity: 0;
  pointer-events: none;
}

.table-checkbox {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.table-checkbox.disabled {
  cursor: default;
  opacity: 0.5;
}

.custom-checkbox {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: 1px solid rgba(148, 163, 184, 0.34);
  border-radius: 7px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.98), rgba(241, 245, 249, 0.92));
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.78),
    0 4px 10px rgba(15, 23, 42, 0.06);
}

.custom-checkbox svg {
  width: 12px;
  height: 12px;
}

.custom-checkbox path {
  fill: none;
  stroke: #fff;
  stroke-width: 2.2;
  stroke-linecap: round;
  stroke-linejoin: round;
  opacity: 0;
}

.custom-checkbox.checked {
  border-color: rgba(47, 111, 237, 0.2);
  background: linear-gradient(135deg, #2f6fed, #5aa1f7);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.22),
    0 8px 18px rgba(47, 111, 237, 0.22);
}

.custom-checkbox.checked path {
  opacity: 1;
}

.data-table-row {
  padding: 10px 12px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.12);
}

.data-table-row:last-child {
  border-bottom: 0;
}

.data-table-row:hover {
  background: rgba(248, 250, 252, 0.65);
}

.profile-avatar {
  display: grid;
  place-items: center;
  flex-shrink: 0;
  background: linear-gradient(135deg, #dbeafe, #eff6ff);
  color: #1d4ed8;
  font-size: 0.96rem;
  font-weight: 700;
  overflow: hidden;
}

.table-avatar {
  width: 36px;
  height: 36px;
  border-radius: 999px;
}

.profile-avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.row-cell {
  min-width: 0;
}

.primary-cell strong {
  display: block;
  font-size: 0.93rem;
  line-height: 1.3;
}

.muted-cell {
  color: var(--muted);
  font-size: 0.86rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.actions-cell {
  display: flex;
  justify-content: flex-end;
}

.checkbox-cell {
  display: flex;
  justify-content: center;
}

.toolbar-action-button {
  padding: 6px 10px;
  border-radius: 10px;
  font-size: 0.84rem;
}

.icon-cell {
  padding-left: 4px;
}

@media (max-width: 900px) {
  .profiles-grid {
    grid-template-columns: 52px 56px minmax(140px, 1fr) minmax(140px, 1fr) 110px 96px;
  }
}

@media (max-width: 720px) {
  .profiles-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .profiles-grid {
    grid-template-columns: 40px minmax(0, 1fr) 72px;
    gap: 8px;
  }

  .data-table-header,
  .data-table-row {
    padding-left: 8px;
    padding-right: 12px;
  }

  .profiles-grid > :nth-child(2),
  .profiles-grid > :nth-child(4),
  .profiles-grid > :nth-child(5) {
    display: none;
  }
}
</style>
