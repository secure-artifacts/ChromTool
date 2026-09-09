<script setup lang="ts" generic="Field extends string">
import { ref } from "vue";
import type { FilterMode, FilterOperator, FilterRule } from "../../types/browser";

const props = defineProps<{
  title: string;
  mode: FilterMode;
  rules: FilterRule<Field>[];
  fields: { value: Field; label: string }[];
  presets?: { value: string; label: string }[];
  activePresets?: string[];
}>();

const emit = defineEmits<{
  "update:mode": [value: FilterMode];
  "update:rules": [value: FilterRule<Field>[]];
  "update:activePresets": [value: string[]];
  close: [];
}>();

const operators: { value: FilterOperator; label: string }[] = [
  { value: "contains", label: "包含" },
  { value: "not_contains", label: "不包含" },
  { value: "equals", label: "等于" },
  { value: "not_equals", label: "不等于" },
  { value: "starts_with", label: "以开头" },
  { value: "not_starts_with", label: "不以开头" },
];

const openSelectKey = ref("");
let nextRuleId = 1;

function selectKey(ruleId: string, type: "field" | "operator") {
  return `${ruleId}:${type}`;
}

function fieldLabel(value: Field) {
  return props.fields.find((field) => field.value === value)?.label ?? value;
}

function operatorLabel(value: FilterOperator) {
  return operators.find((operator) => operator.value === value)?.label ?? value;
}

function toggleSelect(ruleId: string, type: "field" | "operator") {
  const key = selectKey(ruleId, type);
  openSelectKey.value = openSelectKey.value === key ? "" : key;
}

function createRule(): FilterRule<Field> {
  return {
    id: `rule-${Date.now()}-${nextRuleId++}`,
    field: props.fields[0].value,
    operator: "contains",
    value: "",
    disabled: false,
  };
}

function addRule() {
  emit("update:rules", [...props.rules, createRule()]);
}

function updateRule(ruleId: string, patch: Partial<Omit<FilterRule<Field>, "id">>) {
  openSelectKey.value = "";
  emit(
    "update:rules",
    props.rules.map((rule) => (rule.id === ruleId ? { ...rule, ...patch } : rule)),
  );
}

function removeRule(ruleId: string) {
  emit(
    "update:rules",
    props.rules.filter((rule) => rule.id !== ruleId),
  );
}

function clearRules() {
  emit("update:rules", []);
}

function isPresetActive(value: string) {
  return props.activePresets?.includes(value) ?? false;
}

function togglePreset(value: string) {
  const activePresets = props.activePresets ?? [];
  emit(
    "update:activePresets",
    activePresets.includes(value)
      ? activePresets.filter((preset) => preset !== value)
      : [...activePresets, value],
  );
}
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <section class="modal-card" @click="openSelectKey = ''">
      <div class="modal-header">
        <div>
          <h3>{{ title }}</h3>
        </div>
        <button class="secondary-button" type="button" @click="emit('close')">关闭</button>
      </div>

      <div class="filter-mode">
        <div class="mode-group">
          <button
            class="mode-button"
            :class="{ active: mode === 'and' }"
            type="button"
            @click="emit('update:mode', 'and')"
          >
            全部满足
          </button>
          <button
            class="mode-button"
            :class="{ active: mode === 'or' }"
            type="button"
            @click="emit('update:mode', 'or')"
          >
            任一满足
          </button>
        </div>
        <template v-if="presets?.length">
          <div class="filter-divider" aria-hidden="true"></div>
          <div class="preset-group">
            <button
              v-for="preset in presets"
              :key="preset.value"
              class="preset-button"
              :class="{ active: isPresetActive(preset.value) }"
              type="button"
              @click="togglePreset(preset.value)"
            >
              {{ preset.label }}
            </button>
          </div>
        </template>
      </div>

      <div v-if="rules.length" class="filter-rules">
        <div v-for="rule in rules" :key="rule.id" class="filter-rule" :class="{ disabled: rule.disabled }">
          <div class="filter-dropdown" @click.stop>
            <button
              class="filter-select-button"
              type="button"
              :disabled="rule.disabled"
              @click="toggleSelect(rule.id, 'field')"
            >
              <span>{{ fieldLabel(rule.field) }}</span>
              <span class="select-arrow" aria-hidden="true"></span>
            </button>
            <div v-if="openSelectKey === selectKey(rule.id, 'field')" class="select-menu">
              <button
                v-for="field in fields"
                :key="field.value"
                class="select-option"
                :class="{ selected: field.value === rule.field }"
                type="button"
                @click="updateRule(rule.id, { field: field.value })"
              >
                {{ field.label }}
              </button>
            </div>
          </div>
          <div class="filter-dropdown" @click.stop>
            <button
              class="filter-select-button"
              type="button"
              :disabled="rule.disabled"
              @click="toggleSelect(rule.id, 'operator')"
            >
              <span>{{ operatorLabel(rule.operator) }}</span>
              <span class="select-arrow" aria-hidden="true"></span>
            </button>
            <div v-if="openSelectKey === selectKey(rule.id, 'operator')" class="select-menu">
              <button
                v-for="operator in operators"
                :key="operator.value"
                class="select-option"
                :class="{ selected: operator.value === rule.operator }"
                type="button"
                @click="updateRule(rule.id, { operator: operator.value })"
              >
                {{ operator.label }}
              </button>
            </div>
          </div>
          <input
            class="filter-input"
            type="text"
            :value="rule.value"
            :disabled="rule.disabled"
            placeholder="输入关键词"
            @input="updateRule(rule.id, { value: ($event.target as HTMLInputElement).value })"
          />
          <button
            class="disable-rule-button"
            :class="{ active: rule.disabled }"
            type="button"
            :title="rule.disabled ? '启用条件' : '禁用条件'"
            @click="updateRule(rule.id, { disabled: !rule.disabled })"
          >
            禁用
          </button>
          <button class="remove-rule-button" type="button" @click="removeRule(rule.id)">
            移除
          </button>
        </div>
      </div>
      <div v-else class="empty-filter">
        <p>还没有过滤条件。</p>
      </div>

      <div class="filter-actions">
        <button class="add-rule-button" type="button" @click="addRule">添加条件</button>
        <button
          v-if="rules.length"
          class="clear-rule-button"
          type="button"
          @click="clearRules"
        >
          清空
        </button>
      </div>
    </section>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  z-index: 50;
  display: grid;
  place-items: center;
  padding: 24px;
  background: rgba(15, 23, 42, 0.26);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
}

