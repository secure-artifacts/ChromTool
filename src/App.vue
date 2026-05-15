<script setup lang="ts">
import { nextTick } from "vue";
import BrowserDataView from "./components/browser-data/BrowserDataView.vue";
import ConfigurationView from "./components/config/ConfigurationView.vue";
import AppSidebar from "./components/sidebar/AppSidebar.vue";
import { useBrowserManager } from "./composables/useBrowserManager";

const appVersion = __APP_VERSION__;

const {
  activeSection,
  associatedProfilesModal,
  bookmarkSortKey,
  bookmarkDeleteBusy,
  bookmarkModalSelectedProfileIds,
  bookmarkRemovalConfirmBookmarkCount,
  bookmarkRemovalConfirmProfileCount,
  bookmarkRemovalError,
  bookmarkRemovalResultOpen,
  bookmarkRemovalResults,
  bookmarkSelectedUrls,
  browserConfigs,
  browserMonogram,
  browsers,
  configError,
  configMonogram,
  configsLoading,
  createConfigForm,
  createCustomBrowserConfig,
  deleteBookmarkFromAllProfiles,
  deleteBookmarkFromProfile,
  cleanupHistoryError,
  cleanupHistoryForProfile,
  cleanupHistoryResults,
  cleanupHistorySelectedProfiles,
  cleanupSelectedHistoryProfiles,
  closeHistoryCleanupConfirm,
  closeHistoryCleanupResult,
  confirmHistoryCleanup,
  currentBrowser,
  deleteCustomBrowserConfig,
  deleteSelectedBookmarkProfiles,
  deleteSelectedBookmarks,
  deleteExtensionFromAllProfiles,
  deleteExtensionFromProfile,
  deleteSelectedExtensionProfiles,
  deleteSelectedExtensions,
  error,
  extensionMonogram,
  extensionDeleteBusy,
  extensionModalSelectedProfileIds,
  extensionRemovalConfirmExtensions,
  extensionRemovalConfirmProfiles,
  extensionRemovalError,
  extensionRemovalResultOpen,
  extensionRemovalResults,
  extensionSelectedIds,
  extensionSortKey,
  historyCleanupBusy,
  historyCleanupConfirmProfiles,
  historyCleanupResultOpen,
  isDeletingConfig,
  isOpeningProfile,
  loading,
  loadPasswordSites,
  openProfileError,
  openBrowserProfile,
  openSelectedProfiles,
  openingSelectedProfiles,
  page,
  pickExecutablePath,
  pickUserDataPath,
  passwordSiteSortKey,
  passwordSitesError,
  passwordSitesLoading,
  profileSelectedIds,
  profileSortKey,
  refreshCurrentBrowser,
  savingConfig,
  sectionCount,
  selectedBrowserId,
  hasLoadedPasswordSites,
  closeBookmarkRemovalConfirm,
  closeBookmarkRemovalResult,
  showBookmarkProfilesModal,
  showExtensionProfilesModal,
  showPasswordSiteProfilesModal,
  sortedBookmarks,
  sortedExtensions,
  sortedPasswordSites,
  sortedProfiles,
  confirmBookmarkRemoval,
  closeExtensionRemovalConfirm,
  closeExtensionRemovalResult,
  confirmExtensionRemoval,
  toggleAllBookmarks,
  toggleAllProfiles,
  toggleAllBookmarkModalProfiles,
  toggleAllExtensions,
  toggleBookmarkModalProfileSelection,
  toggleBookmarkSelection,
  toggleProfileSelection,
  toggleAllExtensionModalProfiles,
  toggleExtensionModalProfileSelection,
  toggleExtensionSelection,
  toggleAllHistoryProfiles,
  toggleHistoryProfile,
  closeAssociatedProfilesModal,
} = useBrowserManager();

const tableScrollPositions = new Map<string, { left: number; top: number }>();

function activeTableScrollKey() {
  const browserId = currentBrowser.value?.browserId ?? selectedBrowserId.value;
  if (!browserId || page.value !== "browserData") return "";

  return `${browserId}:${activeSection.value}`;
}

function currentTableScrollTarget() {
  return document.querySelector<HTMLElement>(".content-scroll-area .data-table-body");
}

function saveActiveTableScroll() {
  const key = activeTableScrollKey();
  const scrollTarget = currentTableScrollTarget();
  if (!key || !scrollTarget) return;

  tableScrollPositions.set(key, {
    left: scrollTarget.scrollLeft,
    top: scrollTarget.scrollTop,
  });
}

