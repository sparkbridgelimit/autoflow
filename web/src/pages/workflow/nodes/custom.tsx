import { memo } from "react"
import type { NodeProps } from 'reactflow'
import { NodeComponentMap } from "../constants"

const CustomNode = (props: NodeProps) => {
  const nodeData = props.data
  const NodeComponent = NodeComponentMap[nodeData.type]

  return (
    <>
      {/* <BaseNode { ...props }> */}
        <NodeComponent />
      {/* </BaseNode> */}
    </>
  )
}
CustomNode.displayName = 'CustomNode'

export default memo(CustomNode);