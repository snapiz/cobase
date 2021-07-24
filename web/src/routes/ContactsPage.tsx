import React from "react";
import { FormattedMessage } from "react-intl";
import {
  Button,
  FormControl,
  FormLabel,
  FormErrorMessage,
  Input,
  Flex,
  Badge,
  Wrap,
  WrapItem,
} from "@chakra-ui/react";
import { useForm } from "react-hook-form";
import { useMutation, useQuery, useQueryClient } from "react-query";
import { useSubscribe } from "@timada/websocket";

import api from "core/api";
import type { AxiosResponse } from "axios";

interface Group {
  id: string;
  name: string;
}

const ContactsPage: React.FC = () => {
  const createGroup = useMutation<AxiosResponse<unknown>, unknown, unknown>(
    (group) => api.post("/groups/create", group)
  );
  const {
    handleSubmit,
    formState: { errors },
    register,
  } = useForm();

  const onSubmit = (values: unknown) => {
    createGroup.mutate(values);
  };

  return (
    <Flex mt="4" direction="column" alignItems="center">
      <Flex as="form" onSubmit={handleSubmit(onSubmit)} alignItems="flex-end">
        <FormControl isInvalid={errors.name}>
          <FormLabel htmlFor="name">
            <FormattedMessage
              id="contacts.form.name.label"
              defaultMessage="Group name"
            />
          </FormLabel>
          <Input
            autoComplete="off"
            {...register("name", {
              required: true,
              minLength: 3,
              maxLength: 15,
            })}
          />
          <FormErrorMessage>
            {errors.name && errors.name.message}
          </FormErrorMessage>
        </FormControl>
        <Button
          ml={4}
          colorScheme="teal"
          isLoading={createGroup.isLoading}
          type="submit"
        >
          <FormattedMessage id="contacts.form.submit" defaultMessage="Create" />
        </Button>
      </Flex>
      <GroupList />
    </Flex>
  );
};

const GroupList: React.FC = () => {
  const { isLoading, isError, data, error } = useQuery("groups", () =>
    api.get<Group[]>("/groups").then((resp) => resp.data)
  );
  const queryClient = useQueryClient();

  useSubscribe("group", (data: Group) => {
    queryClient.setQueryData<Group[] | undefined>("groups", (old) =>
      old ? [...old, data] : old
    );
  });

  if (isLoading) {
    return <div>Loading groups...</div>;
  }

  if (isError) {
    return <div>{(error as Error).message}</div>;
  }

  return (
    <Wrap maxW="300px" mt="4">
      {data?.map((group) => (
        <WrapItem key={group.id}>
          <Badge variant="outline" colorScheme="green">
            {group.name}
          </Badge>
        </WrapItem>
      ))}
    </Wrap>
  );
};

export default ContactsPage;
