import { computed, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

import { sortBookmarks, sortExtensions, sortPasswordSites, sortProfiles } from "../utils/sort";
import type {
  ActiveSection,
  AppPage,
  AssociatedProfileSummary,
  BookmarkAssociatedProfileSummary,
  BookmarkSortKey,
  BrowserConfigEntry,
  BrowserConfigListResponse,
  BrowserView,
  BookmarkRemovalRequest,
  CleanupHistoryInput,
  CleanupHistoryResponse,
  CreateCustomBrowserConfigInput,
  ExtensionRemovalRequest,
  ExtensionSummary,
  ExtensionSortKey,
  PasswordSiteSortKey,
  ProfileSortKey,
  RemoveBookmarksInput,
  RemoveBookmarksResponse,
  RemoveExtensionsInput,
  RemoveExtensionsResponse,
  PasswordSitesResponse,
  ScanResponse,
} from "../types/browser";

export function useBrowserManager() {
  const page = ref<AppPage>("browserData");
  const loading = ref(true);
  const error = ref("");
  const openProfileError = ref("");
  const openingProfileKey = ref("");
  const response = ref<ScanResponse>({ browsers: [] });
  const browserConfigs = ref<BrowserConfigEntry[]>([]);
  const configsLoading = ref(true);
  const configError = ref("");
  const savingConfig = ref(false);
  const deletingConfigId = ref("");
  const createConfigForm = ref<CreateCustomBrowserConfigInput>({
    name: "",
    iconKey: "chrome",
    executablePath: "",
    userDataPath: "",
  });
  const selectedBrowserId = ref("");
  const activeSection = ref<ActiveSection>("profiles");
  const profileSelectedIds = ref<string[]>([]);
  const openingSelectedProfiles = ref(false);
  const associatedProfilesModal = ref<{
    title: string;
    browserId: string;
    profiles: (
      | AssociatedProfileSummary
      | BookmarkAssociatedProfileSummary
      | ExtensionSummary["profiles"][number]
    )[];
    isBookmark: boolean;
    isExtension?: boolean;
    extensionId?: string;
    bookmarkUrl?: string;
  } | null>(null);
  const profileSortKey = ref<ProfileSortKey>("name");
  const extensionSortKey = ref<ExtensionSortKey>("name");
  const bookmarkSortKey = ref<BookmarkSortKey>("title");
  const passwordSiteSortKey = ref<PasswordSiteSortKey>("domain");
  const passwordSitesLoading = ref(false);
  const passwordSitesError = ref("");
  const passwordSitesLoadedBrowserIds = ref<string[]>([]);
  const bookmarkSelectedUrls = ref<string[]>([]);
  const bookmarkModalSelectedProfileIds = ref<string[]>([]);
  const bookmarkDeleteBusy = ref(false);
  const bookmarkRemovalError = ref("");
  const bookmarkRemovalResults = ref<RemoveBookmarksResponse["results"]>([]);
  const bookmarkRemovalResultOpen = ref(false);
  const bookmarkRemovalConfirmRemovals = ref<BookmarkRemovalRequest[]>([]);
  const bookmarkRemovalConfirmUrls = ref<string[]>([]);
  const bookmarkRemovalConfirmProfileIds = ref<string[]>([]);
  const extensionSelectedIds = ref<string[]>([]);
  const extensionModalSelectedProfileIds = ref<string[]>([]);
  const extensionDeleteBusy = ref(false);
  const extensionRemovalError = ref("");
  const extensionRemovalResults = ref<RemoveExtensionsResponse["results"]>([]);
  const extensionRemovalResultOpen = ref(false);
  const extensionRemovalConfirmRemovals = ref<ExtensionRemovalRequest[]>([]);
  const extensionRemovalConfirmExtensionIds = ref<string[]>([]);
  const extensionRemovalConfirmProfileIds = ref<string[]>([]);
  const cleanupHistorySelectedProfiles = ref<string[]>([]);
  const historyCleanupBusy = ref(false);
  const cleanupHistoryError = ref("");
  const cleanupHistoryResults = ref<CleanupHistoryResponse["results"]>([]);
  const historyCleanupConfirmProfileIds = ref<string[]>([]);
  const historyCleanupResultOpen = ref(false);

  const browsers = computed(() => response.value.browsers);
  const currentBrowser = computed<BrowserView | null>(
    () =>
      browsers.value.find((browser) => browser.browserId === selectedBrowserId.value) ??
      browsers.value[0] ??
      null,
  );

  const sortedProfiles = computed(() =>
    sortProfiles(currentBrowser.value?.profiles ?? [], profileSortKey.value),
  );
  const sortedExtensions = computed(() =>
    sortExtensions(currentBrowser.value?.extensions ?? [], extensionSortKey.value),
  );
  const sortedBookmarks = computed(() =>
    sortBookmarks(currentBrowser.value?.bookmarks ?? [], bookmarkSortKey.value),
  );
  const sortedPasswordSites = computed(() =>
    sortPasswordSites(currentBrowser.value?.passwordSites ?? [], passwordSiteSortKey.value),
  );

  watch(
    browsers,
    (items) => {
      if (!items.length) {
        selectedBrowserId.value = "";
        return;
      }

      const hasSelected = items.some(
        (browser) => browser.browserId === selectedBrowserId.value,
      );

      if (!hasSelected) {
        selectedBrowserId.value = items[0].browserId;
      }
    },
    { immediate: true },
  );

  watch(selectedBrowserId, () => {
    openProfileError.value = "";
    associatedProfilesModal.value = null;
    profileSelectedIds.value = [];
    openingSelectedProfiles.value = false;
    cleanupHistorySelectedProfiles.value = [];
    cleanupHistoryResults.value = [];
    cleanupHistoryError.value = "";
    bookmarkSelectedUrls.value = [];
    bookmarkModalSelectedProfileIds.value = [];
    bookmarkRemovalError.value = "";
    bookmarkRemovalResults.value = [];
    bookmarkRemovalResultOpen.value = false;
    bookmarkRemovalConfirmRemovals.value = [];
    bookmarkRemovalConfirmUrls.value = [];
    bookmarkRemovalConfirmProfileIds.value = [];
    extensionSelectedIds.value = [];
    extensionModalSelectedProfileIds.value = [];
    extensionRemovalError.value = "";
    extensionRemovalResults.value = [];
    extensionRemovalResultOpen.value = false;
    extensionRemovalConfirmRemovals.value = [];
    extensionRemovalConfirmExtensionIds.value = [];
    extensionRemovalConfirmProfileIds.value = [];
    historyCleanupConfirmProfileIds.value = [];
    historyCleanupResultOpen.value = false;
    passwordSitesError.value = "";
  });

  async function loadBrowserConfigs() {
    configsLoading.value = true;
    configError.value = "";

    try {
      const result = await invoke<BrowserConfigListResponse>("list_browser_configs");
      browserConfigs.value = result.configs;
    } catch (loadError) {
      configError.value =
        loadError instanceof Error ? loadError.message : "加载浏览器配置失败。";
    } finally {
      configsLoading.value = false;
    }
  }

  async function scanBrowsers() {
    loading.value = true;
    error.value = "";

    try {
      response.value = await invoke<ScanResponse>("scan_browsers");
      passwordSitesLoadedBrowserIds.value = [];
      passwordSitesError.value = "";
    } catch (scanError) {
      error.value =
        scanError instanceof Error
          ? scanError.message
          : "扫描浏览器数据失败。";
    } finally {
      loading.value = false;
    }
  }

  async function refreshAll() {
    await Promise.all([loadBrowserConfigs(), scanBrowsers()]);
  }

  async function refreshCurrentBrowser() {
    if (page.value === "configuration" || !selectedBrowserId.value) {
      await refreshAll();
      return;
    }

    const browserId = selectedBrowserId.value;
    const shouldRefreshPasswordSites = hasLoadedPasswordSites(browserId);
    loading.value = true;
    error.value = "";

    try {
      const [browser, passwordSitesResponse] = await Promise.all([
        invoke<BrowserView | null>("scan_browser", { browserId }),
        shouldRefreshPasswordSites
          ? invoke<PasswordSitesResponse>("scan_password_sites", { browserId })
          : Promise.resolve(null),
      ]);

      if (!browser) {
        response.value = {
          browsers: response.value.browsers.filter((item) => item.browserId !== browserId),
        };
        return;
      }

      if (passwordSitesResponse) {
        browser.passwordSites = passwordSitesResponse.passwordSites;
        browser.stats.passwordSiteCount = passwordSitesResponse.passwordSites.length;
      }

      const existingIndex = response.value.browsers.findIndex(
        (item) => item.browserId === browserId,
      );
      const browsers =
        existingIndex >= 0
          ? response.value.browsers.map((item, index) =>
              index === existingIndex ? browser : item,
            )
          : [...response.value.browsers, browser];

      response.value = { browsers };
      passwordSitesError.value = "";
    } catch (refreshError) {
      error.value =
        refreshError instanceof Error ? refreshError.message : "刷新当前浏览器数据失败。";
    } finally {
      loading.value = false;
    }
  }

  async function openBrowserProfile(browserId: string, profileId: string, resetError = true) {
    const profileKey = `${browserId}:${profileId}`;
    openingProfileKey.value = profileKey;
    if (resetError) {
      openProfileError.value = "";
    }

    try {
      await invoke("open_browser_profile", {
        browserId,
        profileId,
      });
      return true;
    } catch (openError) {
      openProfileError.value =
        openError instanceof Error
          ? openError.message
          : "打开所选浏览器资料失败。";
      return false;
    } finally {
      openingProfileKey.value = "";
    }
  }

  function wait(ms: number) {
    return new Promise((resolve) => {
      window.setTimeout(resolve, ms);
    });
  }

  function toggleProfileSelection(profileId: string) {
    if (openingSelectedProfiles.value) return;

    if (profileSelectedIds.value.includes(profileId)) {
      profileSelectedIds.value = profileSelectedIds.value.filter(
        (selectedId) => selectedId !== profileId,
      );
      return;
    }

    profileSelectedIds.value = [...profileSelectedIds.value, profileId];
  }

  function toggleAllProfiles() {
    if (openingSelectedProfiles.value) return;

    const profileIds = sortedProfiles.value.map((profile) => profile.id);
    const allSelected =
      profileIds.length > 0 &&
      profileIds.every((profileId) => profileSelectedIds.value.includes(profileId));

    profileSelectedIds.value = allSelected ? [] : profileIds;
  }

  async function openSelectedProfiles() {
    const browser = currentBrowser.value;
    if (!browser || openingSelectedProfiles.value || !profileSelectedIds.value.length) return;

    const selectedProfiles = sortedProfiles.value.filter((profile) =>
      profileSelectedIds.value.includes(profile.id),
    );
    if (!selectedProfiles.length) return;

    openingSelectedProfiles.value = true;
    openProfileError.value = "";

    try {
      for (const [index, profile] of selectedProfiles.entries()) {
        await openBrowserProfile(browser.browserId, profile.id, false);
        if (index < selectedProfiles.length - 1) {
          await wait(900);
        }
      }
    } finally {
      openingSelectedProfiles.value = false;
    }
  }

  async function createCustomBrowserConfig() {
    savingConfig.value = true;
    configError.value = "";

    try {
      const result = await invoke<BrowserConfigListResponse>("create_custom_browser_config", {
        input: createConfigForm.value,
      });
      browserConfigs.value = result.configs;
      createConfigForm.value = {
        name: "",
        iconKey: "chrome",
        executablePath: "",
        userDataPath: "",
      };
      await scanBrowsers();
    } catch (saveError) {
      configError.value =
        saveError instanceof Error ? saveError.message : "创建浏览器配置失败。";
    } finally {
      savingConfig.value = false;
    }
  }

  async function deleteCustomBrowserConfig(configId: string) {
    deletingConfigId.value = configId;
    configError.value = "";

    try {
      const result = await invoke<BrowserConfigListResponse>("delete_custom_browser_config", {
        configId,
      });
      browserConfigs.value = result.configs;
      await scanBrowsers();
    } catch (deleteError) {
      configError.value =
        deleteError instanceof Error ? deleteError.message : "删除浏览器配置失败。";
    } finally {
      deletingConfigId.value = "";
    }
  }

  async function pickExecutablePath() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Executable",
          extensions: ["exe"],
        },
      ],
    });

    if (typeof selected === "string") {
      createConfigForm.value.executablePath = selected;
    }
  }

  async function pickUserDataPath() {
    const selected = await open({
      multiple: false,
      directory: true,
    });

    if (typeof selected === "string") {
      createConfigForm.value.userDataPath = selected;
    }
  }

  function isDeletingConfig(configId: string) {
    return deletingConfigId.value === configId;
  }

  function isOpeningProfile(browserId: string, profileId: string) {
    return openingProfileKey.value === `${browserId}:${profileId}`;
  }

  function browserMonogram(browserId: string) {
    const current = browsers.value.find((browser) => browser.browserId === browserId);
    const iconKey = current?.iconKey ?? current?.browserFamilyId;
    if (iconKey === "chrome") return "CH";
    if (iconKey === "edge") return "ED";
    if (iconKey === "brave") return "BR";
    if (iconKey === "vivaldi") return "VI";
    if (iconKey === "yandex") return "YA";
    if (iconKey === "chromium") return "CR";

    const name = current?.browserName?.trim() ?? "";
    if (name) {
      const letters = name
        .split(/\s+/)
        .filter(Boolean)
        .slice(0, 2)
        .map((part) => part[0]);
      if (letters.length) return letters.join("").toUpperCase();
    }

    return browserId.slice(0, 2).toUpperCase();
  }

  function configMonogram(config: BrowserConfigEntry) {
    const iconKey = config.iconKey ?? config.browserFamilyId;
    if (iconKey === "chrome") return "CH";
    if (iconKey === "edge") return "ED";
    if (iconKey === "brave") return "BR";
    if (iconKey === "vivaldi") return "VI";
    if (iconKey === "yandex") return "YA";
    if (iconKey === "chromium") return "CR";

    const letters = config.name
      .trim()
      .split(/\s+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((part) => part[0]);
    return (letters.join("") || config.id.slice(0, 2)).toUpperCase();
  }

  function extensionMonogram(name: string) {
    return name.trim().slice(0, 1).toUpperCase() || "?";
  }

  function sectionCount(section: ActiveSection) {
    if (!currentBrowser.value) return 0;
    if (section === "profiles") return currentBrowser.value.profiles.length;
    if (section === "extensions") return currentBrowser.value.extensions.length;
    if (section === "bookmarks") return currentBrowser.value.bookmarks.length;
    if (section === "passwords") return currentBrowser.value.passwordSites.length;
    return currentBrowser.value.stats.historyCleanupProfileCount;
  }

  function showExtensionProfilesModal(extensionId: string) {
    const extension = currentBrowser.value?.extensions.find((item) => item.id === extensionId);
    if (!extension || !currentBrowser.value) return;
    extensionModalSelectedProfileIds.value = [];
    associatedProfilesModal.value = {
      title: extension.name,
      browserId: currentBrowser.value.browserId,
      profiles: extension.profiles,
      isBookmark: false,
      isExtension: true,
      extensionId,
    };
  }

  function showBookmarkProfilesModal(url: string) {
    const bookmark = currentBrowser.value?.bookmarks.find((item) => item.url === url);
    if (!bookmark || !currentBrowser.value) return;
    bookmarkModalSelectedProfileIds.value = [];
    associatedProfilesModal.value = {
      title: bookmark.title,
      browserId: currentBrowser.value.browserId,
      profiles: bookmark.profiles,
      isBookmark: true,
      bookmarkUrl: url,
    };
  }

  function showPasswordSiteProfilesModal(url: string) {
    const passwordSite = currentBrowser.value?.passwordSites.find((item) => item.url === url);
    if (!passwordSite || !currentBrowser.value) return;
    associatedProfilesModal.value = {
      title: passwordSite.domain,
      browserId: currentBrowser.value.browserId,
      profiles: passwordSite.profiles,
      isBookmark: false,
    };
  }

  function hasLoadedPasswordSites(browserId: string) {
    return passwordSitesLoadedBrowserIds.value.includes(browserId);
  }

  async function loadPasswordSites() {
    const browser = currentBrowser.value;
    if (!browser || passwordSitesLoading.value) return;

    const confirmed = window.confirm("将按需读取当前浏览器的已保存登录站点，是否继续？");
    if (!confirmed) return;

    passwordSitesLoading.value = true;
    passwordSitesError.value = "";

    try {
      const result = await invoke<PasswordSitesResponse>("scan_password_sites", {
        browserId: browser.browserId,
      });
      browser.passwordSites = sortPasswordSites(result.passwordSites, passwordSiteSortKey.value);
      browser.stats.passwordSiteCount = browser.passwordSites.length;
      if (!passwordSitesLoadedBrowserIds.value.includes(browser.browserId)) {
        passwordSitesLoadedBrowserIds.value = [
          ...passwordSitesLoadedBrowserIds.value,
          browser.browserId,
        ];
      }
    } catch (loadError) {
      passwordSitesError.value =
        loadError instanceof Error ? loadError.message : "加载已保存登录站点失败。";
    } finally {
      passwordSitesLoading.value = false;
    }
  }

  function toggleHistoryProfile(profileId: string) {
    if (cleanupHistorySelectedProfiles.value.includes(profileId)) {
      cleanupHistorySelectedProfiles.value = cleanupHistorySelectedProfiles.value.filter(
        (selectedId) => selectedId !== profileId,
      );
      return;
    }

    cleanupHistorySelectedProfiles.value = [
      ...cleanupHistorySelectedProfiles.value,
      profileId,
    ];
  }

  function toggleAllHistoryProfiles() {
    const current = currentBrowser.value;
    if (!current) return;

    const selectableIds = current.profiles
      .filter((profile) => {
        const cleanup = profile.historyCleanup;
        return (
          cleanup.history === "found" ||
          cleanup.topSites === "found" ||
          cleanup.visitedLinks === "found" ||
          cleanup.shortcuts === "found" ||
          cleanup.sessions === "found"
        );
      })
      .map((profile) => profile.id);

    const allSelected =
      selectableIds.length > 0 &&
      selectableIds.every((profileId) =>
        cleanupHistorySelectedProfiles.value.includes(profileId),
      );

    cleanupHistorySelectedProfiles.value = allSelected ? [] : selectableIds;
  }

  function cleanupProfileIdsWithHistory(browser: BrowserView) {
    return browser.profiles
      .filter((profile) => {
        const cleanup = profile.historyCleanup;
        return (
          cleanup.history === "found" ||
          cleanup.topSites === "found" ||
          cleanup.visitedLinks === "found" ||
          cleanup.shortcuts === "found" ||
          cleanup.sessions === "found"
        );
      })
      .map((profile) => profile.id);
  }

  function historyCleanupConfirmProfiles() {
    const browser = currentBrowser.value;
    if (!browser) return [];
    return browser.profiles.filter((profile) =>
      historyCleanupConfirmProfileIds.value.includes(profile.id),
    );
  }

  function cleanupSelectedHistoryProfiles() {
    if (!cleanupHistorySelectedProfiles.value.length) return;
    historyCleanupConfirmProfileIds.value = [...cleanupHistorySelectedProfiles.value];
  }

  function cleanupHistoryForProfile(profileId: string) {
    historyCleanupConfirmProfileIds.value = [profileId];
  }

  function closeHistoryCleanupConfirm() {
    if (historyCleanupBusy.value) return;
    historyCleanupConfirmProfileIds.value = [];
  }

  function closeHistoryCleanupResult() {
    historyCleanupResultOpen.value = false;
    cleanupHistoryResults.value = [];
    cleanupHistoryError.value = "";
  }

  function applyCleanupHistoryResults(results: CleanupHistoryResponse["results"]) {
    const browser = currentBrowser.value;
    if (!browser) return;

    const succeededProfileIds = results
      .filter((result) => !result.error)
      .map((result) => result.profileId);

    if (!succeededProfileIds.length) return;

    for (const profile of browser.profiles) {
      if (!succeededProfileIds.includes(profile.id)) continue;

      const deletedFiles = results.find((result) => result.profileId === profile.id)?.deletedFiles ?? [];
      if (deletedFiles.includes("History")) {
        profile.historyCleanup.history = "missing";
      }
      if (deletedFiles.includes("Top Sites")) {
        profile.historyCleanup.topSites = "missing";
      }
      if (deletedFiles.includes("Visited Links")) {
        profile.historyCleanup.visitedLinks = "missing";
      }
      if (deletedFiles.includes("Shortcuts")) {
        profile.historyCleanup.shortcuts = "missing";
      }
      if (deletedFiles.includes("Sessions")) {
        profile.historyCleanup.sessions = "missing";
      }
    }

    browser.stats.historyCleanupProfileCount = cleanupProfileIdsWithHistory(browser).length;
  }

  async function confirmHistoryCleanup() {
    const browser = currentBrowser.value;
    const profileIds = [...historyCleanupConfirmProfileIds.value];
    if (!browser || profileIds.length === 0) return;

    if (!currentBrowser.value || profileIds.length === 0) return;

    historyCleanupBusy.value = true;
    cleanupHistoryError.value = "";
    cleanupHistoryResults.value = [];
    historyCleanupResultOpen.value = false;

    try {
      const input: CleanupHistoryInput = {
        browserId: browser.browserId,
        profileIds,
      };
      const result = await invoke<CleanupHistoryResponse>("cleanup_history_files", { input });
      applyCleanupHistoryResults(result.results);
      cleanupHistoryResults.value = result.results;
      cleanupHistorySelectedProfiles.value = cleanupHistorySelectedProfiles.value.filter(
        (profileId) => !profileIds.includes(profileId),
      );
      historyCleanupConfirmProfileIds.value = [];
      historyCleanupResultOpen.value = true;
    } catch (cleanupErrorValue) {
      historyCleanupConfirmProfileIds.value = [];
      cleanupHistoryError.value =
        cleanupErrorValue instanceof Error
          ? cleanupErrorValue.message
          : "清理历史文件失败。";
      historyCleanupResultOpen.value = true;
    } finally {
      historyCleanupBusy.value = false;
    }
  }

  function closeAssociatedProfilesModal() {
    associatedProfilesModal.value = null;
    extensionModalSelectedProfileIds.value = [];
    bookmarkModalSelectedProfileIds.value = [];
  }

  function toggleBookmarkSelection(url: string) {
    if (bookmarkSelectedUrls.value.includes(url)) {
      bookmarkSelectedUrls.value = bookmarkSelectedUrls.value.filter((item) => item !== url);
      return;
    }

    bookmarkSelectedUrls.value = [...bookmarkSelectedUrls.value, url];
  }

  function toggleAllBookmarks() {
    const bookmarkUrls = currentBrowser.value?.bookmarks.map((bookmark) => bookmark.url) ?? [];
    const allSelected =
      bookmarkUrls.length > 0 &&
      bookmarkUrls.every((url) => bookmarkSelectedUrls.value.includes(url));
    bookmarkSelectedUrls.value = allSelected ? [] : bookmarkUrls;
  }

  function toggleBookmarkModalProfileSelection(profileId: string) {
    if (bookmarkModalSelectedProfileIds.value.includes(profileId)) {
      bookmarkModalSelectedProfileIds.value = bookmarkModalSelectedProfileIds.value.filter(
        (id) => id !== profileId,
      );
      return;
    }

    bookmarkModalSelectedProfileIds.value = [...bookmarkModalSelectedProfileIds.value, profileId];
  }

  function toggleAllBookmarkModalProfiles() {
    if (!associatedProfilesModal.value?.isBookmark) return;
    const profileIds = associatedProfilesModal.value.profiles.map((profile) => profile.id);
    const allSelected =
      profileIds.length > 0 &&
      profileIds.every((profileId) => bookmarkModalSelectedProfileIds.value.includes(profileId));
    bookmarkModalSelectedProfileIds.value = allSelected ? [] : profileIds;
  }

  function bookmarkRemovalConfirmBookmarkCount() {
    return bookmarkRemovalConfirmUrls.value.length;
  }

  function bookmarkRemovalConfirmProfileCount() {
    return bookmarkRemovalConfirmProfileIds.value.length;
  }

  function requestBookmarkRemoval(removals: BookmarkRemovalRequest[]) {
    if (!removals.length) return;

    bookmarkRemovalConfirmRemovals.value = removals;
    bookmarkRemovalConfirmUrls.value = [...new Set(removals.map((item) => item.url))];
    bookmarkRemovalConfirmProfileIds.value = [...new Set(removals.flatMap((item) => item.profileIds))];
  }

  function resetBookmarkRemovalConfirmState() {
    bookmarkRemovalConfirmRemovals.value = [];
    bookmarkRemovalConfirmUrls.value = [];
    bookmarkRemovalConfirmProfileIds.value = [];
  }

  function deleteBookmarkFromAllProfiles(url: string) {
    const bookmark = currentBrowser.value?.bookmarks.find((item) => item.url === url);
    if (!bookmark) return;

    requestBookmarkRemoval([
      {
        url,
        profileIds: [...bookmark.profileIds],
      },
    ]);
  }

  function deleteSelectedBookmarks() {
    const browser = currentBrowser.value;
    if (!browser || !bookmarkSelectedUrls.value.length) return;

    const removals = browser.bookmarks
      .filter((bookmark) => bookmarkSelectedUrls.value.includes(bookmark.url))
      .map((bookmark) => ({
        url: bookmark.url,
        profileIds: [...bookmark.profileIds],
      }));
    requestBookmarkRemoval(removals);
  }

  function deleteBookmarkFromProfile(profileId: string) {
    const modal = associatedProfilesModal.value;
    if (!modal?.isBookmark || !modal.bookmarkUrl) return;

    requestBookmarkRemoval([
      {
        url: modal.bookmarkUrl,
        profileIds: [profileId],
      },
    ]);
  }

  function deleteSelectedBookmarkProfiles() {
    const modal = associatedProfilesModal.value;
    if (!modal?.isBookmark || !modal.bookmarkUrl || !bookmarkModalSelectedProfileIds.value.length) {
      return;
    }

    requestBookmarkRemoval([
      {
        url: modal.bookmarkUrl,
        profileIds: [...bookmarkModalSelectedProfileIds.value],
      },
    ]);
  }

  function closeBookmarkRemovalConfirm() {
    if (bookmarkDeleteBusy.value) return;
    resetBookmarkRemovalConfirmState();
  }

  function closeBookmarkRemovalResult() {
    bookmarkRemovalResultOpen.value = false;
    bookmarkRemovalResults.value = [];
    bookmarkRemovalError.value = "";
  }

  function applyBookmarkRemovalResults(results: RemoveBookmarksResponse["results"]) {
    const browser = currentBrowser.value;
    if (!browser) return;

    for (const result of results) {
      if (result.error || result.removedCount === 0) continue;
      const bookmark = browser.bookmarks.find((item) => item.url === result.url);
      if (!bookmark) continue;

      bookmark.profileIds = bookmark.profileIds.filter((id) => id !== result.profileId);
      bookmark.profiles = bookmark.profiles.filter((profile) => profile.id !== result.profileId);
    }

    browser.bookmarks = browser.bookmarks.filter((bookmark) => bookmark.profileIds.length > 0);
    browser.stats.bookmarkCount = browser.bookmarks.length;

    bookmarkSelectedUrls.value = bookmarkSelectedUrls.value.filter((selectedUrl) =>
      browser.bookmarks.some((bookmark) => bookmark.url === selectedUrl),
    );

    if (associatedProfilesModal.value?.isBookmark) {
      const currentBookmark = browser.bookmarks.find(
        (bookmark) => bookmark.url === associatedProfilesModal.value?.bookmarkUrl,
      );
      if (!currentBookmark) {
        associatedProfilesModal.value = null;
        bookmarkModalSelectedProfileIds.value = [];
      } else {
        associatedProfilesModal.value = {
          ...associatedProfilesModal.value,
          title: currentBookmark.title,
          profiles: currentBookmark.profiles,
        };
        bookmarkModalSelectedProfileIds.value = bookmarkModalSelectedProfileIds.value.filter((id) =>
          currentBookmark.profiles.some((profile) => profile.id === id),
        );
      }
    }
  }

  async function confirmBookmarkRemoval() {
    const browser = currentBrowser.value;
    const removals = bookmarkRemovalConfirmRemovals.value.map((item) => ({
      url: item.url,
      profileIds: [...item.profileIds],
    }));
    if (!browser || !removals.length) return;

    bookmarkDeleteBusy.value = true;
    bookmarkRemovalError.value = "";
    bookmarkRemovalResults.value = [];
    bookmarkRemovalResultOpen.value = false;

    try {
      const input: RemoveBookmarksInput = {
        browserId: browser.browserId,
        removals,
      };
      const result = await invoke<RemoveBookmarksResponse>("remove_bookmarks", { input });
      applyBookmarkRemovalResults(result.results);
      bookmarkRemovalResults.value = result.results;
      resetBookmarkRemovalConfirmState();
      bookmarkRemovalResultOpen.value = true;
    } catch (removeError) {
      resetBookmarkRemovalConfirmState();
      bookmarkRemovalError.value =
        removeError instanceof Error ? removeError.message : "删除书签失败。";
      bookmarkRemovalResultOpen.value = true;
    } finally {
      bookmarkDeleteBusy.value = false;
    }
  }

  function toggleExtensionSelection(extensionId: string) {
    if (extensionSelectedIds.value.includes(extensionId)) {
      extensionSelectedIds.value = extensionSelectedIds.value.filter((id) => id !== extensionId);
      return;
    }

    extensionSelectedIds.value = [...extensionSelectedIds.value, extensionId];
  }

  function toggleAllExtensions() {
    const extensionIds = currentBrowser.value?.extensions.map((extension) => extension.id) ?? [];
    const allSelected =
      extensionIds.length > 0 &&
      extensionIds.every((extensionId) => extensionSelectedIds.value.includes(extensionId));
    extensionSelectedIds.value = allSelected ? [] : extensionIds;
  }

  function toggleExtensionModalProfileSelection(profileId: string) {
    if (extensionModalSelectedProfileIds.value.includes(profileId)) {
      extensionModalSelectedProfileIds.value = extensionModalSelectedProfileIds.value.filter(
        (id) => id !== profileId,
      );
      return;
    }

    extensionModalSelectedProfileIds.value = [
      ...extensionModalSelectedProfileIds.value,
      profileId,
    ];
  }

  function toggleAllExtensionModalProfiles() {
    if (!associatedProfilesModal.value?.isExtension) return;
    const profileIds = associatedProfilesModal.value.profiles.map((profile) => profile.id);
    const allSelected =
      profileIds.length > 0 &&
      profileIds.every((profileId) => extensionModalSelectedProfileIds.value.includes(profileId));
    extensionModalSelectedProfileIds.value = allSelected ? [] : profileIds;
  }

  function extensionRemovalConfirmExtensions() {
    const browser = currentBrowser.value;
    if (!browser) return [];
    return browser.extensions.filter((extension) =>
      extensionRemovalConfirmExtensionIds.value.includes(extension.id),
    );
  }

  function extensionRemovalConfirmProfiles() {
    const browser = currentBrowser.value;
    if (!browser) return [];
    return browser.profiles.filter((profile) =>
      extensionRemovalConfirmProfileIds.value.includes(profile.id),
    );
  }

  function requestExtensionRemoval(removals: ExtensionRemovalRequest[]) {
    if (!removals.length) return;

    extensionRemovalConfirmRemovals.value = removals;
    extensionRemovalConfirmExtensionIds.value = [...new Set(removals.map((item) => item.extensionId))];
    extensionRemovalConfirmProfileIds.value = [
      ...new Set(removals.flatMap((item) => item.profileIds)),
    ];
  }

  function resetExtensionRemovalConfirmState() {
    extensionRemovalConfirmRemovals.value = [];
    extensionRemovalConfirmExtensionIds.value = [];
    extensionRemovalConfirmProfileIds.value = [];
  }

  function deleteExtensionFromAllProfiles(extensionId: string) {
    const extension = currentBrowser.value?.extensions.find((item) => item.id === extensionId);
    if (!extension) return;
    requestExtensionRemoval([
      {
        extensionId,
        profileIds: [...extension.profileIds],
      },
    ]);
  }

  function deleteSelectedExtensions() {
    const browser = currentBrowser.value;
    if (!browser || !extensionSelectedIds.value.length) return;

    const removals = browser.extensions
      .filter((extension) => extensionSelectedIds.value.includes(extension.id))
      .map((extension) => ({
        extensionId: extension.id,
        profileIds: [...extension.profileIds],
      }));
    requestExtensionRemoval(removals);
  }

  function deleteExtensionFromProfile(profileId: string) {
    const modal = associatedProfilesModal.value;
    if (!modal?.isExtension || !modal.extensionId) {
      return;
    }
    requestExtensionRemoval([
      {
        extensionId: modal.extensionId,
        profileIds: [profileId],
      },
    ]);
  }

  function deleteSelectedExtensionProfiles() {
    const modal = associatedProfilesModal.value;
    if (
      !modal?.isExtension ||
      !modal.extensionId ||
      !extensionModalSelectedProfileIds.value.length
    ) {
      return;
    }
    requestExtensionRemoval([
      {
        extensionId: modal.extensionId,
        profileIds: [...extensionModalSelectedProfileIds.value],
      },
    ]);
  }

  function closeExtensionRemovalConfirm() {
    if (extensionDeleteBusy.value) return;
    resetExtensionRemovalConfirmState();
  }

  function closeExtensionRemovalResult() {
    extensionRemovalResultOpen.value = false;
    extensionRemovalResults.value = [];
    extensionRemovalError.value = "";
  }

  function applyExtensionRemovalResults(results: RemoveExtensionsResponse["results"]) {
    const browser = currentBrowser.value;
    if (!browser) return;

    for (const result of results) {
      if (result.error) continue;
      const extension = browser.extensions.find((item) => item.id === result.extensionId);
      if (!extension) continue;

      extension.profileIds = extension.profileIds.filter((id) => id !== result.profileId);
      extension.profiles = extension.profiles.filter((profile) => profile.id !== result.profileId);
    }

    browser.extensions = browser.extensions.filter((extension) => extension.profileIds.length > 0);
    browser.stats.extensionCount = browser.extensions.length;

    extensionSelectedIds.value = extensionSelectedIds.value.filter((selectedId) =>
      browser.extensions.some((extension) => extension.id === selectedId),
    );

    if (associatedProfilesModal.value?.isExtension && "extensionId" in associatedProfilesModal.value) {
      const currentExtension = browser.extensions.find(
        (extension) => extension.id === associatedProfilesModal.value?.extensionId,
      );
      if (!currentExtension) {
        associatedProfilesModal.value = null;
        extensionModalSelectedProfileIds.value = [];
      } else {
        associatedProfilesModal.value = {
          ...associatedProfilesModal.value,
          profiles: currentExtension.profiles,
        };
        extensionModalSelectedProfileIds.value = extensionModalSelectedProfileIds.value.filter((id) =>
          currentExtension.profiles.some((profile) => profile.id === id),
        );
      }
    }
  }

  async function confirmExtensionRemoval() {
    const browser = currentBrowser.value;
    const removals = extensionRemovalConfirmRemovals.value.map((item) => ({
      extensionId: item.extensionId,
      profileIds: [...item.profileIds],
    }));
    if (!browser || !removals.length) return;

    extensionDeleteBusy.value = true;
    extensionRemovalError.value = "";
    extensionRemovalResults.value = [];
    extensionRemovalResultOpen.value = false;

    try {
      const input: RemoveExtensionsInput = {
        browserId: browser.browserId,
        removals,
      };
      const result = await invoke<RemoveExtensionsResponse>("remove_extensions", { input });
      applyExtensionRemovalResults(result.results);
      extensionRemovalResults.value = result.results;
      resetExtensionRemovalConfirmState();
      extensionRemovalResultOpen.value = true;
    } catch (removeError) {
      resetExtensionRemovalConfirmState();
      extensionRemovalError.value =
        removeError instanceof Error ? removeError.message : "删除插件失败。";
      extensionRemovalResultOpen.value = true;
    } finally {
      extensionDeleteBusy.value = false;
    }
  }

  onMounted(() => {
    void refreshAll();
  });

  return {
    activeSection,
    associatedProfilesModal,
    bookmarkSortKey,
    bookmarkDeleteBusy,
    bookmarkModalSelectedProfileIds,
    bookmarkRemovalConfirmBookmarkCount: computed(bookmarkRemovalConfirmBookmarkCount),
    bookmarkRemovalConfirmProfileCount: computed(bookmarkRemovalConfirmProfileCount),
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
    currentBrowser,
    deleteCustomBrowserConfig,
    deleteBookmarkFromAllProfiles,
    deleteBookmarkFromProfile,
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
    extensionRemovalConfirmExtensions: computed(extensionRemovalConfirmExtensions),
    extensionRemovalConfirmProfiles: computed(extensionRemovalConfirmProfiles),
    extensionRemovalError,
    extensionRemovalResultOpen,
    extensionRemovalResults,
    extensionSelectedIds,
    extensionSortKey,
    cleanupHistoryError,
    cleanupHistoryResults,
    cleanupHistorySelectedProfiles,
    cleanupSelectedHistoryProfiles,
    closeHistoryCleanupConfirm,
    closeHistoryCleanupResult,
    confirmHistoryCleanup,
    historyCleanupBusy,
    historyCleanupConfirmProfiles: computed(historyCleanupConfirmProfiles),
    historyCleanupResultOpen,
    isDeletingConfig,
    isOpeningProfile,
    loading,
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
    refreshAll,
    savingConfig,
    sectionCount,
    selectedBrowserId,
    hasLoadedPasswordSites,
    showBookmarkProfilesModal,
    showExtensionProfilesModal,
    loadPasswordSites,
    showPasswordSiteProfilesModal,
    sortedBookmarks,
    sortedExtensions,
    sortedPasswordSites,
    sortedProfiles,
    closeExtensionRemovalConfirm,
    closeExtensionRemovalResult,
    closeBookmarkRemovalConfirm,
    closeBookmarkRemovalResult,
    confirmExtensionRemoval,
    confirmBookmarkRemoval,
    cleanupHistoryForProfile,
    toggleAllBookmarks,
    toggleAllProfiles,
    toggleAllExtensions,
    toggleAllBookmarkModalProfiles,
    toggleAllExtensionModalProfiles,
    toggleBookmarkModalProfileSelection,
    toggleBookmarkSelection,
    toggleProfileSelection,
    toggleExtensionModalProfileSelection,
    toggleExtensionSelection,
    toggleAllHistoryProfiles,
    toggleHistoryProfile,
    closeAssociatedProfilesModal,
  };
}
