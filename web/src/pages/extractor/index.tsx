import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { useState } from "react";

const platformRegex = {
  xiaohongshu: /^https?:\/\/(www\.)?xiaohongshu\.com/,
  twitter: /^https?:\/\/(www\.)?twitter\.com/,
  douyin: /^https?:\/\/(www\.)?douyin\.com/,
};

function identifyPlatform(url: string) {
  for (const [platform, regex] of Object.entries(platformRegex)) {
    if (regex.test(url)) {
      return platform;
    }
  }
  return "unknown";
}

const Extractor = () => {
  const [url, setUrl] = useState('')
  const [logs, setLogs] = useState<string[]>([])
  const [result, setResult] = useState<string | null>(null)
  const [platform, setPlatform] = useState<string | null>(null)

  const handleUrlChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newUrl = e.target.value
    setUrl(newUrl)
    setPlatform(identifyPlatform(newUrl))
  }

  const handleDownload = async () => {
    setLogs([])
    setResult(null)

    if (!url) {
      setLogs(['请输入有效的URL'])
      return
    }

    setLogs(prev => [...prev, `开始下载: ${url}`])
    setLogs(prev => [...prev, `识别平台: ${platform}`])

    // 这里应该是实际的下载逻辑
    // 为了演示,我们用setTimeout模拟下载过程
    await new Promise(resolve => setTimeout(resolve, 2000))

    setLogs(prev => [...prev, '下载完成'])
    setResult('下载的内容将在这里显示')
  }
  
  return (
    <>
      <div className=" mx-auto p-4 space-y-4">
        <Card>
          <CardHeader>
            <CardTitle>素材下载器</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex space-x-2">
              <Input
                placeholder="粘贴URL"
                value={url}
                onChange={handleUrlChange}
              />
              <Button onClick={handleDownload}>下载</Button>
            </div>
            {platform && <div>识别的平台: {platform}</div>}
            <div className="h-32">
              {logs.join("\n")}
            </div>
            {result && (
              <Card>
                <CardHeader>
                  <CardTitle>下载结果</CardTitle>
                </CardHeader>
                <CardContent>{result}</CardContent>
              </Card>
            )}
          </CardContent>
        </Card>
      </div>
    </>
  );
};

export default Extractor;
