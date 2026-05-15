<script setup lang="ts">
import type {
  ActiveSection,
  AssociatedProfileSummary,
  BookmarkAssociatedProfileSummary,
  BookmarkSortKey,
  BrowserView,
  ExtensionSortKey,
  CleanupHistoryResult,
  ExtensionAssociatedProfileSummary,
  RemoveBookmarkResult,
  PasswordSiteSortKey,
  ProfileSortKey,
  RemoveExtensionResult,
} from "../../types/browser";
import AssociatedProfilesModal from "./AssociatedProfilesModal.vue";
import BookmarkRemovalModal from "./BookmarkRemovalModal.vue";
import BookmarksList from "./BookmarksList.vue";
import ExtensionRemovalModal from "./ExtensionRemovalModal.vue";
import ExtensionsList from "./ExtensionsList.vue";
import HistoryCleanupList from "./HistoryCleanupList.vue";
import HistoryCleanupModal from "./HistoryCleanupModal.vue";
import PasswordSitesList from "./PasswordSitesList.vue";
import ProfilesList from "./ProfilesList.vue";

defineProps<{
  currentBrowser: BrowserView;
  activeSection: ActiveSection;
  profileSortKey: ProfileSortKey;
  extensionSortKey: ExtensionSortKey;
  bookmarkSortKey: BookmarkSortKey;
  passwordSiteSortKey: PasswordSiteSortKey;
  passwordSitesLoaded: boolean;
  passwordSitesLoading: boolean;
  passwordSitesError: string;
  sortedProfiles: BrowserView["profiles"];
  sortedExtensions: BrowserView["extensions"];
  sortedBookmarks: BrowserView["bookmarks"];
  sortedPasswordSites: BrowserView["passwordSites"];
  profileSelectedIds: string[];
  openingSelectedProfiles: boolean;
  historySelectedProfileIds: string[];
  cleanupHistoryBusy: boolean;
  historyCleanupConfirmProfiles: BrowserView["profiles"];
  historyCleanupResultOpen: boolean;
  cleanupHistoryError: string;
  cleanupHistoryResults: CleanupHistoryResult[];
  bookmarkSelectedUrls: string[];
  bookmarkModalSelectedProfileIds: string[];
  bookmarkDeleteBusy: boolean;
  bookmarkRemovalConfirmBookmarkCount: number;
  bookmarkRemovalConfirmProfileCount: number;
  bookmarkRemovalResultOpen: boolean;
  bookmarkRemovalError: string;
  bookmarkRemovalResults: RemoveBookmarkResult[];
  extensionSelectedIds: string[];
  extensionModalSelectedProfileIds: string[];
  extensionDeleteBusy: boolean;
  extensionRemovalConfirmExtensions: BrowserView["extensions"];
  extensionRemovalConfirmProfiles: BrowserView["profiles"];
  extensionRemovalResultOpen: boolean;
  extensionRemovalError: string;
  extensionRemovalResults: RemoveExtensionResult[];
  openProfileError: string;
  sectionCount: (section: ActiveSection) => number;
  isOpeningProfile: (browserId: string, profileId: string) => boolean;
  extensionMonogram: (name: string) => string;
  associatedProfilesModal: {
    title: string;
    browserId: string;
    profiles: (
      | AssociatedProfileSummary
      | BookmarkAssociatedProfileSummary
      | ExtensionAssociatedProfileSummary
    )[];
    isBookmark: boolean;
    isExtension?: boolean;
    extensionId?: string;
    bookmarkUrl?: string;
  } | null;
}>();

