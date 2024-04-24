const baseUrl = import.meta.env.DEV ? "http://127.0.0.1:7000" : "";

export const fetchWithBaseUrl = async (path) => {
  const url = `${baseUrl}${path}`;
  const response = await fetch(url);
  return response;
};
