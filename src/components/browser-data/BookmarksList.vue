<script setup lang="ts">
import { computed, ref } from "vue";

import type {
  BookmarkFilterField,
  BookmarkFilterPreset,
  BookmarkSortKey,
  BookmarkSummary,
  FilterMode,
  FilterRule,
  SortDirection,
} from "../../types/browser";
import FilterRules from "./FilterRules.vue";

const props = defineProps<{
  bookmarks: BookmarkSummary[];
  totalCount: number;
  sortKey: BookmarkSortKey;
  sortDirection: SortDirection;
  filterMode: FilterMode;
  filterPresets: BookmarkFilterPreset[];
  filterRules: FilterRule<BookmarkFilterField>[];
  selectedBookmarkUrls: string[];
  deleteBusy: boolean;
}>();

const emit = defineEmits<{
  "update:sortKey": [value: BookmarkSortKey];
  "update:filterMode": [value: FilterMode];
  "update:filterPresets": [value: BookmarkFilterPreset[]];
  "update:filterRules": [value: FilterRule<BookmarkFilterField>[]];
  showProfiles: [url: string];
  toggleBookmark: [url: string];
  toggleAllBookmarks: [];
  deleteBookmark: [url: string];
  deleteSelected: [];
}>();

const allSelected = computed(
  () =>
    props.bookmarks.length > 0 &&
    props.bookmarks.every((bookmark) => props.selectedBookmarkUrls.includes(bookmark.url)),
);
const filterOpen = ref(false);
const activeFilterCount = computed(
  () =>
    props.filterPresets.length +
    props.filterRules.filter((rule) => !rule.disabled && rule.value.trim().length > 0).length,
);

function isSelected(url: string) {
  return props.selectedBookmarkUrls.includes(url);
}

function sortIndicator(sortKey: BookmarkSortKey) {
  if (props.sortKey !== sortKey) return "↕";
  return props.sortDirection === "asc" ? "↑" : "↓";
}

const bookmarkFilterFields: { value: BookmarkFilterField; label: string }[] = [
  { value: "profileName", label: "Profile 名称" },
  { value: "profileId", label: "Profile ID" },
  { value: "bookmarkTitle", label: "书签名称" },
  { value: "url", label: "URL" },
];

const bookmarkFilterPresets: { value: BookmarkFilterPreset; label: string }[] = [
  { value: "exclude_meta", label: "不看 FB/IG/MSG" },
  { value: "google_only", label: "只看谷歌" },
];
</script>

<template>
  <section class="table-section">
    <div v-if="totalCount" class="data-table">
      <div class="bookmarks-toolbar">
        <label class="toolbar-checkbox" :class="{ disabled: !bookmarks.length }">
          <input
            type="checkbox"
            class="native-checkbox"
            :checked="allSelected"
            :disabled="!bookmarks.length || deleteBusy"
            @change="emit('toggleAllBookmarks')"
          />
          <span class="custom-checkbox" :class="{ checked: allSelected }" aria-hidden="true">
            <svg viewBox="0 0 16 16">
              <path d="M3.5 8.2L6.4 11.1L12.5 4.9" />
            </svg>
          </span>
          <span>全选</span>
        </label>
        <div class="toolbar-actions">
          <span class="toolbar-count" :class="{ filtered: bookmarks.length !== totalCount }">
            显示 {{ bookmarks.length }} / {{ totalCount }}
          </span>
          <button class="filter-button" type="button" @click="filterOpen = true">
            过滤
            <span v-if="activeFilterCount" class="filter-badge">{{ activeFilterCount }}</span>
          </button>
          <button
            class="danger-button toolbar-danger-button"
            type="button"
            :disabled="!selectedBookmarkUrls.length || deleteBusy"
            @click="emit('deleteSelected')"
          >
            {{ deleteBusy ? "删除中..." : `删除所选（${selectedBookmarkUrls.length}）` }}
          </button>
        </div>
      </div>

      <div class="data-table-header bookmarks-grid">
        <div class="header-cell checkbox-cell">选择</div>
        <button class="header-cell sortable" :class="{ active: sortKey === 'title' }" type="button" @click="emit('update:sortKey', 'title')">
          <span>名称</span>
          <span class="sort-indicator" :class="{ active: sortKey === 'title' }">{{ sortIndicator("title") }}</span>
        </button>
        <button class="header-cell sortable" :class="{ active: sortKey === 'url' }" type="button" @click="emit('update:sortKey', 'url')">
          <span>URL</span>
          <span class="sort-indicator" :class="{ active: sortKey === 'url' }">{{ sortIndicator("url") }}</span>
        </button>
        <div class="header-cell actions-cell">操作</div>
      </div>
      <div class="data-table-body styled-scrollbar">
        <div v-if="!bookmarks.length" class="filtered-empty">
          <p>没有符合当前过滤条件的书签。</p>
        </div>
        <article v-for="bookmark in bookmarks" :key="bookmark.url" class="data-table-row bookmarks-grid">
          <div class="row-cell checkbox-cell">
            <label class="table-checkbox" :class="{ disabled: deleteBusy }">
              <input
                type="checkbox"
                class="native-checkbox"
                :checked="isSelected(bookmark.url)"
                :disabled="deleteBusy"
                @change="emit('toggleBookmark', bookmark.url)"
              />
              <span class="custom-checkbox" :class="{ checked: isSelected(bookmark.url) }" aria-hidden="true">
                <svg viewBox="0 0 16 16">
                  <path d="M3.5 8.2L6.4 11.1L12.5 4.9" />
                </svg>
              </span>
            </label>
          </div>
          <div class="row-cell primary-cell">
            <strong :title="bookmark.title">{{ bookmark.title }}</strong>
          </div>
          <div class="row-cell muted-cell" :title="bookmark.url">{{ bookmark.url }}</div>
          <div class="row-cell actions-cell">
            <button class="disclosure-button" type="button" @click="emit('showProfiles', bookmark.url)">
              <span>查看</span>
              <span class="badge neutral">{{ bookmark.profileIds.length }}</span>
            </button>
            <button
              class="danger-button inline-danger-button"
              type="button"
              :disabled="deleteBusy"
              @click="emit('deleteBookmark', bookmark.url)"
            >
              删除
            </button>
          </div>
        </article>
      </div>
    </div>
    <div v-else class="empty-card">
      <p>这个浏览器没有扫描到任何书签。</p>
    </div>

    <FilterRules
      v-if="filterOpen"
      title="过滤书签"
      :mode="filterMode"
      :active-presets="filterPresets"
      :presets="bookmarkFilterPresets"
      :rules="filterRules"
      :fields="bookmarkFilterFields"
      @update:mode="emit('update:filterMode', $event)"
      @update:active-presets="emit('update:filterPresets', $event as BookmarkFilterPreset[])"
      @update:rules="emit('update:filterRules', $event)"
      @close="filterOpen = false"
    />
  </section>
