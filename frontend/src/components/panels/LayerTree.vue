<template>
	<LayoutCol class="layer-tree" @dragleave="dragInPanel = false">
		<LayoutRow class="options-bar" :scrollableX="true">
			<WidgetLayout :layout="layerTreeOptionsLayout" />
		</LayoutRow>
		<LayoutRow class="layer-tree-rows" :scrollableY="true">
			<LayoutCol class="list" ref="list" @click="() => deselectAllLayers()" @dragover="(e: DragEvent) => draggable && updateInsertLine(e)" @dragend="() => draggable && drop()">
				<LayoutRow
					class="layer-row"
					v-for="(listing, index) in layers"
					:key="String(listing.entry.path.slice(-1))"
					:class="{ 'insert-folder': draggingData?.highlightFolder && draggingData?.insertFolder === listing.entry.path }"
				>
					<LayoutRow class="visibility">
						<IconButton
							:action="(e?: MouseEvent) => (toggleLayerVisibility(listing.entry.path), e?.stopPropagation())"
							:size="24"
							:icon="listing.entry.visible ? 'EyeVisible' : 'EyeHidden'"
							:title="listing.entry.visible ? 'Visible' : 'Hidden'"
						/>
					</LayoutRow>

					<div class="indent" :style="{ marginLeft: layerIndent(listing.entry) }"></div>

					<button
						v-if="listing.entry.layerType === 'Folder'"
						class="expand-arrow"
						:class="{ expanded: listing.entry.layerMetadata.expanded }"
						@click.stop="handleExpandArrowClick(listing.entry.path)"
						tabindex="0"
					></button>
					<LayoutRow
						class="layer"
						:class="{ selected: fakeHighlight ? fakeHighlight.includes(listing.entry.path) : listing.entry.layerMetadata.selected }"
						:data-layer="String(listing.entry.path)"
						:data-index="index"
						:title="listing.entry.tooltip"
						:draggable="draggable"
						@dragstart="(e: DragEvent) => draggable && dragStart(e, listing)"
						@click.exact="(e: MouseEvent) => selectLayer(false, false, false, listing, e)"
						@click.shift.exact="(e: MouseEvent) => selectLayer(false, false, true, listing, e)"
						@click.ctrl.exact="(e: MouseEvent) => selectLayer(true, false, false, listing, e)"
						@click.ctrl.shift.exact="(e: MouseEvent) => selectLayer(true, false, true, listing, e)"
						@click.meta.exact="(e: MouseEvent) => selectLayer(false, true, false, listing, e)"
						@click.meta.shift.exact="(e: MouseEvent) => selectLayer(false, true, true, listing, e)"
						@click.ctrl.meta="(e: MouseEvent) => e.stopPropagation()"
						@click.alt="(e: MouseEvent) => e.stopPropagation()"
					>
						<LayoutRow class="layer-type-icon">
							<IconLabel :icon="layerTypeData(listing.entry.layerType).icon" :title="layerTypeData(listing.entry.layerType).name" />
						</LayoutRow>
						<LayoutRow class="layer-name" @dblclick="() => onEditLayerName(listing)">
							<input
								data-text-input
								type="text"
								:value="listing.entry.name"
								:placeholder="layerTypeData(listing.entry.layerType).name"
								:disabled="!listing.editingName"
								@blur="() => onEditLayerNameDeselect(listing)"
								@keydown.esc="onEditLayerNameDeselect(listing)"
								@keydown.enter="(e) => onEditLayerNameChange(listing, e)"
								@change="(e) => onEditLayerNameChange(listing, e)"
							/>
						</LayoutRow>
						<div class="thumbnail" v-html="listing.entry.thumbnail"></div>
					</LayoutRow>
				</LayoutRow>
			</LayoutCol>
			<div
				class="insert-mark"
				v-if="draggingData && !draggingData.highlightFolder && dragInPanel"
				:style="{ left: markIndent(draggingData.insertFolder), top: markTopOffset(draggingData.markerHeight) }"
			></div>
		</LayoutRow>
	</LayoutCol>
</template>