const emit = defineEmits<{
  "update:activeSection": [value: ActiveSection];
  "update:profileSortKey": [value: ProfileSortKey];
  "update:extensionSortKey": [value: ExtensionSortKey];
  "update:bookmarkSortKey": [value: BookmarkSortKey];
  "update:passwordSiteSortKey": [value: PasswordSiteSortKey];
  loadPasswordSites: [];
  openProfile: [browserId: string, profileId: string];
  toggleProfileSelection: [profileId: string];
  toggleAllProfiles: [];
  openSelectedProfiles: [];
  showExtensionProfiles: [extensionId: string];
  showBookmarkProfiles: [url: string];
  showPasswordSiteProfiles: [url: string];
  toggleBookmarkSelection: [url: string];
  toggleAllBookmarks: [];
  deleteBookmarkFromAllProfiles: [url: string];
  deleteSelectedBookmarks: [];
  toggleBookmarkModalProfileSelection: [profileId: string];
  toggleAllBookmarkModalProfiles: [];
  deleteBookmarkFromProfile: [profileId: string];
  deleteSelectedBookmarkProfiles: [];
  confirmBookmarkRemoval: [];
  closeBookmarkRemovalConfirm: [];
  closeBookmarkRemovalResult: [];
  toggleExtensionSelection: [extensionId: string];
  toggleAllExtensions: [];
  deleteExtensionFromAllProfiles: [extensionId: string];
  deleteSelectedExtensions: [];
  toggleExtensionModalProfileSelection: [profileId: string];
  toggleAllExtensionModalProfiles: [];
  deleteExtensionFromProfile: [profileId: string];
  deleteSelectedExtensionProfiles: [];
  confirmExtensionRemoval: [];
  closeExtensionRemovalConfirm: [];
  closeExtensionRemovalResult: [];
  toggleHistoryProfile: [profileId: string];
  toggleAllHistoryProfiles: [];
  cleanupSelectedHistory: [];
  cleanupHistoryForProfile: [profileId: string];
  confirmHistoryCleanup: [];
  closeHistoryCleanupConfirm: [];
  closeHistoryCleanupResult: [];
  closeAssociatedProfiles: [];
}>();
</script>

