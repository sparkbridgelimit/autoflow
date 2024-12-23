import { Button } from "@/components/ui/button";
import { IResourceComponentsProps, useNavigation } from "@refinedev/core";
import { useTable } from "@refinedev/react-table";
import React, { useRef } from "react";
import { ColumnDef } from "@tanstack/react-table";
import AutoSizer from "react-virtualized-auto-sizer";
import { VariableSizeGrid as Grid } from "react-window";

interface IArtwork {
  id: number;
  user_id: number;
  title: string;
  description: string;
  cover_url: string;
  cover_width: number;
  cover_height: number;
}

export const ArtWorkList: React.FC<IResourceComponentsProps> = () => {
  const columns = React.useMemo<ColumnDef<IArtwork>[]>(() => [], []);
  const { create } = useNavigation();

  const {
    getRowModel,
    getState,
    setPageIndex,
    getPageCount,
    nextPage,
   } = useTable({
    columns,
    refineCoreProps: {
      resource: "artworks",
      pagination: {
        mode: 'server'
      }
    },
  });


  const artWorkList = getRowModel().rows.map((row) => row.original);

  const columnWidth = 256;
  const gutterSize = 8;

  const isFetching = useRef(false);

  // 动态计算列数
  const getColumnCount = (containerWidth: number) =>
    Math.floor(containerWidth / (columnWidth + gutterSize));

  // 计算每个单元格的高度
  const getItemHeight = (artwork: IArtwork) => {
    const ratio = artwork.cover_height / artwork.cover_width;
    return columnWidth * ratio + 60 + gutterSize; // 图片高度 + 描述部分
  };
  const CellRenderer = ({
    columnIndex,
    rowIndex,
    style,
    data,
  }: {
    columnIndex: number;
    rowIndex: number;
    style: React.CSSProperties;
    data: IArtwork[];
  }) => {
    const itemIndex = rowIndex * data.columns + columnIndex;
    const artwork = data.items[itemIndex];

    if (!artwork) return null;

    const cardHeight =
      columnWidth * (artwork.cover_height / artwork.cover_width);

    return (
      <div
        style={{
          ...style,
          width: columnWidth,
          height: cardHeight + 40,
          margin: `${gutterSize / 2}px`,
        }}
        className="rounded-lg border shadow-md overflow-hidden"
      >
        <img
          src={artwork.cover_url}
          alt={artwork.title}
          className="w-full object-cover"
          style={{ height: cardHeight }}
        />
        <div className="p-2">
          <h2 className="text-sm font-semibold line-clamp-1">
            {artwork.title}
          </h2>
          <p className="text-xs text-gray-500 line-clamp-2">
            {artwork.description}
          </p>
        </div>
      </div>
    );
  };

  return (
    <>
      <div>
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-xl font-semibold">我的作品</h1>
          <Button onClick={() => create("artworks")}>添加</Button>
        </div>
        {artWorkList.length > 0 ? (
          <div style={{ height: "80vh", width: "100%" }}>
            <AutoSizer>
              {({ height, width }) => {
                const columnCount = getColumnCount(width);
                const rowCount = Math.ceil(artWorkList.length / columnCount);

                return (
                  <Grid
                    height={height}
                    width={width}
                    columnCount={columnCount}
                    rowCount={rowCount}
                    columnWidth={() => columnWidth + gutterSize}
                    rowHeight={(index) => {
                      const itemIndex = index * columnCount;
                      const artwork = artWorkList[itemIndex];
                      return artwork ? getItemHeight(artwork) : 0;
                    }}
                    overscanRowCount={2}
                    overscanColumnCount={1}
                    itemData={{
                      items: artWorkList,
                      columns: columnCount,
                    }}
                  >
                    {CellRenderer}
                  </Grid>
                );
              }}
            </AutoSizer>
          </div>
        ) : (
          <div className="text-center text-gray-500">暂无作品</div>
        )}
      </div>
    </>
  );
};
