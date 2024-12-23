import { useState } from "react";

const StuffList = () => {
  const [materials, setMaterials] = useState<string[]>([]);

  // 添加素材
  const handleAddMaterial = () => {
    const newMaterial = window.prompt("请输入素材 URL：");
    if (newMaterial) {
      setMaterials((prev) => [...prev, newMaterial]);
    }
  };

  // 替换素材
  const handleReplaceMaterial = (index: number) => {
    const newMaterial = window.prompt("请输入新的素材 URL：");
    if (newMaterial) {
      setMaterials((prev) =>
        prev.map((item, i) => (i === index ? newMaterial : item))
      );
    }
  };

  // 删除素材
  const handleDeleteMaterial = (index: number) => {
    setMaterials((prev) => prev.filter((_, i) => i !== index));
  };

  return (
    <div className="flex space-x-4">
      {/* 添加按钮 */}
      <div
        onClick={handleAddMaterial}
        className="w-24 h-24 flex flex-col justify-center items-center border border-dashed border-gray-400 rounded-md cursor-pointer hover:border-gray-600"
      >
        <span className="text-2xl font-bold">+</span>
        <span>添加</span>
      </div>

      {/* 素材展示 */}
      {materials.map((material, index) => (
        <div key={index} className="relative w-24 h-24">
          {/* 素材图片 */}
          <img
            src={material}
            alt={`素材-${index}`}
            className="w-full h-full object-cover rounded-md"
          />

          {/* 替换和删除按钮 */}
          <div className="absolute inset-0 bg-black bg-opacity-50 flex justify-center items-end text-white rounded-md">
            <div className="flex space-x-2 p-2">
              <button
                onClick={() => handleReplaceMaterial(index)}
                className="bg-blue-500 text-xs px-2 py-1 rounded-md"
              >
                替换
              </button>
              <button
                onClick={() => handleDeleteMaterial(index)}
                className="bg-red-500 text-xs px-2 py-1 rounded-md"
              >
                删除
              </button>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};

export default StuffList;