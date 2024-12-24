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
import React from "react";
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

export const WorkflowList: React.FC<IResourceComponentsProps> = () => {
  const { edit, create } = useNavigation();

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
      resource: "workflow",
    },
  });

  const dynamicAccountList = getRowModel().rows.map((row) => row.original);

  return (
    <>
      <div className="p-8">
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-xl font-semibold">工作流</h1>
          <div>
            <Button onClick={() => create("workflow")}>添加</Button>
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
                  onClick={() => edit("workflow", app.id)}
                >
                  <div className="mb-8 flex items-center justify-between">
                    <div
                      className={`flex size-10 items-center justify-center rounded-lg bg-muted p-2`}
                    ></div>
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
      </div>
    </>
  );
};
