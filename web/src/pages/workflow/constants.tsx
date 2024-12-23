import { ComponentType } from "react";
import { BlockEnum } from "./types";
import StartNode from './nodes/start';
import EndNode from './nodes/end';

export const NodeComponentMap: Record<string, ComponentType<any>> = {
  [BlockEnum.Start]: StartNode,
  [BlockEnum.End]: EndNode,
}

export const NODE_WIDTH = 240
export const X_OFFSET = 60

export const NODE_WIDTH_X_OFFSET = NODE_WIDTH + X_OFFSET

export const START_INITIAL_POSITION = { x: 80, y: 282 }

export const CUSTOM_NODE = 'custom'
export const CUSTOM_EDGE = 'custom'