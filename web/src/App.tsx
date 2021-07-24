import React from "react";
import { BrowserRouter as Router } from "react-router-dom";
import { ReactKeycloakProvider } from "@react-keycloak/web";
import { Timada } from "@timada/ui";
import { WebSocketProvider } from "@timada/websocket";
import { QueryClientProvider } from "react-query";
import { captureException } from "@sentry/react";

import keycloak from "core/keycloak";
import queryClient from "core/query";
import RootRoutes from "routes/RootRoutes";

const App: React.FC = () => {
  return (
    <Timada>
      <ReactKeycloakProvider
        authClient={keycloak}
        initOptions={{ onLoad: "login-required" }}
        LoadingComponent={<></>}
      >
        <WebSocketProvider
          url={import.meta.env.VITE_WS_URL}
          onError={captureException}
        >
          <QueryClientProvider client={queryClient}>
            <Router>
              <RootRoutes />
            </Router>
          </QueryClientProvider>
        </WebSocketProvider>
      </ReactKeycloakProvider>
    </Timada>
  );
};

export default App;