</template>

<style scoped>
.table-section {
  padding: 0;
  height: 100%;
  min-height: 0;
}

.bookmarks-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px 8px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.1);
}

.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toolbar-count {
  padding: 3px 7px;
  border-radius: 9px;
  color: var(--muted);
  font-size: 0.82rem;
  white-space: nowrap;
}

.toolbar-count.filtered {
  background: rgba(239, 68, 68, 0.1);
  color: #b42318;
  font-weight: 700;
}

.data-table {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 22px;
  background: var(--panel);
  overflow: hidden;
}

.data-table-body {
  min-height: 0;
  overflow: auto;
  scrollbar-gutter: stable;
}

.filtered-empty {
  display: grid;
  min-height: 160px;
  place-items: center;
  padding: 24px;
  color: var(--muted);
  font-size: 0.9rem;
}

.bookmarks-grid {
  display: grid;
  grid-template-columns: 52px minmax(180px, 0.9fr) minmax(260px, 1.2fr) 250px;
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
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 0;
  text-align: left;
  background: transparent;
  cursor: pointer;
}

.header-cell.sortable.active {
  color: var(--text);
}

.sort-indicator {
  color: rgba(100, 116, 139, 0.42);
  font-size: 0.78rem;
  line-height: 1;
}

.sort-indicator.active {
  color: var(--accent);
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

.row-cell {
  min-width: 0;
}

.primary-cell strong {
  display: block;
  font-size: 0.93rem;
  line-height: 1.3;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.muted-cell {
  color: var(--muted);
  font-size: 0.87rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.disclosure-button {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  width: fit-content;
  min-width: 120px;
  padding: 5px 10px;
  border-radius: 10px;
  background: rgba(241, 245, 249, 0.9);
  color: var(--badge-text);
  font-size: 0.8rem;
  cursor: pointer;
}

.disclosure-button .badge {
  min-width: 24px;
  padding: 4px 8px;
  font-size: 0.76rem;
}

.actions-cell {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.checkbox-cell {
  display: flex;
  justify-content: center;
}

.inline-danger-button {
  padding: 5px 10px;
  border-radius: 10px;
  font-size: 0.8rem;
}

.toolbar-danger-button {
  padding: 6px 10px;
  border-radius: 10px;
}

.filter-button {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  border-radius: 10px;
  background: rgba(241, 245, 249, 0.9);
  color: var(--badge-text);
  font-size: 0.8rem;
  cursor: pointer;
}

.filter-badge {
  min-width: 20px;
  padding: 2px 6px;
  border-radius: 999px;
  background: rgba(47, 111, 237, 0.12);
  color: var(--accent);
  font-size: 0.72rem;
  font-weight: 700;
  text-align: center;
}

@media (max-width: 900px) {
  .bookmarks-grid {
    grid-template-columns: 52px minmax(160px, 0.9fr) minmax(200px, 1fr) 236px;
  }
}

@media (max-width: 720px) {
  .bookmarks-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .toolbar-actions {
    justify-content: space-between;
  }

  .bookmarks-grid {
    grid-template-columns: 52px minmax(0, 1fr) 152px;
  }

  .bookmarks-grid > :nth-child(3) {
    display: none;
  }
}
</style>
