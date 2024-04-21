const baseUrl = "";

export const fetchWithBaseUrl = async (path) => {
  const url = `${baseUrl}${path}`;
  const response = await fetch(url);
  return response;
};
