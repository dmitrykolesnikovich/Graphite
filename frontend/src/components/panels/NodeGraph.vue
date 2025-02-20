<template>
	<LayoutCol class="node-graph">
		<LayoutRow class="options-bar"></LayoutRow>
		<LayoutRow
			class="graph"
			ref="graph"
			@wheel="(e: WheelEvent) => scroll(e)"
			@pointerdown="(e: PointerEvent) => pointerDown(e)"
			@pointermove="(e: PointerEvent) => pointerMove(e)"
			@pointerup="(e: PointerEvent) => pointerUp(e)"
			:style="{
				'--grid-spacing': `${gridSpacing}px`,
				'--grid-offset-x': `${transform.x * transform.scale}px`,
				'--grid-offset-y': `${transform.y * transform.scale}px`,
				'--dot-radius': `${dotRadius}px`,
			}"
		>
			<LayoutCol class="node-list" v-if="nodeListLocation" :style="{ marginLeft: `${nodeListX}px`, marginTop: `${nodeListY}px` }">
				<TextInput placeholder="Search Nodes..." :value="searchTerm" @update:value="(val) => (searchTerm = val)" v-focus />
				<LayoutCol v-for="nodeCategory in nodeCategories" :key="nodeCategory[0]">
					<TextLabel>{{ nodeCategory[0] }}</TextLabel>
					<TextButton v-for="nodeType in nodeCategory[1]" v-bind:key="String(nodeType)" :label="nodeType.name" :action="() => createNode(nodeType.name)" />
				</LayoutCol>
				<TextLabel v-if="nodeCategories.length === 0">No search results :(</TextLabel>
			</LayoutCol>
			<div
				class="nodes"
				ref="nodesContainer"
				:style="{
					transform: `scale(${transform.scale}) translate(${transform.x}px, ${transform.y}px)`,
					transformOrigin: `0 0`,
				}"
			>
				<div
					v-for="node in nodes"
					:key="String(node.id)"
					class="node"
					:class="{ selected: selected.includes(node.id) }"
					:style="{
						'--offset-left': (node.position?.x || 0) + (selected.includes(node.id) ? draggingNodes?.roundX || 0 : 0),
						'--offset-top': (node.position?.y || 0) + (selected.includes(node.id) ? draggingNodes?.roundY || 0 : 0),
					}"
					:data-node="node.id"
				>
					<div class="primary">
						<div class="ports">
							<div
								v-if="node.primaryInput"
								class="input port"
								data-port="input"
								:data-datatype="node.primaryInput"
								:style="{ '--data-color': `var(--color-data-${node.primaryInput})`, '--data-color-dim': `var(--color-data-${node.primaryInput}-dim)` }"
							>
								<div></div>
							</div>
							<div
								v-if="node.outputs.length > 0"
								class="output port"
								data-port="output"
								:data-datatype="node.outputs[0]"
								:style="{ '--data-color': `var(--color-data-${node.outputs[0]})`, '--data-color-dim': `var(--color-data-${node.outputs[0]}-dim)` }"
							>
								<div></div>
							</div>
						</div>
						<IconLabel :icon="nodeIcon(node.displayName)" />
						<TextLabel>{{ node.displayName }}</TextLabel>
					</div>
					<div v-if="node.exposedInputs.length > 0" class="arguments">
						<div v-for="(argument, index) in node.exposedInputs" :key="index" class="argument">
							<div class="ports">
								<div
									class="input port"
									data-port="input"
									:data-datatype="argument.dataType"
									:style="{ '--data-color': `var(--color-data-${argument.dataType})`, '--data-color-dim': `var(--color-data-${argument.dataType}-dim)` }"
								>
									<div></div>
								</div>
							</div>
							<TextLabel>{{ argument.name }}</TextLabel>
						</div>
					</div>
				</div>
			</div>
			<div
				class="wires"
				:style="{
					transform: `scale(${transform.scale}) translate(${transform.x}px, ${transform.y}px)`,
					transformOrigin: `0 0`,
				}"
			>
				<svg>
					<path
						v-for="([pathString, dataType], index) in linkPaths"
						:key="index"
						:d="pathString"
						:style="{ '--data-color': `var(--color-data-${dataType})`, '--data-color-dim': `var(--color-data-${dataType}-dim)` }"
					/>
				</svg>
			</div>
		</LayoutRow>
	</LayoutCol>