<template>
  <section class="section-tabs">
    <button
      class="section-tab"
      :class="{ active: activeSection === 'profiles' }"
      type="button"
      @click="emit('update:activeSection', 'profiles')"
    >
      <span>资料</span>
      <span class="count-pill">{{ sectionCount("profiles") }}</span>
    </button>
    <button
      class="section-tab"
      :class="{ active: activeSection === 'extensions' }"
      type="button"
      @click="emit('update:activeSection', 'extensions')"
    >
      <span>插件</span>
      <span class="count-pill">{{ sectionCount("extensions") }}</span>
    </button>
    <button
      class="section-tab"
      :class="{ active: activeSection === 'bookmarks' }"
      type="button"
      @click="emit('update:activeSection', 'bookmarks')"
    >
      <span>书签</span>
      <span class="count-pill">{{ sectionCount("bookmarks") }}</span>
    </button>
    <button
      class="section-tab"
      :class="{ active: activeSection === 'passwords' }"
      type="button"
      @click="emit('update:activeSection', 'passwords')"
    >
      <span>密码</span>
      <span class="count-pill">{{ sectionCount("passwords") }}</span>
    </button>
    <button
      class="section-tab"
      :class="{ active: activeSection === 'history' }"
      type="button"
      @click="emit('update:activeSection', 'history')"
    >
      <span>历史</span>
      <span class="count-pill">{{ sectionCount("history") }}</span>
    </button>
  </section>

  <div class="content-scroll-area">
    <ProfilesList
      v-if="activeSection === 'profiles'"
      :profiles="sortedProfiles"
      :sort-key="profileSortKey"
      :open-profile-error="openProfileError"
      :browser-id="currentBrowser.browserId"
      :browser-family-id="currentBrowser.browserFamilyId"
      :selected-profile-ids="profileSelectedIds"
      :open-selected-busy="openingSelectedProfiles"
      :is-opening-profile="isOpeningProfile"
      @update:sort-key="emit('update:profileSortKey', $event)"
      @open-profile="(browserId, profileId) => emit('openProfile', browserId, profileId)"
      @toggle-profile="emit('toggleProfileSelection', $event)"
      @toggle-all-profiles="emit('toggleAllProfiles')"
      @open-selected="emit('openSelectedProfiles')"
    />

    <ExtensionsList
      v-else-if="activeSection === 'extensions'"
      :extensions="sortedExtensions"
      :sort-key="extensionSortKey"
      :extension-monogram="extensionMonogram"
      :selected-extension-ids="extensionSelectedIds"
      :delete-busy="extensionDeleteBusy"
      @update:sort-key="emit('update:extensionSortKey', $event)"
      @show-profiles="emit('showExtensionProfiles', $event)"
      @toggle-extension="emit('toggleExtensionSelection', $event)"
      @toggle-all-extensions="emit('toggleAllExtensions')"
      @delete-extension="emit('deleteExtensionFromAllProfiles', $event)"
      @delete-selected="emit('deleteSelectedExtensions')"
    />

    <BookmarksList
      v-else-if="activeSection === 'bookmarks'"
      :bookmarks="sortedBookmarks"
      :sort-key="bookmarkSortKey"
      :selected-bookmark-urls="bookmarkSelectedUrls"
      :delete-busy="bookmarkDeleteBusy"
      @update:sort-key="emit('update:bookmarkSortKey', $event)"
      @show-profiles="emit('showBookmarkProfiles', $event)"
      @toggle-bookmark="emit('toggleBookmarkSelection', $event)"
      @toggle-all-bookmarks="emit('toggleAllBookmarks')"
      @delete-bookmark="emit('deleteBookmarkFromAllProfiles', $event)"
      @delete-selected="emit('deleteSelectedBookmarks')"
    />

    <PasswordSitesList
      v-else-if="activeSection === 'passwords'"
      :password-sites="sortedPasswordSites"
      :sort-key="passwordSiteSortKey"
      :loaded="passwordSitesLoaded"
      :loading="passwordSitesLoading"
      :error="passwordSitesError"
      @update:sort-key="emit('update:passwordSiteSortKey', $event)"
      @load="emit('loadPasswordSites')"
      @show-profiles="emit('showPasswordSiteProfiles', $event)"
    />

    <HistoryCleanupList
      v-else
      :browser-family-id="currentBrowser.browserFamilyId"
      :profiles="sortedProfiles"
      :selected-profile-ids="historySelectedProfileIds"
      :cleanup-busy="cleanupHistoryBusy"
      @toggle-profile="emit('toggleHistoryProfile', $event)"
      @toggle-all-profiles="emit('toggleAllHistoryProfiles')"
      @cleanup-selected="emit('cleanupSelectedHistory')"
      @cleanup-profile="emit('cleanupHistoryForProfile', $event)"
    />
  </div>

  <HistoryCleanupModal
    v-if="historyCleanupConfirmProfiles.length"
    mode="confirm"
    title="确认清理历史"
    :profiles="historyCleanupConfirmProfiles"
    :results="[]"
    :busy="cleanupHistoryBusy"
    @close="emit('closeHistoryCleanupConfirm')"
    @confirm="emit('confirmHistoryCleanup')"
  />

  <HistoryCleanupModal
    v-if="historyCleanupResultOpen"
    mode="result"
    title="清理结果"
    :profiles="[]"
    :results="cleanupHistoryResults"
    :general-error="cleanupHistoryError"
    @close="emit('closeHistoryCleanupResult')"
  />

  <BookmarkRemovalModal
    v-if="bookmarkRemovalConfirmBookmarkCount > 0"
    mode="confirm"
    title="确认删除书签"
    :bookmark-count="bookmarkRemovalConfirmBookmarkCount"
    :profile-count="bookmarkRemovalConfirmProfileCount"
    :results="[]"
    :busy="bookmarkDeleteBusy"
    @close="emit('closeBookmarkRemovalConfirm')"
    @confirm="emit('confirmBookmarkRemoval')"
  />

  <BookmarkRemovalModal
    v-if="bookmarkRemovalResultOpen"
    mode="result"
    title="书签删除结果"
    :bookmark-count="0"
    :profile-count="0"
    :results="bookmarkRemovalResults"
    :general-error="bookmarkRemovalError"
    @close="emit('closeBookmarkRemovalResult')"
  />

  <ExtensionRemovalModal
    v-if="extensionRemovalConfirmExtensions.length || extensionRemovalConfirmProfiles.length"
    mode="confirm"
    title="确认删除插件"
    :extensions="extensionRemovalConfirmExtensions"
    :profiles="extensionRemovalConfirmProfiles"
    :results="[]"
    :busy="extensionDeleteBusy"
    @close="emit('closeExtensionRemovalConfirm')"
    @confirm="emit('confirmExtensionRemoval')"
  />

  <ExtensionRemovalModal
    v-if="extensionRemovalResultOpen"
    mode="result"
    title="插件删除结果"
    :extensions="[]"
    :profiles="[]"
    :results="extensionRemovalResults"
    :general-error="extensionRemovalError"
    @close="emit('closeExtensionRemovalResult')"
  />

  <AssociatedProfilesModal
    v-if="associatedProfilesModal"
    :title="associatedProfilesModal.title"
    :profiles="associatedProfilesModal.profiles"
    :browser-id="associatedProfilesModal.browserId"
    :browser-family-id="currentBrowser.browserFamilyId"
    :is-bookmark="associatedProfilesModal.isBookmark"
    :is-extension="associatedProfilesModal.isExtension"
    :selected-profile-ids="
      associatedProfilesModal.isExtension
        ? extensionModalSelectedProfileIds
        : bookmarkModalSelectedProfileIds
    "
    :delete-busy="associatedProfilesModal.isExtension ? extensionDeleteBusy : bookmarkDeleteBusy"
    :is-opening-profile="isOpeningProfile"
    @close="emit('closeAssociatedProfiles')"
    @open-profile="(browserId, profileId) => emit('openProfile', browserId, profileId)"
    @toggle-profile-selection="
      associatedProfilesModal.isExtension
        ? emit('toggleExtensionModalProfileSelection', $event)
        : emit('toggleBookmarkModalProfileSelection', $event)
    "
    @toggle-all-profile-selection="
      associatedProfilesModal.isExtension
        ? emit('toggleAllExtensionModalProfiles')
        : emit('toggleAllBookmarkModalProfiles')
    "
    @delete-profile="
      associatedProfilesModal.isExtension
        ? emit('deleteExtensionFromProfile', $event)
        : emit('deleteBookmarkFromProfile', $event)
    "
    @delete-selected-profiles="
      associatedProfilesModal.isExtension
        ? emit('deleteSelectedExtensionProfiles')
        : emit('deleteSelectedBookmarkProfiles')
    "
  />
