// platform-account/edit.tsx

import { z } from "zod";
import { useForm } from "@refinedev/react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const formSchema = z.object({
  platform: z.string().min(1, "平台名称是必填的"),
  name: z.string().min(1, "账号名称是必填的"),
  description: z.string().optional().default(""),
  cookie: z.string().nullable().default(""),
  fingerprint: z.object({
    userAgent: z.string().optional().default(""),
    language: z.string().optional().default(""),
    screenResolution: z.string().optional().default(""),
    timeZone: z.string().optional().default(""),
  }),
});

export const PlatformAccountEdit = () => {
  const form = useForm({
    resolver: zodResolver(formSchema),
    defaultValues: {
      platform: "",
      name: "",
      description: "",
      cookie: "",
      fingerprint: {
        userAgent: "",
        language: "",
        screenResolution: "",
        timeZone: "",
      },
    },
  });

  const {
    control,
    refineCore: { onFinish },
    handleSubmit,
  } = form;

  const onSubmit = async (data: any) => {
    // 提交数据
    onFinish(data);
  };

  return (
    <>
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-8">
          <Tabs defaultValue="account" className="w-[400px]">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="account">账号</TabsTrigger>
              <TabsTrigger value="fingerprint">浏览器指纹</TabsTrigger>
            </TabsList>
            <TabsContent value="account">
              {/* 账号名称 */}
              <FormField
                control={control}
                name="name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>账号名称</FormLabel>
                    <FormControl>
                      <Input placeholder="请输入账号名称" {...field} />
                    </FormControl>
                    <FormDescription>请输入账号名称。</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 描述 */}
              <FormField
                control={control}
                name="description"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>描述</FormLabel>
                    <FormControl>
                      <Input placeholder="请输入描述（可选）" {...field} />
                    </FormControl>
                    <FormDescription>
                      此字段是可选的，用于描述账号。
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </TabsContent>
            <TabsContent value="fingerprint">
              {/* User Agent */}
              <FormField
                control={control}
                name="fingerprint.userAgent"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>User Agent</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="请输入 User Agent（可选）"
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>
                      请输入浏览器的 User Agent。
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* Language */}
              <FormField
                control={control}
                name="fingerprint.language"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>语言</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="请输入浏览器语言（可选）"
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>例如：zh-CN, en-US。</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* Screen Resolution */}
              <FormField
                control={control}
                name="fingerprint.screenResolution"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>屏幕分辨率</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="请输入屏幕分辨率（可选）"
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>例如：1920x1080。</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* Time Zone */}
              <FormField
                control={control}
                name="fingerprint.timeZone"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>时区</FormLabel>
                    <FormControl>
                      <Input placeholder="请输入时区（可选）" {...field} />
                    </FormControl>
                    <FormDescription>例如：GMT+8。</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </TabsContent>
          </Tabs>

          <Button type="submit">提交</Button>
        </form>
      </Form>
    </>
  );
};

export default PlatformAccountEdit;
