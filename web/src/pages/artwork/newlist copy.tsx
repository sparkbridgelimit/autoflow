import { Button } from "@/components/ui/button";
import { IResourceComponentsProps, useNavigation } from "@refinedev/core";
import React, { useState, useCallback, useEffect } from "react";
import AutoSizer from "react-virtualized-auto-sizer";
import { VariableSizeGrid as Grid } from "react-window";
import InfiniteLoader from "react-window-infinite-loader";
import { supabaseClient } from "@/util";

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
  const { create } = useNavigation();

  const pageSize = 5;
  const columnWidth = 256;
  const gutterSize = 16;

  // 状态：数据和是否还有更多
  const [data, setData] = useState<IArtwork[]>([]);
  const [hasMore, setHasMore] = useState(true);

  // 初始加载数据
  useEffect(() => {
    const initializeData = async () => {
      const { data: newData, error } = await supabaseClient
        .from("artworks")
        .select("*")
        .order("id", { ascending: true })
        .range(0, pageSize - 1);

      if (error) {
        console.error(error);
        return;
      }

      setData(newData);
      setHasMore(newData.length === pageSize); // 判断是否还有更多数据
    };

    initializeData();
  }, []);
  
  // 动态计算列数
  const getColumnCount = useCallback(
    (containerWidth: number) =>
      Math.floor(containerWidth / (columnWidth + gutterSize)),
    [columnWidth, gutterSize]
  );

  // 根据宽高比计算单元格高度
  const getItemHeight = useCallback(
    (artwork: IArtwork) => {
      const ratio = artwork.cover_height / artwork.cover_width;
      return columnWidth * ratio + 40 + gutterSize; // 图片高度 + 描述部分高度
    },
    [columnWidth, gutterSize]
  );

  // 判断某个项目是否已加载
  const isItemLoaded = useCallback(
    (index: number) => index < data.length,
    [data]
  );

  // 加载更多数据
  const loadMoreItems = useCallback(
    async () => {
      if (!hasMore) return;
      const page = Math.floor(data.length / pageSize);
      const { data: newData, error } = await supabaseClient
        .from("artworks")
        .select("*")
        .order("id", { ascending: true })
        .range(page * pageSize, (page + 1) * pageSize - 1);

      if (error) {
        console.error(error);
        return;
      }

      setData((prev) => [...prev, ...newData]);
      setHasMore(newData.length === pageSize);
    },
    [data.length, hasMore, pageSize]
  );

  // 渲染单元格
  const CellRenderer = useCallback(
    ({ columnIndex, rowIndex, style, data }: any) => {
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
          className="overflow-hidden"
        >
          <img
            src={artwork.cover_url}
            alt={artwork.title}
            className="w-full object-cover rounded-lg"
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
    },
    [columnWidth, gutterSize]
  );

  return (
    <div>
      <div className="flex justify-between items-center mb-4">
        <h1 className="text-xl font-semibold">我的作品</h1>
        <Button onClick={() => create("artworks")}>添加</Button>
      </div>
      {data.length > 0 ? (
        <div style={{ height: "80vh", width: "100%" }}>
          <AutoSizer>
            {({ height, width }) => {
              const columnCount = getColumnCount(width);
              const rowCount = Math.ceil(data.length / columnCount);

              return (
                <InfiniteLoader
                  isItemLoaded={isItemLoaded}
                  itemCount={data.length + (hasMore ? 1 : 0)}
                  loadMoreItems={loadMoreItems}
                >
                  {({ onItemsRendered, ref }) => (
                    <Grid
                      height={height}
                      width={width}
                      columnCount={columnCount}
                      rowCount={rowCount}
                      columnWidth={() => columnWidth + gutterSize}
                      rowHeight={(index) => {
                        const itemIndex = index * columnCount;
                        const artwork = data[itemIndex];
                        return artwork ? getItemHeight(artwork) : 0;
                      }}
                      overscanRowCount={2}
                      overscanColumnCount={1}
                      itemData={{
                        items: data,
                        columns: columnCount,
                      }}
                      onItemsRendered={({
                        visibleRowStartIndex,
                        visibleRowStopIndex,
                      }) => {
                        onItemsRendered({
                          overscanStartIndex: visibleRowStartIndex,
                          overscanStopIndex: visibleRowStopIndex,
                          visibleStartIndex: visibleRowStartIndex,
                          visibleStopIndex: visibleRowStopIndex,
                        });
                      }}
                      ref={ref}
                    >
                      {CellRenderer}
                    </Grid>
                  )}
                </InfiniteLoader>
              );
            }}
          </AutoSizer>
        </div>
      ) : (
        <div className="text-center text-gray-500">暂无作品</div>
      )}
    </div>
  );
};