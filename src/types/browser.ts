export type BrowserStats = {
  profileCount: number;
  extensionCount: number;
  bookmarkCount: number;
  passwordSiteCount: number;
  historyCleanupProfileCount: number;
};

export type ProfileSummary = {
  id: string;
  name: string;
  email: string | null;
  emails: string[];
  avatarDataUrl: string | null;
  avatarIcon: string | null;
  defaultAvatarFillColor: number | null;
  defaultAvatarStrokeColor: number | null;
  avatarLabel: string;
  path: string;
  historyCleanup: HistoryCleanupSummary;
};

export type ExtensionSummary = {
  id: string;
  name: string;
  version: string | null;
  iconDataUrl: string | null;
  profileIds: string[];
  profiles: ExtensionAssociatedProfileSummary[];
};

export type BookmarkSummary = {
  url: string;
  title: string;
  profileIds: string[];
  profiles: BookmarkAssociatedProfileSummary[];
};

export type PasswordSiteSummary = {
  url: string;
  domain: string;
  profileIds: string[];
  profiles: AssociatedProfileSummary[];
};

export type HistoryCleanupSummary = {
  history: CleanupFileStatus;
  topSites: CleanupFileStatus;
  visitedLinks: CleanupFileStatus;
  shortcuts: CleanupFileStatus;
  sessions: CleanupFileStatus;
};

export type CleanupFileStatus = "found" | "missing";

export type CleanupHistoryInput = {
  browserId: string;
  profileIds: string[];
};

export type CleanupHistoryResult = {
  profileId: string;
  deletedFiles: string[];
  skippedFiles: string[];
  error: string | null;
};

export type CleanupHistoryResponse = {
  results: CleanupHistoryResult[];
};

export type RemoveExtensionsInput = {
  browserId: string;
  removals: ExtensionRemovalRequest[];
};

export type RemoveBookmarksInput = {
  browserId: string;
  removals: BookmarkRemovalRequest[];
};

export type ExtensionRemovalRequest = {
  extensionId: string;
  profileIds: string[];
};

export type BookmarkRemovalRequest = {
  url: string;
  profileIds: string[];
};

export type RemoveExtensionsResponse = {
  results: RemoveExtensionResult[];
};

export type RemoveBookmarksResponse = {
  results: RemoveBookmarkResult[];
};

export type RemoveExtensionResult = {
  extensionId: string;
  profileId: string;
  removedFiles: string[];
  skippedFiles: string[];
  error: string | null;
};

export type RemoveBookmarkResult = {
  url: string;
  profileId: string;
  removedCount: number;
  removedFiles: string[];
  skippedFiles: string[];
  error: string | null;
};

export type AssociatedProfileSummary = {
  id: string;
  name: string;
  avatarDataUrl: string | null;
  avatarIcon: string | null;
  defaultAvatarFillColor: number | null;
  defaultAvatarStrokeColor: number | null;
  avatarLabel: string;
};

export type ExtensionInstallSource = "store" | "external";

export type ExtensionAssociatedProfileSummary = {
  id: string;
  name: string;
  avatarDataUrl: string | null;
  avatarIcon: string | null;
  defaultAvatarFillColor: number | null;
  defaultAvatarStrokeColor: number | null;
  avatarLabel: string;
  installSource: ExtensionInstallSource;
};

export type BookmarkAssociatedProfileSummary = {
  id: string;
  name: string;
  avatarDataUrl: string | null;
  avatarIcon: string | null;
  defaultAvatarFillColor: number | null;
  defaultAvatarStrokeColor: number | null;
  avatarLabel: string;
  bookmarkPath: string;
};

export type ProfileSortKey = "name" | "email" | "id";
export type ExtensionSortKey = "name" | "id";
export type BookmarkSortKey = "title" | "url";
export type PasswordSiteSortKey = "domain" | "url";
export type AssociatedProfileSortKey = "id" | "name";
export type ActiveSection = "profiles" | "extensions" | "bookmarks" | "passwords" | "history";
export type AppPage = "browserData" | "configuration";
export type BrowserConfigSource = "default" | "custom";

export type BrowserConfigEntry = {
  id: string;
  source: BrowserConfigSource;
  browserFamilyId: string | null;
  iconKey: string | null;
  name: string;
  executablePath: string;
  userDataPath: string;
  deletable: boolean;
};

export type BrowserConfigListResponse = {
  configs: BrowserConfigEntry[];
};

export type CreateCustomBrowserConfigInput = {
  name: string;
  iconKey: string | null;
  executablePath: string;
  userDataPath: string;
};

export type BrowserView = {
  browserId: string;
  browserFamilyId: string | null;
  browserName: string;
  iconKey: string | null;
  dataRoot: string;
  profiles: ProfileSummary[];
  extensions: ExtensionSummary[];
  bookmarks: BookmarkSummary[];
  passwordSites: PasswordSiteSummary[];
  stats: BrowserStats;
};

export type ScanResponse = {
  browsers: BrowserView[];
};

export type PasswordSitesResponse = {
  browserId: string;
  passwordSites: PasswordSiteSummary[];
};
