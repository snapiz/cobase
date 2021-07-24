import React from "react";
import { Routes, Route, Navigate, Link } from "react-router-dom";
import {
  Layout,
  LayoutNavigation,
  LazyIntlProvider,
  useQueryNavbarData,
} from "@timada/ui";
import { ErrorBoundary, Scope } from "@sentry/react";
import {
  Alert,
  AlertDescription,
  AlertIcon,
  Icon,
  Spinner,
} from "@chakra-ui/react";
import { FormattedMessage } from "react-intl";
import { FaUsers } from "react-icons/fa";
import { useKeycloak } from "@react-keycloak/web";
import { ROUTES } from "./constants";
import ContactsPage from "./ContactsPage";

const translations = {
  fr: () => import("./app_fr"),
};

const beforeCapture = (scope: Scope) => {
  scope.setTag("application", "web");
};

const RootRoutes: React.FC = () => {
  return (
    <ErrorBoundary beforeCapture={beforeCapture}>
      <React.Suspense fallback={<Spinner />}>
        <LazyIntlProvider ckey="app" translations={translations}>
          <Container />
        </LazyIntlProvider>
      </React.Suspense>
    </ErrorBoundary>
  );
};

interface User {
  name?: string;
  preferred_username?: string;
}

const Container: React.FC = () => {
  const { keycloak } = useKeycloak();
  const { data, isLoading } = useQueryNavbarData("navbarData", {
    url: import.meta.env.VITE_NAVBAR_DATA_URL,
  });

  if (isLoading) {
    return <Spinner />;
  }

  if (!data) {
    return (
      <Alert status="error">
        <AlertIcon />
        <AlertDescription>
          <FormattedMessage
            id="layout.navbar.alertDescriptionError"
            defaultMessage="Failed to load navbar data"
          />
        </AlertDescription>
      </Alert>
    );
  }

  const navigations: LayoutNavigation[] = [
    {
      items: [
        {
          icon: <Icon as={FaUsers} />,
          text: (
            <FormattedMessage
              id="layout.sidebar.contacts"
              defaultMessage="Contacts"
            />
          ),
          link: ROUTES.CONTACTS,
        },
      ],
    },
  ];

  const user = keycloak.tokenParsed as User;

  return (
    <Layout
      logo={<Link to="/">Cobase</Link>}
      navigations={navigations}
      navbarData={data}
      user={{
        name: user.name || user.preferred_username || "John doe",
      }}
    >
      <React.Suspense fallback={<Spinner />}>
        <Routes>
          <Route path={ROUTES.CONTACTS} element={<ContactsPage />} />
          <Route path="*" element={<Navigate to={ROUTES.CONTACTS} replace />} />
        </Routes>
      </React.Suspense>
    </Layout>
  );
};

export default RootRoutes;