.modal-card {
  width: min(680px, 100%);
  max-height: min(76vh, 820px);
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
  border: 1px solid var(--panel-border);
  border-radius: 22px;
  background: rgba(255, 255, 255, 0.96);
  box-shadow: 0 28px 70px rgba(15, 23, 42, 0.18);
}

.modal-header,
.filter-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
}

.modal-header h3,
.empty-filter p {
  margin: 0;
}

.modal-header h3 {
  font-weight: 600;
  letter-spacing: -0.03em;
}

.filter-mode,
.mode-group,
.preset-group {
  display: flex;
  align-items: center;
}

.filter-mode {
  gap: 4px;
  padding: 3px;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.82);
}

.mode-group,
.preset-group {
  gap: 4px;
}

.filter-divider {
  width: 1px;
  height: 20px;
  margin: 0 6px;
  background: rgba(148, 163, 184, 0.24);
}

.mode-button,
.preset-button {
  padding: 5px 10px;
  border-radius: 9px;
  color: var(--muted);
  font-size: 0.8rem;
  cursor: pointer;
}

.mode-button.active,
.preset-button.active {
  background: rgba(47, 111, 237, 0.1);
  color: var(--accent);
  font-weight: 700;
}

.filter-rules {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.filter-rule {
  display: grid;
  grid-template-columns: minmax(116px, 0.9fr) 92px minmax(160px, 1.4fr) auto auto;
  gap: 8px;
  align-items: center;
}

.filter-rule.disabled .filter-select-button,
.filter-rule.disabled .filter-input {
  opacity: 0.55;
}

.filter-select-button,
.filter-input {
  min-width: 0;
  height: 32px;
  border: 1px solid rgba(148, 163, 184, 0.24);
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.92);
  color: var(--text);
  font-size: 0.82rem;
}

.filter-dropdown {
  position: relative;
  min-width: 0;
}

.filter-select-button {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding: 0 10px;
  cursor: pointer;
}

.filter-select-button:disabled {
  cursor: default;
}

.select-arrow {
  width: 8px;
  height: 8px;
  flex-shrink: 0;
  border-right: 1.8px solid currentColor;
  border-bottom: 1.8px solid currentColor;
  transform: translateY(-2px) rotate(45deg);
}

.select-menu {
  position: absolute;
  top: calc(100% + 5px);
  left: 0;
  z-index: 5;
  width: 100%;
  min-width: max-content;
  overflow: hidden;
  padding: 4px;
  border: 1px solid rgba(148, 163, 184, 0.22);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.98);
  box-shadow: 0 18px 36px rgba(15, 23, 42, 0.14);
}

.select-option {
  display: block;
  width: 100%;
  padding: 7px 10px;
  border-radius: 9px;
  background: transparent;
  color: var(--text);
  font-size: 0.82rem;
  text-align: left;
  white-space: nowrap;
  cursor: pointer;
}

.select-option:hover,
.select-option.selected {
  background: rgba(47, 111, 237, 0.08);
  color: var(--accent);
  font-weight: 700;
}

.filter-input {
  padding: 0 10px;
}

.filter-input::placeholder {
  color: rgba(100, 116, 139, 0.68);
}

.add-rule-button,
.clear-rule-button,
.disable-rule-button,
.remove-rule-button {
  border-radius: 10px;
  font-size: 0.8rem;
  cursor: pointer;
}

.add-rule-button {
  padding: 6px 10px;
  background: rgba(47, 111, 237, 0.1);
  color: var(--accent);
  font-weight: 700;
}

.clear-rule-button,
.disable-rule-button,
.remove-rule-button {
  padding: 5px 9px;
  background: rgba(241, 245, 249, 0.9);
  color: var(--muted);
}

.disable-rule-button.active {
  background: rgba(239, 68, 68, 0.12);
  color: #b42318;
  font-weight: 700;
}

.disable-rule-button,
.remove-rule-button {
  white-space: nowrap;
}

.empty-filter {
  display: grid;
  min-height: 96px;
  place-items: center;
  border: 1px dashed rgba(148, 163, 184, 0.28);
  border-radius: 16px;
  color: var(--muted);
  font-size: 0.88rem;
}

@media (max-width: 760px) {
  .modal-backdrop {
    padding: 12px;
  }

  .filter-rule {
    grid-template-columns: minmax(0, 1fr) 86px;
  }

  .filter-input {
    grid-column: 1 / -1;
  }
}
</style>
