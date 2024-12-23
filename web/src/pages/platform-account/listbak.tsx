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
import { ColumnDef, flexRender } from "@tanstack/react-table";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationPrevious,
  PaginationNext,
} from "@/components/ui/pagination";
import {
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
  Table,
} from "@/components/ui/table";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@radix-ui/react-select";
import { invoke } from "@tauri-apps/api/core";
import { facebook, ins, threads, twitter, xhs } from "./app-icons";

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

type IAccountList = IPlatformAccount[];

const accountList: IAccountList = [
  {
    id: 1,
    user_id: 101,
    platform: "xhs",
    name: "小红书",
    description: "Manage your Facebook marketing campaigns and pages.",
    cookie: null,
    online: true,
    create_time: "2024-01-01T10:00:00Z",
    update_time: "2024-11-01T12:00:00Z",
  },
  {
    id: 2,
    user_id: 102,
    platform: "Google",
    name: "Google Account",
    description: "Access your Google Analytics and Ads.",
    cookie: "auth_cookie_123",
    online: false,
    create_time: "2024-02-15T08:30:00Z",
    update_time: "2024-11-15T14:45:00Z",
  },
  {
    id: 3,
    user_id: 103,
    platform: "Twitter",
    name: "Twitter Account",
    description: "Manage your tweets and campaigns.",
    cookie: null,
    online: true,
    create_time: "2024-03-10T09:15:00Z",
    update_time: "2024-10-25T16:30:00Z",
  },
];

export const iconMap: Record<string, React.ReactElement> = {
  twitter,
  xhs,
  facebook,
  threads,
  ins,
};

export const PlatformAccountList: React.FC<IResourceComponentsProps> = () => {
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
  const { edit, show, create } = useNavigation();

  const {
    getHeaderGroups,
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

  return (
    <>
      <div className="p-4">
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-xl font-semibold">社媒账号</h1>
          <Button onClick={() => create("platform_account")}>添加</Button>
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
              dynamicAccountList.map((app) => (
                <div
                  key={app.name}
                  className="rounded-lg border p-4 hover:shadow-md"
                >
                  <div className="mb-8 flex items-center justify-between">
                    <div
                      className={`flex size-10 items-center justify-center rounded-lg bg-muted p-2`}
                    >
                      {iconMap[app.platform] || (
                        <span className="text-gray-500">Logo</span>
                      )}
                    </div>
                    <Button
                      variant="outline"
                      size="sm"
                      className={`${
                        app.online
                          ? "border border-blue-300 bg-blue-50 hover:bg-blue-100 dark:border-blue-700 dark:bg-blue-950 dark:hover:bg-blue-900"
                          : "border-gray-300 bg-gray-50 hover:bg-gray-100"
                      }`}
                    >
                      {app.online ? "Connected" : "Connect"}
                    </Button>
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
          <Table>
            <TableHeader>
              {getHeaderGroups().map((headerGroup) => (
                <TableRow key={headerGroup.id}>
                  {headerGroup.headers.map((header) => (
                    <TableHead key={header.id}>
                      {flexRender(
                        header.column.columnDef.header,
                        header.getContext()
                      )}
                    </TableHead>
                  ))}
                </TableRow>
              ))}
            </TableHeader>
            <TableBody>
              {getRowModel().rows.length > 0 ? (
                getRowModel().rows.map((row) => (
                  <TableRow key={row.id}>
                    {row.getVisibleCells().map((cell) => (
                      <TableCell key={cell.id}>
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext()
                        )}
                      </TableCell>
                    ))}
                  </TableRow>
                ))
              ) : (
                <TableRow>
                  <TableCell colSpan={columns.length} className="text-center">
                    No data available.
                  </TableCell>
                </TableRow>
              )}
            </TableBody>
          </Table>
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
                Page {getState().pagination.pageIndex + 1} of {getPageCount()}
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
