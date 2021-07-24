import { init } from "@sentry/react";
import { Integrations } from "@sentry/tracing";

init({
  dsn: import.meta.env.VITE_SENTRY_DSN,
  release: `cobase-web@${import.meta.env.NPM_PACKAGE_VERSION}`,
  autoSessionTracking: true,
  integrations: [new Integrations.BrowserTracing()],

  // We recommend adjusting this value in production, or using tracesSampler
  // for finer control
  tracesSampleRate: 1.0,
});