</template>

<style scoped>
.section-tabs {
  display: flex;
  gap: 10px;
  margin-top: 0;
  padding: 6px;
  flex-shrink: 0;
  min-width: 0;
  overflow-x: auto;
  overflow-y: hidden;
  scrollbar-width: thin;
  border: 1px solid var(--panel-border);
  border-radius: 22px;
  background: var(--panel);
  box-shadow: var(--shadow);
}

.section-tabs::-webkit-scrollbar {
  height: 8px;
}

.section-tabs::-webkit-scrollbar-track {
  background: transparent;
}

.section-tabs::-webkit-scrollbar-thumb {
  border: 2px solid transparent;
  border-radius: 999px;
  background: rgba(100, 116, 139, 0.36);
  background-clip: padding-box;
}

.section-tab {
  display: inline-flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-width: 104px;
  flex: 1 0 104px;
  padding: 8px 10px;
  border-radius: 13px;
  color: var(--muted);
  background: rgba(255, 255, 255, 0.58);
  cursor: pointer;
  transition:
    background 160ms ease,
    color 160ms ease,
    transform 160ms ease,
    box-shadow 160ms ease;
}

.section-tab > span:first-child {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.section-tab:hover {
  transform: translateY(-1px);
}

.section-tab.active {
  color: var(--text);
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.96), rgba(232, 240, 255, 0.92));
  box-shadow: 0 12px 24px rgba(15, 23, 42, 0.08);
}

.content-scroll-area {
  display: flex;
  min-width: 0;
  overflow: hidden;
}

.content-scroll-area > * {
  flex: 1;
  min-width: 0;
  min-height: 0;
}
</style>
