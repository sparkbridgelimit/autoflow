import { Button } from "@/components/ui/button";
import { IResourceComponentsProps, useNavigation } from "@refinedev/core";
import { useTable } from "@refinedev/react-table";
import {
  ArrowLeftToLine,
  ArrowRightToLine,
  ChevronLeftIcon,
  ChevronRightIcon,
  LucideEdit,
  LucideEye,
} from "lucide-react";
import React, { useEffect, useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationPrevious,
  PaginationNext,
} from "@/components/ui/pagination";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@radix-ui/react-select";
import { iconMap } from "./app-icons";
import { invoke, list_browsers } from "@/util/invoke";
import { listen } from "@tauri-apps/api/event";
import Space from "@/components/ui/space";
import { useToast } from "@/hooks/use-toast";

interface IPlatformAccount {
  id: number;
  user_id: number;
  platform: string;
  name: string;
  description: string;
  cookie: string | null;
  online: boolean;
  create_time: string;
  update_time: string;
}

type JsonResult = {
  result: string;
};

type BrowserInfo = {
  id: string;
  user_data_dir: string;
  task: string;
  headless: boolean;
  auto_close: boolean;
};

type BrowserInfoList = BrowserInfo[];

export const PlatformAccountList: React.FC<IResourceComponentsProps> = () => {
  const { edit, show, create } = useNavigation();
  const [browserList = [], setBrowserList] = useState<BrowserInfoList>([]);
  const { toast } = useToast();

  const lanuch = async (account_id: string) => {
    try {
      await invoke("lanuch", { accountId: account_id.toString() });
      console.log(`Browser launched for account: ${account_id}`);
    } catch (error) {
      console.error(
        `Failed to launch browser for account: ${account_id}`,
        error
      );
    }
  };

  const columns = React.useMemo<ColumnDef<IPlatformAccount>[]>(
    () => [
      {
        id: "id",
        accessorKey: "id",
        header: "ID",
      },
      {
        id: "platform",
        accessorKey: "platform", // 匹配返回数据中的 "platform"
        header: "平台",
      },
      {
        id: "name",
        accessorKey: "name", // 匹配返回数据中的 "name"
        header: "名称",
      },
      {
        id: "description",
        accessorKey: "description", // 匹配返回数据中的 "description"
        header: "描述",
      },
      {
        id: "actions",
        accessorKey: "id",
        header: "操作",
        cell: function render({ getValue }) {
          return (
            <div className="flex flex-row flex-nowrap gap-0">
              <Button
                variant="ghost"
                size="icon"
                onClick={() => {
                  lanuch(getValue() as string);
                }}
              >
                <LucideEye size={16} />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                onClick={() => {
                  show("platform_account", getValue() as string);
                }}
              >
                <LucideEye size={16} />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                onClick={() => {
                  edit("platform_account", getValue() as string);
                }}
              >
                <LucideEdit size={16} />
              </Button>
            </div>
          );
        },
      },
    ],
    []
  );

  const {
    getRowModel,
    getState,
    setPageIndex,
    getPageCount,
    nextPage,
    previousPage,
    setPageSize,
  } = useTable({
    columns,
    refineCoreProps: {
      resource: "platform_account",
    },
  });

  const dynamicAccountList = getRowModel().rows.map((row) => row.original);

  useEffect(() => {
    const l = listen<JsonResult>("download-started", (event) => {
      console.log(`${event.payload.result}`);
    });
    return () => {
      l.then((unlisten) => unlisten());
    };
  }, []);

  // 读取已经运行的浏览器实例列表
  useEffect(() => {
    list_browsers().then((list) => {
      console.log(list);
      setBrowserList(list as BrowserInfoList);
    });
  }, [dynamicAccountList]);

  return (
    <>
      <div className="flex justify-between items-center mb-4">
        <h1 className="text-xl font-semibold">社媒管理</h1>
        <div>
          <Button onClick={() => create("platform_account")}>添加</Button>
        </div>
      </div>
      <div style={{ maxWidth: "100%", overflowX: "auto" }}>
        <div
          className={`faded-bottom no-scrollbar grid gap-4 overflow-auto pb-16 pt-4 
              ${
                dynamicAccountList.length === 0
                  ? "grid-cols-1"
                  : "md:grid-cols-2 lg:grid-cols-3"
              }`}
        >
          {dynamicAccountList.length > 0 ? (
            dynamicAccountList.map((app, index) => (
              <div
                key={index}
                className="rounded-lg border p-4 hover:shadow-md"
                onClick={() => edit("platform_account", app.id)}
              >
                <div className="mb-8 flex items-center justify-between">
                  <div
                    className={`flex size-10 items-center justify-center rounded-lg bg-muted p-2`}
                  >
                    {iconMap[app.platform] || (
                      <span className="text-gray-500">Logo</span>
                    )}
                  </div>
                  <div>
                    <Space>
                      <Button
                        variant="outline"
                        size="sm"
                        className={`${
                          app.online
                            ? "border border-blue-300 bg-blue-50 hover:bg-blue-100 dark:border-blue-700 dark:bg-blue-950 dark:hover:bg-blue-900"
                            : "border-gray-300 bg-gray-50 hover:bg-gray-100"
                        }`}
                        onClick={async (e) => {
                          e.stopPropagation();

                          const res = await invoke("launch_browser", {
                            id: String(app.id),
                            task: `${app.platform}_connect`,
                            payload: {},
                            headless: false,
                            autoClose: true,
                          });
                          console.log(res);
                        }}
                      >
                        {app.online ? "已授权" : "授权"}
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        className="border-gray-300 bg-gray-50 hover:bg-gray-100"
                        onClick={async (e) => {
                          e.stopPropagation();
                          await invoke("launch_browser", {
                            id: String(app.id),
                            task: "fingerprint_detect",
                            payload: {},
                          });
                        }}
                      >
                        指纹检测
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        className="border-gray-300 bg-gray-50 hover:bg-gray-100"
                        onClick={async (e) => {
                          e.stopPropagation();
                          const res = await invoke("get_user_cache", {
                            id: String(app.id),
                          });
                          console.log(res);
                        }}
                      >
                        缓存目录
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        className="border-gray-300 bg-gray-50 hover:bg-gray-100"
                        onClick={async (e) => {
                          e.stopPropagation();
                          const res = await invoke("clear_user_data_dir", {
                            id: String(app.id),
                          });
                          toast({
                            description: "成功删除",
                          })
                          console.log(res);
                        }}
                      >
                        清空缓存
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        className="border-gray-300 bg-gray-50 hover:bg-gray-100"
                        onClick={async (e) => {
                          e.stopPropagation();
                          const res = await invoke("close_browser", {
                            id: String(app.id),
                          });
                          console.log(res);
                        }}
                      >
                        关闭
                      </Button>
                    </Space>
                  </div>
                </div>
                <div>
                  <h2 className="mb-1 font-semibold">{app.name}</h2>
                  <p className="line-clamp-2 text-gray-500">
                    {app.description}
                  </p>
                </div>
              </div>
            ))
          ) : (
            <div className="text-center text-gray-500">
              No accounts available.
            </div>
          )}
        </div>
      </div>
      <Pagination className="mt-4">
        <PaginationContent>
          <PaginationItem>
            <PaginationLink onClick={() => setPageIndex(0)}>
              <ArrowLeftToLine className="h-4 w-4" />
            </PaginationLink>
          </PaginationItem>
          <PaginationItem>
            <PaginationPrevious onClick={() => previousPage()}>
              <ChevronLeftIcon className="h-4 w-4" />
            </PaginationPrevious>
          </PaginationItem>
          <PaginationItem>
            <PaginationNext onClick={() => nextPage()}>
              <ChevronRightIcon className="h-4 w-4" />
            </PaginationNext>
          </PaginationItem>
          <PaginationItem>
            <PaginationLink onClick={() => setPageIndex(getPageCount() - 1)}>
              <ArrowRightToLine className="h-4 w-4" />
            </PaginationLink>
          </PaginationItem>
          <div className="flex items-center space-x-6 lg:space-x-8">
            <div className="text-sm font-medium">
              {getState().pagination.pageIndex + 1} of {getPageCount()}
            </div>
            <div className="flex items-center space-x-2">
              <p className="text-sm font-medium">Rows per page</p>
              <Select
                value={`${getState().pagination.pageSize}`}
                onValueChange={(value) => setPageSize(Number(value))}
              >
                <SelectTrigger className="h-8 w-[70px]">
                  <SelectValue placeholder={getState().pagination.pageSize} />
                </SelectTrigger>
                <SelectContent side="top">
                  {[5, 10, 20, 30, 40, 50].map((pageSize) => (
                    <SelectItem key={pageSize} value={`${pageSize}`}>
                      {pageSize}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </PaginationContent>
      </Pagination>
    </>
  );
};
