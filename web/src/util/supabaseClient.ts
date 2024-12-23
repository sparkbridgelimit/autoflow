import { createClient } from "@refinedev/supabase";

const SUPABASE_URL = "https://nsojvcibsputmpugtrfa.supabase.co";
const SUPABASE_KEY =
  "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im5zb2p2Y2lic3B1dG1wdWd0cmZhIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MzI1MzU1NzcsImV4cCI6MjA0ODExMTU3N30.Jck55b9mYuboBbNaijZsFLhjdvD6JCcjQNnP-MWL-Fc";

export const supabaseClient = createClient(SUPABASE_URL, SUPABASE_KEY, {
  db: {
    schema: "public",
  },
  auth: {
    persistSession: true,
    detectSessionInUrl: true, // 允许在 URL 中检测和解析会话信息
  },
});