<style lang="scss">
.layer-tree {
	min-height: 0;

	// Options bar
	.options-bar {
		height: 32px;
		flex: 0 0 auto;
		margin: 0 4px;
		align-items: center;

		.widget-layout {
			width: 100%;
			min-width: 300px;
		}

		// Blend mode selector
		.dropdown-input {
			max-width: 120px;
		}

		// Blend mode selector and opacity slider
		.dropdown-input,
		.number-input {
			flex: 1 1 auto;
		}
	}

	// Layer tree
	.layer-tree-rows {
		margin-top: 4px;
		// Crop away the 1px border below the bottom layer entry when it uses the full space of this panel
		margin-bottom: -1px;
		position: relative;

		.layer-row {
			flex: 0 0 auto;
			align-items: center;
			position: relative;
			height: 32px;
			margin: 0 4px;
			border-bottom: 1px solid var(--color-4-dimgray);

			.visibility {
				flex: 0 0 auto;
				height: 100%;
				align-items: center;

				.icon-button {
					height: 100%;
					width: calc(24px + 2 * 4px);
				}
			}

			.expand-arrow {
				padding: 0;
				margin: 0;
				margin-left: -16px;
				width: 16px;
				height: 100%;
				border: none;
				position: relative;
				background: none;
				flex: 0 0 auto;
				display: flex;
				align-items: center;
				justify-content: center;
				border-radius: 2px;

				&:hover {
					background: var(--color-6-lowergray);
				}

				&::after {
					content: "";
					position: absolute;
					width: 0;
					height: 0;
					border-style: solid;
					border-width: 3px 0 3px 6px;
					border-color: transparent transparent transparent var(--color-e-nearwhite);

					&:hover {
						color: var(--color-f-white);
					}
				}

				&.expanded::after {
					border-width: 6px 3px 0 3px;
					border-color: var(--color-e-nearwhite) transparent transparent transparent;

					&:hover {
						color: var(--color-f-white);
					}
				}
			}

			.layer {
				align-items: center;
				z-index: 1;
				width: 100%;
				height: 100%;
				padding: 0 4px;
				border-radius: 2px;
				margin-right: 8px;

				&.selected {
					background: var(--color-5-dullgray);
					color: var(--color-f-white);
				}

				.layer-type-icon {
					flex: 0 0 auto;
					margin: 0 4px;
				}

				.layer-name {
					flex: 1 1 100%;
					margin: 0 4px;

					input {
						color: inherit;
						background: none;
						border: none;
						outline: none; // Ok for input element
						margin: 0;
						padding: 0;
						text-overflow: ellipsis;
						white-space: nowrap;
						overflow: hidden;
						border-radius: 2px;
						height: 24px;
						width: 100%;

						&:disabled {
							-webkit-user-select: none; // Required as of Safari 15.0 (Graphite's minimum version) through the latest release
							user-select: none;
							// Workaround for `user-select: none` not working on <input> elements
							pointer-events: none;
						}

						&::placeholder {
							color: inherit;
							font-style: italic;
						}

						&:focus {
							background: var(--color-1-nearblack);
							padding: 0 4px;

							&::placeholder {
								opacity: 0.5;
							}
						}
					}
				}

				.thumbnail {
					width: 36px;
					height: 24px;
					margin: 2px 0;
					margin-left: 4px;
					background: white;
					border-radius: 2px;
					flex: 0 0 auto;

					svg {
						width: calc(100% - 4px);
						height: calc(100% - 4px);
						margin: 2px;
					}
				}
			}

			&.insert-folder .layer {
				outline: 3px solid var(--color-e-nearwhite);
				outline-offset: -3px;
			}
		}

		.insert-mark {
			position: absolute;
			// `left` is applied dynamically
			right: 0;
			background: var(--color-e-nearwhite);
			margin-top: -2px;
			height: 5px;
			z-index: 1;
			pointer-events: none;
		}
	}
}
</style>

<script lang="ts">
import { defineComponent, nextTick } from "vue";

import { beginDraggingElement } from "@/io-managers/drag";
import { platformIsMac } from "@/utility-functions/platform";
import {
	type LayerType,
	type LayerTypeData,
	type LayerPanelEntry,
	defaultWidgetLayout,
	UpdateDocumentLayerDetails,
	UpdateDocumentLayerTreeStructureJs,
	UpdateLayerTreeOptionsLayout,
	layerTypeData,
} from "@/wasm-communication/messages";

import LayoutCol from "@/components/layout/LayoutCol.vue";
import LayoutRow from "@/components/layout/LayoutRow.vue";
import IconButton from "@/components/widgets/buttons/IconButton.vue";
import IconLabel from "@/components/widgets/labels/IconLabel.vue";
import WidgetLayout from "@/components/widgets/WidgetLayout.vue";

type LayerListingInfo = { folderIndex: number; bottomLayer: boolean; editingName: boolean; entry: LayerPanelEntry };

const RANGE_TO_INSERT_WITHIN_BOTTOM_FOLDER_NOT_ROOT = 20;
const LAYER_INDENT = 16;
const INSERT_MARK_MARGIN_LEFT = 4 + 32 + LAYER_INDENT;
const INSERT_MARK_OFFSET = 2;

type DraggingData = { select?: () => void; insertFolder: BigUint64Array; insertIndex: number; highlightFolder: boolean; markerHeight: number };

