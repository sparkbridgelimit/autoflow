import React, { useState, useEffect, useCallback } from "react";
import Masonry from "react-masonry-css";
import { Button } from "@/components/ui/button";
import { IResourceComponentsProps, useNavigation } from "@refinedev/core";
import { supabaseClient } from "@/util";
import { throttle } from "lodash";
import { useInView } from "react-intersection-observer";

interface IArtwork {
  id: number;
  user_id: number;
  title: string;
  description: string;
  cover_url: string;
  cover_width: number;
  cover_height: number;
}

const LazyImage: React.FC<{ src: string; alt: string; height: number }> = ({
  src,
  alt,
  height,
}) => {
  const [isLoaded, setIsLoaded] = useState(false);
  const { ref, inView } = useInView({
    triggerOnce: true, // 仅触发一次
    threshold: 0.1, // 图片进入视口 10% 时触发
  });

  return (
    <div
      ref={ref}
      className="relative bg-gray-200"
      style={{ height, width: "100%" }}
    >
      {inView && (
        <img
          src={src}
          alt={alt}
          className={`w-full object-cover transition-opacity duration-500 ${
            isLoaded ? "opacity-100" : "opacity-0"
          }`}
          style={{ height }}
          onLoad={() => setIsLoaded(true)}
        />
      )}
    </div>
  );
};

export const ArtWorkList: React.FC<IResourceComponentsProps> = () => {
  const { create } = useNavigation();

  const pageSize = 10; // 每页加载数量
  const [data, setData] = useState<IArtwork[]>([]);
  const [hasMore, setHasMore] = useState(true);
  const [isLoading, setIsLoading] = useState(false);

  // 初始加载数据
  const loadMoreData = useCallback(async () => {
    if (isLoading || !hasMore) return; // 避免重复加载
    setIsLoading(true);

    const page = Math.floor(data.length / pageSize);
    const { data: newData, error } = await supabaseClient
      .from("artworks")
      .select("*")
      .order("id", { ascending: true })
      .range(page * pageSize, (page + 1) * pageSize - 1);

    if (error) {
      console.error(error);
      setIsLoading(false);
      return;
    }

    setData((prev) => [...prev, ...newData]);
    setHasMore(newData.length === pageSize); // 如果新数据不足 pageSize，则认为没有更多数据
    setIsLoading(false);
  }, [data.length, hasMore, isLoading, pageSize]);

  // 监听滚动触底加载更多
  useEffect(() => {
    const handleScroll = throttle(() => {
      const scrollHeight = document.documentElement.scrollHeight;
      const scrollTop = document.documentElement.scrollTop;
      const clientHeight = document.documentElement.clientHeight;

      if (
        scrollHeight - scrollTop <= clientHeight * 1.5 &&
        hasMore &&
        !isLoading
      ) {
        loadMoreData();
      }
    }, 200); // 每 200ms 触发一次

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, [loadMoreData, hasMore, isLoading]);

  // Masonry 布局断点配置
  const breakpointColumnsObj = {
    default: 4, // 默认 4 列
    1100: 3, // 宽度小于 1100px 时 3 列
    700: 2, // 宽度小于 700px 时 2 列
    500: 1, // 宽度小于 500px 时 1 列
  };

  useEffect(() => {
    if (data.length === 0) {
      loadMoreData();
    }
  }, [data.length, loadMoreData]);

  return (
    <div>
      <div className="flex justify-between items-center mb-4">
        <h1 className="text-xl font-semibold">我的作品</h1>
        <Button onClick={() => create("artworks")}>添加</Button>
      </div>

      {data.length > 0 ? (
        <Masonry
          breakpointCols={breakpointColumnsObj}
          className="flex gap-4" // Masonry 容器样式
          columnClassName="masonry-column" // 每列的样式
        >
          {data.map((artwork) => {
            const ratio = artwork.cover_height / artwork.cover_width;
            const cardHeight = 256 * ratio;

            return (
              <div
                key={artwork.id}
                className="overflow-hidden rounded-lg border shadow-md"
                style={{ marginBottom: "16px" }}
              >
                {/* <img
                  src={artwork.cover_url}
                  alt={artwork.title}
                  className="w-full object-cover"
                  style={{ height: cardHeight }}
                /> */}
                <LazyImage
                  src={artwork.cover_url}
                  alt={artwork.title}
                  height={cardHeight}
                />
                <div className="p-2">
                  <h2 className="text-sm font-semibold line-clamp-1">
                    {artwork.id}
                  </h2>
                  <p className="text-xs text-gray-500 line-clamp-2">
                    {artwork.description}
                  </p>
                </div>
              </div>
            );
          })}
        </Masonry>
      ) : (
        <div className="text-center text-gray-500">暂无作品</div>
      )}

      {isLoading && (
        <div className="text-center text-gray-500 mt-4">加载中...</div>
      )}
      {!hasMore && (
        <div className="text-center text-gray-500 mt-4">没有更多数据了</div>
      )}
    </div>
  );
};
