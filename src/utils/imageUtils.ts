export const fetchImageAsBytes = async (url: string): Promise<Uint8Array> => {
  const response = await fetch(url);
  const blob = await response.blob();
  const arrayBuffer = await blob.arrayBuffer();
  return new Uint8Array(arrayBuffer);
};