async function restoreActiveTableScroll() {
  const key = activeTableScrollKey();
  if (!key) return;

  const scrollPosition = tableScrollPositions.get(key);
  if (!scrollPosition) return;

  await nextTick();
  window.requestAnimationFrame(() => {
    currentTableScrollTarget()?.scrollTo(scrollPosition);
  });
}

async function selectBrowser(browserId: string) {
  saveActiveTableScroll();
  selectedBrowserId.value = browserId;
  page.value = "browserData";
  await restoreActiveTableScroll();
}

async function selectBrowserSection(section: typeof activeSection.value) {
  saveActiveTableScroll();
  activeSection.value = section;
  await restoreActiveTableScroll();
}

async function refreshCurrentBrowserPreservingScroll() {
  saveActiveTableScroll();
  await refreshCurrentBrowser();
  await restoreActiveTableScroll();
}
</script>

<template>
  <div class="app-shell">
    <AppSidebar
      :browsers="browsers"
      :current-browser-id="currentBrowser?.browserId ?? null"
      :page="page"
      :loading="loading"
      :configs-loading="configsLoading"
      :browser-monogram="browserMonogram"
      :app-version="appVersion"
      @select-browser="selectBrowser"
      @select-configuration="page = 'configuration'"
      @refresh="refreshCurrentBrowserPreservingScroll"
    />

    <main class="content-panel">
      <template v-if="page === 'configuration'">
        <ConfigurationView
          :config-error="configError"
          :configs-loading="configsLoading"
          :browser-configs="browserConfigs"
          :create-config-form="createConfigForm"
          :saving-config="savingConfig"
          :config-monogram="configMonogram"
          :is-deleting-config="isDeletingConfig"
          @update-name="createConfigForm.name = $event"
          @update-executable-path="createConfigForm.executablePath = $event"
          @update-user-data-path="createConfigForm.userDataPath = $event"
          @update-icon-key="createConfigForm.iconKey = $event"
          @pick-executable-path="pickExecutablePath"
          @pick-user-data-path="pickUserDataPath"
          @create-config="createCustomBrowserConfig"
          @delete-config="deleteCustomBrowserConfig"
        />
      </template>

      <template v-else-if="loading">
        <section class="state-panel scanning-panel">
          <div class="scan-hero" aria-hidden="true">
            <div class="scan-orbit orbit-one"></div>
            <div class="scan-orbit orbit-two"></div>
            <div class="scan-core">
              <div class="scan-core-ring"></div>
              <div class="scan-core-ring secondary"></div>
              <div class="scan-dot dot-one"></div>
              <div class="scan-dot dot-two"></div>
              <div class="scan-dot dot-three"></div>
            </div>
          </div>
          <p class="eyebrow">扫描中</p>
          <h2>正在读取本地浏览器数据</h2>
          <p>正在收集用户资料、插件、书签和历史文件状态。</p>
          <div class="loading-steps" aria-hidden="true">
            <span></span>
            <span></span>
            <span></span>
          </div>
        </section>
      </template>

      <template v-else-if="error">
        <section class="state-panel error">
          <p class="eyebrow">错误</p>
          <h2>扫描失败</h2>
          <p>{{ error }}</p>
        </section>
      </template>

      <BrowserDataView
        v-else-if="currentBrowser"
        :current-browser="currentBrowser"
        :active-section="activeSection"
        :profile-sort-key="profileSortKey"
        :extension-sort-key="extensionSortKey"
        :bookmark-sort-key="bookmarkSortKey"
        :password-site-sort-key="passwordSiteSortKey"
        :password-sites-loaded="hasLoadedPasswordSites(currentBrowser.browserId)"
        :password-sites-loading="passwordSitesLoading"
        :password-sites-error="passwordSitesError"
        :sorted-profiles="sortedProfiles"
        :sorted-extensions="sortedExtensions"
        :sorted-bookmarks="sortedBookmarks"
        :sorted-password-sites="sortedPasswordSites"
        :profile-selected-ids="profileSelectedIds"
        :opening-selected-profiles="openingSelectedProfiles"
        :history-selected-profile-ids="cleanupHistorySelectedProfiles"
        :cleanup-history-busy="historyCleanupBusy"
        :history-cleanup-confirm-profiles="historyCleanupConfirmProfiles"
        :history-cleanup-result-open="historyCleanupResultOpen"
        :cleanup-history-error="cleanupHistoryError"
        :cleanup-history-results="cleanupHistoryResults"
        :bookmark-selected-urls="bookmarkSelectedUrls"
        :bookmark-modal-selected-profile-ids="bookmarkModalSelectedProfileIds"
        :bookmark-delete-busy="bookmarkDeleteBusy"
        :bookmark-removal-confirm-bookmark-count="bookmarkRemovalConfirmBookmarkCount"
        :bookmark-removal-confirm-profile-count="bookmarkRemovalConfirmProfileCount"
        :bookmark-removal-result-open="bookmarkRemovalResultOpen"
        :bookmark-removal-error="bookmarkRemovalError"
        :bookmark-removal-results="bookmarkRemovalResults"
        :extension-selected-ids="extensionSelectedIds"
        :extension-modal-selected-profile-ids="extensionModalSelectedProfileIds"
        :extension-delete-busy="extensionDeleteBusy"
        :extension-removal-confirm-extensions="extensionRemovalConfirmExtensions"
        :extension-removal-confirm-profiles="extensionRemovalConfirmProfiles"
        :extension-removal-result-open="extensionRemovalResultOpen"
        :extension-removal-error="extensionRemovalError"
        :extension-removal-results="extensionRemovalResults"
        :open-profile-error="openProfileError"
        :section-count="sectionCount"
        :is-opening-profile="isOpeningProfile"
        :extension-monogram="extensionMonogram"
        :associated-profiles-modal="associatedProfilesModal"
        @update:active-section="selectBrowserSection"
        @update:profile-sort-key="profileSortKey = $event"
        @update:extension-sort-key="extensionSortKey = $event"
        @update:bookmark-sort-key="bookmarkSortKey = $event"
        @update:password-site-sort-key="passwordSiteSortKey = $event"
        @load-password-sites="loadPasswordSites"
        @open-profile="(browserId, profileId) => openBrowserProfile(browserId, profileId)"
        @toggle-profile-selection="toggleProfileSelection"
        @toggle-all-profiles="toggleAllProfiles"
        @open-selected-profiles="openSelectedProfiles"
        @show-extension-profiles="showExtensionProfilesModal"
        @show-bookmark-profiles="showBookmarkProfilesModal"
        @show-password-site-profiles="showPasswordSiteProfilesModal"
        @toggle-bookmark-selection="toggleBookmarkSelection"
        @toggle-all-bookmarks="toggleAllBookmarks"
        @delete-bookmark-from-all-profiles="deleteBookmarkFromAllProfiles"
        @delete-selected-bookmarks="deleteSelectedBookmarks"
        @toggle-bookmark-modal-profile-selection="toggleBookmarkModalProfileSelection"
        @toggle-all-bookmark-modal-profiles="toggleAllBookmarkModalProfiles"
        @delete-bookmark-from-profile="deleteBookmarkFromProfile"
        @delete-selected-bookmark-profiles="deleteSelectedBookmarkProfiles"
        @confirm-bookmark-removal="confirmBookmarkRemoval"
        @close-bookmark-removal-confirm="closeBookmarkRemovalConfirm"
        @close-bookmark-removal-result="closeBookmarkRemovalResult"
        @toggle-extension-selection="toggleExtensionSelection"
        @toggle-all-extensions="toggleAllExtensions"
        @delete-extension-from-all-profiles="deleteExtensionFromAllProfiles"
        @delete-selected-extensions="deleteSelectedExtensions"
        @toggle-extension-modal-profile-selection="toggleExtensionModalProfileSelection"
        @toggle-all-extension-modal-profiles="toggleAllExtensionModalProfiles"
        @delete-extension-from-profile="deleteExtensionFromProfile"
        @delete-selected-extension-profiles="deleteSelectedExtensionProfiles"
        @confirm-extension-removal="confirmExtensionRemoval"
        @close-extension-removal-confirm="closeExtensionRemovalConfirm"
        @close-extension-removal-result="closeExtensionRemovalResult"
        @toggle-history-profile="toggleHistoryProfile"
        @toggle-all-history-profiles="toggleAllHistoryProfiles"
        @cleanup-selected-history="cleanupSelectedHistoryProfiles"
        @cleanup-history-for-profile="cleanupHistoryForProfile"
        @confirm-history-cleanup="confirmHistoryCleanup"
        @close-history-cleanup-confirm="closeHistoryCleanupConfirm"
        @close-history-cleanup-result="closeHistoryCleanupResult"
        @close-associated-profiles="closeAssociatedProfilesModal"
      />

      <template v-else>
        <section class="state-panel">
          <p class="eyebrow">无数据</p>
          <h2>没有检测到受支持的浏览器</h2>
          <p>请安装或登录 Chrome、Edge、Brave 等浏览器后再刷新扫描。</p>
        </section>
      </template>
    </main>
  </div>
</template>