</template>

<style lang="scss">
.node-graph {
	height: 100%;
	position: relative;

	.node-list {
		width: max-content;
		position: fixed;
		padding: 5px;
		z-index: 3;
		background-color: var(--color-3-darkgray);

		.text-button + .text-button {
			margin-left: 0;
			margin-top: 4px;
		}
	}

	.options-bar {
		height: 32px;
		margin: 0 4px;
		flex: 0 0 auto;
		align-items: center;
	}

	.graph {
		position: relative;
		background: var(--color-2-mildblack);
		width: calc(100% - 8px);
		margin-left: 4px;
		margin-bottom: 4px;
		border-radius: 2px;
		overflow: hidden;

		// We're displaying the dotted grid in a pseudo-element because `image-rendering` is an inherited property and we don't want it to apply to child elements
		&::before {
			content: "";
			position: absolute;
			width: 100%;
			height: 100%;
			background-size: var(--grid-spacing) var(--grid-spacing);
			background-position: calc(var(--grid-offset-x) - var(--dot-radius)) calc(var(--grid-offset-y) - var(--dot-radius));
			background-image: radial-gradient(circle at var(--dot-radius) var(--dot-radius), var(--color-3-darkgray) var(--dot-radius), transparent 0);
			image-rendering: pixelated;
			mix-blend-mode: screen;
		}
	}

	.nodes,
	.wires {
		position: absolute;
		width: 100%;
		height: 100%;

		&.wires {
			width: 100%;
			height: 100%;
			pointer-events: none;

			svg {
				width: 100%;
				height: 100%;
				overflow: visible;

				path {
					fill: none;
					// stroke: var(--color-data-raster-dim);
					stroke: var(--data-color-dim);
					stroke-width: 2px;
				}
			}
		}

		&.nodes {
			.node {
				position: absolute;
				display: flex;
				flex-direction: column;
				min-width: 120px;
				border-radius: 4px;
				background: var(--color-4-dimgray);
				left: calc((var(--offset-left) + 0.5) * 24px);
				top: calc((var(--offset-top) - 0.5) * 24px);

				&.selected {
					border: 1px solid var(--color-e-nearwhite);
					margin: -1px;
				}

				.primary {
					display: flex;
					align-items: center;
					position: relative;
					gap: 4px;
					width: 100%;
					height: 24px;
					background: var(--color-5-dullgray);
					border-radius: 4px;

					.icon-label {
						margin-left: 4px;
					}

					.text-label {
						margin-right: 4px;
					}
				}

				.arguments {
					display: flex;
					flex-direction: column;
					width: 100%;
					position: relative;

					.argument {
						position: relative;
						display: flex;
						align-items: center;
						height: 24px;
						width: 100%;
						margin-left: 24px;
						margin-right: 24px;
					}

					// Squares to cover up the rounded corners of the primary area and make them have a straight edge
					&::before,
					&::after {
						content: "";
						position: absolute;
						background: var(--color-5-dullgray);
						width: 4px;
						height: 4px;
						top: -4px;
					}

					&::before {
						left: 0;
					}

					&::after {
						right: 0;
					}
				}

				.ports {
					position: absolute;
					width: 100%;
					height: 100%;

					.port {
						position: absolute;
						margin: auto 0;
						top: 0;
						bottom: 0;
						width: 12px;
						height: 12px;
						border-radius: 50%;
						background: var(--data-color-dim);
						// background: var(--color-data-raster-dim);

						div {
							background: var(--data-color);
							// background: var(--color-data-raster);
							width: 8px;
							height: 8px;
							border-radius: 50%;
							position: absolute;
							top: 0;
							bottom: 0;
							left: 0;
							right: 0;
							margin: auto;
						}

						&.input {
							left: calc(-12px - 6px);
						}

						&.output {
							right: calc(-12px - 6px);
						}
					}
				}
			}
		}
	}
}
</style>

