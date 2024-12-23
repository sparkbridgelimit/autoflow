import { FC, memo, useRef } from "react";

import ReactFlow, {
  Background,
  ReactFlowProvider,
  SelectionMode,
  useEdgesState,
  useNodesState,
  useOnViewportChange,
  useReactFlow,
  useStoreApi,
} from "reactflow";
import type { Viewport } from "reactflow";
import type { Edge, Node } from "./types";
import CustomNode from './nodes/custom';
import CustomEdge from './edges/custom';

const nodeTypes = {
  'custom': CustomNode,
}
const edgeTypes = {
  'custom': CustomEdge,
}

type WorkflowProps = {
  nodes: Node[];
  edges: Edge[];
  viewport?: Viewport;
};

// 这里接受现有数据进行渲染
const WorkflowCanvas: FC<WorkflowProps> = ({
  nodes: originalNodes,
  edges: originalEdges,
  viewport,
}) => {
  const workflowContainerRef = useRef<HTMLDivElement>(null);
  const reactflow = useReactFlow()
  const [nodes, setNodes] = useNodesState(originalNodes)
  const [edges, setEdges] = useEdgesState(originalEdges)
  const store = useStoreApi();

  console.log(nodes, edges);

  return (
    <>
      <div
        ref={workflowContainerRef}
        className="relative w-full min-w-[960px] h-full"
      >
        <ReactFlow
          nodeTypes={nodeTypes}
          edgeTypes={edgeTypes}
          nodes={nodes}
          edges={edges}
          // onNodeDragStart={handleNodeDragStart}
          // onNodeDrag={handleNodeDrag}
          // onNodeDragStop={handleNodeDragStop}
          // onNodeMouseEnter={handleNodeEnter}
          // onNodeMouseLeave={handleNodeLeave}
          // onNodeClick={handleNodeClick}
          // onNodeContextMenu={handleNodeContextMenu}
          // onConnect={handleNodeConnect}
          // onConnectStart={handleNodeConnectStart}
          // onConnectEnd={handleNodeConnectEnd}
          // onEdgeMouseEnter={handleEdgeEnter}
          // onEdgeMouseLeave={handleEdgeLeave}
          // onEdgesChange={handleEdgesChange}
          // onSelectionStart={handleSelectionStart}
          // onSelectionChange={handleSelectionChange}
          // onSelectionDrag={handleSelectionDrag}
          // onPaneContextMenu={handlePaneContextMenu}
          // connectionLineComponent={CustomConnectionLine}
          // connectionLineContainerStyle={{ zIndex: ITERATION_CHILDREN_Z_INDEX }}
          defaultViewport={viewport}
          multiSelectionKeyCode={null}
          deleteKeyCode={null}
          // nodesDraggable={!nodesReadOnly}
          // nodesConnectable={!nodesReadOnly}
          // nodesFocusable={!nodesReadOnly}
          // edgesFocusable={!nodesReadOnly}
          // panOnDrag={controlMode === ControlMode.Hand && !workflowReadOnly}
          // zoomOnPinch={!workflowReadOnly}
          // zoomOnScroll={!workflowReadOnly}
          // zoomOnDoubleClick={!workflowReadOnly}
          // isValidConnection={isValidConnection}
          // selectionKeyCode={null}
          // selectionMode={SelectionMode.Partial}
          // selectionOnDrag={
          //   controlMode === ControlMode.Pointer && !workflowReadOnly
          // }
          minZoom={0.25}
        >
          <Background
            gap={[14, 14]}
            size={2}
            className="bg-workflow-canvas-workflow-bg"
            color="#ccc"
          />
        </ReactFlow>
      </div>
    </>
  );
};

export default memo(WorkflowCanvas);
