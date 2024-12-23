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
import { useForm } from "@refinedev/react-hook-form";
import { supabaseClient } from "@/util";
import { Textarea } from "@/components/ui/textarea";
import { useState } from "react";

const formSchema = z.object({
  title: z.string().min(1, "作品标题是必填的"),
  description: z.string().optional(),
  content: z.array(z.string()).min(1, "至少需要一个素材"), // 修改为数组校验
  status: z.enum(["draft", "ready", "published"]).default("draft"),
  tags: z.array(z.string()).optional(),
});

type LocalFile = {
  url: string; // 文件的本地路径
  type: string; // 文件类型
};

export const ArtWorkCreate = () => {
  const [localFiles, setLocalFiles] = useState<LocalFile[]>([]); // 修正数据结构
  // 初始化表单
  const form = useForm({
    refineCoreProps: {
      resource: "artworks",
    },
    resolver: zodResolver(formSchema),
    defaultValues: {
      title: "",
      description: "",
      content: [] as string[],
      status: "draft",
      tags: [],
    },
  });

  const { control, handleSubmit, setValue, refineCore } = form;
  const { onFinish } = refineCore;

  const onSubmit = async (data: any) => {
    const { data: res, error } = await supabaseClient.auth.getUser();
    if (error || !res) {
      console.error("无法获取用户信息", error);
      return;
    }

    // 将 localFiles 转换为 JSON 格式
    const content = localFiles.map((file) => file.url); // 提取所有本地路径
    console.log(content);
    // 合并用户 ID 并提交数据
    const formData = {
      ...data,
      content,
      cover_url: 'https://sns-webpic-qc.xhscdn.com/202411282256/4209df17e8c25e05adbd8ab4b4700d4e/1040g2sg319t4g1i1ng705pk2ln2hounsrjr49to!nd_dft_wlteh_webp_3',
      cover_width: 256,
      cover_height: 313,
      user_id: res.user.id };
    console.log(formData);
    onFinish(formData);
  };

  const onFileDelete = (index: number) => {
    setLocalFiles((prev) => {
      const updatedFiles = prev.filter((_, i) => i !== index);
      // 更新表单的 content 字段
      setValue(
        "content",
        updatedFiles.map((file) => file.url)
      );
      return updatedFiles;
    });
  };

  const onFileUpload = (files: FileList | null) => {
    if (!files) return;

    const paths = Array.from(files).map((file) => ({
      url: URL.createObjectURL(file), // 本地路径
      type: file.type, // 文件类型
    }));

    setLocalFiles((prev) => {
      const updatedFiles = [...prev, ...paths];
      // 更新表单的 content 字段
      setValue(
        "content",
        updatedFiles.map((file) => file.url)
      );
      return updatedFiles;
    });
  };

  return (
    <div>
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-8">
          {/* 作品标题 */}
          <FormField
            control={control}
            name="title"
            render={({ field }) => (
              <FormItem>
                <FormLabel>作品标题</FormLabel>
                <FormControl>
                  <Input placeholder="请输入作品标题" {...field} />
                </FormControl>
                <FormDescription>请输入作品标题。</FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* 作品描述 */}
          <FormField
            control={control}
            name="description"
            render={({ field }) => (
              <FormItem>
                <FormLabel>描述</FormLabel>
                <FormControl>
                  <Textarea placeholder="请输入作品描述（可选）" {...field} />
                </FormControl>
                <FormDescription>简要描述作品内容。</FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* 素材 */}
          <FormField
            control={control}
            name="content"
            render={({ field }) => (
              <FormItem>
                <FormLabel>素材</FormLabel>
                <FormControl>
                  {/* 上传区域和预览区域 */}
                  <div className="flex space-x-4 items-start">
                    {/* 上传区域 */}
                    <div>
                      <input
                        type="file"
                        multiple
                        accept="image/*,video/*"
                        onChange={(e) => onFileUpload(e.target.files)}
                        className="hidden"
                        id="file-upload"
                      />
                      <label
                        htmlFor="file-upload"
                        className="w-24 h-24 flex flex-col justify-center items-center border border-dashed border-gray-400 rounded-md cursor-pointer hover:border-gray-600"
                      >
                        <span className="text-2xl font-bold">+</span>
                        <span>添加</span>
                      </label>
                    </div>

                    {/* 素材预览 */}
                    <div className="flex flex-wrap gap-4 max-w-full">
                      {localFiles.map((file, index) => (
                        <div
                          key={index}
                          className="relative w-24 h-24 border rounded-md overflow-hidden"
                        >
                          {/* 根据文件类型渲染 */}
                          {file.type.startsWith("image/") ? (
                            <img
                              src={file.url}
                              alt={`素材-${index}`}
                              className="w-full h-full object-cover"
                            />
                          ) : file.type.startsWith("video/") ? (
                            <video
                              src={file.url}
                              controls
                              className="w-full h-full object-cover"
                              preload="metadata"
                            />
                          ) : (
                            <div className="w-full h-full flex items-center justify-center text-red-500">
                              不支持的格式
                            </div>
                          )}
                          {/* 删除按钮 */}
                          <button
                            type="button"
                            onClick={() => onFileDelete(index)}
                            className="absolute top-1 right-1 bg-red-500 text-white rounded-full w-5 h-5 text-xs flex items-center justify-center"
                          >
                            ×
                          </button>
                        </div>
                      ))}
                    </div>
                  </div>
                </FormControl>
                <FormDescription>
                  请上传图片或视频（支持多个）。
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
