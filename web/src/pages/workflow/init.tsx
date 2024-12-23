import { memo, useMemo } from "react";
import WorkflowCanvas from "./canvas";
import { ReactFlowProvider } from "reactflow";
import { useOne, useResourceParams } from "@refinedev/core";
import { initialEdges, initialNodes } from "./utils";
import { Graph } from "./types";

// 这个组件用于获取画布的数据, 同时做好各种预处理工作
function WorkflowInit() {
  console.log("WorkflowInit");

  // 获取资源参数和工作流数据
  const { id } = useResourceParams();
  const { data, isLoading } = useOne({
    id,
    resource: "workflow",
  });

  const workflow = data?.data;

  const { nodesData, edgesData } = useMemo(() => {
    if (workflow?.graph) {
      const graph = JSON.parse(workflow.graph) as Graph;
  
      // 分别初始化节点和边
      return {
        nodesData: initialNodes(graph.nodes, graph.edges),
        edgesData: initialEdges(graph.edges, graph.nodes),
      };
    }
  
    // 如果没有 workflow.graph，返回空数组
    return { nodesData: [], edgesData: [] };
  }, [workflow]);

  if (!workflow || isLoading) {
    return (
      <div className="flex justify-center items-center relative w-full h-full">
        Loading...
      </div>
    );
  }

  return (
    <ReactFlowProvider>
      <WorkflowCanvas nodes={nodesData} edges={edgesData} />
    </ReactFlowProvider>
  );
}

export default memo(WorkflowInit);