<script lang="ts">
import { defineComponent, nextTick } from "vue";

// import type { FrontendNode } from "@/wasm-communication/messages";

import type { IconName } from "@/utility-functions/icons";

import LayoutCol from "@/components/layout/LayoutCol.vue";
import LayoutRow from "@/components/layout/LayoutRow.vue";
import TextButton from "@/components/widgets/buttons/TextButton.vue";
import TextInput from "@/components/widgets/inputs/TextInput.vue";
import IconLabel from "@/components/widgets/labels/IconLabel.vue";
import TextLabel from "@/components/widgets/labels/TextLabel.vue";

const WHEEL_RATE = (1 / 600) * 3;
const GRID_COLLAPSE_SPACING = 10;
const GRID_SIZE = 24;

export default defineComponent({
	inject: ["nodeGraph", "editor"],
	data() {
		return {
			transform: { scale: 1, x: 0, y: 0 },
			panning: false,
			selected: [] as bigint[],
			draggingNodes: undefined as { startX: number; startY: number; roundX: number; roundY: number } | undefined,
			selectIfNotDragged: undefined as undefined | bigint,
			linkInProgressFromConnector: undefined as HTMLDivElement | undefined,
			linkInProgressToConnector: undefined as HTMLDivElement | DOMRect | undefined,
			nodeLinkPaths: [] as [string, string][],
			searchTerm: "",
			nodeListLocation: undefined as { x: number; y: number } | undefined,
		};
	},
	computed: {
		gridSpacing(): number {
			const dense = this.transform.scale * GRID_SIZE;
			let sparse = dense;

			while (sparse > 0 && sparse < GRID_COLLAPSE_SPACING) {
				sparse *= 2;
			}

			return sparse;
		},
		dotRadius(): number {
			return 1 + Math.floor(this.transform.scale - 0.5 + 0.001) / 2;
		},
		nodes() {
			return this.nodeGraph.state.nodes;
		},
		nodeCategories() {
			const categories = new Map();
			this.nodeGraph.state.nodeTypes.forEach((node) => {
				if (this.searchTerm.length && !node.name.toLowerCase().includes(this.searchTerm.toLowerCase()) && !node.category.toLowerCase().includes(this.searchTerm.toLowerCase())) return;

				const category = categories.get(node.category);
				if (category) category.push(node);
				else categories.set(node.category, [node]);
			});

			const result = Array.from(categories);
			return result;
		},
		nodeListX() {
			return ((this.nodeListLocation?.x || 0) * GRID_SIZE + this.transform.x) * this.transform.scale;
		},
		nodeListY() {
			return ((this.nodeListLocation?.y || 0) * GRID_SIZE + this.transform.y) * this.transform.scale;
		},
		linkPathInProgress(): [string, string] | undefined {
			if (this.linkInProgressFromConnector && this.linkInProgressToConnector) {
				return this.createWirePath(this.linkInProgressFromConnector, this.linkInProgressToConnector, false, false);
			}
			return undefined;
		},
		linkPaths(): [string, string][] {
			const linkPathInProgress = this.linkPathInProgress ? [this.linkPathInProgress] : [];
			return [...linkPathInProgress, ...this.nodeLinkPaths];
		},
	},
	watch: {
		nodes: {
			immediate: true,
			async handler() {
				await this.refreshLinks();
			},
		},
	},
	methods: {
		async refreshLinks(): Promise<void> {
			await nextTick();

			const containerBounds = this.$refs.nodesContainer as HTMLDivElement | undefined;
			if (!containerBounds) return;

			const links = this.nodeGraph.state.links;
			this.nodeLinkPaths = links.flatMap((link) => {
				const connectorIndex = Number(link.linkEndInputIndex);

				const nodePrimaryOutput = (containerBounds.querySelector(`[data-node="${String(link.linkStart)}"] [data-port="output"]`) || undefined) as HTMLDivElement | undefined;

				const nodeInputConnectors = containerBounds.querySelectorAll(`[data-node="${String(link.linkEnd)}"] [data-port="input"]`) || undefined;
				const nodePrimaryInput = nodeInputConnectors?.[connectorIndex] as HTMLDivElement | undefined;

				if (!nodePrimaryInput || !nodePrimaryOutput) return [];
				return [this.createWirePath(nodePrimaryOutput, nodePrimaryInput.getBoundingClientRect(), false, false)];
			});
		},
		nodeIcon(nodeName: string): IconName {
			const iconMap: Record<string, IconName> = {
				Output: "NodeOutput",
				"Hue Shift Image": "NodeColorCorrection",
				"Brighten Image": "NodeColorCorrection",
				"Grayscale Image": "NodeColorCorrection",
			};
			return iconMap[nodeName] || "NodeNodes";
		},
		buildWirePathString(outputBounds: DOMRect, inputBounds: DOMRect, verticalOut: boolean, verticalIn: boolean): string {
			const containerBounds = (this.$refs.nodesContainer as HTMLDivElement | undefined)?.getBoundingClientRect();
			if (!containerBounds) return "[error]";

			const outX = verticalOut ? outputBounds.x + outputBounds.width / 2 : outputBounds.x + outputBounds.width - 1;
			const outY = verticalOut ? outputBounds.y + 1 : outputBounds.y + outputBounds.height / 2;
			const outConnectorX = (outX - containerBounds.x) / this.transform.scale;
			const outConnectorY = (outY - containerBounds.y) / this.transform.scale;

			const inX = verticalIn ? inputBounds.x + inputBounds.width / 2 : inputBounds.x + 1;
			const inY = verticalIn ? inputBounds.y + inputBounds.height - 1 : inputBounds.y + inputBounds.height / 2;
			const inConnectorX = (inX - containerBounds.x) / this.transform.scale;
			const inConnectorY = (inY - containerBounds.y) / this.transform.scale;
			const horizontalGap = Math.abs(outConnectorX - inConnectorX);
			const verticalGap = Math.abs(outConnectorY - inConnectorY);

			const curveLength = 200;
			const curveFalloffRate = curveLength * Math.PI * 2;

			const horizontalCurveAmount = -(2 ** ((-10 * horizontalGap) / curveFalloffRate)) + 1;
			const verticalCurveAmount = -(2 ** ((-10 * verticalGap) / curveFalloffRate)) + 1;
			const horizontalCurve = horizontalCurveAmount * curveLength;
			const verticalCurve = verticalCurveAmount * curveLength;

			return `M${outConnectorX},${outConnectorY} C${verticalOut ? outConnectorX : outConnectorX + horizontalCurve},${verticalOut ? outConnectorY - verticalCurve : outConnectorY} ${
				verticalIn ? inConnectorX : inConnectorX - horizontalCurve
			},${verticalIn ? inConnectorY + verticalCurve : inConnectorY} ${inConnectorX},${inConnectorY}`;
		},
		createWirePath(outputPort: HTMLDivElement, inputPort: HTMLDivElement | DOMRect, verticalOut: boolean, verticalIn: boolean): [string, string] {
			const inputPortRect = inputPort instanceof HTMLDivElement ? inputPort.getBoundingClientRect() : inputPort;

			const pathString = this.buildWirePathString(outputPort.getBoundingClientRect(), inputPortRect, verticalOut, verticalIn);
			const dataType = outputPort.getAttribute("data-datatype") || "general";

			return [pathString, dataType];
		},
		scroll(e: WheelEvent) {
			const scrollX = e.deltaX;
			const scrollY = e.deltaY;

			// Zoom
			if (e.ctrlKey) {
				let zoomFactor = 1 + Math.abs(scrollY) * WHEEL_RATE;
				if (scrollY > 0) zoomFactor = 1 / zoomFactor;

				const graphDiv: HTMLDivElement | undefined = (this.$refs.graph as typeof LayoutCol | undefined)?.$el;
				if (!graphDiv) return;
				const { x, y, width, height } = graphDiv.getBoundingClientRect();

				this.transform.scale *= zoomFactor;

				const newViewportX = width / zoomFactor;
				const newViewportY = height / zoomFactor;

				const deltaSizeX = width - newViewportX;
				const deltaSizeY = height - newViewportY;

				const deltaX = deltaSizeX * ((e.x - x) / width);
				const deltaY = deltaSizeY * ((e.y - y) / height);

				this.transform.x -= (deltaX / this.transform.scale) * zoomFactor;
				this.transform.y -= (deltaY / this.transform.scale) * zoomFactor;

				// Prevent actually zooming into the page when pinch-zooming on laptop trackpads
				e.preventDefault();
			}
			// Pan
			else if (!e.shiftKey) {
				this.transform.x -= scrollX / this.transform.scale;
				this.transform.y -= scrollY / this.transform.scale;
			} else {
				this.transform.x -= scrollY / this.transform.scale;
			}
		},
		pointerDown(e: PointerEvent) {
			if (e.button === 2) {
				const graphDiv: HTMLDivElement | undefined = (this.$refs.graph as typeof LayoutCol | undefined)?.$el;
				const graph = graphDiv?.getBoundingClientRect() || new DOMRect();
				this.nodeListLocation = {
					x: Math.round(((e.clientX - graph.x) / this.transform.scale - this.transform.x) / GRID_SIZE),
					y: Math.round(((e.clientY - graph.y) / this.transform.scale - this.transform.y) / GRID_SIZE),
				};
				return;
			}

			const port = (e.target as HTMLDivElement).closest("[data-port]") as HTMLDivElement;
			const node = (e.target as HTMLElement).closest("[data-node]") as HTMLElement | undefined;
			const nodeList = (e.target as HTMLElement).closest(".node-list") as HTMLElement | undefined;

			if (port) {
				const isOutput = Boolean(port.getAttribute("data-port") === "output");

				if (isOutput) this.linkInProgressFromConnector = port;
			} else {
				const nodeId = node?.getAttribute("data-node") || undefined;
				if (nodeId) {
					const id = BigInt(nodeId);
					if (e.shiftKey || e.ctrlKey) {
						if (this.selected.includes(id)) this.selected.splice(this.selected.lastIndexOf(id), 1);
						else this.selected.push(id);
					} else if (!this.selected.includes(id)) {
						this.selected = [id];
					} else {
						this.selectIfNotDragged = id;
					}

					if (this.selected.includes(id)) {
						this.draggingNodes = { startX: e.x, startY: e.y, roundX: 0, roundY: 0 };
						const graphDiv: HTMLDivElement | undefined = (this.$refs.graph as typeof LayoutCol | undefined)?.$el;
						graphDiv?.setPointerCapture(e.pointerId);
					}

					this.editor.instance.selectNodes(new BigUint64Array(this.selected));
				} else if (!nodeList) {
					this.selected = [];
					this.editor.instance.selectNodes(new BigUint64Array(this.selected));
					const graphDiv: HTMLDivElement | undefined = (this.$refs.graph as typeof LayoutCol | undefined)?.$el;
					graphDiv?.setPointerCapture(e.pointerId);

					this.panning = true;
				}
			}
		},
		pointerMove(e: PointerEvent) {
			if (this.panning) {
				this.transform.x += e.movementX / this.transform.scale;
				this.transform.y += e.movementY / this.transform.scale;
			} else if (this.linkInProgressFromConnector) {
				const target = e.target as Element | undefined;
				const dot = (target?.closest(`[data-port="input"]`) || undefined) as HTMLDivElement | undefined;
				if (dot) {
					this.linkInProgressToConnector = dot;
				} else {
					this.linkInProgressToConnector = new DOMRect(e.x, e.y);
				}
			} else if (this.draggingNodes) {
				const deltaX = Math.round((e.x - this.draggingNodes.startX) / this.transform.scale / GRID_SIZE);
				const deltaY = Math.round((e.y - this.draggingNodes.startY) / this.transform.scale / GRID_SIZE);
				if (this.draggingNodes.roundX !== deltaX || this.draggingNodes.roundY !== deltaY) {
					this.draggingNodes.roundX = deltaX;
					this.draggingNodes.roundY = deltaY;
					this.refreshLinks();
				}
			}
		},
		pointerUp(e: PointerEvent) {
			const graph: HTMLDivElement | undefined = (this.$refs.graph as typeof LayoutCol | undefined)?.$el;
			graph?.releasePointerCapture(e.pointerId);

			this.panning = false;

			if (this.linkInProgressToConnector instanceof HTMLDivElement && this.linkInProgressFromConnector) {
				const outputNode = this.linkInProgressFromConnector.closest("[data-node]");
				const inputNode = this.linkInProgressToConnector.closest("[data-node]");

				const outputConnectedNodeID = outputNode?.getAttribute("data-node") ?? undefined;
				const inputConnectedNodeID = inputNode?.getAttribute("data-node") ?? undefined;

				if (outputNode && inputNode && outputConnectedNodeID && inputConnectedNodeID) {
					const inputNodeInPorts = Array.from(inputNode.querySelectorAll(`[data-port="input"]`));
					const inputNodeConnectionIndexSearch = inputNodeInPorts.indexOf(this.linkInProgressToConnector);
					const inputNodeConnectionIndex = inputNodeConnectionIndexSearch > -1 ? inputNodeConnectionIndexSearch : undefined;

					if (inputNodeConnectionIndex !== undefined) {
						// const oneBasedIndex = inputNodeConnectionIndex + 1;

						this.editor.instance.connectNodesByLink(BigInt(outputConnectedNodeID), BigInt(inputConnectedNodeID), inputNodeConnectionIndex);
					}
				}
			} else if (this.draggingNodes) {
				if (this.draggingNodes.startX === e.x || this.draggingNodes.startY === e.y) {
					if (this.selectIfNotDragged) {
						this.selected = [this.selectIfNotDragged];
						this.editor.instance.selectNodes(new BigUint64Array(this.selected));
					}
				}
				this.editor.instance.moveSelectedNodes(this.draggingNodes.roundX, this.draggingNodes.roundY);
				this.draggingNodes = undefined;
				this.selectIfNotDragged = undefined;
			}

			this.linkInProgressFromConnector = undefined;
			this.linkInProgressToConnector = undefined;
		},
		createNode(nodeType: string): void {
			if (!this.nodeListLocation) return;

			this.editor.instance.createNode(nodeType, this.nodeListLocation.x, this.nodeListLocation.y);
			this.nodeListLocation = undefined;
		},
	},
	mounted() {
		const outputPort1 = document.querySelectorAll(`[data-port="output"]`)[4] as HTMLDivElement | undefined;
		const inputPort1 = document.querySelectorAll(`[data-port="input"]`)[1] as HTMLDivElement | undefined;
		if (outputPort1 && inputPort1) this.createWirePath(outputPort1, inputPort1.getBoundingClientRect(), true, true);

		const outputPort2 = document.querySelectorAll(`[data-port="output"]`)[6] as HTMLDivElement | undefined;
		const inputPort2 = document.querySelectorAll(`[data-port="input"]`)[3] as HTMLDivElement | undefined;
		if (outputPort2 && inputPort2) this.createWirePath(outputPort2, inputPort2.getBoundingClientRect(), true, false);
	},
	components: {
		IconLabel,
		LayoutCol,
		LayoutRow,
		TextLabel,
		TextButton,
		TextInput,
	},
});
</script>