export default defineComponent({
	inject: ["editor"],
	data() {
		return {
			// Layer data
			layerCache: new Map() as Map<string, LayerPanelEntry>, // TODO: replace with BigUint64Array as index
			layers: [] as LayerListingInfo[],

			// Interactive dragging
			draggable: true,
			draggingData: undefined as undefined | DraggingData,
			fakeHighlight: undefined as undefined | BigUint64Array[],
			dragInPanel: false,

			// Layouts
			layerTreeOptionsLayout: defaultWidgetLayout(),
		};
	},
	methods: {
		layerIndent(layer: LayerPanelEntry): string {
			return `${layer.path.length * LAYER_INDENT}px`;
		},
		markIndent(path: BigUint64Array): string {
			return `${INSERT_MARK_MARGIN_LEFT + path.length * LAYER_INDENT}px`;
		},
		markTopOffset(height: number): string {
			return `${height}px`;
		},
		toggleLayerVisibility(path: BigUint64Array) {
			this.editor.instance.toggleLayerVisibility(path);
		},
		handleExpandArrowClick(path: BigUint64Array) {
			this.editor.instance.toggleLayerExpansion(path);
		},
		async onEditLayerName(listing: LayerListingInfo) {
			if (listing.editingName) return;

			listing.editingName = true;
			this.draggable = false;

			await nextTick();

			const tree: HTMLDivElement | undefined = (this.$refs.list as typeof LayoutCol | undefined)?.$el;
			const textInput: HTMLInputElement | undefined = tree?.querySelector("[data-text-input]:not([disabled])") || undefined;
			textInput?.select();
		},
		onEditLayerNameChange(listing: LayerListingInfo, e: Event) {
			// Eliminate duplicate events
			if (!listing.editingName) return;

			this.draggable = true;

			const name = (e.target as HTMLInputElement | undefined)?.value;
			listing.editingName = false;
			if (name) this.editor.instance.setLayerName(listing.entry.path, name);
		},
		async onEditLayerNameDeselect(listing: LayerListingInfo) {
			this.draggable = true;

			listing.editingName = false;

			await nextTick();
			window.getSelection()?.removeAllRanges();
		},
		async selectLayer(ctrl: boolean, cmd: boolean, shift: boolean, listing: LayerListingInfo, event: Event) {
			if (listing.editingName) return;

			const ctrlOrCmd = platformIsMac() ? cmd : ctrl;
			// Pressing the Ctrl key on a Mac, or the Cmd key on another platform, is a violation of the `.exact` qualifier so we filter it out here
			const opposite = platformIsMac() ? ctrl : cmd;

			if (!opposite) this.editor.instance.selectLayer(listing.entry.path, ctrlOrCmd, shift);

			// We always want to stop propagation so the click event doesn't pass through the layer and cause a deselection by clicking the layer panel background
			// This is also why we cover the remaining cases not considered by the `.exact` qualifier, in the last two bindings on the layer element, with a `stopPropagation()` call
			event.stopPropagation();
		},
		async deselectAllLayers() {
			this.editor.instance.deselectAllLayers();
		},
		calculateDragIndex(tree: HTMLDivElement, clientY: number, select?: () => void): DraggingData {
			const treeChildren = tree.children;
			const treeOffset = tree.getBoundingClientRect().top;

			// Closest distance to the middle of the row along the Y axis
			let closest = Infinity;

			// Folder to insert into
			let insertFolder = new BigUint64Array();

			// Insert index
			let insertIndex = -1;

			// Whether you are inserting into a folder and should show the folder outline
			let highlightFolder = false;

			let markerHeight = 0;
			let previousHeight = undefined as undefined | number;

			Array.from(treeChildren).forEach((treeChild, index) => {
				const layerComponents = treeChild.getElementsByClassName("layer");
				if (layerComponents.length !== 1) return;
				const child = layerComponents[0];

				const indexAttribute = child.getAttribute("data-index");
				if (!indexAttribute) return;
				const { folderIndex, entry: layer } = this.layers[parseInt(indexAttribute, 10)];

				const rect = child.getBoundingClientRect();
				const position = rect.top + rect.height / 2;
				const distance = position - clientY;

				// Inserting above current row
				if (distance > 0 && distance < closest) {
					insertFolder = layer.path.slice(0, layer.path.length - 1);
					insertIndex = folderIndex;
					highlightFolder = false;
					closest = distance;
					markerHeight = previousHeight || treeOffset + INSERT_MARK_OFFSET;
				}
				// Inserting below current row
				else if (distance > -closest && distance > -RANGE_TO_INSERT_WITHIN_BOTTOM_FOLDER_NOT_ROOT && distance < 0) {
					insertFolder = layer.layerType === "Folder" ? layer.path : layer.path.slice(0, layer.path.length - 1);
					insertIndex = layer.layerType === "Folder" ? 0 : folderIndex + 1;
					highlightFolder = layer.layerType === "Folder";
					closest = -distance;
					markerHeight = index === treeChildren.length - 1 ? rect.bottom - INSERT_MARK_OFFSET : rect.bottom;
				}
				// Inserting with no nesting at the end of the panel
				else if (closest === Infinity) {
					if (layer.path.length === 1) insertIndex = folderIndex + 1;

					markerHeight = rect.bottom - INSERT_MARK_OFFSET;
				}
				previousHeight = rect.bottom;
			});

			markerHeight -= treeOffset;

			return { select, insertFolder, insertIndex, highlightFolder, markerHeight };
		},
		async dragStart(event: DragEvent, listing: LayerListingInfo) {
			const layer = listing.entry;
			this.dragInPanel = true;
			if (!layer.layerMetadata.selected) {
				this.fakeHighlight = [layer.path];
			}
			const select = (): void => {
				if (!layer.layerMetadata.selected) this.selectLayer(false, false, false, listing, event);
			};

			const target = (event.target || undefined) as HTMLElement | undefined;
			const draggingELement = (target?.closest("[data-layer]") || undefined) as HTMLElement | undefined;
			if (draggingELement) beginDraggingElement(draggingELement);

			// Set style of cursor for drag
			if (event.dataTransfer) {
				event.dataTransfer.dropEffect = "move";
				event.dataTransfer.effectAllowed = "move";
			}

			const tree: HTMLDivElement | undefined = (this.$refs.list as typeof LayoutCol | undefined)?.$el;
			if (tree) this.draggingData = this.calculateDragIndex(tree, event.clientY, select);
		},
		updateInsertLine(event: DragEvent) {
			// Stop the drag from being shown as cancelled
			event.preventDefault();
			this.dragInPanel = true;

			const tree: HTMLDivElement | undefined = (this.$refs.list as typeof LayoutCol | undefined)?.$el;
			if (tree) this.draggingData = this.calculateDragIndex(tree, event.clientY, this.draggingData?.select);
		},
		async drop() {
			if (this.draggingData && this.dragInPanel) {
				const { select, insertFolder, insertIndex } = this.draggingData;

				select?.();
				this.editor.instance.moveLayerInTree(insertFolder, insertIndex);
			}
			this.draggingData = undefined;
			this.fakeHighlight = undefined;
			this.dragInPanel = false;
		},
		rebuildLayerTree(updateDocumentLayerTreeStructure: UpdateDocumentLayerTreeStructureJs) {
			const layerWithNameBeingEdited = this.layers.find((layer: LayerListingInfo) => layer.editingName);
			const layerPathWithNameBeingEdited = layerWithNameBeingEdited?.entry.path;
			const layerIdWithNameBeingEdited = layerPathWithNameBeingEdited?.slice(-1)[0];
			const path = [] as bigint[];
			this.layers = [] as LayerListingInfo[];

			const recurse = (folder: UpdateDocumentLayerTreeStructureJs, layers: LayerListingInfo[], cache: Map<string, LayerPanelEntry>): void => {
				folder.children.forEach((item, index) => {
					// TODO: fix toString
					const layerId = BigInt(item.layerId.toString());
					path.push(layerId);

					const mapping = cache.get(path.toString());
					if (mapping) {
						layers.push({
							folderIndex: index,
							bottomLayer: index === folder.children.length - 1,
							entry: mapping,
							editingName: layerIdWithNameBeingEdited === layerId,
						});
					}

					// Call self recursively if there are any children
					if (item.children.length >= 1) recurse(item, layers, cache);

					path.pop();
				});
			};

			recurse(updateDocumentLayerTreeStructure, this.layers, this.layerCache);
		},
		layerTypeData(layerType: LayerType): LayerTypeData {
			return layerTypeData(layerType) || { name: "Error", icon: "Info" };
		},
	},
	mounted() {
		this.editor.subscriptions.subscribeJsMessage(UpdateDocumentLayerTreeStructureJs, (updateDocumentLayerTreeStructure) => {
			this.rebuildLayerTree(updateDocumentLayerTreeStructure);
		});

		this.editor.subscriptions.subscribeJsMessage(UpdateLayerTreeOptionsLayout, (updateLayerTreeOptionsLayout) => {
			this.layerTreeOptionsLayout = updateLayerTreeOptionsLayout;
		});

		this.editor.subscriptions.subscribeJsMessage(UpdateDocumentLayerDetails, (updateDocumentLayerDetails) => {
			const targetPath = updateDocumentLayerDetails.data.path;
			const targetLayer = updateDocumentLayerDetails.data;

			const layer = this.layerCache.get(targetPath.toString());
			if (layer) {
				Object.assign(layer, targetLayer);
			} else {
				this.layerCache.set(targetPath.toString(), targetLayer);
			}
		});
	},
	components: {
		IconButton,
		IconLabel,
		LayoutCol,
		LayoutRow,
		WidgetLayout,
	},
});
</script>
