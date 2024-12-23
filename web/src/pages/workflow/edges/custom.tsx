import { memo, useMemo } from 'react';
import { BaseEdge, EdgeLabelRenderer, getBezierPath, Position } from 'reactflow';
import type { EdgeProps } from 'reactflow';
import { getEdgeColor } from '../utils';
import { NodeRunningStatus } from '../types';

const CustomEdge = ({
  id,
  data,
  sourceX,
  sourceY,
  targetX,
  targetY,
  selected,
}: EdgeProps) => {
  // 计算贝塞尔曲线路径
  const [edgePath, labelX, labelY] = getBezierPath({
    sourceX: sourceX - 8,
    sourceY,
    sourcePosition: Position.Right,
    targetX: targetX + 8,
    targetY,
    targetPosition: Position.Left,
    curvature: 0.16,
  });

  // 计算边颜色
  const stroke = useMemo(() => {
    return selected ? getEdgeColor(NodeRunningStatus.Running) : getEdgeColor();
  }, [selected]);

  return (
    <>
      {/* 渲染边 */}
      <BaseEdge
        id={id}
        path={edgePath}
        style={{
          stroke,
          strokeWidth: 2,
          opacity: data?._waitingRun ? 0.7 : 1,
        }}
      />

      {/* 渲染边标签 */}
      <EdgeLabelRenderer>
        <div
          style={{
            position: 'absolute',
            transform: `translate(-50%, -50%) translate(${labelX}px, ${labelY}px)`,
            pointerEvents: 'all',
            opacity: data?._waitingRun ? 0.7 : 1,
          }}
        >
          {/* 标签样式，这里可以根据需要扩展 */}
          <span className="bg-white px-2 py-1 rounded text-xs shadow">
            Edge Label
          </span>
        </div>
      </EdgeLabelRenderer>
    </>
  );
};

export default memo(CustomEdge);