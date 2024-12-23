import {
  Authenticated,
  AuthPage,
  ErrorComponent,
  Refine,
} from "@refinedev/core";

import routerBindings, {
  CatchAllNavigate,
  DocumentTitleHandler,
  NavigateToResource,
} from "@refinedev/react-router-v6";

import { dataProvider, liveProvider } from "@refinedev/supabase";

import "./App.css";
import { BrowserRouter, Outlet, Route, Routes } from "react-router-dom";
import authProvider from "./authProvider";
import { supabaseClient } from "./util";
import {
  PlatformAccountCreate,
  PlatformAccountEdit,
  PlatformAccountList,
  PlatformAccountShow,
} from "./pages/platform-account";
import AppShell from "./components/app-shell";
import { ArtWorkCreate, ArtWorkList } from "./pages/artwork";
import { ArtWorkEdit } from "./pages/artwork/edit";
import { ArtWorkShow } from "./pages/artwork/show";
import AssetList from "./pages/asset";
import Extractor from "./pages/extractor";
import { WorkflowList } from "./pages/workflow/list";
import WorkflowEdit from "./pages/workflow/edit";

function App() {
  return (
    <BrowserRouter>
      <Refine
        Title={({}) => (
          <div>
            <span>Custom Title</span>
          </div>
        )}
        dataProvider={dataProvider(supabaseClient)}
        liveProvider={liveProvider(supabaseClient)}
        authProvider={authProvider}
        routerProvider={routerBindings}
        resources={[
          {
            name: "workflow",
            list: "/workflow",
            create: "/workflow/create",
            edit: "/workflow/edit/:id",
            show: "/workflow/show/:id",
            meta: {
              canDelete: true,
            },
          },
          {
            name: "platform_account",
            list: "/platform-account",
            create: "/platform-account/create",
            edit: "/platform-account/edit/:id",
            show: "/platform-account/show/:id",
            meta: {
              canDelete: true,
            },
          },
          {
            name: "artworks",
            list: "/artworks",
            create: "/artworks/create",
            edit: "/artworks/edit/:id",
            show: "/artworks/show/:id",
            meta: {
              canDelete: true,
            },
          },
        ]}
        options={{
          syncWithLocation: true,
          warnWhenUnsavedChanges: true,
        }}
      >
        <Routes>
          <Route
            element={
              <Authenticated
                key="authenticated-inner"
                fallback={<CatchAllNavigate to="/login" />}
              >
                <AppShell>
                  <Outlet />
                </AppShell>
              </Authenticated>
            }
          >
            <Route
              index
              element={<NavigateToResource resource="platform_account" />}
            />
            <Route path="/workflow">
              <Route index element={<WorkflowList />} />
              <Route path="edit/:id" element={<WorkflowEdit />} />
            </Route>
            <Route path="/platform-account">
              <Route index element={<PlatformAccountList />} />
              <Route path="create" element={<PlatformAccountCreate />} />
              <Route path="edit/:id" element={<PlatformAccountEdit />} />
              <Route path="show/:id" element={<PlatformAccountShow />} />
            </Route>
            <Route path="/artworks">
              <Route index element={<ArtWorkList />} />
              <Route path="create" element={<ArtWorkCreate />} />
              <Route path="edit/:id" element={<ArtWorkEdit />} />
              <Route path="show/:id" element={<ArtWorkShow />} />
            </Route>
            <Route path="/assets">
              <Route index element={<AssetList />} />
            </Route>
            <Route path="/extractor">
              <Route index element={<Extractor />} />
            </Route>
            <Route path="*" element={<ErrorComponent />} />
          </Route>
          <Route
            element={
              <Authenticated key="authenticated-outer" fallback={<Outlet />}>
                <NavigateToResource />
              </Authenticated>
            }
          >
            <Route
              path="/login"
              element={
                <AuthPage
                  type="login"
                  renderContent={(content) => (
                    <div>
                      <p
                        style={{
                          padding: 10,
                          color: "#004085",
                          backgroundColor: "#cce5ff",
                          borderColor: "#b8daff",
                          textAlign: "center",
                        }}
                      >
                        email: info@refine.dev
                        <br /> password: refine-supabase
                      </p>
                      {content}
                    </div>
                  )}
                />
              }
            />
            <Route path="/register" element={<AuthPage type="register" />} />
            <Route
              path="/forgot-password"
              element={<AuthPage type="forgotPassword" />}
            />
          </Route>
        </Routes>
        <DocumentTitleHandler />
      </Refine>
    </BrowserRouter>
  );
}

export default App;
