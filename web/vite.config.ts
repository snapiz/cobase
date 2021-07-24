import { defineConfig, Plugin } from "vite";
import reactRefresh from "@vitejs/plugin-react-refresh";

let envInjectionFailed = false;

const npmPackageVersion = (): Plugin => {
  return {
    name: "vite-plugin-npm-package-version",
    config: (_, env) => {
      if (env) {
        const key = "import.meta.env.NPM_PACKAGE_VERSION";
        const val = JSON.stringify(process.env.npm_package_version);

        return { define: { [key]: val } };
      } else {
        envInjectionFailed = true;
      }
    },
    configResolved(config) {
      if (envInjectionFailed) {
        config.logger.warn(
          `[vite-plugin-package-version] import.meta.env.NPM_PACKAGE_VERSION was not injected due ` +
            `to incompatible vite version (requires vite@^2.0.0-beta.69).`
        );
      }
    },
  };
};

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: [{ find: /^(components|core|routes)\//, replacement: "/src/$1/" }],
  },
  server: {
    host: "cobase.timada.dev",
    port: 3000,
    hmr: {
      port: 443,
    },
  },
  plugins: [npmPackageVersion(), reactRefresh()],
});
