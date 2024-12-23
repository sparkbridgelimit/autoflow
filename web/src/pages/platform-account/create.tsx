import { z } from "zod";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
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
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useForm } from "@refinedev/react-hook-form";
import { supabaseClient } from "@/util";
// import init, { create_image_note } from "bindings";

const formSchema = z.object({
  platform: z.string().min(1, "平台名称是必填的"),
  name: z.string().min(1, "账号名称是必填的"),
  description: z.string().optional(),
  cookie: z.string().nullable(),
});

export const PlatformAccountCreate = () => {
  // 使用 useForm 初始化表单
  const form = useForm({
    refineCoreProps: {
      resource: "platform_account",
    },
    resolver: zodResolver(formSchema),
    defaultValues: {
      platform: "",
      name: "",
      description: "",
      cookie: "",
    },
  });

  const { control, handleSubmit, refineCore } = form;
  const { onFinish } = refineCore;

  const onSubmit = async (data: any) => {
    // 获取用户 ID
    const { data: res, error } = await supabaseClient.auth.getUser();
    if (error || !res) {
      console.error("无法获取用户信息", error);
      return;
    }

    // 合并用户 ID
    const formData = { ...data, user_id: res.user.id };
    // 提交数据
    onFinish(formData);
  };

  return (
    <div>
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-8">
          {/* 平台 */}
          <FormField
            control={control}
            name="platform"
            render={({ field }) => (
              <FormItem>
                <FormLabel>平台</FormLabel>
                <Select
                  onValueChange={field.onChange}
                  value={field.value || ""} // 确保 value 是字符串
                >
                  <FormControl>
                    <SelectTrigger>
                      <SelectValue placeholder="请选择平台" />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    <SelectGroup>
                      <SelectLabel>国内</SelectLabel>
                      <SelectItem value="xhs">小红书</SelectItem>
                      <SelectItem value="douyin">抖音</SelectItem>
                      <SelectItem value="bilibili">Bilibili</SelectItem>
                      <SelectItem value="wepublic">微信公众号</SelectItem>
                      <SelectItem value="wechannel">视频号</SelectItem>
                    </SelectGroup>
                    <SelectGroup>
                      <SelectLabel>海外</SelectLabel>
                      <SelectItem value="facebook">Facebook</SelectItem>
                      <SelectItem value="twitter">Twitter</SelectItem>
                      <SelectItem value="tiktok">TikTok</SelectItem>
                      <SelectItem value="youtube">Youtube</SelectItem>
                      <SelectItem value="instagram">Instagram</SelectItem>
                      <SelectItem value="threads">Threads</SelectItem>
                    </SelectGroup>
                  </SelectContent>
                </Select>
                <FormDescription>请选择社媒渠道</FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />

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

          {/* Cookie */}
          <FormField
            control={control}
            name="cookie"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Cookie</FormLabel>
                <FormControl>
                  <Input placeholder="请输入 Cookie 值（可选）" {...field} />
                </FormControl>
                <FormDescription>
                  此字段是可选的，用户的授权 Cookie。
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />

          <Button type="submit">提交</Button>
        </form>
      </Form>
    </div>
  );
};
